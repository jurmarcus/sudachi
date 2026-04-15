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

//! Dictionary fetching and loading utilities
//!
//! Provides functions to load dictionaries from URLs (browser) or file paths (Node.js).
//! Includes IndexedDB caching for browser environments.

use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::{Function, Promise, Uint8Array};
use web_sys::{js_sys, ReadableStreamDefaultReader, Request, RequestInit, Response};

// Import global fetch (works in both Window and Worker contexts)
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch)]
    fn global_fetch(input: &Request) -> Promise;
}

use super::compression::decompress_dictionary;
use super::WasmError;
use crate::cache;

#[wasm_bindgen]
extern "C" {
    fn get_default_dic_path() -> String;
}

/// Progress info passed to progress callback
#[derive(Serialize)]
pub struct DownloadProgress {
    pub loaded: usize,
    pub total: usize,
    pub percent: f64,
    pub cached: bool,
}

/// Report download progress to callback if provided
fn report_progress(callback: &Option<Function>, loaded: usize, total: usize, cached: bool) {
    if let Some(cb) = callback {
        let percent = if total > 0 {
            (loaded as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let progress = DownloadProgress {
            loaded,
            total,
            percent,
            cached,
        };
        if let Ok(js_val) = serde_wasm_bindgen::to_value(&progress) {
            let _ = cb.call1(&JsValue::NULL, &js_val);
        }
    }
}

/// Load dictionary from a file path using a provided read function (Node.js).
///
/// # Arguments
/// * `dict_path` - Path to the dictionary file, or None for default
/// * `read_file_func` - JavaScript function to read the file (e.g., fs.readFileSync)
///
/// # Returns
/// Decompressed dictionary bytes
pub async fn load_dict_from_path(
    dict_path: Option<String>,
    read_file_func: Function,
) -> Result<Vec<u8>, JsValue> {
    let path = dict_path.unwrap_or_else(get_default_dic_path);
    let this = JsValue::NULL;
    let path_js = JsValue::from_str(&path);
    let mut result = read_file_func.call1(&this, &path_js).map_err(|e| {
        JsValue::from(WasmError {
            error: "FileReadError".to_string(),
            details: format!("Failed to call read_file_func function: {:?}", e),
        })
    })?;

    if result.is_instance_of::<Promise>() {
        let promise: Promise = result.dyn_into().map_err(|e| {
            JsValue::from(WasmError {
                error: "FileReadError".to_string(),
                details: format!("Expected a Promise from read_file_func, got: {:?}", e),
            })
        })?;

        result = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(|e| {
                JsValue::from(WasmError {
                    error: "FileReadError".to_string(),
                    details: format!("Failed to resolve read_file_func promise: {:?}", e),
                })
            })?;
    }

    if !(result.is_instance_of::<Uint8Array>()) {
        return Err(JsValue::from(WasmError {
            error: "FileReadError".to_string(),
            details: format!("Expected Uint8Array from read_file_func, got: {:?}", result),
        }));
    }
    let dict_bytes = Uint8Array::from(result).to_vec();

    // Decompress if needed (gzip or brotli based on path extension)
    decompress_dictionary(&path, &dict_bytes)
}

/// Load dictionary from URL (browser).
///
/// Uses IndexedDB caching - subsequent loads will use the cached version.
///
/// # Arguments
/// * `dict_url` - URL to fetch the dictionary from, or None for default
///
/// # Returns
/// Decompressed dictionary bytes
pub async fn load_dict_from_url(dict_url: Option<String>) -> Result<Vec<u8>, JsValue> {
    load_dict_from_url_impl(dict_url, None).await
}

/// Load dictionary from URL with progress callback (browser).
///
/// # Arguments
/// * `dict_url` - URL to fetch the dictionary from, or None for default
/// * `progress_callback` - Called with `{ loaded, total, percent, cached }` during download
///
/// # Returns
/// Decompressed dictionary bytes
pub async fn load_dict_from_url_with_progress(
    dict_url: Option<String>,
    progress_callback: Function,
) -> Result<Vec<u8>, JsValue> {
    load_dict_from_url_impl(dict_url, Some(progress_callback)).await
}

/// Internal implementation for URL loading with optional progress callback
async fn load_dict_from_url_impl(
    dict_url: Option<String>,
    progress_callback: Option<Function>,
) -> Result<Vec<u8>, JsValue> {
    let path = dict_url.unwrap_or_else(get_default_dic_path);

    // Check IndexedDB cache first
    if let Some(cached) = cache::get_cached_dict(&path).await {
        log::info!(
            "Loaded dictionary from cache: {} ({} bytes)",
            path,
            cached.len()
        );
        report_progress(&progress_callback, cached.len(), cached.len(), true);
        return Ok(cached);
    }

    log::info!("Cache miss, fetching dictionary: {}", path);

    let opts = RequestInit::new();
    opts.set_method("GET");

    let request = Request::new_with_str_and_init(&path, &opts).map_err(|e| {
        JsValue::from(WasmError {
            error: "FetchError".to_string(),
            details: format!("Failed to create fetch request: {:?}", e),
        })
    })?;

    // Use global fetch (works in both Window and Worker contexts)
    let resp_value = wasm_bindgen_futures::JsFuture::from(global_fetch(&request))
        .await
        .map_err(|e| {
            JsValue::from(WasmError {
                error: "FetchError".to_string(),
                details: format!("Failed to fetch file: {:?}", e),
            })
        })?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().map_err(|e| {
        JsValue::from(WasmError {
            error: "FetchError".to_string(),
            details: format!("Invalid fetch response: {:?}", e),
        })
    })?;

    if !resp.ok() {
        return Err(JsValue::from(WasmError {
            error: "FetchError".to_string(),
            details: format!("HTTP error: {} {}", resp.status(), resp.status_text()),
        }));
    }

    let body = resp.body().ok_or_else(|| {
        JsValue::from(WasmError {
            error: "FetchError".to_string(),
            details: "No body in response".to_string(),
        })
    })?;
    let reader: ReadableStreamDefaultReader = body.get_reader().dyn_into().map_err(|e| {
        JsValue::from(WasmError {
            error: "StreamError".to_string(),
            details: format!("Failed to cast to ReadableStreamDefaultReader: {:?}", e),
        })
    })?;

    let content_length: usize = resp
        .headers()
        .get("Content-Length")
        .and_then(|s| {
            s.unwrap_or("0".to_string())
                .parse()
                .map_err(|e| {
                    JsValue::from(WasmError {
                        error: "ParseError".to_string(),
                        details: format!("Failed to parse data: {:?}", e),
                    })
                })
        })
        .unwrap_or(0);
    let mut dict_bytes: Vec<u8> = if content_length > 0 {
        Vec::with_capacity(content_length)
    } else {
        Vec::new()
    };

    loop {
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read())
            .await
            .map_err(|e| {
                JsValue::from(WasmError {
                    error: "StreamError".to_string(),
                    details: format!("Failed to read stream chunk: {:?}", e),
                })
            })?;

        let done = js_sys::Reflect::get(&read_result, &JsValue::from_str("done"))
            .map_err(|e| {
                JsValue::from(WasmError {
                    error: "StreamError".to_string(),
                    details: format!("Failed to get 'done' from chunk: {:?}", e),
                })
            })?
            .as_bool()
            .unwrap_or(false);

        if done {
            break;
        }

        let value = js_sys::Reflect::get(&read_result, &JsValue::from_str("value")).map_err(|e| {
            JsValue::from(WasmError {
                error: "StreamError".to_string(),
                details: format!("Failed to get 'value' from chunk: {:?}", e),
            })
        })?;

        let chunk: Uint8Array = value.dyn_into().map_err(|e| {
            JsValue::from(WasmError {
                error: "StreamError".to_string(),
                details: format!("Chunk is not Uint8Array: {:?}", e),
            })
        })?;

        dict_bytes.extend_from_slice(&chunk.to_vec());

        // Report progress
        report_progress(&progress_callback, dict_bytes.len(), content_length, false);
    }

    if dict_bytes.is_empty() {
        return Err(JsValue::from(WasmError {
            error: "EmptyResponse".to_string(),
            details: "Received empty data from stream".to_string(),
        }));
    }

    // Decompress if needed (gzip or brotli based on URL extension)
    let decompressed = decompress_dictionary(&path, &dict_bytes)?;

    // Cache the decompressed dictionary for future loads
    if let Err(e) = cache::cache_dict(&path, &decompressed).await {
        log::warn!("Failed to cache dictionary: {:?}", e);
        // Continue anyway - caching is best-effort
    }

    Ok(decompressed)
}
