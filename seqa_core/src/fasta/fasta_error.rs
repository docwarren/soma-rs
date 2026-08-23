use std::string::FromUtf8Error;
use thiserror::Error;
use crate::stores::error::StoreError;

#[derive(Debug, Error)]
pub enum FastaError {
    #[error("Invalid range request ({requested}) => {reason}")]
    InvalidRequest {
        requested: String,
        reason: String
    },

    #[error("FAI index error")]
    IndexError {
        file_path: String,
        #[source]
        source: FaiIndexError
    },

    #[error("Error fetching requested data for {file_path}")]
    FetchError {
        file_path: String,
        #[source]
        source: StoreError
    },

    #[error("Error parsing fasta file: {file_path}")]
    ParseError {
        file_path: String,
        #[source]
        source: FromUtf8Error
    }
}

#[derive(Debug, Error)]
pub enum FaiIndexError {
    #[error("Invalid request ({requested}) => {reason}")]
    InvalidRequest {
        requested: String,
        reason: String
    },

    #[error("Error reading FAI index file: {file_path}")]
    ReadError {
        file_path: String,
        #[source]
        source: StoreError
    },

    #[error("Error parsing FAI index file content")]
    ParseError {
        #[source]
        source: FromUtf8Error
    }
}