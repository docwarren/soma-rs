// Copyright 2026 Seqa23
//
// Author: Andrew Warren
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::ops::Range;
use crate::api::search_error::SearchError;
use crate::api::search_options::SearchOptions;
use crate::api::search_result::SearchResult;
use crate::bigwig::bigwig_data::BigwigData;
use crate::bigwig::index::bigwig_index::BigwigIndex;
use crate::bigwig::index::chr_tree::BigwigChrTree;
use crate::bigwig::index::r_tree::RTree;
use crate::bigwig::index::r_tree::overlaps::Overlaps;
use crate::bigwig::index::r_tree::r_tree_leaf::RTreeLeaf;
use crate::bigwig::index::util::get_zoom_strings;
use crate::bigwig::index::zoom_header::ZoomHeader;
use crate::bigwig::zoom_data::ZoomData;
use crate::codecs::decompress_auto;
use crate::stores::StoreService;

pub fn get_data_strings(bytes: &[u8], chr_tree: &BigwigChrTree, options: &SearchOptions) -> Vec<String> {
    let mut str_array = Vec::new();
    let mut offset = 0;
    let chrom_id = chr_tree.get_chromosome_id(&options.chromosome).unwrap_or(0);

    while offset < bytes.len() + BigwigData::SIZE {
        if let Ok(bigwig_data) = BigwigData::from_bytes(&bytes[offset..], chr_tree) {
            offset += bigwig_data.size;
            for data_point in &bigwig_data.data {
                if data_point.overlaps(chrom_id, chrom_id, options.begin, options.end) {
                    str_array.push(format!("{}", data_point));
                }
            }

        } else {
            break; // Exit if we can't parse a complete RTreeLeaf
        }
    }

    str_array
}
/// Converts raw data bytes into a vector of strings, processing each line according to the search options.
/// # Arguments:
/// * `data` - A vector of bytes representing the raw data to be processed.
/// * `options` - A `SearchOptions` struct containing the search parameters such as output format,
///  whether to include headers, and the range of positions to consider.
/// # Returns:
/// * A vector of strings containing the processed lines, which may include headers.
pub fn data_to_lines(bytes: Vec<Vec<u8>>, is_zoom: bool, chr_tree: &BigwigChrTree, options: &SearchOptions) -> Vec<String> {

    let mut str_array: Vec<String> = if options.include_header {
        if is_zoom {
            vec!(ZoomData::COLUMNS.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\t"))
        } else {
            vec!(BigwigData::COLUMNS.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\t"))
        }
    } else {
        Vec::new()
    };

    for block in bytes {
        let strings = if is_zoom {
            get_zoom_strings(block, chr_tree, options)
        } else {
            get_data_strings(&block.to_vec(), chr_tree, options)
        };
        str_array.extend(strings);
    }
    str_array
}

/// Searches for data in a bigwig file based on the provided search options.
/// Returns a vector of strings containing the results, which may include headers.
/// # Arguments:
/// * `options` - A `SearchOptions` struct containing the search parameters such as file paths, chromosome,
///  start and end positions, output format, and whether to include headers or only headers.
/// # Returns:
/// * A Result containing a vector of strings with the search results, or an error message if the search fails.
pub async fn bigwig_search(
    store_service: &StoreService,
    options: &SearchOptions,
) -> Result<SearchResult, SearchError> {
    let mut result = SearchResult::new();

    let index = match &options.bigwig_index {
        Some(index) => {
            index
        },
        _ => &BigwigIndex::new(store_service, &options.file_path).await?
    };
    result.bigwig_index = Some(index.clone());

    let zoom_header = index.get_zoom_header(options);
    let is_compressed = index.header.is_compressed();

    let chr_id = match index.chromosome_tree.get_chromosome_id(&options.chromosome) {
        Some(id) => id,
        _ => {
            return Err(SearchError::InvalidRequest {
                reason: "Chromosome not found".to_string(),
                requested: options.chromosome.to_string()
            });
        }
    };

    let index_offset = get_index_begin(index, zoom_header);
    let index_end = get_index_end(store_service, index, zoom_header).await?;
    let r_tree = RTree::from_file(store_service, &options.file_path, index_offset..index_end).await?;
    let leaves = r_tree.get_overlapping_leaves(chr_id, options.begin, options.end);

    if leaves.is_empty() {
        return Ok(result);
    }

    let range = get_range_from_leaves(&leaves);
    let data = index.get_data(store_service, &range, &options.file_path).await?;
    let mut decompressed_blocks: Vec<Vec<u8>> = Vec::new();

    for leaf in leaves {
        let begin = (leaf.data_offset - range.start) as usize;
        let end = begin + leaf.data_size as usize;
        let compressed = data[begin..end].to_vec();

        if is_compressed {
            let decompressed_block = decompress_auto(&compressed)
                .map_err(|e| SearchError::CodecError {
                    path: options.file_path.to_string(),
                    source: e
                })?;
            decompressed_blocks.push(decompressed_block);
        } else {
            decompressed_blocks.push(compressed);
        };
    }
    let is_zoom = zoom_header.is_some();
    result.lines = data_to_lines(decompressed_blocks, is_zoom, &index.chromosome_tree, options);
    Ok(result)
}

fn get_index_begin(index: &BigwigIndex, zoom_header: Option<&ZoomHeader>) -> u64 {
    match zoom_header {
        Some(zoom_header) => zoom_header.index_offset,
        None => index.header.full_index_offset,
    }
}

async fn get_index_end(
    store_service: &StoreService,
    index: &BigwigIndex,
    zoom_header: Option<&ZoomHeader>,
) -> Result<u64, SearchError> {
    match zoom_header {
        Some(zoom_header) => index.get_end_for_zoom_header(store_service, zoom_header, &index.file_path).await,
        None => index.get_full_index_end(store_service, &index.file_path).await,
    }
}

pub fn get_range_from_leaves(leaves: &[&RTreeLeaf]) -> Range<u64> {
    let begin = leaves
        .iter()
        .map(|l| l.data_offset)
        .min()
        .unwrap_or(u64::MAX);
    let end = leaves
        .iter()
        .map(|l| l.data_offset + l.data_size)
        .max()
        .unwrap_or(u64::MAX);
    begin..end
}
