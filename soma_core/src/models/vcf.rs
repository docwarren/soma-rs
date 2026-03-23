use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::traits::feature::Feature;

use super::constants::SnvType;

#[derive(Debug, Serialize, Deserialize)]
pub struct VcfLine {
    pub chromosome: String,
    pub position: u32,
    pub id: String,
    pub ref_allele: String,
    pub alt_alleles: Vec<String>,
    pub quality: Option<f32>,
    pub filter: Vec<String>,
    pub info: Vec<(String, String)>,
    pub sample_data: Vec<Vec<(String, String)>>,
}

impl VcfLine {
    pub fn from_line(line: String) -> Result<VcfLine, String> {
        let tokens = line.split('\t').collect::<Vec<&str>>();
        if tokens.len() < 8 {
            return Err(format!("Invalid VCF line: {}", line));
        }
        let chromosome = tokens[0].to_string();
        let position = match tokens[1]
            .parse::<u32>()
        {
            Ok(pos) => pos,
            Err(_) => {
                return Err(format!("Invalid position in VCF line: {}", line));
            }
        };
        let id = tokens[2].to_string();
        let ref_allele = tokens[3].to_string();
        let alt_alleles = tokens[4]
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        // Parse the quality field
        let quality = if tokens[5] == "." {
            None
        } else {
            Some(
                match tokens[5].parse::<f32>() {
                    Ok(q) => q,
                    Err(_) => {
                        return Err(format!("Invalid quality in VCF line: {}", line));
                    }
                },
            )
        };

        // Parse the filter field
        let filter = if tokens[6] == "." {
            Vec::new()
        } else {
            tokens[6]
                .split(';')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        };

        // Parse the INFO field
        let info = tokens[7]
            .split(';')
            .map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 {
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    (parts[0].to_string(), String::new())
                }
            })
            .collect::<Vec<(String, String)>>();

        let mut sample_data = Vec::new();

        // If there are sample data, parse them
        // The format is expected to be in the 9th column onwards
        if tokens.len() > 8 {
            let format_keys = tokens[8].split(':').collect::<Vec<&str>>();

            for sample in tokens[9..].iter() {
                let sample_values = sample.split(':').collect::<Vec<&str>>();
                let mut sample_map = Vec::new();
                for (i, key) in format_keys.iter().enumerate() {
                    if i < sample_values.len() {
                        sample_map.push((key.to_string(), sample_values[i].to_string()));
                    } else {
                        sample_map.push((key.to_string(), String::new()));
                    }
                }
                sample_data.push(sample_map);
            }
        }

        Ok(VcfLine {
            chromosome,
            position,
            id,
            ref_allele,
            alt_alleles,
            quality,
            filter,
            info,
            sample_data,
        })
    }
}

impl Display for VcfLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let quality = match self.quality {
            Some(q) => q.to_string(),
            None => ".".to_string(),
        };

        let info = self
            .info
            .iter()
            .map(|(k, v)| if v.is_empty() { format!("{}", k) } else { format!("{}={}", k, v) })
            .collect::<Vec<String>>()
            .join(";");

        let format = if self.sample_data.is_empty() {
            String::new()
        } else {
            self.sample_data[0]
                .iter()
                .map(|(k, _)| k.clone())
                .collect::<Vec<String>>()
                .join(":")
        };

        let mut samples = Vec::new();
        for sample in &self.sample_data {
            let sample_values = sample
                .iter()
                .map(|(_, v)| v.to_string())
                .collect::<Vec<String>>()
                .join(":");
            samples.push(sample_values);
        }

        let sample_str = samples.join("\t");

        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.chromosome,
            self.position,
            self.id,
            self.ref_allele,
            self.alt_alleles.join(","),
            quality,
            self.filter.join(";"),
            info,
            format,
            sample_str
        )
    }
}

impl Feature for VcfLine {
    fn get_chromosome(&self) -> String {
        self.chromosome.clone()
    }
    // If there are multiple alts all values
    // are calculated based on the longest alt allele
    // which is the most significant variant
    // i.e. the one with the largest length difference
    fn get_begin(&self) -> u32 {
        let variant_type = self.get_variant_type().unwrap_or(SnvType::SUBSTITUTION);
        let prefix_len = self.prefix_len().unwrap_or(0);

        match variant_type {
            SnvType::INSERTION => self.position + prefix_len - 1 as u32,
            SnvType::DELETION => self.position + prefix_len - 1 as u32,
            SnvType::SUBSTITUTION => self.position,
        }
    }

