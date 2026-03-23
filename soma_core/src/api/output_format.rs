use serde::{ Serialize, Deserialize};

use crate::models::bedgraph::BedGraphLine;
use crate::models::vcf::VcfLine;
use crate::models::bed::BedLine;
use crate::traits::feature::Feature;
use crate::models::gff::GffLine;
use crate::models::gtf::GtfLine;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    BAM,
    FASTA,
    VCF,
    BED,
    BEDGRAPH,
    BIGWIG,
    BIGBED,
    GFF,
    GTF,
    STRING
}

impl OutputFormat {
    pub fn from_str(format: &str) -> Result<OutputFormat, String> {
        match format.to_lowercase().as_str() {
            "bam" => Ok(OutputFormat::BAM),
            "fasta" => Ok(OutputFormat::FASTA),
            "vcf" => Ok(OutputFormat::VCF),
            "bed" => Ok(OutputFormat::BED),
            "bedgraph" => Ok(OutputFormat::BEDGRAPH),
            "bigwig" => Ok(OutputFormat::BIGWIG),
            "bigbed" => Ok(OutputFormat::BIGBED),
            "gff" => Ok(OutputFormat::GFF),
            "gtf" => Ok(OutputFormat::GTF),
            "string" => Ok(OutputFormat::STRING),
            _ => Err(format!("Unknown output format: {}", format)),
        }
    }

    pub fn get_model(&self) -> fn(&str) -> Result<Box<dyn Feature>, String> {
        match self {
            OutputFormat::VCF => vcf_from_line,
            OutputFormat::BED => bed_from_line,
            OutputFormat::BEDGRAPH => bedgraph_from_line,
            OutputFormat::GFF => gff_from_line,
            OutputFormat::GTF => gtf_from_line,
            _ => unsupported_format,
        }
    }
}

fn vcf_from_line(line: &str) -> Result<Box<dyn Feature>, String> {
    match VcfLine::from_line(line.to_string()) {
        Ok(line) => Ok(Box::new(line)),
        Err(err) => Err(err),
    }
}

fn bed_from_line(line: &str) -> Result<Box<dyn Feature>, String> {
    match BedLine::from_line(line.to_string()) {
        Ok(line) => Ok(Box::new(line)),
        Err(err) => Err(err),
    }
}

fn bedgraph_from_line(line: &str) -> Result<Box<dyn Feature>, String> {
    match BedGraphLine::from_line(line.to_string()) {
        Ok(line) => Ok(Box::new(line)),
        Err(err) => Err(err),
    }
}

fn gff_from_line(line: &str) -> Result<Box<dyn Feature>, String> {
    match GffLine::from_line(line.to_string()) {
        Ok(line) => Ok(Box::new(line)),
        Err(err) => Err(err),
    }
}

fn gtf_from_line(line: &str) -> Result<Box<dyn Feature>, String> {
    match GtfLine::from_line(line.to_string()) {
        Ok(line) => Ok(Box::new(line)),
        Err(err) => Err(err),
    }
}

fn unsupported_format(_: &str) -> Result<Box<dyn Feature>, String> {
    Err("Unsupported output format".to_string())
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::BAM => write!(f, "BAM"),
            OutputFormat::FASTA => write!(f, "FASTA"),
            OutputFormat::VCF => write!(f, "VCF"),
            OutputFormat::BED => write!(f, "BED"),
            OutputFormat::BEDGRAPH => write!(f, "BEDGRAPH"),
            OutputFormat::BIGWIG => write!(f, "BIGWIG"),
            OutputFormat::BIGBED => write!(f, "BIGBED"),
            OutputFormat::GFF => write!(f, "GFF"),
            OutputFormat::GTF => write!(f, "GTF"),
            OutputFormat::STRING => write!(f, "STRING"),
        }
    }
}
