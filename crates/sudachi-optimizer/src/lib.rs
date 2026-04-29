//! # sudachi-optimizer
//!
//! Token-stream rewriter that fixes known Sudachi mis-tokenisations.
//!
//! Sudachi's UniDic tokenization is correct for the dictionary but has
//! known weaknesses around colloquial Japanese, fused particles, and
//! compound auxiliary verbs. This crate runs a sequence of small,
//! named transformations on the raw Sudachi morpheme stream to produce
//! a "post-optimised" stream better-suited for grammar / vocab span
//! matching downstream.
//!
//! ```text
//! sudachi raw morphemes → OPTIMIZER PIPELINE → post-optimised morphemes
//!                              ↓
//!                     applied_rules tracked per morpheme
//! ```
//!
//! ## The single Sudachi gateway
//!
//! Every other crate in this workspace (sudachi-search, sudachi-sqlite,
//! sudachi-tantivy, sudachi-wasm) imports Sudachi types through
//! [`sudachi`] (this crate's re-export module) — never the upstream
//! `sudachi` crate directly. That gives us one place to apply the
//! optimization rules so all consumers see the same canonical
//! morpheme stream.
//!
//! ## Phases
//!
//! Mirrors the categorization in
//! [Sirush/Jiten Stages/](https://github.com/Sirush/Jiten/tree/master/Jiten.Parser/Stages):
//!
//! | Phase           | Purpose                                              |
//! |-----------------|------------------------------------------------------|
//! | Split           | Break apart over-merged Sudachi morphemes            |
//! | Repair          | Fix specific known mis-tokenisations                 |
//! | Combine         | Glue together morphemes that should have been one    |
//! | Cleanup         | Reclassify orphans, filter misparses                 |
//! | Disambiguation  | Fix reading ambiguity using neighbouring context     |
//!
//! ## Quick start
//!
//! ```ignore
//! use std::sync::Arc;
//! use sudachi_optimizer::{Optimizer, Pipeline};
//!
//! let dict = Arc::new(sudachi_optimizer::load_dictionary("/path/to/system_full.dic")?);
//! let optimizer = Optimizer::new(dict).with_pipeline(Pipeline::analysis());
//! let morphemes = optimizer.tokenize("食べてしまった")?;
//! for m in &morphemes {
//!     println!("{}\t{}\t{:?}\t{:?}", m.surface, m.reading_form, m.pos, m.applied_rules);
//! }
//! ```
//!
//! ## Adding a new rule
//!
//! 1. Pick a phase subdirectory (`split/`, `repair/`, `combine/`,
//!    `cleanup/`, `disambiguation/`).
//! 2. Add a new `<rule_name>.rs` file (one rule per file).
//! 3. Define `pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon)
//!    -> Vec<Morpheme>`.
//! 4. Register it in [`pipeline::canonical_stages`] with the right
//!    [`Phase`] and [`MorphemeFeatures`] gate.
//! 5. Write unit tests in the same file.
//!
//! ## Source attribution
//!
//! Initial set of rules ported from
//! [Sirush/Jiten](https://github.com/Sirush/Jiten) (MIT). Each rule's
//! docstring links back to its C# original so future audits can
//! verify behaviour.

pub mod data;
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

mod optimizer;

pub use lookup::{EmptyLexicon, Lexicon};
pub use optimizer::{OptimizeError, Optimizer};
pub use pipeline::{Pipeline, optimize};
pub use stage::{Phase, Stage};
pub use token::{Morpheme, Pos};
pub use token_features::MorphemeFeatures;

/// Convenience: load a Sudachi dictionary from a system-dic path.
/// Thin wrapper over upstream sudachi's loader so callers don't need
/// to reach the `sudachi` crate directly.
pub fn load_dictionary<P: AsRef<std::path::Path>>(
    system_dic_path: P,
) -> Result<sudachi::JapaneseDictionary, ::sudachi::error::SudachiError> {
    use ::sudachi::config::Config;
    use ::sudachi::dic::dictionary::JapaneseDictionary;

    let config = Config::new(None, None, Some(system_dic_path.as_ref().to_path_buf()))?;
    JapaneseDictionary::from_cfg(&config)
}
