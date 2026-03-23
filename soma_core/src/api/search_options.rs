use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{indexes::{bai::BaiIndex, bigwig::BigwigIndex, fai::FaiIndex, tabix::Tabix}, models::{bam_header::header::BamHeader, tabix_header::TabixHeader}, traits::feature::Feature};

use super::output_format::OutputFormat;

/// Specifies the format of the CIGAR string in BAM output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CigarFormat {
    /// Standard CIGAR string as stored in the BAM file (e.g., "10M1D5M")
    #[default]
    Standard,
    /// Merged CIGAR with mismatch and deletion bases included (e.g., "10M1DA5M1XG")
    Merged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    pub file_path: String,
    pub index_path: String,
    pub chromosome: String,
    pub begin: u32,
    pub end: u32,
    pub output_format: OutputFormat,
    pub include_header: bool,
    pub header_only: bool,
    pub cigar_format: CigarFormat,
    pub bigwig_index: Option<BigwigIndex>,
    pub bigbed_index: Option<BigwigIndex>,  // BigBed uses same index structure as BigWig
    pub bam_index: Option<BaiIndex>,
    pub bam_header: Option<BamHeader>,
    pub tabix_index: Option<Tabix>,
    pub tabix_header: Option<TabixHeader>,
    pub fasta_index: Option<FaiIndex>
}

impl SearchOptions {
    pub fn new() -> Self {
        SearchOptions {
            file_path: String::new(),
            index_path: String::new(),
            chromosome: String::new(),
            begin: 0,
            end: 0,
            output_format: OutputFormat::STRING, // Default output format
            include_header: true,
            header_only: false,
            cigar_format: CigarFormat::Standard,
            bigwig_index: None,
            bigbed_index: None,
            bam_header: None,
            bam_index: None,
            tabix_index: None,
            tabix_header: None,
            fasta_index: None
        }
    }

    pub fn set_cigar_format(&mut self, cigar_format: CigarFormat) -> Self {
        self.cigar_format = cigar_format;
        self.clone()
    }

    pub fn set_file_path(&mut self, file_path: &str) -> Self {
        self.file_path = file_path.to_string();
        self.clone()
    }

    pub fn set_index_path(&mut self, index_path: &str) -> Self {
        self.index_path = index_path.to_string();
        self.clone()
    }

    pub fn set_coordinates(&mut self, coords: &str) -> Self {
        let string:String = coords.replace(",", "");
        let parts: Vec<&str> = string.split(':').collect();
        if parts.len() == 2 {
            let range: Vec<&str> = parts[1].split('-').collect();
            if range.len() == 2 {
                self.chromosome = parts[0].to_string();
                self.begin = range[0].parse().unwrap_or(1);
                self.end = range[1].parse().unwrap_or(1);
            }
        }
        self.clone()
    }

    pub fn set_chromosome(&mut self, chromosome: &str) -> Self {
        self.chromosome = chromosome.to_string();
        self.clone()
    }

    pub fn set_begin(&mut self, begin: u32) -> Self {
        self.begin = begin;
        self.clone()
    }

    pub fn set_end(&mut self, end: u32) -> Self {
        self.end = end;
        self.clone()
    }

    pub fn set_output_format(&mut self, output_format: &str) -> Self {
        let format = output_format.to_lowercase();
        self.output_format = OutputFormat::from_str(&format).unwrap_or(OutputFormat::VCF);
        self.clone()
    }

    pub fn set_include_header(&mut self, include_header: bool) -> Self {
        self.include_header = include_header;
        self.clone()
    }

    pub fn set_header_only(&mut self, header_only: bool) -> Self {
        self.header_only = header_only;
        self.clone()
    }

}

impl Feature for SearchOptions {

    fn get_begin(&self) -> u32 {
        self.begin
    }

    fn get_end(&self) -> u32 {
        self.end
    }

    fn get_length(&self) -> u32 {
        self.end - self.begin + 1
    }

    fn get_id(&self) -> String {
        format!("{}:{}-{}", self.chromosome, self.begin, self.end)
    }

    fn coordinate_system(&self) -> crate::models::coordinates::CoordinateSystem {
        crate::models::coordinates::CoordinateSystem::OneBasedClosed
    }

    fn get_chromosome(&self) -> String {
        self.chromosome.clone()
    }
}

impl Display for SearchOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.chromosome, self.begin, self.end)
    }
}
