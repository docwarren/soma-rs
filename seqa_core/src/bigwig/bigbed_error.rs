use crate::bigwig::index::bigwig_index_error::BigwigIndexError;
use crate::bigwig::index::r_tree::r_tree_error::RTreeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BigbedError {
    #[error("Data processing error: {0}")]
    DataProcessingError(String),

    #[error("Bigbed index error: {0}")]
    BigbedIndexError(#[from] BigwigIndexError),

    #[error("RTree Error: {0}")]
    RTreeError(#[from] RTreeError),
}