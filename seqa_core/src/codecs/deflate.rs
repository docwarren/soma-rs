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

use flate2::read::DeflateDecoder;
use std::io::Read;
use crate::codecs::codec_error::CodecError;

pub fn decompress_deflate(compressed_data: &[u8]) -> Result<Vec<u8>, CodecError> {
    let mut decoder = DeflateDecoder::new(compressed_data);
    let mut decompressed_data = Vec::new();
    let result = decoder.read_to_end(&mut decompressed_data);
    match result {
        Ok(_) => Ok(decompressed_data),
        Err(e) => Err(CodecError::ReaderError {
            decoder_used: "Deflate".to_string(),
            source: e
        }),
    }
}