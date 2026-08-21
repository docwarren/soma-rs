use crate::api::search::SearchError;
use crate::codecs::codec_error::CodecError;
use crate::indexes::chunk::ChunkError;
use crate::stores::error::StoreError;
use crate::tabix::tabix_header::TabixHeaderError;
use std::array::TryFromSliceError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TabixError {
    #[error("Failed to read Tabix index file: {0}")]
    ReadError(String),

    #[error("Failed to parse Tabix index file: {0}")]
    ParseError(#[from] FromUtf8Error),

    #[error("Failed to decompress Tabix index file: {0}")]
    DecompressError(#[from] CodecError),

    #[error("Store error: {0}")]
    StoreError(#[from] StoreError),

    #[error("Parsing Error: {0}")]
    ParsingError(#[from] TryFromSliceError),

    #[error("Chunk Error: {0}")]
    ChunkError(#[from] ChunkError),
}

#[derive(Debug, Error)]
pub enum TabixSearchError {
    #[error("Search error: {0}")]
    SearchError(String),

    #[error("Tabix index error: {0}")]
    TabixIndexError(#[from] TabixError),

    #[error("async threads error: {0}")]
    AsyncThreadsError(#[from] SearchError),

    #[error("Failed to read tabix header: {0}")]
    FailedToReadTabixHeader(#[from] TabixHeaderError),
}