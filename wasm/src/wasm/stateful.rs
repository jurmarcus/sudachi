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

//! Stateful tokenizer wrapper for WASM

use std::sync::Arc;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Function;

use crate::analysis::stateful_tokenizer::StatefulTokenizer;
use crate::config::Config;
use crate::dic::dictionary::JapaneseDictionary;
use crate::prelude::MorphemeList;

use super::fetch::{load_dict_from_path, load_dict_from_url, load_dict_from_url_with_progress};
use super::types::{TokenMorpheme, TokenizeMode};
use super::{create_config_and_dictionary, WasmError, MAX_INPUT_BYTES};

/// Implementation of a Tokenizer which has tokenization state.
///
/// Useful when you don't need to specify TokenizeMode and/or debug every time.
/// The mode is set once at initialization and can be changed via the `mode` setter.
#[wasm_bindgen]
pub struct SudachiStateful {
    tokenizer: Option<StatefulTokenizer<Arc<JapaneseDictionary>>>,
    config: Option<Config>,
    morpheme_list: Option<MorphemeList<Arc<JapaneseDictionary>>>,
}

#[wasm_bindgen]
impl SudachiStateful {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tokenizer: None,
            config: None,
            morpheme_list: None,
        }
    }

    /// Initializes the tokenizer with the given dictionary file url. If not given, the default one is used.
    ///
    /// Returns an error object `{ error: string, details: string }` if initialization fails.
    #[wasm_bindgen]
    pub async fn initialize_browser(
        &mut self,
        mode: TokenizeMode,
        debug: Option<bool>,
        dict_url: Option<String>,
    ) -> Result<(), JsValue> {
        let dict_bytes = load_dict_from_url(dict_url).await?;
        self.initialize_from_bytes(dict_bytes.as_slice(), mode, debug)
    }

    /// Initializes the tokenizer with progress callback.
    ///
    /// The callback receives `{ loaded: number, total: number, percent: number, cached: boolean }`.
    /// Returns an error object `{ error: string, details: string }` if initialization fails.
    #[wasm_bindgen]
    pub async fn initialize_browser_with_progress(
        &mut self,
        mode: TokenizeMode,
        debug: Option<bool>,
        dict_url: Option<String>,
        progress_callback: Function,
    ) -> Result<(), JsValue> {
        let dict_bytes = load_dict_from_url_with_progress(dict_url, progress_callback).await?;
        self.initialize_from_bytes(dict_bytes.as_slice(), mode, debug)
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
        mode: TokenizeMode,
        debug: Option<bool>,
        dict_path: Option<String>,
    ) -> Result<(), JsValue> {
        let dict_bytes = load_dict_from_path(dict_path, read_file_func).await?;
        self.initialize_from_bytes(dict_bytes.as_slice(), mode, debug)
    }

    /// Initializes the tokenizer with the given bytes.
    /// Returns an error object `{ error: string, details: string }` if initialization fails.
    #[wasm_bindgen]
    pub fn initialize_from_bytes(
        &mut self,
        dict_bytes: &[u8],
        mode: TokenizeMode,
        debug: Option<bool>,
    ) -> Result<(), JsValue> {
        let (config, dictionary) = create_config_and_dictionary(dict_bytes)?;
        let debug = debug.unwrap_or(false);

        let tokenizer = StatefulTokenizer::create(dictionary, debug, mode.into());
        let morpheme_list = MorphemeList::empty(tokenizer.dict_clone());

        self.tokenizer = Some(tokenizer);
        self.config = Some(config);
        self.morpheme_list = Some(morpheme_list);

        Ok(())
    }

    /// Internal method to tokenize the input string.
    fn _tokenize(
        &mut self,
        input: String,
        mode: Option<TokenizeMode>,
    ) -> Result<Vec<TokenMorpheme>, JsValue> {
        let mode = mode.map(|v| v.into());

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

        let tokenizer = self.tokenizer.as_mut().ok_or_else(|| {
            JsValue::from(WasmError {
                error: "InitializationError".to_string(),
                details: "Tokenizer not initialized. Call initialize() first.".to_string(),
            })
        })?;
        let morpheme_list = self.morpheme_list.as_mut().ok_or_else(|| {
            JsValue::from(WasmError {
                error: "InitializationError".to_string(),
                details: "Morpheme List not initialized. Call initialize() first.".to_string(),
            })
        })?;

        let previous_mode = mode.map(|m| tokenizer.set_mode(m));
        let mut tokenizer = scopeguard::guard(tokenizer, |t| {
            if let Some(m) = previous_mode {
                t.set_mode(m);
            }
        });

        tokenizer.reset().push_str(&input);
        tokenizer.do_tokenize().map_err(|e| {
            JsValue::from(WasmError {
                error: "TokenizationError".to_string(),
                details: format!("Failed to tokenize: {}", e),
            })
        })?;

        morpheme_list.collect_results(&mut tokenizer).map_err(|e| {
            JsValue::from(WasmError {
                error: "TokenizationError".to_string(),
                details: format!("Failed to get morpheme list: {}", e),
            })
        })?;

        let described_morphemes = morpheme_list
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

    /// Tokenizes the input string using the defined mode from TokenizeMode at initialization.
    /// If a mode is provided, that mode will be used temporarily until the end of the function execution.
    ///
    /// Returns a JSON string of morphemes on success, or an error object `{ error: string, details: string }` on failure.
    #[wasm_bindgen]
    pub fn tokenize_stringified(
        &mut self,
        input: String,
        mode: Option<TokenizeMode>,
    ) -> Result<String, JsValue> {
        let described_morphemes = self._tokenize(input, mode)?;

        serde_json::to_string(&described_morphemes).map_err(|e| {
            JsValue::from(WasmError {
                error: "SerializationError".to_string(),
                details: format!("Failed to serialize morphemes: {}", e),
            })
        })
    }

    /// Tokenizes the input string using the defined mode from TokenizeMode at initialization.
    /// If a mode is provided, that mode will be used temporarily until the end of the function execution.
    ///
    /// Returns an array of morpheme objects on success, or an error object `{ error: string, details: string }` on failure.
    #[wasm_bindgen]
    pub fn tokenize_raw(
        &mut self,
        input: String,
        mode: Option<TokenizeMode>,
    ) -> Result<Vec<TokenMorpheme>, JsValue> {
        self._tokenize(input, mode)
    }

    /// Tokenizes multiple input strings in a single call.
    ///
    /// More efficient than calling tokenize_raw multiple times due to reduced JS<->WASM overhead.
    /// Uses the mode set at initialization, or temporarily overrides if mode is provided.
    /// Returns an array of arrays of morpheme objects on success.
    #[wasm_bindgen]
    pub fn tokenize_batch(
        &mut self,
        inputs: Vec<String>,
        mode: Option<TokenizeMode>,
    ) -> Result<JsValue, JsValue> {
        let mut results: Vec<Vec<TokenMorpheme>> = Vec::with_capacity(inputs.len());
        for input in inputs {
            results.push(self._tokenize(input, mode)?);
        }

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
        self.morpheme_list = None;
    }

    #[wasm_bindgen]
    pub fn is_initialized(&self) -> bool {
        self.tokenizer.is_some()
    }

    /// SplitMode of the tokenizer.
    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> Result<TokenizeMode, JsValue> {
        let tokenizer = self.tokenizer.as_ref().ok_or_else(|| {
            JsValue::from(WasmError {
                error: "InitializationError".to_string(),
                details: "Tokenizer not initialized. Call initialize() first.".to_string(),
            })
        })?;
        Ok(tokenizer.mode().into())
    }

    #[wasm_bindgen(setter)]
    pub fn set_mode(&mut self, mode: TokenizeMode) -> Result<(), JsValue> {
        let tokenizer = self.tokenizer.as_mut().ok_or_else(|| {
            JsValue::from(WasmError {
                error: "InitializationError".to_string(),
                details: "Tokenizer not initialized. Call initialize() first.".to_string(),
            })
        })?;
        tokenizer.set_mode(mode.into());
        Ok(())
    }

    /// Debug mode of the tokenizer.
    #[wasm_bindgen(getter)]
    pub fn debug(&self) -> Result<bool, JsValue> {
        let tokenizer = self.tokenizer.as_ref().ok_or_else(|| {
            JsValue::from(WasmError {
                error: "InitializationError".to_string(),
                details: "Tokenizer not initialized. Call initialize() first.".to_string(),
            })
        })?;
        Ok(tokenizer.debug())
    }

    #[wasm_bindgen(setter)]
    pub fn set_debug(&mut self, debug: bool) -> Result<(), JsValue> {
        let tokenizer = self.tokenizer.as_mut().ok_or_else(|| {
            JsValue::from(WasmError {
                error: "InitializationError".to_string(),
                details: "Tokenizer not initialized. Call initialize() first.".to_string(),
            })
        })?;
        tokenizer.set_debug(debug);
        Ok(())
    }
}

impl Default for SudachiStateful {
    fn default() -> Self {
        Self::new()
    }
}
