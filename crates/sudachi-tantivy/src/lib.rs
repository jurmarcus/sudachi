//! # sudachi-tantivy
//!
//! A [Sudachi](https://github.com/WorksApplications/sudachi.rs) tokenizer implementation
//! for [Tantivy](https://github.com/quickwit-oss/tantivy) full-text search.
//!
//! Sudachi provides superior Japanese morphological analysis with:
//! - Three split modes (A, B, C) for different granularity
//! - Normalized forms for better matching
//! - High-quality readings (furigana)
//!
//! ## Example
//!
//! ```rust,ignore
//! use sudachi_tantivy::{SudachiTokenizer, SplitMode};
//! use tantivy::Index;
//!
//! // Create tokenizer with mode C (longest units)
//! let tokenizer = SudachiTokenizer::new(SplitMode::C)?;
//!
//! // Register with Tantivy
//! index.tokenizers().register("lang_ja", tokenizer);
//! ```

pub mod stream;
pub mod tokenizer;

pub use stream::SudachiTokenStream;
pub use tokenizer::{SplitMode, SudachiError, SudachiTokenizer, TokenData};
