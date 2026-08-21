use thiserror::Error;
use crate::codecs::codec_error::CodecError;
use crate::stores::error::StoreError;

#[derive(Debug, Error)]
pub enum BamHeaderError {
    #[error("Invalid BAM header line: {0}")]
    InvalidHeaderLine(String),

    #[error("StoreError: {0}")]
    StoreError(#[from] StoreError),

    #[error("BgZip Error: {0}")]
    BgZipError(#[from] CodecError),

    #[error("Parsing Error: {0}")]
    ParsingError(#[from] core::array::TryFromSliceError),
}