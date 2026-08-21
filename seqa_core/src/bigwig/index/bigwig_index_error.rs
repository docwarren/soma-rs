use crate::bigwig::index::chr_tree::chr_tree_error::ChrTreeError;
use crate::bigwig::index::{header, total_summary, zoom_header};
use std::array::TryFromSliceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BigwigIndexError {
    #[error("Store Error: {0}")]
    StoreError(#[from] crate::stores::error::StoreError),

    #[error("Parsing Error: {0}")]
    BigwigError(String),

    #[error("Zoom Header Error: {0}")]
    ZoomHeaderError(#[from] zoom_header::ZoomHeaderError),

    #[error("Header Error: {0}")]
    HeaderError(#[from] header::BigwigHeaderError),

    #[error("Total Summary Error: {0}")]
    TotalSummaryError(#[from] total_summary::TotalSummaryError),

    #[error("Parsing error: {0}")]
    BigwigParsingError(#[from] TryFromSliceError),

    #[error("Chromosome Tree Error: {0}")]
    ChrTreeError(#[from] ChrTreeError),
}