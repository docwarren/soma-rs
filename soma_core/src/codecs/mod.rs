/// Individual codec implementations used internally by the index parsers.
pub mod gzip;
/// BGZF (Blocked GZIP Format) used by BAM and bgzipped tabix files.
pub mod bgzip;
pub mod deflate;
pub mod zlib;

use flate2::read::{GzDecoder, ZlibDecoder, DeflateDecoder};
use std::io::Read;

/// Decompresses `compressed_data` by inspecting the magic bytes to identify the format.
///
/// | Magic bytes | Format |
/// |-------------|--------|
/// | `0x1f 0x8b` | Gzip |
/// | `0x78 {0x01,0x5e,0x9c,0xda}` | Zlib |
/// | Everything else | Raw deflate (best-effort; falls back to returning the data unchanged) |
///
/// Bzip2 (`0x42 0x5a`), Zstd (`0x28 0xb5 0x2f 0xfd`), and XZ/LZMA formats are
/// recognised but not supported — an error is returned for those.
///
/// If `compressed_data` is shorter than 2 bytes, or raw-deflate decoding fails,
/// the input is returned unchanged (assumed to be uncompressed).
///
/// # Errors
///
/// Returns an error when an unsupported compression format is detected, or when
/// a supported decompressor encounters a malformed stream.
pub fn decompress_auto(compressed_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
   if compressed_data.len() < 2 {
       return Ok(compressed_data.to_vec());
   }
   
   let first_two = (compressed_data[0], compressed_data[1]);
   let first_four = if compressed_data.len() >= 4 {
       [compressed_data[0], compressed_data[1], compressed_data[2], compressed_data[3]]
   } else {
       [0, 0, 0, 0]
   };
   
   match first_two {
       // Gzip magic number
       (0x1f, 0x8b) => {
           let mut decoder = GzDecoder::new(compressed_data);
           let mut decompressed = Vec::new();
           decoder.read_to_end(&mut decompressed)?;
           Ok(decompressed)
       },
       
       // Zlib magic numbers (0x78 followed by various flags)
       (0x78, 0x01) | (0x78, 0x5e) | (0x78, 0x9c) | (0x78, 0xda) => {
           let mut decoder = ZlibDecoder::new(compressed_data);
           let mut decompressed = Vec::new();
           decoder.read_to_end(&mut decompressed)?;
           Ok(decompressed)
       },
       
       // Bzip2 magic number
       (0x42, 0x5a) => {
           return Err("Bzip2 compression not supported".into());
       },
       
       // Zstd magic number (first 4 bytes: 0x28, 0xb5, 0x2f, 0xfd)
       _ if first_four == [0x28, 0xb5, 0x2f, 0xfd] => {
           return Err("Zstd compression not supported".into());
       },
       
       // XZ/LZMA magic number (first 6 bytes start with 0xfd, 0x37, 0x7a, 0x58, 0x5a)
       _ if compressed_data.len() >= 6 && 
            compressed_data[0..6] == [0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00] => {
           return Err("XZ/LZMA compression not supported".into());
       },
       
       // Try raw deflate as fallback
       _ => {
           match DeflateDecoder::new(compressed_data).read_to_end(&mut Vec::new()) {
               Ok(_) => {
                   let mut decoder = DeflateDecoder::new(compressed_data);
                   let mut decompressed = Vec::new();
                   decoder.read_to_end(&mut decompressed)?;
                   Ok(decompressed)
               },
               Err(_) => {
                   // If all decompression methods fail, return data as-is
                   // (might not actually be compressed)
                   Ok(compressed_data.to_vec())
               }
           }
       }
   }
}