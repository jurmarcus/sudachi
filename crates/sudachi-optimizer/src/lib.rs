//! # sudachi-optimizer
//!
//! Token-stream rewriter that fixes known Sudachi mis-tokenisations.
//!
//! Sudachi's UniDic tokenization is correct for the dictionary but has
//! known weaknesses around colloquial Japanese, fused particles, and
//! compound auxiliary verbs. This crate runs a sequence of small,
//! named transformations on the raw Sudachi token stream to produce
//! a "post-optimised" stream that's better-suited for grammar / vocab
//! span matching downstream.
//!
//! ```text
//! sudachi raw tokens → OPTIMIZER PIPELINE → post-optimised tokens
//!                            ↓
//!                    applied_rules tracked per token
//! ```
//!
//! ## The single Sudachi gateway
//!
//! Every other crate in this workspace (sudachi-search, sudachi-sqlite,
//! sudachi-tantivy, sudachi-wasm) imports Sudachi types through
//! [`sudachi`] (this crate's re-export module) — never the upstream
//! `sudachi` crate directly. That gives us one place to apply the
//! optimization rules so all consumers see the same canonical token
//! stream.
//!
//! ## Stage groups
//!
//! Mirrors the categorization in
//! [Jiten](https://github.com/Sirush/Jiten/tree/master/Jiten.Parser/Stages):
//!
//! | Group           | Purpose                                              |
//! |-----------------|------------------------------------------------------|
//! | Split           | Break apart over-merged Sudachi tokens               |
//! | Repair          | Fix specific known mis-tokenisations                 |
//! | Combine         | Glue together tokens that should have been one       |
//! | Cleanup         | Reclassify orphans, filter misparses                 |
//! | Disambiguation  | Fix reading ambiguity using neighbouring context     |
//!
//! ## Quick start
//!
//! ```ignore
//! use sudachi_optimizer::{Tokenizer, RuleSet};
//!
//! let dict = sudachi_optimizer::load_dictionary("/path/to/system_full.dic")?;
//! let tokenizer = Tokenizer::new(dict).with_rules(RuleSet::analysis());
//! let tokens = tokenizer.tokenize("食べてしまった")?;
//! // Post-optimization: ["食べて", "しまった"] (compound auxiliary recognised)
//! ```
//!
//! ## Adding a new rule
//!
//! 1. Pick a category subdirectory (`split/`, `repair/`, `combine/`,
//!    `cleanup/`, `disambiguation/`).
//! 2. Add a new `<rule_name>.rs` file (one rule per file).
//! 3. Define `pub fn apply<L: OptimizerLookup>(tokens: Vec<OptimizerToken>,
//!    lookup: &L) -> Vec<OptimizerToken>`.
//! 4. Register it in [`RuleSet::all`] with the right [`StageGroup`] and
//!    [`TokenFeatures`] gate.
//! 5. Write a unit test in the same file.
//!
//! ## Source attribution
//!
//! Initial set of rules ported from
//! [Sirush/Jiten](https://github.com/Sirush/Jiten) (MIT). Each rule's
//! docstring links back to its C# original so future audits can verify
//! behaviour.

pub mod lookup;
pub mod pipeline;
pub mod stage;
pub mod sudachi;
pub mod token;
pub mod token_features;

pub mod cleanup;
pub mod combine;
pub mod disambiguation;
pub mod repair;
pub mod split;

mod tokenizer;

pub use lookup::{NoLookup, OptimizerLookup};
pub use pipeline::{RuleSet, optimize_tokens};
pub use stage::{Stage, StageGroup};
pub use token::{OptimizerToken, SemanticPos};
pub use token_features::TokenFeatures;
pub use tokenizer::{TokenizeError, Tokenizer};

/// Convenience: load a Sudachi dictionary from a system-dic path. Thin
/// wrapper over upstream sudachi's loader to keep callers off the
/// `sudachi` crate directly.
pub fn load_dictionary<P: AsRef<std::path::Path>>(
    system_dic_path: P,
) -> Result<sudachi::JapaneseDictionary, ::sudachi::error::SudachiError> {
    use ::sudachi::config::Config;
    use ::sudachi::dic::dictionary::JapaneseDictionary;

    let config = Config::new(None, None, Some(system_dic_path.as_ref().to_path_buf()))?;
    JapaneseDictionary::from_cfg(&config)
}
