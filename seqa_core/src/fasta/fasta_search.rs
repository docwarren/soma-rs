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

use crate::api::search_options::SearchOptions;
use crate::api::search_result::SearchResult;
use crate::fasta::fasta_error::FastaError;
use crate::stores::StoreService;
use std::ops::Range;
use crate::fasta::fai::FaiIndex;

/// Searches for data in a FASTA file based on the provided search options.
/// Returns a vector of strings containing the results.
/// # Arguments:
/// * `options` - A `SearchOptions` struct containing the search parameters such as file
///   paths, chromosome, start and end positions, output format, and whether to include headers or only headers.
/// # Returns:
/// * A Result containing a vector of strings with the search results, or an error message if the search fails.
pub async fn fasta_search(
    store_service: &StoreService,
    options: &SearchOptions,
) -> Result<SearchResult, FastaError> {
    let mut result = SearchResult::new();

    if options.end - options.begin > 250_000_000 {
        return Err(FastaError::InvalidRequest {
            requested: format!("{}:{}-{}", options.chromosome, options.begin, options.end),
            reason: "Maximum query size for fasta files 250MBases".to_string()
        });
    }

    let index = match &options.fasta_index {
        Some(index) => index,
        None => &FaiIndex::from_file(store_service, &options.index_path)
            .await
            .map_err(|e| FastaError::IndexError {
                source: e,
                file_path: options.index_path.to_string()
            })?
    };
    result.fasta_index = Some(index.clone());

    let byte_range: Range<u64> = index.get_offsets(options).map_err(|e| FastaError::IndexError {
        source: e,
        file_path: options.index_path.to_string()
    })?;

    let bytes = store_service.get_range(&options.file_path, byte_range)
        .await
        .map_err(|e| FastaError::FetchError {
            file_path: options.file_path.to_string(),
            source: e
        })?;

    let line_string = String::from_utf8(bytes).map_err(|e| FastaError::ParseError {
        file_path: options.file_path.to_string(),
        source: e
    })?;
    result.lines = line_string.lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    Ok(result)
}
