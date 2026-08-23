use thiserror::Error;
use std::array::TryFromSliceError;
use std::string::FromUtf8Error;
use crate::stores::error::StoreError;
use crate::codecs::codec_error::CodecError;
use crate::indexes::chunk::ChunkError;

#[derive(Debug, Error)]
pub enum TabixError {
    #[error("Error reading from file")]
    ReadError {
        description: String,
        #[source]
        source: StoreError
    },

    #[error("Error decompressing")]
    CompressionError {
        description: String,
        source: CodecError
    },

    #[error("Invalid Request {request} caused by {reason}")]
    InvalidRequest {
        request: String,
        reason: String
    },

    #[error("Error parsing bytes")]
    ByteParsingError(#[from] TryFromSliceError),

    #[error("Error parsing utf8")]
    Utf8ParseError(#[from] FromUtf8Error),

    #[error("Error parsing chunk")]
    ChunkError {
        #[source]
        source: ChunkError
    }
}