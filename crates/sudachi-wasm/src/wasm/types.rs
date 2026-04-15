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

//! Type definitions for WASM bindings

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::analysis::stateless_tokenizer::DictionaryAccess;
use crate::analysis::Mode;
use crate::prelude::Morpheme;
use crate::sentence_splitter::{SentenceSplitter, SplitSentences};

use super::MAX_INPUT_BYTES;

/// Module version and features info
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct ModuleInfo {
    /// Version of sudachi-wasm
    pub version: String,
    /// Rust version used to compile
    pub rust_version: String,
    /// Supported compression formats
    pub compression_formats: Vec<String>,
    /// Whether IndexedDB caching is available
    pub indexeddb_cache: bool,
    /// Maximum input size in bytes
    pub max_input_bytes: usize,
}

/// Returns information about the sudachi-wasm module.
#[wasm_bindgen]
pub fn get_module_info() -> ModuleInfo {
    ModuleInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        rust_version: env!("CARGO_PKG_RUST_VERSION").to_string(),
        compression_formats: vec!["gzip".to_string(), "brotli".to_string()],
        indexeddb_cache: true,
        max_input_bytes: MAX_INPUT_BYTES,
    }
}

/// Split Japanese text into sentences.
///
/// This function detects sentence boundaries in Japanese text using
/// punctuation marks (etc.) and other heuristics. It does NOT
/// require a dictionary to be loaded.
///
/// # Arguments
/// * `input` - The Japanese text to split
/// * `limit` - Optional maximum characters to process at once (default: 4096)
///
/// # Returns
/// A vector of sentence strings
///
/// # Example
/// ```javascript
/// const sentences = split_sentences("", "");
/// // ["", ""]
/// ```
#[wasm_bindgen]
pub fn split_sentences(input: String, limit: Option<usize>) -> Vec<String> {
    let splitter = match limit {
        Some(l) => SentenceSplitter::with_limit(l),
        None => SentenceSplitter::new(),
    };

    splitter
        .split(&input)
        .map(|(_, text)| text.to_string())
        .collect()
}

/// Unit to split text
///
/// Some examples:
/// ```text
/// A://///
/// B:///
/// C:
///
/// A:///
/// B://
/// C:
///
/// A:////
/// B:///
/// C:
///
/// A:///
/// B://
/// C:
/// ```
///
/// See [Sudachi documentation](https://github.com/WorksApplications/Sudachi#the-modes-of-splitting)
/// for more details
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenizeMode {
    /// Short
    A,

    /// Middle (similar to "word")
    B,

    /// Named Entity
    C,
}

impl From<TokenizeMode> for Mode {
    fn from(mode: TokenizeMode) -> Self {
        match mode {
            TokenizeMode::A => Mode::A,
            TokenizeMode::B => Mode::B,
            TokenizeMode::C => Mode::C,
        }
    }
}

impl From<Mode> for TokenizeMode {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::A => TokenizeMode::A,
            Mode::B => TokenizeMode::B,
            Mode::C => TokenizeMode::C,
        }
    }
}

/// A morpheme (token) from Sudachi tokenization.
///
/// Contains all information about a single token including surface form,
/// part of speech, readings, and dictionary information.
#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenMorpheme {
    /// The surface form (as it appears in the text)
    pub surface: String,
    /// Part of speech tags
    pub poses: Vec<String>,
    /// Normalized form (standardized spelling)
    pub normalized_form: String,
    /// Reading in katakana
    pub reading_form: String,
    /// Dictionary/lemma form
    pub dictionary_form: String,
    /// Internal word ID
    pub word_id: i32,
    /// Whether this is an out-of-vocabulary word
    pub oov: bool,
    /// Byte offset of start in original text
    pub begin: usize,
    /// Byte offset of end in original text
    pub end: usize,
    /// Dictionary ID (-1 for OOV words)
    pub dictionary_id: i32,
    /// Synonym group IDs (may be empty)
    pub synonym_group_ids: Vec<u32>,
    /// Word IDs for mode A split (finer granularity)
    pub a_unit_split: Vec<i32>,
    /// Word IDs for mode B split (medium granularity)
    pub b_unit_split: Vec<i32>,
}

impl<'a, D: DictionaryAccess> From<Morpheme<'a, D>> for TokenMorpheme {
    fn from(morpheme: Morpheme<'a, D>) -> Self {
        let word_info = morpheme.get_word_info();
        Self {
            surface: morpheme.surface().to_string(),
            poses: morpheme.part_of_speech().to_vec(),
            normalized_form: morpheme.normalized_form().to_string(),
            reading_form: morpheme.reading_form().to_string(),
            dictionary_form: morpheme.dictionary_form().to_string(),
            word_id: morpheme.word_id().as_raw() as i32,
            oov: morpheme.is_oov(),
            begin: morpheme.begin(),
            end: morpheme.end(),
            dictionary_id: morpheme.dictionary_id(),
            synonym_group_ids: morpheme.synonym_group_ids().to_vec(),
            a_unit_split: word_info
                .a_unit_split()
                .iter()
                .map(|w| w.as_raw() as i32)
                .collect(),
            b_unit_split: word_info
                .b_unit_split()
                .iter()
                .map(|w| w.as_raw() as i32)
                .collect(),
        }
    }
}
