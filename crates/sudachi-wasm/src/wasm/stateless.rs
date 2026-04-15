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

//! Stateless tokenizer wrapper for WASM

use std::sync::Arc;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Function;

use crate::analysis::stateless_tokenizer::StatelessTokenizer;
use crate::analysis::Tokenize;
use crate::config::Config;
use crate::dic::dictionary::JapaneseDictionary;

use super::fetch::{load_dict_from_path, load_dict_from_url, load_dict_from_url_with_progress};
use super::types::{TokenMorpheme, TokenizeMode};
use super::{create_config_and_dictionary, WasmError, MAX_INPUT_BYTES};

/// Implementation of a Tokenizer which does not have tokenization state.
///
/// This is a wrapper which is generic over dictionary pointers.
/// Use this when you need to tokenize with different modes frequently.
#[wasm_bindgen]
pub struct SudachiStateless {
    tokenizer: Option<StatelessTokenizer<Arc<JapaneseDictionary>>>,
    config: Option<Config>,
}

#[wasm_bindgen]
impl SudachiStateless {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tokenizer: None,
            config: None,
        }
    }

    /// Initializes the tokenizer with the given dictionary file url. If not given, the default one is used.
    ///
    /// Returns an error object `{ error: string, details: string }` if initialization fails.
    #[wasm_bindgen]
    pub async fn initialize_browser(&mut self, dict_url: Option<String>) -> Result<(), JsValue> {
        let dict_bytes = load_dict_from_url(dict_url).await?;
        self.initialize_from_bytes(dict_bytes.as_slice())
    }

    /// Initializes the tokenizer with progress callback.
    ///
    /// The callback receives `{ loaded: number, total: number, percent: number, cached: boolean }`.
    /// Returns an error object `{ error: string, details: string }` if initialization fails.
    #[wasm_bindgen]
    pub async fn initialize_browser_with_progress(
        &mut self,
        dict_url: Option<String>,
        progress_callback: Function,
    ) -> Result<(), JsValue> {
        let dict_bytes = load_dict_from_url_with_progress(dict_url, progress_callback).await?;
        self.initialize_from_bytes(dict_bytes.as_slice())
    }

    /// Initializes the tokenizer with the given dictionary file path. If not given, the default one is used.
    ///
    /// Uses the provided `read_file_func` function (e.g., fs.readFileSync) to load the file.
    ///
    /// Returns an error object `{ error: string, details: string }` if initialization fails.
    #[wasm_bindgen]
    pub async fn initialize_node(
        &mut self,
        read_file_func: Function,
        dict_path: Option<String>,
    ) -> Result<(), JsValue> {
        let dict_bytes = load_dict_from_path(dict_path, read_file_func).await?;
        self.initialize_from_bytes(dict_bytes.as_slice())
    }

    /// Initializes the tokenizer with the given bytes.
    /// Returns an error object `{ error: string, details: string }` if initialization fails.
    #[wasm_bindgen]
    pub fn initialize_from_bytes(&mut self, dict_bytes: &[u8]) -> Result<(), JsValue> {
        let (config, dictionary) = create_config_and_dictionary(dict_bytes)?;
        let tokenizer = StatelessTokenizer::new(dictionary);

        self.tokenizer = Some(tokenizer);
        self.config = Some(config);

        Ok(())
    }

    /// Internal method to tokenize the input string.
    fn _tokenize(
        &self,
        input: String,
        mode: TokenizeMode,
        enable_debug: Option<bool>,
    ) -> Result<Vec<TokenMorpheme>, JsValue> {
        let enable_debug = enable_debug.unwrap_or(false);

        if input.len() > MAX_INPUT_BYTES {
            return Err(JsValue::from(WasmError {
                error: "InputTooLarge".to_string(),
                details: format!(
                    "Input size {} bytes exceeds limit of {} bytes",
                    input.len(),
                    MAX_INPUT_BYTES
                ),
            }));
        }

        let tokenizer = self.tokenizer.as_ref().ok_or_else(|| {
            JsValue::from(WasmError {
                error: "InitializationError".to_string(),
                details: "Tokenizer not initialized. Call initialize() first.".to_string(),
            })
        })?;

        let morphemes = tokenizer
            .tokenize(&input, mode.into(), enable_debug)
            .map_err(|e| {
                JsValue::from(WasmError {
                    error: "TokenizationError".to_string(),
                    details: format!("Failed to tokenize: {}", e),
                })
            })?;

        let described_morphemes = morphemes
            .iter()
            .map(|m| Ok(TokenMorpheme::from(m)))
            .collect::<Result<Vec<_>, std::string::FromUtf8Error>>()
            .map_err(|e| {
                JsValue::from(WasmError {
                    error: "TokenizationError".to_string(),
                    details: format!("Failed to process morphemes: {}", e),
                })
            })?;

        Ok(described_morphemes)
    }

    /// Tokenizes the input string using the specified mode from TokenizeMode.
    ///
    /// Returns a JSON string of morphemes on success, or an error object `{ error: string, details: string }` on failure.
    #[wasm_bindgen]
    pub fn tokenize_stringified(
        &self,
        input: String,
        mode: TokenizeMode,
        enable_debug: Option<bool>,
    ) -> Result<String, JsValue> {
        let described_morphemes = self._tokenize(input, mode, enable_debug)?;

        serde_json::to_string(&described_morphemes).map_err(|e| {
            JsValue::from(WasmError {
                error: "SerializationError".to_string(),
                details: format!("Failed to serialize morphemes: {}", e),
            })
        })
    }

    /// Tokenizes the input string using the specified mode from TokenizeMode.
    ///
    /// Returns an array of morpheme objects on success, or an error object `{ error: string, details: string }` on failure.
    #[wasm_bindgen]
    pub fn tokenize_raw(
        &self,
        input: String,
        mode: TokenizeMode,
        enable_debug: Option<bool>,
    ) -> Result<Vec<TokenMorpheme>, JsValue> {
        self._tokenize(input, mode, enable_debug)
    }

    /// Tokenizes multiple input strings in a single call.
    ///
    /// More efficient than calling tokenize_raw multiple times due to reduced JS<->WASM overhead.
    /// Returns an array of arrays of morpheme objects on success.
    #[wasm_bindgen]
    pub fn tokenize_batch(
        &self,
        inputs: Vec<String>,
        mode: TokenizeMode,
    ) -> Result<JsValue, JsValue> {
        let results: Vec<Vec<TokenMorpheme>> = inputs
            .into_iter()
            .map(|input| self._tokenize(input, mode, None))
            .collect::<Result<Vec<_>, _>>()?;

        serde_wasm_bindgen::to_value(&results).map_err(|e| {
            JsValue::from(WasmError {
                error: "SerializationError".to_string(),
                details: format!("Failed to serialize batch results: {}", e),
            })
        })
    }

    /// Resets the tokenizer, uninitializing it.
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.tokenizer = None;
        self.config = None;
    }

    #[wasm_bindgen]
    pub fn is_initialized(&self) -> bool {
        self.tokenizer.is_some()
    }
}

impl Default for SudachiStateless {
    fn default() -> Self {
        Self::new()
    }
}
