use crate::bam::bam_error::BamError;
use crate::bigwig::bigbed_error::BigbedError;
use crate::bigwig::bigwig_search::BigwigError;
use crate::codecs::codec_error::CodecError;
use crate::fasta::fasta_error::FastaSearchError;
use crate::tabix::tabix_error::TabixSearchError;
use crate::utils::UtilError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Failed to process data: {0}")]
    DataProcessingError(String),

    #[error("Store Error: {0}")]
    StoreError(#[from] crate::stores::error::StoreError),

    #[error("Object Store Error: {0}")]
    ObjectStoreError(#[from] object_store::Error),

    #[error("BgZip Error: {0}")]
    BgZipError(#[from] CodecError),
}

/// Unified error returned by [`StoreService::search_features`].
///
/// Wraps the format-specific error from the underlying search function.
#[derive(Debug, Error)]
pub enum SearchFeaturesError {
    #[error("Search Error: {0}")]
    String(String),

    #[error("BAM error occurred")]
    Bam(#[from] BamError),

    #[error("Fasta error occurred")]
    Fasta(#[from] FastaSearchError),

    #[error("Tabix error occurred")]
    Tabix(#[from] TabixSearchError),

    #[error("BigWig error occurred")]
    BigWig(#[from] BigwigError),

    #[error("BigBed error occurred")]
    BigBed(#[from] BigbedError),

    #[error("Utility error occurred")]
    Util(#[from] UtilError),
}