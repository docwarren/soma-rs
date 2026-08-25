// Copyright 2026 Seqa23
//
// Author: Andrew Warren
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// BGZF (Blocked GZIP Format) used by BAM and bgzipped tabix files.
pub mod bgzip;
pub mod codec_error;
pub mod deflate;
/// Individual codec implementations used internally by the index parsers.
pub mod gzip;
pub mod zlib;

use crate::codecs::codec_error::CodecError;
use crate::codecs::deflate::decompress_deflate;
use crate::codecs::gzip::gzip_decompress;
use crate::codecs::zlib::decompress_zlib;

/// Decompresses `compressed_data`
/// Checks the header for the compression type. If its not bgzip then an error is thrown
/// # Errors
///
/// Returns an error when an unsupported compression format is detected, or when
/// a supported decompressor encounters a malformed stream.
pub fn decompress_auto(compressed_data: &[u8]) -> Result<Vec<u8>, CodecError> {
    if compressed_data.len() < 2 {
        return Ok(compressed_data.to_vec());
    }

    let mut first_two: [u8; 2] = [0, 0];
    first_two.copy_from_slice(&compressed_data[0..2]);

    let mut first_four: [u8; 4] = [0, 0, 0, 0];
    first_four.copy_from_slice(&compressed_data[0..4]);

    let mut first_six: [u8; 6] = [0, 0, 0, 0, 0, 0];
    first_six.copy_from_slice(&compressed_data[0..6]);

    match first_two {
        // Gzip magic number
        [0x1f, 0x8b] => gzip_decompress(compressed_data),

        // Zlib magic numbers (0x78 followed by various flags)
        [0x78, 0x01] | [0x78, 0x5e] | [0x78, 0x9c] | [0x78, 0xda] => {
            decompress_zlib(compressed_data)
        }

        // Bzip2 magic number
        [0x42, 0x5a] => Err(CodecError::CodecNotSupported("Bzip2".to_string())),

        // ZStd magic number (first 4 bytes: 0x28, 0xb5, 0x2f, 0xfd)
        _ if first_four == [0x28, 0xb5, 0x2f, 0xfd] => {
            Err(CodecError::CodecNotSupported("ZStd".to_string()))
        }

        // XZ/LZMA magic number (first 6 bytes start with 0xfd, 0x37, 0x7a, 0x58, 0x5a)
        _ if first_six == [0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00] => {
            Err(CodecError::CodecNotSupported("XZ/LZMA".to_string()))
        }
        // Try raw deflate as fallback
        _ => decompress_deflate(compressed_data),
    }
}
