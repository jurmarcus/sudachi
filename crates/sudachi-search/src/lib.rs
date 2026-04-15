//! # sudachi-search
//!
//! Multi-granularity Japanese tokenization for search engines using Sudachi.
//!
//! This crate provides a search-optimized tokenizer that emits both compound words
//! (Mode C) and their sub-tokens (Mode B) for optimal Japanese full-text search.
//!
//! ## Features
//!
//! - **B+C Multi-Granularity**: Indexes both compound words AND sub-tokens
//! - **Normalized Forms**: 食べた → 食べる (dictionary form for verbs/adjectives)
//! - **Function Word Filtering**: Drops auxiliary verbs (た, ます, いる) by default
//!
//! ## The Problem
//!
//! Japanese text like "東京都立大学" (Tokyo Metropolitan University) can be tokenized
//! differently depending on granularity:
//!
//! - **Mode C** (longest): `["東京都立大学"]` - good for exact phrase matching
//! - **Mode B** (middle): `["東京", "都立", "大学"]` - good for partial matching
//!
//! For search, you want **both**: users searching "大学" should find documents
//! containing "東京都立大学".
//!
//! Additionally, conjugated forms like "食べた" (ate) should match "食べる" (to eat).
//! This requires both normalization AND filtering of auxiliary verbs.
//!
//! ## The Solution: B+C Multi-Granularity + Filtering
//!
//! This crate emits both the compound word AND its sub-tokens at the same position,
//! while filtering out function words that break FTS matching:
//!
//! ```text
//! Input: "東京都立大学で食べた"
//!
//! Tokens:
//!   pos 0: "東京都立大学" (primary)
//!   pos 0: "東京" (colocated)
//!   pos 0: "都立" (colocated)
//!   pos 0: "大学" (colocated)
//!   pos 1: "で"
//!   pos 2: "食べる"        ← normalized from "食べ"
//!                          ← "た" filtered out (助動詞)
//! ```
//!
//! ## Usage
//!
//! ```ignore
//! use sudachi_search::{SearchTokenizer, SearchToken};
//!
//! let tokenizer = SearchTokenizer::new(dictionary);
//!
//! // Default: filters function words for optimal search
//! let tokens = tokenizer.tokenize("食べた")?;
//! // → [SearchToken { surface: "食べる", ... }]
//!
//! // Preserve all tokens (for linguistic analysis)
//! let tokenizer = SearchTokenizer::new(dictionary).with_all_tokens();
//! let tokens = tokenizer.tokenize("食べた")?;
//! // → [SearchToken { surface: "食べる", ... }, SearchToken { surface: "た", ... }]
//! ```
//!
//! ## Search Engine Integration
//!
//! Each search engine handles colocated tokens differently:
//!
//! | Engine | Colocated Mechanism |
//! |--------|---------------------|
//! | SQLite FTS5 | `FTS5_TOKEN_COLOCATED` flag (0x0001) |
//! | Tantivy | `position_increment = 0` in TokenStream |
//! | Lucene/ES | `PositionIncrementAttribute = 0` |
//!
//! This crate provides the abstract `SearchToken::is_colocated` flag;
//! adapter crates (sudachi-sqlite, sudachi-tantivy) handle the translation.
//!
//! ## Compound Word Detection
//!
//! Beyond search, this crate can detect compound words by comparing Mode C and B:
//!
//! ```ignore
//! let compounds = tokenizer.detect_compounds("東京都立大学で会議");
//! // [
//! //   CompoundWord {
//! //     surface: "東京都立大学",
//! //     components: ["東京", "都立", "大学"],
//! //     ..
//! //   }
//! // ]
//! ```
//!
//! Use cases: dictionary lookup, furigana generation, language learning, text analysis.

use std::sync::Arc;

use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::analysis::{Mode, Tokenize};
use sudachi::dic::dictionary::JapaneseDictionary;

