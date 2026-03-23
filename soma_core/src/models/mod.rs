use serde::{ Serialize, Deserialize };

use crate::{indexes::{bai::BaiIndex, bigwig::BigwigIndex, fai::FaiIndex, tabix::Tabix}, models::{bam_header::header::BamHeader, tabix_header::TabixHeader}};

pub mod vcf;
pub mod bed;
pub mod bedgraph;
pub mod gff;
pub mod gtf;
pub mod constants;
pub mod bam_header;
pub mod bam;
pub mod bigwig;
pub mod bigbed;
pub mod tabix_header;
pub mod gene_coordinate;
pub mod cytoband;
pub mod coordinates;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileSearchRequest {
    pub path: String,
    pub coordinates: String,
    pub bigwig_index: Option<BigwigIndex>,
    pub bigbed_index: Option<BigwigIndex>,  // BigBed uses same index structure as BigWig
    pub bam_index: Option<BaiIndex>,
    pub bam_header: Option<BamHeader>,
    pub tabix_index: Option<Tabix>,
    pub tabix_header: Option<TabixHeader>,
    pub fasta_index: Option<FaiIndex>
}

impl FileSearchRequest {
    pub fn new(path: String, coordinates: String) -> FileSearchRequest {
        FileSearchRequest {
            path: path,
            coordinates: coordinates,
            bigwig_index: None,
            bigbed_index: None,
            bam_index: None,
            bam_header: None,
            tabix_header: None,
            tabix_index: None,
            fasta_index: None
        }
    }
}
