use serde::{ Serialize, Deserialize };
use std::fmt::Display;
use crate::models::bam::read_utils::{get_tag_value, map_tag_type_to_result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub bytes: Vec<u8>,
}

impl Tags {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Tags { bytes }
    }

    /// Get the value of a specific tag by its 2-character name (e.g., "MD")
    pub fn get_value(&self, tag_name: &str) -> Option<String> {
        if self.bytes.is_empty() || tag_name.len() != 2 {
            return None;
        }

        let tag_bytes: Vec<u8> = tag_name.bytes().collect();
        let mut i = 0;

        while i + 2 < self.bytes.len() {
            // Check if this is the tag we're looking for
            if self.bytes[i] == tag_bytes[0] && self.bytes[i + 1] == tag_bytes[1] {
                i += 2;
                let value_type = self.bytes[i];
                i += 1;

                // Handle array type
                if value_type == b'B' {
                    let val_type = self.bytes[i];
                    i += 1;
                    let count = i32::from_le_bytes(
                        self.bytes[i..i + 4].try_into().ok()?
                    );
                    i += 4;

                    let mut values = Vec::new();
                    for _ in 0..count {
                        let (value, new_i) = map_tag_type_to_result(&self.bytes, i, val_type).ok()?;
                        values.push(value);
                        i = new_i;
                    }
                    return Some(values.join(","));
                } else {
                    let (value, _) = map_tag_type_to_result(&self.bytes, i, value_type).ok()?;
                    return Some(value);
                }
            }

            // Skip to next tag
            let (_, new_i) = get_tag_value(&self.bytes, i).ok()?;
            i = new_i;
        }

        None
    }

    pub fn to_string(&self) -> Result<String, String> {
        if self.bytes.is_empty() {
            return Ok(String::from("*"));
        }

        let mut i = 0;
        let mut result = String::new();

        while i < self.bytes.len() {
            let (tag_str, new_i) = get_tag_value(&self.bytes, i)?;
            i = new_i;

            result.push_str(&tag_str);
            if i < self.bytes.len() {
                result.push('\t'); // Add tab between tags
            }
        }
        Ok(result)
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().unwrap_or_default())
    }
}
