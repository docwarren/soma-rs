use crate::bam::bam_error::BamError;
use crate::bigwig::bigbed_error::BigbedError;
use crate::bigwig::bigwig_search::BigwigError;
use crate::fasta::fasta_error::FastaError;
use crate::tabix::tabix_error::TabixError;
use crate::utils::UtilError;

use thiserror::Error;

/// Unified error returned by [`StoreService::search_features`].
///
/// Wraps the format-specific error from the underlying search function.
#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Unsupported file format: {0}")]
    UnsupportedFileFormat(String),

    #[error("BAM error")]
    Bam {
        path: String,
        #[source]
        source: BamError
    },

    #[error("Fasta error")]
    Fasta {
        path: String,
        #[source]
        source: FastaError
    },

    #[error("Tabix error")]
    Tabix {
        path: String,
        #[source]
        source: TabixError
    },

    #[error("BigWig error")]
    BigWig {
        path: String,
        #[source]
        source: BigwigError
    },

    #[error("BigBed error occurred")]
    BigBed {
        path: String,
        #[source]
        source: BigbedError
    },

    #[error("{description}")]
    Util{
        description: String,
        #[source]
        source: UtilError
    },
}