use thiserror::Error;
use core::array::TryFromSliceError;
use super::overlaps::Overlaps;

#[derive(Debug, Clone, Error)]
pub enum RTreeLeafError {
    #[error("Failed to read RTree leaf: {0}")]
    RTreeLeafReadError(String),

    #[error("Parsing error: {0}")]
    RTreeLeafParseError(#[from] TryFromSliceError),
}

#[derive(Debug, Clone)]
pub struct RTreeLeaf {
    pub start_chrom_idx: u32,
    pub start_base: u32,
    pub end_chrom_idx: u32,
    pub end_base: u32,
    pub data_offset: u64,
    pub data_size: u64, // Size of the data in bytes
}

impl RTreeLeaf {

    pub const SIZE: usize = 32; // Size of the RTreeLeaf in bytes

    pub fn new() -> Self {
        RTreeLeaf {
            start_chrom_idx: 0,
            start_base: 0,
            end_chrom_idx: 0,
            end_base: 0,
            data_offset: 0,
            data_size: 0,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RTreeLeafError> {
        if bytes.len() < RTreeLeaf::SIZE {
            return Err(RTreeLeafError::RTreeLeafReadError("Not enough bytes for a complete RTree leaf".into()));
        }

        let start_chrom_idx = u32::from_le_bytes(bytes[0..4].try_into()?);
        let start_base = u32::from_le_bytes(bytes[4..8].try_into()?);
        let end_chrom_idx = u32::from_le_bytes(bytes[8..12].try_into()?);
        let end_base = u32::from_le_bytes(bytes[12..16].try_into()?);
        let data_offset = u64::from_le_bytes(bytes[16..24].try_into()?);
        let data_size = u64::from_le_bytes(bytes[24..32].try_into()?);

        Ok(RTreeLeaf {
            start_chrom_idx,
            start_base,
            end_chrom_idx,
            end_base,
            data_offset,
            data_size,
        })
    }
}

impl Overlaps for RTreeLeaf {
    fn overlaps(&self, chr_id1: u32, chr_id2: u32,start: u32, end: u32) -> bool {
        ((chr_id2 > self.start_chrom_idx) || (chr_id2 == self.start_chrom_idx && end >= self.start_base)) &&
        ((chr_id1 < self.end_chrom_idx) || (chr_id1 == self.end_chrom_idx && start <= self.end_base))
    }
}