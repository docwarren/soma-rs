use std::array::TryFromSliceError;
use thiserror::Error;
use crate::codecs::codec_error::CodecError;
use crate::indexes::chunk::ChunkError;
use crate::stores::error::StoreError;

#[derive(Debug, Error)]
pub enum BamError {
    #[error("Invalid range request ({requested}) => {reason}")]
    InvalidRequest {
        requested: String,
        reason: String
    },

    #[error("Error fetching bam data")]
    FetchError {
        #[source]
        source: StoreError
    },

    #[error("Error decompressing data")]
    DecompressionError {
        #[source]
        source: CodecError
    },

    #[error("Chromosome not found: {0}")]
    ChromosomeNotFound(String),

    #[error("Error parsing header line: {0}")]
    HeaderParseError(String),

    #[error("Error reading index file {path}")]
    IndexReadError{
        path: String,
        #[source]
        source: StoreError
    },

    #[error("Error parsing bytes")]
    ByteParsingError(#[from] TryFromSliceError),

    #[error("Error parsing file")]
    ParseError {
        #[source]
        source: ChunkError
    }
}