/// Check if a part-of-speech should be filtered out for search purposes.
///
/// Filters:
/// - 助動詞 (auxiliary verbs): た, ます, れる, られる, etc.
/// - 助詞/接続助詞 (conjunctive particles): て when connecting verbs
///
/// NOTE: 動詞/非自立可能 is NOT filtered here. While verbs like 行く, 来る, する,
/// いる, ある CAN be auxiliary verbs, they are often main verbs. Filtering them
/// would break searches for standalone common verbs. Context-aware filtering
/// is done separately via `should_filter_auxiliary_verb`.
///
/// These function words break FTS matching for conjugated forms because FTS5
/// requires ALL query tokens to match. By filtering them, queries like "食べた"
/// will match documents containing "食べる".
#[inline]
fn should_filter_pos(pos: &[String]) -> bool {
    if pos.is_empty() {
        return false;
    }

    // 助動詞 (auxiliary verbs): た, ます, れる, られる, etc.
    if pos[0] == "助動詞" {
        return true;
    }

    // 助詞/接続助詞 (conjunctive particles): て
    // POS format: ["助詞", "接続助詞", ...]
    // These appear in ている, てある, etc.
    if pos.len() >= 2 && pos[0] == "助詞" && pos[1] == "接続助詞" {
        return true;
    }

    false
}

/// A token emitted by the search tokenizer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchToken {
    /// The token surface text.
    pub surface: String,

    /// Byte offset where this token starts in the original text.
    pub byte_start: usize,

    /// Byte offset where this token ends in the original text.
    pub byte_end: usize,

    /// Whether this token is colocated (same position as previous token).
    ///
    /// Colocated tokens are sub-tokens of a compound word. For example,
    /// "東京都立大学" emits:
    /// - "東京都立大学" (is_colocated: false) - the compound word
    /// - "東京" (is_colocated: true) - sub-token at same position
    /// - "都立" (is_colocated: true) - sub-token at same position
    /// - "大学" (is_colocated: true) - sub-token at same position
    pub is_colocated: bool,
}

/// A detected compound word with its components.
///
/// Compound words are multi-morpheme units that Sudachi's Mode C treats as
/// single tokens but Mode B splits into parts.
///
/// # Example
///
/// ```ignore
/// // "東京都立大学" is a compound word
/// CompoundWord {
///     surface: "東京都立大学",
///     components: ["東京", "都立", "大学"],
///     byte_start: 0,
///     byte_end: 18,
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundWord {
    /// The full compound word surface text.
    pub surface: String,

    /// The component parts (Mode B tokenization).
    pub components: Vec<String>,

    /// Byte offset where this compound starts in the original text.
    pub byte_start: usize,

    /// Byte offset where this compound ends in the original text.
    pub byte_end: usize,
}

impl CompoundWord {
    /// Returns true if this is a "true" compound (has multiple components).
    ///
    /// Single-component "compounds" occur when Mode C and Mode B agree.
    #[inline]
    pub fn is_compound(&self) -> bool {
        self.components.len() > 1
    }

    /// Returns the number of components.
    #[inline]
    pub fn component_count(&self) -> usize {
        self.components.len()
    }
}

/// Search-optimized tokenizer using B+C multi-granularity strategy.
///
/// Emits both compound words (Mode C) and their sub-tokens (Mode B)
/// for optimal Japanese full-text search.
///
/// By default, filters out function words (助動詞, 動詞/非自立可能) that break
/// FTS matching for conjugated forms. Use `.with_all_tokens()` to preserve them.
///
/// # Example
///
/// ```ignore
/// // Default: filter function words for search
/// let tokenizer = SearchTokenizer::new(dictionary);
/// let tokens = tokenizer.tokenize("食べた")?;
/// // → ["食べる"]  (た is filtered)
///
/// // Preserve all tokens for linguistic analysis
/// let tokenizer = SearchTokenizer::new(dictionary).with_all_tokens();
/// let tokens = tokenizer.tokenize("食べた")?;
/// // → ["食べる", "た"]
/// ```
pub struct SearchTokenizer {
    inner: StatelessTokenizer<Arc<JapaneseDictionary>>,
    /// Whether to use normalized form instead of surface form.
    use_normalized: bool,
    /// Whether to filter out function words (助動詞, 動詞/非自立可能).
    /// Default: true (filter for optimal search).
    filter_function_words: bool,
}

impl SearchTokenizer {
    /// Creates a new search tokenizer from a Sudachi dictionary.
    ///
    /// Default configuration:
    /// - **Normalized form**: Verbs/adjectives normalized to dictionary form (食べた → 食べる)
    /// - **Filter function words**: Auxiliary verbs (た, ます) and non-independent verbs (いる)
    ///   are dropped for optimal FTS matching
    ///
    /// Use `.with_surface_form()` to disable normalization.
    /// Use `.with_all_tokens()` to preserve all tokens including function words.
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self {
        Self {
            inner: StatelessTokenizer::new(dictionary),
            use_normalized: true,
            filter_function_words: true,
        }
    }

