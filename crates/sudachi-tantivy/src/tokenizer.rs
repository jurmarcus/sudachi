//! Sudachi tokenizer implementation for Tantivy.

use std::sync::Arc;
use tantivy_tokenizer_api::{Token, Tokenizer};

use sudachi::analysis::Tokenize;
use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use sudachi_search::SearchTokenizer;

use crate::stream::SudachiTokenStream;

/// Split mode for Sudachi tokenization.
///
/// Controls the granularity of tokenization:
/// - `A`: Finest granularity (components of compound words)
/// - `B`: Medium granularity (named entities preserved)
/// - `C`: Coarsest granularity (longest tokens)
/// - `Search`: B+C multi-granularity (best for full-text search)
///
/// Example with "東京都立大学" (Tokyo Metropolitan University):
/// - Mode C: ["東京都立大学"]
/// - Mode B: ["東京", "都立", "大学"]
/// - Mode A: ["東京", "都", "立", "大学"]
/// - Search: ["東京都立大学", "東京"*, "都立"*, "大学"*] (* = colocated)
///
/// Search mode emits both compound words AND their sub-tokens at the same
/// position, enabling both exact phrase and partial matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitMode {
    /// Finest granularity - splits compound words into components
    A,
    /// Medium granularity - preserves named entities
    B,
    /// Coarsest granularity - longest tokens (default)
    #[default]
    C,
    /// B+C multi-granularity - emits both compound words and sub-tokens
    /// for optimal full-text search. Sub-tokens are colocated (same position).
    Search,
}

impl SplitMode {
    /// Convert to Sudachi's Mode enum.
    /// Returns None for Search mode (uses SearchTokenizer instead).
    fn to_sudachi_mode(self) -> Option<sudachi::analysis::Mode> {
        match self {
            SplitMode::A => Some(sudachi::analysis::Mode::A),
            SplitMode::B => Some(sudachi::analysis::Mode::B),
            SplitMode::C => Some(sudachi::analysis::Mode::C),
            SplitMode::Search => None,
        }
    }
}

/// Internal tokenizer enum to handle both standard and search modes.
enum TokenizerInner {
    /// Standard Sudachi tokenizer for modes A, B, C
    Standard(StatelessTokenizer<Arc<JapaneseDictionary>>),
    /// Search tokenizer for B+C multi-granularity
    Search(SearchTokenizer),
}

/// A Tantivy tokenizer using Sudachi for Japanese morphological analysis.
///
/// # Example
///
/// ```rust,ignore
/// use sudachi_tantivy::{SudachiTokenizer, SplitMode};
///
/// // Standard mode C (longest units)
/// let tokenizer = SudachiTokenizer::new(SplitMode::C)?;
///
/// // Search mode (B+C multi-granularity, best for full-text search)
/// let search_tokenizer = SudachiTokenizer::new(SplitMode::Search)?;
/// ```
#[derive(Clone)]
pub struct SudachiTokenizer {
    /// Internal tokenizer (standard or search)
    inner: Arc<TokenizerInner>,
    /// Split mode to use
    mode: SplitMode,
    /// Reusable token (reset for each stream)
    token: Token,
    /// Whether to use normalized form instead of surface
    use_normalized: bool,
}

impl SudachiTokenizer {
    /// Create a new SudachiTokenizer with the specified split mode.
    ///
    /// Loads the dictionary from `SUDACHI_DICT_PATH` environment variable.
    ///
    /// # Arguments
    ///
    /// * `mode` - The split mode to use (A, B, C, or Search)
    ///
    /// # Errors
    ///
    /// Returns an error if the Sudachi dictionary cannot be loaded.
    pub fn new(mode: SplitMode) -> Result<Self, SudachiError> {
        let dict_path = std::env::var("SUDACHI_DICT_PATH").map_err(|_| {
            SudachiError::DictionaryLoad(
                "SUDACHI_DICT_PATH environment variable not set".to_string(),
            )
        })?;

        let dict_path = std::path::PathBuf::from(&dict_path);

        if !dict_path.exists() {
            return Err(SudachiError::DictionaryLoad(format!(
                "Dictionary not found at {:?}",
                dict_path
            )));
        }

        let dict_bytes = std::fs::read(&dict_path).map_err(|e| {
            SudachiError::DictionaryLoad(format!("Failed to read dictionary: {}", e))
        })?;

        let storage = SudachiDicData::new(Storage::Owned(dict_bytes));
        let resource_dir = dict_path.parent().unwrap_or(std::path::Path::new("."));
        let config = Config::minimal_at(resource_dir);

        let dictionary = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(
            &config, storage,
        )
        .map_err(|e| SudachiError::DictionaryLoad(format!("Failed to load dictionary: {}", e)))?;

        let dictionary = Arc::new(dictionary);

        // Create the appropriate tokenizer based on mode
        let inner = match mode {
            SplitMode::Search => TokenizerInner::Search(SearchTokenizer::new(dictionary)),
            _ => TokenizerInner::Standard(StatelessTokenizer::new(dictionary)),
        };

        Ok(Self {
            inner: Arc::new(inner),
            mode,
            token: Token::default(),
            use_normalized: true,
        })
    }

