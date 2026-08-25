use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("Byte length error: expected {expected} bytes, got {actual} bytes")]
    ByteLengthError{
        expected: usize,
        actual: usize
    },

    #[error("BgZip header parsing error at {}", field)]
    ParsingError {
        field: String,
        #[source]
        source: Box<dyn Error + Send + Sync>
    },

    #[error("Codec not supported: {0}")]
    CodecNotSupported(String),

    #[error("Error decompressing byte array. {}", source)]
    ReaderError{
        decoder_used: String,
        #[source]
        source: std::io::Error
    },

    #[error("Unknown decoder header: {:#?}", magic)]
    UnknownCodec {
        magic: (u8, u8)
    },

    #[error("Unknown error decoding data: {0}")]
    UnknownError(String)
}