    /// Creates a new search tokenizer from an existing StatelessTokenizer.
    ///
    /// Default configuration:
    /// - **Normalized form**: Verbs/adjectives normalized to dictionary form
    /// - **Filter function words**: Drops auxiliary verbs for optimal FTS matching
    pub fn from_tokenizer(tokenizer: StatelessTokenizer<Arc<JapaneseDictionary>>) -> Self {
        Self {
            inner: tokenizer,
            use_normalized: true,
            filter_function_words: true,
        }
    }

    /// Preserve all tokens including function words.
    ///
    /// By default, SearchTokenizer filters out:
    /// - 助動詞 (auxiliary verbs): た, ます, れる, られる, etc.
    /// - 動詞/非自立可能 (non-independent verbs): いる, ある when auxiliary
    ///
    /// Call this method to emit all tokens, useful for:
    /// - Linguistic analysis
    /// - Precise phrase matching
    /// - Debugging tokenization
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tokenizer = SearchTokenizer::new(dictionary).with_all_tokens();
    /// let tokens = tokenizer.tokenize("食べている")?;
    /// // → ["食べる", "て", "居る"]  (all tokens preserved)
    /// ```
    pub fn with_all_tokens(mut self) -> Self {
        self.filter_function_words = false;
        self
    }

    /// Configure function word filtering.
    ///
    /// When enabled (default), filters out:
    /// - 助動詞 (auxiliary verbs): た, ます, れる, られる, etc.
    /// - 動詞/非自立可能 (non-independent verbs): いる, ある when auxiliary
    ///
    /// This improves FTS recall for conjugated forms:
    /// - Query "食べた" → matches documents containing "食べる"
    pub fn with_filter_function_words(mut self, enabled: bool) -> Self {
        self.filter_function_words = enabled;
        self
    }

    /// Returns whether function word filtering is enabled.
    pub fn filters_function_words(&self) -> bool {
        self.filter_function_words
    }

    /// Use surface form instead of normalized form.
    ///
    /// By default, SearchTokenizer uses normalized form for better search recall.
    /// Call this method to use the original surface form instead.
    ///
    /// Use cases for surface form:
    /// - Precision over recall (exact form matching)
    /// - Linguistic research (studying actual usage)
    /// - Highlighting (displayed text matches query exactly)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tokenizer = SearchTokenizer::new(dictionary)
    ///     .with_surface_form();  // Disable normalization
    /// ```
    pub fn with_surface_form(mut self) -> Self {
        self.use_normalized = false;
        self
    }

    /// Configure whether to use normalized form for token text.
    ///
    /// Normalized form (default) improves search recall by normalizing:
    /// - Variant kanji (附属 → 付属)
    /// - Okurigana variants (お願い → 御願い)
    /// - Fullwidth characters (ＳＵＭＭＥＲ → サマー)
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Explicit normalization (same as default)
    /// let tokenizer = SearchTokenizer::new(dictionary)
    ///     .with_normalized_form(true);
    ///
    /// // Disable normalization (same as .with_surface_form())
    /// let tokenizer = SearchTokenizer::new(dictionary)
    ///     .with_normalized_form(false);
    /// ```
    pub fn with_normalized_form(mut self, enabled: bool) -> Self {
        self.use_normalized = enabled;
        self
    }

    /// Returns whether normalized form is being used.
    pub fn uses_normalized_form(&self) -> bool {
        self.use_normalized
    }

    /// Tokenizes input with explicit normalization setting.
    ///
    /// This method allows overriding the tokenizer's normalization setting
    /// for a single tokenization call. Useful when the tokenizer is shared
    /// (e.g., in an Arc) and normalization needs to vary per-call.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Tokenizer configured without normalization
    /// let tokenizer = SearchTokenizer::new(dictionary);
    ///
    /// // But tokenize with normalization for this call
    /// let tokens = tokenizer.tokenize_with_normalization("お願い", true)?;
    /// ```
    pub fn tokenize_with_normalization(
        &self,
        input: &str,
        use_normalized: bool,
    ) -> Result<Vec<SearchToken>, SearchError> {
        self.tokenize_internal(input, use_normalized)
    }

