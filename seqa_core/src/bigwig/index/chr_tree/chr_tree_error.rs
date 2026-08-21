use crate::bigwig::index::chr_tree::chr_tree_non_leaf::ChrTreeNonLeafError;
use std::array::TryFromSliceError;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ChrTreeError {
    #[error("Parsing Error: {0}")]
    InvalidData(#[from] TryFromSliceError),

    #[error("ChromTreeNode Error: {0}")]
    NodeError(#[from] ChrTreeNodeError),

    #[error("Chromosome Tree Header Error: {0}")]
    HeaderError(#[from] ChrTreeHeaderError),
}

#[derive(Debug, Error, Clone)]
pub enum ChrTreeLeafError {
    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Parsing error: {0}")]
    ParsingError(#[from] TryFromSliceError),
}

#[derive(Debug, Error, Clone)]
pub enum ChrTreeNodeError {
    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Parsing error: {0}")]
    ParsingError(#[from] TryFromSliceError),

    #[error("Non Leaf Error: {0}")]
    NonLeafError(#[from] ChrTreeNonLeafError),

    #[error("Leaf Error: {0}")]
    LeafError(#[from] ChrTreeLeafError),
}

#[derive(Debug, Error, Clone)]
pub enum ChrTreeHeaderError {
    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Parsing error: {0}")]
    ParsingError(#[from] TryFromSliceError),
}