use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::read_utils::map_sequence_code;

#[derive(Debug, Serialize, Deserialize)]
pub struct Seq {
    pub bytes: Vec<u8>
}

impl Seq {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Seq { bytes }
    }

    pub fn to_string(&self) -> String {
        if self.bytes.is_empty() {
            return String::from("*");
        }

        let mut i = 0;
        let mut result = String::new();
        let read_last_base: bool = self.bytes.len() % 2 == 0;

        while i < self.bytes.len() {
            let byte = self.bytes[i];
            let left: u8 = byte >> 4;
            let right: u8 = byte & 0x0F;
            let left_char = map_sequence_code(left);
            let right_char = map_sequence_code(right);
            result.push(left_char);
            if i < self.bytes.len() - 1 || read_last_base {
                result.push(right_char);
            }
            i += 1;
        }
        result
    }
}

impl Display for Seq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