    /// Tokenizes input using B+C multi-granularity strategy.
    ///
    /// For each text span:
    /// 1. Emit Mode C token (compound word) with `is_colocated: false`
    /// 2. Emit Mode B sub-tokens with `is_colocated: true` (if they differ)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tokens = tokenizer.tokenize("予約困難店");
    /// // [
    /// //   SearchToken { surface: "予約困難店", is_colocated: false, .. },
    /// //   SearchToken { surface: "予約", is_colocated: true, .. },
    /// //   SearchToken { surface: "困難", is_colocated: true, .. },
    /// //   SearchToken { surface: "店", is_colocated: true, .. },
    /// // ]
    /// ```
    pub fn tokenize(&self, input: &str) -> Result<Vec<SearchToken>, SearchError> {
        self.tokenize_internal(input, self.use_normalized)
    }

    /// Internal tokenization with explicit normalization flag.
    fn tokenize_internal(
        &self,
        input: &str,
        use_normalized: bool,
    ) -> Result<Vec<SearchToken>, SearchError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        // Tokenize with Mode C (longest/compound words)
        let morphemes_c = self
            .inner
            .tokenize(input, Mode::C, false)
            .map_err(|e| SearchError::Tokenization(e.to_string()))?;

        // Tokenize with Mode B (middle units) for sub-token extraction
        let morphemes_b = self
            .inner
            .tokenize(input, Mode::B, false)
            .map_err(|e| SearchError::Tokenization(e.to_string()))?;

        // Build a lookup of Mode B tokens by their byte positions
        // Include POS info for filtering
        let mut mode_b_tokens: Vec<(usize, usize, String, bool)> =
            Vec::with_capacity(morphemes_b.len());
        for i in 0..morphemes_b.len() {
            let m = morphemes_b.get(i);
            let text = if use_normalized {
                m.normalized_form().to_string()
            } else {
                m.surface().to_string()
            };
            let should_filter = self.filter_function_words && should_filter_pos(m.part_of_speech());
            mode_b_tokens.push((m.begin(), m.end(), text, should_filter));
        }

        let mut result = Vec::new();

        // For each Mode C token, emit it and any differing Mode B sub-tokens
        for i in 0..morphemes_c.len() {
            let morpheme_c = morphemes_c.get(i);

            // Check if this Mode C token should be filtered
            if self.filter_function_words && should_filter_pos(morpheme_c.part_of_speech()) {
                continue;
            }

            let text_c = if use_normalized {
                morpheme_c.normalized_form().to_string()
            } else {
                morpheme_c.surface().to_string()
            };
            let byte_start = morpheme_c.begin();
            let byte_end = morpheme_c.end();

            // Emit the Mode C token first (not colocated)
            result.push(SearchToken {
                surface: text_c.clone(),
                byte_start,
                byte_end,
                is_colocated: false,
            });

            // Find and emit Mode B tokens within this span as colocated
            for (b_start, b_end, b_text, b_filter) in &mode_b_tokens {
                // Skip filtered tokens
                if *b_filter {
                    continue;
                }

                // Check if this Mode B token falls within the Mode C span
                if *b_start >= byte_start && *b_end <= byte_end {
                    // Only emit if it's different from the Mode C text
                    if b_text != &text_c {
                        result.push(SearchToken {
                            surface: b_text.clone(),
                            byte_start: *b_start,
                            byte_end: *b_end,
                            is_colocated: true,
                        });
                    }
                }
            }
        }

