/*
 *  Copyright (c) 2021 Works Applications Co., Ltd.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

//! Dictionary decompression utilities
//!
//! Supports gzip and Brotli compression formats. Gzip is detected by magic bytes,
//! while Brotli is detected by file extension (`.br`) since it has no magic bytes.

use std::io::Read;
use wasm_bindgen::prelude::*;

use super::WasmError;

/// Compression type for dictionary files
pub enum CompressionType {
    None,
    Gzip,
    Brotli,
}

/// Check if data is gzip-compressed (magic bytes 0x1f, 0x8b)
pub fn is_gzip(data: &[u8]) -> bool {
    data.len() >= 2 && data[0] == 0x1f && data[1] == 0x8b
}

/// Detect compression type from URL extension and data.
///
/// Brotli has no magic bytes, so we rely on file extension.
fn detect_compression(url: &str, data: &[u8]) -> CompressionType {
    // Check file extension first (required for Brotli)
    let url_lower = url.to_lowercase();
    if url_lower.ends_with(".br") {
        return CompressionType::Brotli;
    }
    if url_lower.ends_with(".gz") {
        return CompressionType::Gzip;
    }
    // Fall back to magic byte detection for gzip
    if is_gzip(data) {
        return CompressionType::Gzip;
    }
    CompressionType::None
}

/// Decompress Brotli data
fn decompress_brotli(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    use brotli::Decompressor;

    log::info!(
        "Detected Brotli-compressed dictionary ({} bytes), decompressing...",
        data.len()
    );

    let mut decoder = Decompressor::new(data, 4096);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| {
        JsValue::from(WasmError {
            error: "DecompressionError".to_string(),
            details: format!("Failed to decompress Brotli dictionary: {}", e),
        })
    })?;

    log::info!("Decompressed dictionary: {} bytes", decompressed.len());
    Ok(decompressed)
}

/// Decompress gzip data
pub(crate) fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    use flate2::read::GzDecoder;

    log::info!(
        "Detected gzip-compressed dictionary ({} bytes), decompressing...",
        data.len()
    );

    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| {
        JsValue::from(WasmError {
            error: "DecompressionError".to_string(),
            details: format!("Failed to decompress gzip dictionary: {}", e),
        })
    })?;

    log::info!("Decompressed dictionary: {} bytes", decompressed.len());
    Ok(decompressed)
}

/// Decompress dictionary data based on URL extension and magic bytes.
///
/// # Arguments
/// * `url` - URL or path used to detect compression type by extension
/// * `data` - Raw dictionary bytes
///
/// # Returns
/// Decompressed bytes, or the original data if not compressed
pub fn decompress_dictionary(url: &str, data: &[u8]) -> Result<Vec<u8>, JsValue> {
    match detect_compression(url, data) {
        CompressionType::Brotli => decompress_brotli(data),
        CompressionType::Gzip => decompress_gzip(data),
        CompressionType::None => Ok(data.to_vec()),
    }
}
