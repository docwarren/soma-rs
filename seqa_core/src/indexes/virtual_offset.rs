use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualOffset {
    pub virtual_pointer: u64,
    pub block_offset: u64,
    pub decompressed_offset: u64
}

impl VirtualOffset {
    pub fn new(virtual_pointer: u64) -> Self {
        let (block_offset, decompressed_offset) = (virtual_pointer >> 16, virtual_pointer & 0xFFFF);
        VirtualOffset {
            virtual_pointer,
            block_offset,
            decompressed_offset
        }
    }

    pub fn split(&self) -> (u64, u64) {
        let c_offset = self.virtual_pointer >> 16;
        let d_offset = self.virtual_pointer & 0xFFFF;
        (c_offset, d_offset)
    }
}

impl Clone for VirtualOffset {
    fn clone(&self) -> Self {
        VirtualOffset {
            virtual_pointer: self.virtual_pointer,
            block_offset: self.block_offset,
            decompressed_offset: self.decompressed_offset
        }
    }
}

impl Ord for VirtualOffset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.virtual_pointer.cmp(&other.virtual_pointer)
    }
}

impl PartialOrd for VirtualOffset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for VirtualOffset {
    fn eq(&self, other: &Self) -> bool {
        self.virtual_pointer == other.virtual_pointer
    }
}

impl Eq for VirtualOffset {}

impl Copy for VirtualOffset {}