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

//! WASM bindings for sudachi-rs
//!
//! This module provides WebAssembly bindings for the Sudachi Japanese morphological analyzer.

mod compression;
mod error;
mod fetch;
mod stateful;
mod stateless;
mod types;

// Internal imports
use compression::is_gzip;

// Re-export public types for wasm_bindgen
pub use error::WasmError;
pub use stateful::SudachiStateful;
pub use stateless::SudachiStateless;
pub use types::{get_module_info, split_sentences, ModuleInfo, TokenMorpheme, TokenizeMode};

use std::sync::Arc;

use crate::config::Config;
use crate::dic::dictionary::JapaneseDictionary;
use crate::dic::storage::{Storage, SudachiDicData};
use wasm_bindgen::prelude::*;

/// Maximum input size in bytes (10MB)
pub const MAX_INPUT_BYTES: usize = 10_000_000;

/// Creates Config and JapaneseDictionary from raw dictionary bytes.
///
/// Handles gzip decompression for backwards compatibility with `initialize_from_bytes`.
/// Brotli-compressed data must be decompressed before calling this function.
pub(crate) fn create_config_and_dictionary(
    dict_bytes: &[u8],
) -> Result<(Config, Arc<JapaneseDictionary>), JsValue> {
    if dict_bytes.is_empty() {
        return Err(JsValue::from(WasmError {
            error: "InvalidDictionary".to_string(),
            details: "Dictionary bytes cannot be empty".to_string(),
        }));
    }

    // Decompress if gzip-compressed (for backwards compat with initialize_from_bytes)
    // Brotli requires URL context so must be decompressed before calling this
    let dict_bytes = if is_gzip(dict_bytes) {
        compression::decompress_gzip(dict_bytes)?
    } else {
        dict_bytes.to_vec()
    };

    let config = Config::new_embedded().map_err(|e| {
        JsValue::from(WasmError {
            error: "ConfigInitError".to_string(),
            details: format!("Failed to initialize config: {}", e),
        })
    })?;

    let storage = SudachiDicData::new(Storage::Owned(dict_bytes));

    let dictionary =
        JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage).map_err(
            |e| {
                JsValue::from(WasmError {
                    error: "DictionaryLoadError".to_string(),
                    details: format!("Failed to load dictionary: {}", e),
                })
            },
        )?;

    Ok((config, Arc::new(dictionary)))
}
