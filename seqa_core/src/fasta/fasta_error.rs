use std::array::TryFromSliceError;
use std::string::FromUtf8Error;
use thiserror::Error;
use crate::stores::error::StoreError;

#[derive(Debug, Error)]
pub enum FastaSearchError {
    #[error("FAI index error: {0}")]
    FaiIndexError(#[from] FaiIndexError),

    #[error("UTF-8 Error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Store Error: {0}")]
    StoreError(#[from] StoreError),

    #[error("Failed to read FASTA file: {0}")]
    FailedToReadFastaFile(String),
}

#[derive(Debug, Error)]
pub enum FaiIndexError {
    #[error("Failed to read FAI index file: {0}")]
    ReadError(String),

    #[error("Store Error: {0}")]
    StoreError(#[from] StoreError),

    #[error("Failed to parse FAI index file: {0}")]
    ParseError(#[from] TryFromSliceError),

    #[error("UTF-8 Error: {0}")]
    Utf8Error(#[from] FromUtf8Error),
}