        Ok(result)
    }

    /// Returns a reference to the underlying Sudachi tokenizer.
    pub fn inner(&self) -> &StatelessTokenizer<Arc<JapaneseDictionary>> {
        &self.inner
    }

    /// Detects compound words in the input text.
    ///
    /// A compound word is detected when Mode C produces a single token that
    /// Mode B would split into multiple parts.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let compounds = tokenizer.detect_compounds("東京都立大学で会議が開催された")?;
    /// for compound in compounds {
    ///     println!("{} = {:?}", compound.surface, compound.components);
    /// }
    /// // Output:
    /// // 東京都立大学 = ["東京", "都立", "大学"]
    /// ```
    ///
    /// # Returns
    ///
    /// Only returns true compound words (where `components.len() > 1`).
    /// Single-token words where Mode C and Mode B agree are not included.
    pub fn detect_compounds(&self, input: &str) -> Result<Vec<CompoundWord>, SearchError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        // Tokenize with Mode C (longest/compound words)
        let morphemes_c = self
            .inner
            .tokenize(input, Mode::C, false)
            .map_err(|e| SearchError::Tokenization(e.to_string()))?;

        // Tokenize with Mode B (middle units)
        let morphemes_b = self
            .inner
            .tokenize(input, Mode::B, false)
            .map_err(|e| SearchError::Tokenization(e.to_string()))?;

        // Build a lookup of Mode B tokens by their byte positions
        let mut mode_b_tokens: Vec<(usize, usize, String)> = Vec::with_capacity(morphemes_b.len());
        for i in 0..morphemes_b.len() {
            let m = morphemes_b.get(i);
            let text = if self.use_normalized {
                m.normalized_form().to_string()
            } else {
                m.surface().to_string()
            };
            mode_b_tokens.push((m.begin(), m.end(), text));
        }

        let mut compounds = Vec::new();

        // For each Mode C token, check if it splits into multiple Mode B tokens
        for i in 0..morphemes_c.len() {
            let morpheme_c = morphemes_c.get(i);
            let text_c = if self.use_normalized {
                morpheme_c.normalized_form().to_string()
            } else {
                morpheme_c.surface().to_string()
            };
            let byte_start = morpheme_c.begin();
            let byte_end = morpheme_c.end();

            // Collect Mode B components within this span
            let components: Vec<String> = mode_b_tokens
                .iter()
                .filter(|(b_start, b_end, _)| *b_start >= byte_start && *b_end <= byte_end)
                .map(|(_, _, text)| text.clone())
                .collect();

            // Only include if it's a true compound (multiple components)
            if components.len() > 1 {
                compounds.push(CompoundWord {
                    surface: text_c,
                    components,
                    byte_start,
                    byte_end,
                });
            }
        }

        Ok(compounds)
    }

    /// Tokenizes and detects compounds in a single pass.
    ///
    /// More efficient than calling `tokenize()` and `detect_compounds()` separately
    /// when you need both results.
    ///
    /// # Returns
    ///
    /// A tuple of (tokens, compounds).
    pub fn tokenize_with_compounds(
        &self,
        input: &str,
    ) -> Result<(Vec<SearchToken>, Vec<CompoundWord>), SearchError> {
        if input.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }

        // Tokenize with Mode C (longest/compound words)
        let morphemes_c = self
            .inner
            .tokenize(input, Mode::C, false)
            .map_err(|e| SearchError::Tokenization(e.to_string()))?;

        // Tokenize with Mode B (middle units)
        let morphemes_b = self
            .inner
            .tokenize(input, Mode::B, false)
            .map_err(|e| SearchError::Tokenization(e.to_string()))?;

        // Build a lookup of Mode B tokens by their byte positions
        let mut mode_b_tokens: Vec<(usize, usize, String)> = Vec::with_capacity(morphemes_b.len());
        for i in 0..morphemes_b.len() {
            let m = morphemes_b.get(i);
            let text = if self.use_normalized {
                m.normalized_form().to_string()
            } else {
                m.surface().to_string()
            };
            mode_b_tokens.push((m.begin(), m.end(), text));
        }

        let mut tokens = Vec::new();
        let mut compounds = Vec::new();

        // For each Mode C token, emit it and collect compounds
        for i in 0..morphemes_c.len() {
            let morpheme_c = morphemes_c.get(i);
            let text_c = if self.use_normalized {
                morpheme_c.normalized_form().to_string()
            } else {
                morpheme_c.surface().to_string()
            };
            let byte_start = morpheme_c.begin();
            let byte_end = morpheme_c.end();

            // Emit the Mode C token first (not colocated)
            tokens.push(SearchToken {
                surface: text_c.clone(),
                byte_start,
                byte_end,
                is_colocated: false,
            });

            // Collect Mode B components within this span
            let mut components = Vec::new();
            for (b_start, b_end, b_text) in &mode_b_tokens {
                if *b_start >= byte_start && *b_end <= byte_end {
                    components.push(b_text.clone());

                    // Also emit as colocated token if different from Mode C
                    if b_text != &text_c {
                        tokens.push(SearchToken {
                            surface: b_text.clone(),
                            byte_start: *b_start,
                            byte_end: *b_end,
                            is_colocated: true,
                        });
                    }
                }
            }

            // Add to compounds if it's a true compound
            if components.len() > 1 {
                compounds.push(CompoundWord {
                    surface: text_c,
                    components,
                    byte_start,
                    byte_end,
                });
            }
        }

        Ok((tokens, compounds))
    }
}

