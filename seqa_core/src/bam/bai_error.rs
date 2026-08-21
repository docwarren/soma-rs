use std::array::TryFromSliceError;
use thiserror::Error;
use crate::indexes::chunk::ChunkError;
use crate::stores::error::StoreError;

#[derive(Debug, Error)]
pub enum BaiError {
    #[error("Failed to read BAI index file: {path}")]
    ReadError{
        path: String,
        #[source]
        source: StoreError
    },

    #[error("Byte parsing Error")]
    ParsingError{
        #[source]
        source: TryFromSliceError
    },

    #[error("Chunk parsing error")]
    ChunkParsingError{
        #[source]
        source: ChunkError
    }
}