// Copyright 2026 Seqa23
//
// Author: Andrew Warren
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::path::absolute;

use crate::api::output_format::OutputFormat;
use crate::api::parsing_error::ParsingError;
use crate::api::search_error::SearchError;
use crate::genome::get_longest_possible_genome;

/// Normalises a file path to a URI accepted by [`crate::stores::StoreService`].
///
/// - Cloud/HTTP URIs (`s3://`, `gs://`, `az://`, `http://`, …) are returned unchanged.
/// - Bare local paths (`/`, `./`, `../`) are resolved to an absolute path and
///   prefixed with `file://`.  An error is returned if the file does not exist.
///
/// # Errors
///
/// Returns [`UtilError`] when a local path does not exist or cannot be canonicalised.
pub fn format_file_path(file_path: &str) -> Result<String, SearchError> {
    if file_path.starts_with("/") || file_path.starts_with("./") || file_path.starts_with("../") {
        match std::fs::exists(file_path) {
            Ok(true) => (),
            Ok(false) => {
                return Err(SearchError::NotFound {
                    path: file_path.to_string()
                });
            },
            Err(e) => return Err(SearchError::AbsolutePathError{
                path: file_path.to_string(),
                source: e
            }),
        }
        let abs_path = absolute(file_path).map_err(|e| SearchError::AbsolutePathError {
            path: file_path.to_string(),
            source: e
        })?;
        Ok(format!("file://{}", abs_path.to_string_lossy()))
    } else {
        Ok(file_path.to_string())
    }
}

/// Parses a genomic coordinate string into `(chromosome, begin, end)`.
///
/// Accepted formats:
/// - `"chr1"` — whole chromosome; `begin` = 1, `end` = chromosome's maximum length.
/// - `"chr1:1000"` — single-base query; `end` = `begin`.
/// - `"chr1:1000-2000"` — explicit range.
/// - Commas in numbers are stripped (e.g. `"chr1:1,000-2,000"`).
///
/// # Errors
///
/// Returns [`FormatError`] when the chromosome name is not recognised or a
/// numeric field cannot be parsed.
pub fn parse_coordinates(coords: &str) -> Result<(String, u32, u32), SearchError> {

    let tokens: Vec<&str> = coords.split(':').collect();
    let chromosome = tokens[0].to_string();
    let chr_idx = crate::genome::chr_index(&chromosome)
        .ok_or(SearchError::InvalidCoordinateFormat(format!("Invalid chromosome: {}.", chromosome)))?;

    let (begin, end) = get_begin_end(&tokens, chr_idx)
        .map_err(|_| SearchError::InvalidCoordinateFormat(format!("Could not parse chr, begin, end {}", coords)))?;
    Ok((chromosome, begin, end))
}

fn get_begin_end(tokens: &[&str], chr_idx: usize) -> Result<(u32, u32), ParsingError> {
    let longest_genome = get_longest_possible_genome();
    if tokens.len() == 2 {
        let parts: Vec<String> = tokens[1].split('-').map(|s| s.replace(",", "")).collect();

        if parts.len() == 2 {
            let begin = parts[0].parse::<u32>()?;
            let end = parts[1].parse::<u32>()?;
            Ok((begin, end))
        } else {
            let begin = parts[0].parse::<u32>()?;
            let end = longest_genome[chr_idx];
            Ok((begin, end))
        }
    } else {
        Ok((1, longest_genome[chr_idx]))
    }
}

/// Infers the companion index URI from a genomic file URI.
///
/// | Extension | Index path |
/// |-----------|------------|
/// | `.bam` | `<file>.bai` |
/// | `.fa`, `.fasta` | `<file>.fai` |
/// | `.bigwig`, `.bw`, `.bigbed`, `.bb` | `"-"` (embedded index) |
/// | `.vcf.gz`, `.gff.gz`, `.bed.gz`, `.gtf.gz`, `.bedgraph.gz`, `.bed` | `<file>.tbi` |
///
/// # Errors
///
/// Returns [`ExtensionError`] when the extension is not in the list above.
pub fn get_index_path(file_path: &str) -> Result<String, SearchError> {
    let lower_path = file_path.to_ascii_lowercase();
    // Logic to determine the index path based on the file type
    if lower_path.ends_with(".bam") {
        Ok(format!("{}.bai", file_path))
    } else if lower_path.ends_with(".fa") || lower_path.ends_with(".fasta") {
        Ok(format!("{}.fai", file_path))
    } else if lower_path.ends_with(".bigwig") || lower_path.ends_with(".bw") ||  lower_path.ends_with(".bigbed") || lower_path.ends_with(".bb") {
        Ok("-".to_string())  // BigBed files have embedded index like BigWig
    } else if lower_path.ends_with(".vcf.gz")
        || lower_path.ends_with(".gff.gz")
        || lower_path.ends_with(".bed.gz")
        || lower_path.ends_with(".gtf.gz")
        || lower_path.ends_with(".bed")
        || lower_path.ends_with(".bedgraph.gz")
    {
        Ok(format!("{}.tbi", file_path))
    } else {
        Err(SearchError::IndexPathError(
            "Unable to get index path from file extension".into(),
        ))
    }
}

