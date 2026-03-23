use flate2::read::DeflateDecoder;
use std::io::Read;

pub fn decompress_deflate(compressed_data: &[u8]) -> Result<Vec<u8>, std::io::Error> {

    let mut decoder = DeflateDecoder::new(compressed_data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}