    /// Create a tokenizer with a pre-loaded dictionary.
    ///
    /// Useful when sharing a dictionary across multiple tokenizer instances.
    pub fn with_dictionary(dictionary: Arc<JapaneseDictionary>, mode: SplitMode) -> Self {
        let inner = match mode {
            SplitMode::Search => TokenizerInner::Search(SearchTokenizer::new(dictionary)),
            _ => TokenizerInner::Standard(StatelessTokenizer::new(dictionary)),
        };

        Self {
            inner: Arc::new(inner),
            mode,
            token: Token::default(),
            use_normalized: true,
        }
    }

    /// Use surface form instead of normalized form.
    pub fn with_surface_form(mut self) -> Self {
        self.use_normalized = false;
        self
    }

    /// Configure whether to use normalized form for token text.
    pub fn with_normalized_form(mut self, enabled: bool) -> Self {
        self.use_normalized = enabled;
        self
    }

    /// Returns whether normalized form is being used.
    pub fn uses_normalized_form(&self) -> bool {
        self.use_normalized
    }

    /// Returns the split mode.
    pub fn mode(&self) -> SplitMode {
        self.mode
    }
}

impl Tokenizer for SudachiTokenizer {
    type TokenStream<'a> = SudachiTokenStream<'a>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> SudachiTokenStream<'a> {
        self.token.reset();

        let tokens: Vec<TokenData> = match &*self.inner {
            TokenizerInner::Standard(tokenizer) => {
                let sudachi_mode = self
                    .mode
                    .to_sudachi_mode()
                    .unwrap_or(sudachi::analysis::Mode::C);
                match tokenizer.tokenize(text, sudachi_mode, false) {
                    Ok(morphemes) => (0..morphemes.len())
                        .map(|i| {
                            let m = morphemes.get(i);
                            TokenData {
                                text: if self.use_normalized {
                                    m.normalized_form().to_string()
                                } else {
                                    m.surface().to_string()
                                },
                                offset_from: m.begin(),
                                offset_to: m.end(),
                                is_colocated: false,
                            }
                        })
                        .collect(),
                    Err(_) => Vec::new(),
                }
            }
            TokenizerInner::Search(tokenizer) => {
                match tokenizer.tokenize_with_normalization(text, self.use_normalized) {
                    Ok(search_tokens) => search_tokens
                        .into_iter()
                        .map(|t| TokenData {
                            text: t.surface,
                            offset_from: t.byte_start,
                            offset_to: t.byte_end,
                            is_colocated: t.is_colocated,
                        })
                        .collect(),
                    Err(_) => Vec::new(),
                }
            }
        };

        SudachiTokenStream::new(tokens, &mut self.token)
    }
}

/// Pre-extracted token data from Sudachi morphemes.
#[derive(Debug, Clone)]
pub struct TokenData {
    pub text: String,
    pub offset_from: usize,
    pub offset_to: usize,
    /// Whether this token is colocated (same position as previous).
    /// Used in Search mode for sub-tokens of compound words.
    pub is_colocated: bool,
}

/// Errors that can occur when using the Sudachi tokenizer.
#[derive(Debug, Clone)]
pub enum SudachiError {
    /// Failed to load the Sudachi dictionary
    DictionaryLoad(String),
}

impl std::fmt::Display for SudachiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudachiError::DictionaryLoad(msg) => {
                write!(f, "Failed to load Sudachi dictionary: {}", msg)
            }
        }
    }
}

impl std::error::Error for SudachiError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_mode_conversion() {
        assert_eq!(
            SplitMode::A.to_sudachi_mode(),
            Some(sudachi::analysis::Mode::A)
        );
        assert_eq!(
            SplitMode::B.to_sudachi_mode(),
            Some(sudachi::analysis::Mode::B)
        );
        assert_eq!(
            SplitMode::C.to_sudachi_mode(),
            Some(sudachi::analysis::Mode::C)
        );
        assert_eq!(SplitMode::Search.to_sudachi_mode(), None);
    }

    #[test]
    fn test_default_mode() {
        assert_eq!(SplitMode::default(), SplitMode::C);
    }
}