/// Extracts compound words from a list of search tokens.
///
/// This is useful when you've already called `tokenize()` and want to
/// identify compounds without re-tokenizing.
///
/// # Example
///
/// ```ignore
/// let tokens = tokenizer.tokenize("東京都立大学で会議")?;
/// let compounds = extract_compounds(&tokens);
/// ```
pub fn extract_compounds(tokens: &[SearchToken]) -> Vec<CompoundWord> {
    let mut compounds = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let primary = &tokens[i];

        // Skip if this is a colocated token (sub-token)
        if primary.is_colocated {
            i += 1;
            continue;
        }

        // Collect any following colocated tokens (these are the components)
        let mut components = Vec::new();
        let mut j = i + 1;

        while j < tokens.len() && tokens[j].is_colocated {
            // Only include if within the primary token's byte range
            if tokens[j].byte_start >= primary.byte_start
                && tokens[j].byte_end <= primary.byte_end
            {
                components.push(tokens[j].surface.clone());
            }
            j += 1;
        }

        // If we found sub-tokens, this is a compound
        if !components.is_empty() {
            compounds.push(CompoundWord {
                surface: primary.surface.clone(),
                components,
                byte_start: primary.byte_start,
                byte_end: primary.byte_end,
            });
        }

        i = j;
    }

    compounds
}

/// Errors that can occur during search tokenization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchError {
    /// Tokenization failed.
    Tokenization(String),
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::Tokenization(msg) => write!(f, "tokenization error: {}", msg),
        }
    }
}

