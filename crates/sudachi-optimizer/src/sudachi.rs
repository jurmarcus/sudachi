//! Re-exports of upstream `sudachi` crate types.
//!
//! This module is the *only* public surface through which downstream
//! crates should reach Sudachi's primitives. Centralising the imports
//! here means we can later swap the upstream rev (or even the
//! tokenizer entirely) without touching every consumer.

// Analysis primitives.
pub use ::sudachi::analysis::Mode;
pub use ::sudachi::analysis::Tokenize;
pub use ::sudachi::analysis::stateless_tokenizer::StatelessTokenizer;

// Dictionary loading.
pub use ::sudachi::config::Config;
pub use ::sudachi::dic::dictionary::JapaneseDictionary;
pub use ::sudachi::dic::storage::{Storage, SudachiDicData};

// Errors and the prelude.
pub use ::sudachi::error::SudachiError;
pub use ::sudachi::prelude::Morpheme;
