use crate::{
    indexes::{bai::BaiIndex, bigwig::BigwigIndex, fai::FaiIndex, tabix::Tabix},
    models::{bam_header::header::BamHeader, tabix_header::TabixHeader},
};

pub struct SearchResult {
    pub bigwig_index: Option<BigwigIndex>,
    pub bigbed_index: Option<BigwigIndex>,  // BigBed uses same index structure as BigWig
    pub tabix_index: Option<Tabix>,
    pub tabix_header: Option<TabixHeader>,
    pub fasta_index: Option<FaiIndex>,
    pub bam_index: Option<BaiIndex>,
    pub bam_header: Option<BamHeader>,
    pub lines: Vec<String>,
}

impl SearchResult {
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
