// Copyright (c) 2023-2026 ParadeDB, Inc.
//
// This file is part of ParadeDB - Postgres for Search and Analytics
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Sudachi Japanese tokenizer for ParadeDB.
//!
//! Provides superior Japanese morphological analysis with B+C multi-granularity
//! tokenization for optimal full-text search.
//!
//! ## Features
//!
//! - **Search mode (default)**: B+C multi-granularity - emits both compound words
//!   AND their sub-tokens at the same position for optimal search
//! - **Mode A/B/C**: Standard Sudachi modes for different granularity levels
//! - **Normalization**: Optional normalized form for better recall
//!
//! ## Example
//!
//! ```sql
//! -- Create index with Sudachi tokenizer (Search mode, normalized by default)
//! CREATE INDEX docs_idx ON documents
//! USING bm25(id, (content::pdb.sudachi))
//! WITH (key_field='id');
//!
//! -- Search for partial matches (works due to B+C multi-granularity)
//! SELECT * FROM documents WHERE id @@@ 'content:大学';
//! -- Finds: "東京都立大学", "大学院", etc.
//! ```

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sudachi_tantivy::{SplitMode, SudachiTokenStream as InnerStream, SudachiTokenizer as Inner};
use tantivy::tokenizer::{Token, TokenStream, Tokenizer};

/// Lazy-loaded Sudachi tokenizers (one per mode).
/// Each mode requires a separate tokenizer due to different internal state.
static SUDACHI_A: Lazy<Option<Arc<Inner>>> = Lazy::new(|| {
    Inner::new(SplitMode::A)
        .map(Arc::new)
        .map_err(|e| tracing::warn!("Failed to initialize Sudachi Mode A: {e}"))
        .ok()
});

static SUDACHI_B: Lazy<Option<Arc<Inner>>> = Lazy::new(|| {
    Inner::new(SplitMode::B)
        .map(Arc::new)
        .map_err(|e| tracing::warn!("Failed to initialize Sudachi Mode B: {e}"))
        .ok()
});

static SUDACHI_C: Lazy<Option<Arc<Inner>>> = Lazy::new(|| {
    Inner::new(SplitMode::C)
        .map(Arc::new)
        .map_err(|e| tracing::warn!("Failed to initialize Sudachi Mode C: {e}"))
        .ok()
});

static SUDACHI_SEARCH: Lazy<Option<Arc<Inner>>> = Lazy::new(|| {
    Inner::new(SplitMode::Search)
        .map(Arc::new)
        .map_err(|e| tracing::warn!("Failed to initialize Sudachi Search mode: {e}"))
        .ok()
});

/// Sudachi split mode for tokenization granularity.
///
/// - **A**: Finest granularity (components of compound words)
/// - **B**: Medium granularity (named entities preserved)
/// - **C**: Coarsest granularity (longest tokens)
/// - **Search**: B+C multi-granularity (best for full-text search) - DEFAULT
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SudachiMode {
    /// Finest granularity - splits compound words into components
    /// Example: "東京都立大学" → ["東京", "都", "立", "大学"]
    A,
    /// Medium granularity - preserves named entities
    /// Example: "東京都立大学" → ["東京", "都立", "大学"]
    B,
    /// Coarsest granularity - longest tokens
    /// Example: "東京都立大学" → ["東京都立大学"]
    C,
    /// B+C multi-granularity - emits both compound words AND sub-tokens
    /// at the same position for optimal full-text search (DEFAULT)
    /// Example: "東京都立大学" → ["東京都立大学", "東京"*, "都立"*, "大学"*]
    /// (* = colocated at same position)
    #[default]
    Search,
}

impl SudachiMode {
    /// Parse from string (case-insensitive).
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "a" => Some(SudachiMode::A),
            "b" => Some(SudachiMode::B),
            "c" => Some(SudachiMode::C),
            "search" => Some(SudachiMode::Search),
            _ => None,
        }
    }

    /// Get the corresponding lazy-loaded tokenizer.
    fn get_tokenizer(&self) -> Option<&'static Arc<Inner>> {
        match self {
            SudachiMode::A => SUDACHI_A.as_ref(),
            SudachiMode::B => SUDACHI_B.as_ref(),
            SudachiMode::C => SUDACHI_C.as_ref(),
            SudachiMode::Search => SUDACHI_SEARCH.as_ref(),
        }
    }
}

/// ParadeDB-compatible Sudachi Japanese tokenizer.
///
/// Wraps `sudachi-tantivy` to provide Japanese morphological analysis
/// with support for B+C multi-granularity tokenization.
#[derive(Clone)]
pub struct SudachiTokenizer {
    mode: SudachiMode,
    normalized: bool,
    token: Token,
}

impl Default for SudachiTokenizer {
    fn default() -> Self {
        Self::new(SudachiMode::Search, true)
    }
}

impl SudachiTokenizer {
    /// Create a new Sudachi tokenizer with the specified mode and normalization setting.
    ///
    /// # Arguments
    ///
    /// * `mode` - The split mode (A, B, C, or Search)
    /// * `normalized` - Whether to use normalized form (recommended for search)
    pub fn new(mode: SudachiMode, normalized: bool) -> Self {
        Self {
            mode,
            normalized,
            token: Token::default(),
        }
    }

    /// Create a tokenizer with Search mode and normalization enabled.
    pub fn search() -> Self {
        Self::new(SudachiMode::Search, true)
    }
}

