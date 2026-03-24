use crate::{
    indexes::{bai::BaiIndex, bigwig::BigwigIndex, fai::FaiIndex, tabix::Tabix},
    models::{bam_header::header::BamHeader, tabix_header::TabixHeader},
};

/// The result of a genomic region search.
///
/// Each format-specific search function populates the relevant index/header fields
/// so that callers can cache the parsed index across subsequent queries without
/// re-downloading it.  The actual data lines are always in [`SearchResult::lines`].
pub struct SearchResult {
    /// Parsed BigWig index; present after a [`crate::api::bigwig_search::bigwig_search`] call.
    pub bigwig_index: Option<BigwigIndex>,
    /// Parsed BigBed index; present after a [`crate::api::bigbed_search::bigbed_search`] call.
    /// BigBed shares the same index structure as BigWig.
    pub bigbed_index: Option<BigwigIndex>,
    /// Parsed tabix index; present after a [`crate::api::tabix_search::tabix_search`] call.
    pub tabix_index: Option<Tabix>,
    /// Tabix file header; present after a [`crate::api::tabix_search::tabix_search`] call.
    pub tabix_header: Option<TabixHeader>,
    /// Parsed FASTA index (`.fai`); present after a [`crate::api::fasta_search::fasta_search`] call.
    pub fasta_index: Option<FaiIndex>,
    /// Parsed BAM index (`.bai`); present after a [`crate::api::bam_search::bam_search`] call.
    pub bam_index: Option<BaiIndex>,
    /// BAM file header; present after a [`crate::api::bam_search::bam_search`] call.
    pub bam_header: Option<BamHeader>,
    /// Query result lines in the native text format of the file (SAM, VCF, BED, etc.).
    pub lines: Vec<String>,
}

impl SearchResult {
    /// Creates an empty `SearchResult` with all optional fields set to `None`.
    pub fn new() -> SearchResult {
        SearchResult {
            bigwig_index: None,
            bigbed_index: None,
            tabix_header: None,
            tabix_index: None,
            bam_header: None,
            bam_index: None,
            fasta_index: None,
            lines: Vec::new(),
        }
    }
}