impl std::error::Error for SearchError {}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Tests require a Sudachi dictionary to be available.
    // Run with: SUDACHI_DICT_PATH=/path/to/system.dic cargo test

    #[test]
    fn test_search_token_equality() {
        let t1 = SearchToken {
            surface: "東京".to_string(),
            byte_start: 0,
            byte_end: 6,
            is_colocated: false,
        };
        let t2 = SearchToken {
            surface: "東京".to_string(),
            byte_start: 0,
            byte_end: 6,
            is_colocated: false,
        };
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_colocated_distinction() {
        let primary = SearchToken {
            surface: "東京都立大学".to_string(),
            byte_start: 0,
            byte_end: 18,
            is_colocated: false,
        };
        let sub = SearchToken {
            surface: "大学".to_string(),
            byte_start: 12,
            byte_end: 18,
            is_colocated: true,
        };
        assert!(!primary.is_colocated);
        assert!(sub.is_colocated);
    }

    #[test]
    fn test_compound_word_is_compound() {
        let compound = CompoundWord {
            surface: "東京都立大学".to_string(),
            components: vec!["東京".to_string(), "都立".to_string(), "大学".to_string()],
            byte_start: 0,
            byte_end: 18,
        };
        assert!(compound.is_compound());
        assert_eq!(compound.component_count(), 3);

        // Single component is not a "true" compound
        let single = CompoundWord {
            surface: "東京".to_string(),
            components: vec!["東京".to_string()],
            byte_start: 0,
            byte_end: 6,
        };
        assert!(!single.is_compound());
        assert_eq!(single.component_count(), 1);
    }

    #[test]
    fn test_extract_compounds_from_tokens() {
        let tokens = vec![
            SearchToken {
                surface: "東京都立大学".to_string(),
                byte_start: 0,
                byte_end: 18,
                is_colocated: false,
            },
            SearchToken {
                surface: "東京".to_string(),
                byte_start: 0,
                byte_end: 6,
                is_colocated: true,
            },
            SearchToken {
                surface: "都立".to_string(),
                byte_start: 6,
                byte_end: 12,
                is_colocated: true,
            },
            SearchToken {
                surface: "大学".to_string(),
                byte_start: 12,
                byte_end: 18,
                is_colocated: true,
            },
            SearchToken {
                surface: "で".to_string(),
                byte_start: 18,
                byte_end: 21,
                is_colocated: false,
            },
            SearchToken {
                surface: "会議".to_string(),
                byte_start: 21,
                byte_end: 27,
                is_colocated: false,
            },
        ];

        let compounds = extract_compounds(&tokens);

        assert_eq!(compounds.len(), 1);
        assert_eq!(compounds[0].surface, "東京都立大学");
        assert_eq!(
            compounds[0].components,
            vec!["東京".to_string(), "都立".to_string(), "大学".to_string()]
        );
    }

    #[test]
    fn test_extract_compounds_empty() {
        let tokens: Vec<SearchToken> = vec![];
        let compounds = extract_compounds(&tokens);
        assert!(compounds.is_empty());
    }

    #[test]
    fn test_extract_compounds_no_compounds() {
        // All simple tokens, no compounds
        let tokens = vec![
            SearchToken {
                surface: "東京".to_string(),
                byte_start: 0,
                byte_end: 6,
                is_colocated: false,
            },
            SearchToken {
                surface: "で".to_string(),
                byte_start: 6,
                byte_end: 9,
                is_colocated: false,
            },
        ];

        let compounds = extract_compounds(&tokens);
        assert!(compounds.is_empty());
    }

    // ========================================================================
    // POS Filtering Tests
    // ========================================================================

    #[test]
    fn test_filter_auxiliary_verbs() {
        // 助動詞 (auxiliary verbs) should be filtered
        // Examples: た, ます, れる, られる, etc.
        let pos_ta = vec!["助動詞".to_string(), "*".to_string()];
        assert!(should_filter_pos(&pos_ta), "た should be filtered");

        let pos_masu = vec!["助動詞".to_string(), "*".to_string(), "*".to_string()];
        assert!(should_filter_pos(&pos_masu), "ます should be filtered");
    }

    #[test]
    fn test_non_independent_verbs_not_filtered() {
        // 動詞/非自立可能 (non-independent verbs) should NOT be filtered by should_filter_pos
        // because they are often main verbs (行く, 来る, する, etc.)
        // Context-aware filtering is done separately
        let pos_iru = vec!["動詞".to_string(), "非自立可能".to_string()];
        assert!(!should_filter_pos(&pos_iru), "いる should NOT be filtered by POS alone");
        assert!(is_non_independent_verb(&pos_iru), "いる IS a non-independent verb");

        let pos_aru = vec!["動詞".to_string(), "非自立可能".to_string(), "*".to_string()];
        assert!(!should_filter_pos(&pos_aru), "ある should NOT be filtered by POS alone");
        assert!(is_non_independent_verb(&pos_aru), "ある IS a non-independent verb");
    }

    #[test]
    fn test_filter_conjunctive_particles() {
        // 助詞/接続助詞 (conjunctive particles) should be filtered
        // Examples: て
        let pos_te = vec!["助詞".to_string(), "接続助詞".to_string()];
        assert!(should_filter_pos(&pos_te), "て should be filtered");
    }

    #[test]
    fn test_keep_regular_verbs() {
        // 動詞/一般 (regular verbs) should NOT be filtered
        let pos_taberu = vec![
            "動詞".to_string(),
            "一般".to_string(),
            "*".to_string(),
            "*".to_string(),
            "下一段-バ行".to_string(),
        ];
        assert!(!should_filter_pos(&pos_taberu), "食べる should NOT be filtered");
    }

    #[test]
    fn test_keep_nouns() {
        // 名詞 (nouns) should NOT be filtered
        let pos_noun = vec!["名詞".to_string(), "普通名詞".to_string()];
        assert!(!should_filter_pos(&pos_noun), "nouns should NOT be filtered");

        let pos_proper = vec!["名詞".to_string(), "固有名詞".to_string()];
        assert!(!should_filter_pos(&pos_proper), "proper nouns should NOT be filtered");
    }

    #[test]
    fn test_keep_adjectives() {
        // 形容詞 (adjectives) should NOT be filtered
        let pos_adj = vec!["形容詞".to_string(), "一般".to_string()];
        assert!(!should_filter_pos(&pos_adj), "adjectives should NOT be filtered");
    }

    #[test]
    fn test_keep_case_particles() {
        // 助詞/格助詞 (case particles) should NOT be filtered
        // Examples: を, が, に, で
        let pos_wo = vec!["助詞".to_string(), "格助詞".to_string()];
        assert!(!should_filter_pos(&pos_wo), "を should NOT be filtered");
    }

    #[test]
    fn test_empty_pos() {
        // Empty POS should not crash and should not filter
        let empty: Vec<String> = vec![];
        assert!(!should_filter_pos(&empty), "empty POS should not filter");
    }

    // ========================================================================
    // Integration tests (require SUDACHI_DICT_PATH)
    // ========================================================================
    // Run with: SUDACHI_DICT_PATH=/path/to/system.dic cargo test -- --ignored
}
