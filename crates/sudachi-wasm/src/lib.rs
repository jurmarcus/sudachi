use std::sync::Arc;

use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi_search::{CompoundWord, SearchToken, SearchTokenizer};
use wasm_bindgen::prelude::*;

/// A Sudachi tokenizer for Japanese text, usable from JavaScript.
///
/// # Example (JavaScript)
///
/// ```js
/// const response = await fetch('/path/to/system_full.dic');
/// const dictBytes = new Uint8Array(await response.arrayBuffer());
/// const tokenizer = new SudachiTokenizer(dictBytes);
///
/// const tokens = tokenizer.tokenize("東京都立大学で研究");
/// // [{ surface: "東京都立大学", isColocated: false },
/// //  { surface: "東京",        isColocated: true  },
/// //  { surface: "都立",        isColocated: true  },
/// //  { surface: "大学",        isColocated: true  },
/// //  { surface: "で",          isColocated: false },
/// //  { surface: "研究",        isColocated: false }]
/// ```
#[wasm_bindgen]
pub struct SudachiTokenizer {
    inner: SearchTokenizer,
}

#[wasm_bindgen]
impl SudachiTokenizer {
    /// Create a tokenizer from dictionary bytes (Uint8Array from JS).
    ///
    /// The dictionary bytes are typically fetched via `fetch()` or bundled
    /// with `include_bytes!`. The caller is responsible for loading the bytes.
    #[wasm_bindgen(constructor)]
    pub fn new(dict_bytes: &[u8]) -> Result<SudachiTokenizer, JsError> {
        let dictionary = JapaneseDictionary::from_system_bytes(dict_bytes.to_vec())
            .map_err(|e| JsError::new(&format!("Failed to load Sudachi dictionary: {e}")))?;
        Ok(SudachiTokenizer {
            inner: SearchTokenizer::new(Arc::new(dictionary)),
        })
    }

    /// Tokenize Japanese text using B+C multi-granularity (Search mode).
    ///
    /// Returns a JS Array of `{ surface, byteStart, byteEnd, isColocated }` objects.
    /// Colocated tokens share the same position as the previous token — they are
    /// sub-tokens of a compound word.
    pub fn tokenize(&self, text: &str) -> Result<JsValue, JsError> {
        let tokens = self
            .inner
            .tokenize(text)
            .map_err(|e| JsError::new(&format!("Tokenization failed: {e}")))?;

        let js_tokens: Vec<JsToken> = tokens.into_iter().map(JsToken::from).collect();
        serde_wasm_bindgen::to_value(&js_tokens)
            .map_err(|e| JsError::new(&format!("Serialization failed: {e}")))
    }

    /// Tokenize and return only surface forms (no position metadata).
    /// Convenient for simple use cases.
    pub fn tokenize_surfaces(&self, text: &str) -> Result<JsValue, JsError> {
        let tokens = self
            .inner
            .tokenize(text)
            .map_err(|e| JsError::new(&format!("Tokenization failed: {e}")))?;

        let surfaces: Vec<&str> = tokens
            .iter()
            .filter(|t| !t.is_colocated)
            .map(|t| t.surface.as_str())
            .collect();

        serde_wasm_bindgen::to_value(&surfaces)
            .map_err(|e| JsError::new(&format!("Serialization failed: {e}")))
    }

    /// Detect compound words in text.
    ///
    /// Returns a JS Array of `{ surface, components, byteStart, byteEnd }` objects.
    /// Only returns tokens that are compound words (more than one component).
    pub fn detect_compounds(&self, text: &str) -> Result<JsValue, JsError> {
        let compounds = self
            .inner
            .detect_compounds(text)
            .map_err(|e| JsError::new(&format!("Compound detection failed: {e}")))?;

        let js_compounds: Vec<JsCompound> = compounds.into_iter().map(JsCompound::from).collect();
        serde_wasm_bindgen::to_value(&js_compounds)
            .map_err(|e| JsError::new(&format!("Serialization failed: {e}")))
    }

    /// Use surface form instead of normalized form.
    /// Returns a new tokenizer configured for surface form output.
    pub fn with_surface_form(self) -> SudachiTokenizer {
        SudachiTokenizer {
            inner: self.inner.with_surface_form(),
        }
    }
}

/// Token data returned to JavaScript.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsToken {
    surface: String,
    byte_start: usize,
    byte_end: usize,
    /// True if this token shares position with the previous token.
    /// Used for B+C multi-granularity: sub-tokens of compound words are colocated.
    is_colocated: bool,
}

impl From<SearchToken> for JsToken {
    fn from(t: SearchToken) -> Self {
        JsToken {
            surface: t.surface,
            byte_start: t.byte_start,
            byte_end: t.byte_end,
            is_colocated: t.is_colocated,
        }
    }
}

/// Compound word data returned to JavaScript.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsCompound {
    surface: String,
    components: Vec<String>,
    byte_start: usize,
    byte_end: usize,
}

impl From<CompoundWord> for JsCompound {
    fn from(c: CompoundWord) -> Self {
        JsCompound {
            surface: c.surface,
            components: c.components,
            byte_start: c.byte_start,
            byte_end: c.byte_end,
        }
    }
}
