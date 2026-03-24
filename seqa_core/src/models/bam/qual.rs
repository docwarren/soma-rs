use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Qual {
    pub bytes: Vec<u8>,
}

impl Qual {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Qual { bytes }
    }

    pub fn to_string(&self) -> String {
        if self.bytes.is_empty() {
            return String::from("*");
        }
        if self.bytes.len() > 0 && self.bytes[0] == 0xFF {
            return String::from_utf8_lossy(&self.bytes).to_string();
        } else {
            let qual = self
                .bytes
                .iter()
                .map(|&q| (q + 33) as char) // Convert to ASCII quality scores
                .collect::<String>();
            return qual;
        }
    }
}

impl Display for Qual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}