use crate::bigwig::index::r_tree::r_tree_non_leaf::RTreeNonLeafError;
use crate::codecs::codec_error::CodecError;
use crate::stores::error::StoreError;
use std::array::TryFromSliceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RTreeError {
    #[error("Failed to read RTree file: {0}")]
    RTreeReadError(String),

    #[error("StoreError: {0}")]
    StoreError(#[from] StoreError),

    #[error("BgZip Error: {0}")]
    BgZipError(#[from] CodecError),

    #[error("Parsing Error: {0}")]
    ParsingError(#[from] TryFromSliceError),

    #[error("RTree Node Error: {0}")]
    RTreeNodeError(#[from] RTreeNodeError),
}

#[derive(Debug, Clone, Error)]
pub enum RTreeLeafError {
    #[error("Failed to read RTree leaf: {0}")]
    RTreeLeafReadError(String),

    #[error("Parsing error: {0}")]
    RTreeLeafParseError(#[from] TryFromSliceError),
}

#[derive(Debug, Error)]
pub enum RTreeNodeError {
    #[error("Failed to read RTree node: {0}")]
    RTreeNodeReadError(String),

    #[error("Error reading RTree Leaf: {0}")]
    RTreeLeafReadError(#[from] RTreeLeafError),

    #[error("Error reading RTree Non Leaf: {0}")]
    RTreeNonLeafReadError(#[from] RTreeNonLeafError),
}