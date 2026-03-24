use std::fs::File;

pub trait WriteIndex {
    fn get_keys(&self) -> Vec<String>;
    fn write_size(&self, file_out: &mut File) -> Result<(), String>;
    fn write_header(&self, file_out: &mut File) -> Result<(), String>;
    fn write_offsets(&self, file_out: &mut File) -> Result<(), String>;
    fn get_byte_count(&self) -> u64;
}