    fn get_end(&self) -> u32 {
        let variant_type = self.get_variant_type().unwrap_or(SnvType::SUBSTITUTION);

        match variant_type {
            SnvType::INSERTION => self.get_begin(),
            SnvType::DELETION => self.get_begin() + self.get_length(),
            SnvType::SUBSTITUTION => self.position,
        }
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_length(&self) -> u32 {
        let variant_type = self.get_variant_type().unwrap_or(SnvType::SUBSTITUTION);
        match variant_type {
            SnvType::INSERTION => {
                match self.longest_alt() {
                    Some(alt) => alt.len() as u32 - self.ref_allele.len() as u32,
                    None => 0,
                }
            },
            SnvType::DELETION => {
                match self.longest_alt() {
                    Some(alt) => self.ref_allele.len() as u32 - alt.len() as u32,
                    None => 0,
                }
            },
            SnvType::SUBSTITUTION => 1,
        }
    }

    fn coordinate_system(&self) -> super::coordinates::CoordinateSystem {
        super::coordinates::CoordinateSystem::OneBasedClosed
    }
}

impl VcfLine {
    /// Returns the type of the longest alt allele
    /// * @return The type of the variant (insertion, deletion, or substitution).
    /// The type is determined based on the length of the longest alt allele compared to the reference allele.
    /// If the longest alt allele is longer than the reference allele, it is an insertion.
    /// If the longest alt allele is shorter than the reference allele, it is a deletion.
    /// If the longest alt allele is the same length as the reference allele, it is a substitution.
    pub fn get_variant_type(&self) -> Result<SnvType, String> {
        let alt = self.longest_alt().ok_or_else(|| "No alt allele found".to_string())?;

        if alt.len() > self.ref_allele.len() {
            Ok(SnvType::INSERTION)
        } else if alt.len() < self.ref_allele.len() {
            Ok(SnvType::DELETION)
        } else {
            Ok(SnvType::SUBSTITUTION)
        }
    }
    /// Returns the longest alt allele based on the length difference from the reference allele.
    /// If there are multiple alleles with the same length difference, it returns the first one.
    /// * @return The longest alt allele.
    pub fn longest_alt(&self) -> Option<String> {
        self.alt_alleles
            .iter()
            .max_by_key(|alt| (alt.len() as i32 - self.ref_allele.len() as i32).abs())
            .cloned()
    }

    /// Returns the length of the prefix that is common between the longest alt allele and the reference allele.
    /// * @return The length of the common prefix.
    pub fn prefix_len(&self) -> Result<u32, String> {
        let longest_alt = self.longest_alt().ok_or_else(|| "No alt allele found".to_string())?;
        let longest = longest_alt.len();
        let shortest = if longest < self.ref_allele.len() {
            longest
        } else {
            self.ref_allele.len()
        };
        let mut i: u32 = 0;
        while i < shortest as u32
            && longest_alt.chars().nth(i as usize) == self.ref_allele.chars().nth(i as usize)
        {
            i += 1;
        }
        Ok(i)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn should_init_from_string() {
        let vcf_line = "20\t2\t.\tTCG\tTG,T,TCAG\t.\tPASS\tDP=100".to_string();
        let vcf_record = VcfLine::from_line(vcf_line).unwrap();
        assert_eq!(vcf_record.chromosome, "20");
        assert_eq!(vcf_record.position, 2);
        assert_eq!(vcf_record.id, ".");
        assert_eq!(vcf_record.ref_allele, "TCG");
        assert_eq!(vcf_record.alt_alleles, vec!["TG", "T", "TCAG"]);
        assert_eq!(vcf_record.quality, None);
        assert_eq!(vcf_record.filter, vec!["PASS"]);
        assert_eq!(
            vcf_record.info,
            Vec::from([("DP".to_string(), "100".to_string())])
        );
        assert_eq!(vcf_record.sample_data.len(), 0);
        assert_eq!(vcf_record.get_id(), ".");
        assert_eq!(vcf_record.get_begin(), 2);
        assert_eq!(vcf_record.get_end(), 4);
        assert_eq!(vcf_record.get_length(), 2);
    }
}
