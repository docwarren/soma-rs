pub trait Overlaps {
    fn overlaps(&self, chr_id1: u32, chr_id2: u32, start: u32, end: u32) -> bool;
}