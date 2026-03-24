use std::fs::File;
use std::io::{ BufRead, BufReader };

const HG38_CYTOBANDS: &str = "../../../data/hg38_cytobands.tsv";
const HG19_CYTOBANDS: &str = "../../../data/hg19_cytobands.tsv";

pub fn get_cytobands(genome: &str) -> Result<Vec<String>, std::io::Error> {
    let file_path = match genome.to_lowercase().as_str() {
        "hg38" => HG38_CYTOBANDS,
        "hg19" => HG19_CYTOBANDS,
        "grch38" => HG38_CYTOBANDS,
        "grch37" => HG19_CYTOBANDS,
        "ch38" => HG38_CYTOBANDS,
        "ch37" => HG19_CYTOBANDS,
        _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported genome build")),
    };

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let cytobands: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(cytobands)
}

#[test]
fn test_get_cytobands() {
    let hg38_cytobands = get_cytobands("hg38").unwrap();
    assert!(!hg38_cytobands.is_empty());

    let hg19_cytobands = get_cytobands("hg19").unwrap();
    assert!(!hg19_cytobands.is_empty());

    let invalid_cytobands = get_cytobands("hg37");
    assert!(invalid_cytobands.is_err());
}