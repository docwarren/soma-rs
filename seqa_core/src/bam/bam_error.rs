use crate::bam::bai_error::BaiError;
use crate::bam::header::bam_header_error::BamHeaderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BamError {

    #[error("Data processing error: {0}")]
    DataProcessingError(String),

    #[error("Chromosome not found: {0}")]
    ChromosomeNotFound(String),

    #[error("Failed to read BAM header: {0}")]
    HeaderError(#[from] BamHeaderError),

    #[error("Failed to read Bai index: {0}")]
    BaiError(#[from] BaiError),

    #[error("Failed to initialise search: {0}")]
    SearchError(#[from] crate::api::search::SearchError),
}