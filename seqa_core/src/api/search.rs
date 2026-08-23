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
use futures::TryStreamExt;
use futures::future::try_join_all;
use object_store::ObjectStore;

use crate::api::output_format::OutputFormat;
use crate::api::search_error::SearchError;
use crate::api::search_options::SearchOptions;
use crate::api::search_result::SearchResult;
use crate::bam::bam_search::bam_search;
use crate::bigwig::bigbed_search::bigbed_search;
use crate::bigwig::bigwig_search::bigwig_search;
use crate::codecs::bgzip;
use crate::codecs::codec_error::CodecError;
use crate::fasta::fasta_search::fasta_search;
use crate::indexes::chunk::Chunk;
use crate::stores::error::StoreError;
use crate::stores::StoreService;
use crate::tabix::tabix_search::tabix_search;

pub async fn chunk_to_stream(
    chunk: &Chunk,
    store: &dyn ObjectStore,
    path: &object_store::path::Path,

) -> Result<impl futures::Stream<Item = Result<Vec<u8>, object_store::Error>>, object_store::Error> {

    let range = chunk.to_range();

    let get_opts = object_store::GetOptions {
        range: Some(object_store::GetRange::Bounded(range.clone())),
        ..Default::default()
    };

    let stream = store
        .get_opts(path, get_opts)
        .await?
        .into_stream()
        .map_ok(|bytes| bytes.to_vec());

    Ok(stream.map_ok(|get_result| get_result.to_vec()))
}

/// Fetches chunks concurrently from the store service.
///
/// Returns a vector of `(Chunk, bytes)` pairs preserving input order.
pub async fn init_fetch_handles(
    store_service: &StoreService,
    options: &SearchOptions,
    chunks: &[Chunk],
) -> Result<Vec<(Chunk, Vec<u8>)>, StoreError> {
    let futures = chunks.iter().map(|chunk| {
        let range = chunk.to_range();
        let file_path = options.file_path.clone();
        let chunk_clone = chunk.clone();
        async move {
            let data = store_service.get_range(&file_path, range).await?;
            Ok((chunk_clone, data))
        }
    });

    try_join_all(futures).await
}

/// Decompresses each fetched chunk and returns the post-offset data.
pub async fn join_fetch_handles(
    fetched: Vec<(Chunk, Vec<u8>)>,
) -> Result<Vec<Vec<u8>>, CodecError> {
    let mut raw_data = Vec::new();

    for (chunk, compressed_bytes) in fetched {
        let block_sizes = bgzip::from_bytes(&compressed_bytes)?;
        let decompressed_bytes = bgzip::decompress(&block_sizes, &compressed_bytes)?;
        let decompressed_start_byte = chunk.begin_vp.decompressed_offset as usize;
        let decompressed_slice = &decompressed_bytes[decompressed_start_byte..];
        raw_data.push(decompressed_slice.to_vec());
    }
    Ok(raw_data)
}

/// Searches for features in a file based on the provided search options.
///
/// Dispatches to the format-specific search function indicated by
/// [`SearchOptions::output_format`] and reuses this `StoreService`'s cached
/// object_store clients for every backend access.
pub async fn search_features(
    store: &StoreService,
    options: &SearchOptions,
) -> Result<SearchResult, SearchError> {
    let result = match options.output_format {
        OutputFormat::BAM => bam_search(store, options).await.map_err(|e| SearchError::Bam{ path: options.file_path.to_string(), source: e}),
        OutputFormat::BIGWIG => bigwig_search(store, options).await.map_err(|e| SearchError::BigWig{ path: options.file_path.to_string(), source: e}),
        OutputFormat::BIGBED => bigbed_search(store, options).await.map_err(|e| SearchError::BigBed{ path: options.file_path.to_string(), source: e}),
        OutputFormat::VCF
        | OutputFormat::BED
        | OutputFormat::BEDGRAPH
        | OutputFormat::GFF
        | OutputFormat::GTF => tabix_search(store, options).await.map_err(|e| SearchError::Tabix{ path: options.file_path.to_string(), source: e}),
        OutputFormat::FASTA => fasta_search(store, options).await.map_err(|e| SearchError::Fasta { path: options.file_path.to_string(), source: e }),
        _ => Err(SearchError::UnsupportedFileFormat(options.file_path.to_string())),
    };

    result
}