/// Infers the [`OutputFormat`] from a file URI's extension.
///
/// # Errors
///
/// Returns [`SearchError`] when the extension does not match any known format.
pub fn get_output_format(file_path: &str) -> Result<OutputFormat, SearchError> {
    let lower_path = file_path.to_ascii_lowercase();
    // Logic to determine the output format based on the file type
    if lower_path.ends_with(".bam") {
        Ok(OutputFormat::BAM)
    } else if lower_path.ends_with(".bigwig") || lower_path.ends_with(".bw") {
        Ok(OutputFormat::BIGWIG)
    } else if lower_path.ends_with(".bigbed") || lower_path.ends_with(".bb") {
        Ok(OutputFormat::BIGBED)
    } else if lower_path.ends_with(".vcf.gz") {
        Ok(OutputFormat::VCF)
    } else if lower_path.ends_with(".gff.gz") {
        Ok(OutputFormat::GFF)
    } else if lower_path.ends_with(".gtf.gz") {
        Ok(OutputFormat::GTF)
    } else if lower_path.ends_with(".bed.gz") {
        Ok(OutputFormat::BED)
    } else if lower_path.ends_with(".bedgraph.gz") {
        Ok(OutputFormat::BEDGRAPH)
    } else if lower_path.ends_with(".fa") || lower_path.ends_with(".fasta") {
        Ok(OutputFormat::FASTA)
    } else {
        Err(SearchError::PathTypeError (
            "Unable to determine file format from extension".into(),
        ))
    }
}

#[test]
fn test_parse_coordinates() {
    let (chr, start, end) = parse_coordinates("chr1:1000-2000").unwrap();
    assert_eq!(chr, "chr1");
    assert_eq!(start, 1000);
    assert_eq!(end, 2000);
}

#[test]
fn test_get_search_options_local() {
    use crate::api::search_options::SearchOptions;
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let options = SearchOptions::new("./mock_data/NA12878.gatk.cnv.vcf.gz", "chr1:1000-2000");
    assert_eq!(
        options.file_path,
        format!("file://{}/mock_data/NA12878.gatk.cnv.vcf.gz", manifest_dir)
    );
    assert_eq!(options.chromosome, "chr1");
    assert_eq!(options.begin, 1000);
    assert_eq!(options.end, 2000);
    assert_eq!(
        options.index_path,
        format!("file://{}/mock_data/NA12878.gatk.cnv.vcf.gz.tbi", manifest_dir)
    );
    assert_eq!(options.output_format, OutputFormat::VCF);

    let options = SearchOptions::new("./mock_data/NA12878.gatk.cnv.vcf.gz", "chr1");
    assert_eq!(options.chromosome, "chr1");
    assert_eq!(options.begin, 1);
    assert_eq!(options.end, 249_250_621); // Assuming chr1 length from longest_possible_genome
}

#[test]
fn test_format_file_path() {
    let path = format_file_path("./mock_data/NA12878.gatk.cnv.vcf.gz").unwrap();
    assert!(path.starts_with("file://"));
    let path = format_file_path("./fake/path.vcf.gz");
    assert!(path.is_err());

    let path = format_file_path("s3://bucket/file.vcf.gz").unwrap();
    assert_eq!(path, "s3://bucket/file.vcf.gz");
    let path = format_file_path("gs://bucket/file.vcf.gz").unwrap();
    assert_eq!(path, "gs://bucket/file.vcf.gz");
    let path = format_file_path("https://example.com/file.vcf.gz").unwrap();
    assert_eq!(path, "https://example.com/file.vcf.gz");
    let path = format_file_path("https://example.com/file.vcf.gz").unwrap();
    assert_eq!(path, "https://example.com/file.vcf.gz");
}

#[test]
fn test_get_index_path() {
    let index = get_index_path("s3://bucket/file.bam").unwrap();
    assert_eq!(index, "s3://bucket/file.bam.bai");
    let index = get_index_path("gs://bucket/file.fa").unwrap();
    assert_eq!(index, "gs://bucket/file.fa.fai");
    let index = get_index_path("https://example.com/file.vcf.gz").unwrap();
    assert_eq!(index, "https://example.com/file.vcf.gz.tbi");
    let index = get_index_path("https://example.com/file.bigwig").unwrap();
    assert_eq!(index, "-");
    let index = get_index_path("file.bed.gz").unwrap();
    assert_eq!(index, "file.bed.gz.tbi");
    let index = get_index_path("file.bed").unwrap();
    assert_eq!(index, "file.bed.tbi");
    let index = get_index_path("file.unknown");
    assert!(index.is_err());
}