impl Tokenizer for SudachiTokenizer {
    type TokenStream<'a> = SudachiTokenStream<'a>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        if text.trim().is_empty() {
            return SudachiTokenStream::Empty;
        }

        // Get the lazy-loaded tokenizer for this mode
        let Some(inner) = self.mode.get_tokenizer() else {
            tracing::error!(
                "Sudachi tokenizer not initialized. Set SUDACHI_DICT_PATH environment variable."
            );
            return SudachiTokenStream::Empty;
        };

        // Clone the inner tokenizer to get a mutable instance
        let mut tokenizer = (**inner).clone();

        // Configure normalization
        if !self.normalized {
            tokenizer = tokenizer.with_surface_form();
        }

        // Tokenize and collect into owned data
        let stream = tokenizer.token_stream(text);
        let tokens = collect_tokens(stream);

        SudachiTokenStream::Sudachi(SudachiTokenStreamInner {
            tokens,
            index: 0,
            token: &mut self.token,
        })
    }
}

/// Collect tokens from the inner stream into owned data.
fn collect_tokens(mut stream: InnerStream<'_>) -> Vec<TokenData> {
    let mut tokens = Vec::new();
    while stream.advance() {
        let t = stream.token();
        tokens.push(TokenData {
            text: t.text.clone(),
            offset_from: t.offset_from,
            offset_to: t.offset_to,
            position: t.position,
            position_length: t.position_length,
        });
    }
    tokens
}

/// Pre-extracted token data (owned, no borrow issues).
#[derive(Debug, Clone)]
struct TokenData {
    text: String,
    offset_from: usize,
    offset_to: usize,
    position: usize,
    position_length: usize,
}

/// Token stream for Sudachi tokenizer.
pub enum SudachiTokenStream<'a> {
    Empty,
    Sudachi(SudachiTokenStreamInner<'a>),
}

/// Inner token stream implementation.
pub struct SudachiTokenStreamInner<'a> {
    tokens: Vec<TokenData>,
    index: usize,
    token: &'a mut Token,
}

impl TokenStream for SudachiTokenStream<'_> {
    fn advance(&mut self) -> bool {
        match self {
            SudachiTokenStream::Empty => false,
            SudachiTokenStream::Sudachi(inner) => inner.advance(),
        }
    }

    fn token(&self) -> &Token {
        match self {
            SudachiTokenStream::Empty => {
                panic!("Cannot call token() on an empty token stream.")
            }
            SudachiTokenStream::Sudachi(inner) => inner.token(),
        }
    }

    fn token_mut(&mut self) -> &mut Token {
        match self {
            SudachiTokenStream::Empty => {
                panic!("Cannot call token_mut() on an empty token stream.")
            }
            SudachiTokenStream::Sudachi(inner) => inner.token_mut(),
        }
    }
}

impl TokenStream for SudachiTokenStreamInner<'_> {
    fn advance(&mut self) -> bool {
        if self.index >= self.tokens.len() {
            return false;
        }

        let token_data = &self.tokens[self.index];

        self.token.text.clear();
        self.token.text.push_str(&token_data.text);
        self.token.offset_from = token_data.offset_from;
        self.token.offset_to = token_data.offset_to;
        self.token.position = token_data.position;
        self.token.position_length = token_data.position_length;

        self.index += 1;
        true
    }

    fn token(&self) -> &Token {
        self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_token_texts<T: Tokenizer>(tokenizer: &mut T, text: &str) -> Vec<String> {
        let mut stream = tokenizer.token_stream(text);
        let mut texts = Vec::new();
        while stream.advance() {
            texts.push(stream.token().text.clone());
        }
        texts
    }

    #[test]
    fn test_sudachi_mode_from_str() {
        assert_eq!(SudachiMode::from_str("search"), Some(SudachiMode::Search));
        assert_eq!(SudachiMode::from_str("SEARCH"), Some(SudachiMode::Search));
        assert_eq!(SudachiMode::from_str("a"), Some(SudachiMode::A));
        assert_eq!(SudachiMode::from_str("B"), Some(SudachiMode::B));
        assert_eq!(SudachiMode::from_str("c"), Some(SudachiMode::C));
        assert_eq!(SudachiMode::from_str("invalid"), None);
    }

    #[test]
    fn test_sudachi_mode_default() {
        assert_eq!(SudachiMode::default(), SudachiMode::Search);
    }

    #[test]
    fn test_empty_input() {
        let mut tokenizer = SudachiTokenizer::default();
        let texts = collect_token_texts(&mut tokenizer, "");
        assert!(texts.is_empty());

        let texts = collect_token_texts(&mut tokenizer, "   ");
        assert!(texts.is_empty());
    }

    // Integration tests require SUDACHI_DICT_PATH to be set
    // Run with: SUDACHI_DICT_PATH=/path/to/system.dic cargo test --features sudachi
    #[test]
    #[ignore]
    fn test_search_mode_multi_granularity() {
        let mut tokenizer = SudachiTokenizer::search();
        let texts = collect_token_texts(&mut tokenizer, "東京都立大学");

        // Search mode should emit compound word AND sub-tokens
        assert!(texts.contains(&"東京都立大学".to_string()));
        assert!(texts.contains(&"大学".to_string()));
    }

    #[test]
    #[ignore]
    fn test_mode_c_single_token() {
        let mut tokenizer = SudachiTokenizer::new(SudachiMode::C, true);
        let texts = collect_token_texts(&mut tokenizer, "東京都立大学");

        // Mode C should emit single compound word
        assert_eq!(texts, vec!["東京都立大学".to_string()]);
    }
}
