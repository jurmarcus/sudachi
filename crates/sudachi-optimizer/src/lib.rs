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

// Upstream `sudachi::error::SudachiError` is a 136-byte enum that
// drives the `clippy::result_large_err` lint on every wrapper that
// returns `Result<_, SudachiError>`. Boxing it into
// `Result<_, Box<SudachiError>>` would change the public API of
// `load_dictionary` / `Optimizer::*` and ripple a `Box<...>` through
// every caller across this workspace and jisho. The error is
// constructed only on dictionary-load failures (rare path), so the
// stack-bloat risk doesn't justify the API churn — silenced with
// rationale here rather than each call site.
#![allow(clippy::result_large_err)]

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
///
/// User dictionaries can be supplied via the
/// `SUDACHI_USER_DICT_PATHS` env var: a colon-separated list of
/// `.dic` paths produced by `sudachi ubuild`. They stack on top of
/// the system dict at runtime (`LexiconSet` searches all of them per
/// lookup). Empty / unset → no user dicts. Missing files → loader
/// fails, surfaced as a `SudachiError`.
///
/// Build / consume convention: keep CSV sources in the consumer repo
/// (e.g. `data/sudachi-custom/*.csv` in jisho) and `sudachi ubuild`
/// them into a deploy directory; point `SUDACHI_USER_DICT_PATHS` at
/// the built `.dic`. See the consumer's README for the full pipeline.
pub fn load_dictionary<P: AsRef<std::path::Path>>(
    system_dic_path: P,
) -> Result<sudachi::JapaneseDictionary, ::sudachi::error::SudachiError> {
    load_dictionary_with_user_dicts(system_dic_path, env_user_dicts())
}

/// As [`load_dictionary`] but with explicit user-dict paths. Useful
/// for tests and for callers that want full control over the dict set
/// instead of going through the env var.
///
/// Layering matches upstream's `Config::new`: starts from the default
/// `sudachi.json` (which carries the OOV / input-text / path-rewrite
/// plugin chain — without it Sudachi rejects the dict at load with
/// "No out of vocabulary plugin provided"), then overrides the system
/// dict path, then appends each user dict.
pub fn load_dictionary_with_user_dicts<P, U>(
    system_dic_path: P,
    user_dict_paths: U,
) -> Result<sudachi::JapaneseDictionary, ::sudachi::error::SudachiError>
where
    P: AsRef<std::path::Path>,
    U: IntoIterator,
    U::Item: AsRef<std::path::Path>,
{
    use ::sudachi::config::ConfigBuilder;
    use ::sudachi::dic::dictionary::JapaneseDictionary;

    // Start from the default sudachi.json (plugins included), then
    // override the system dict path, then layer user dicts on top.
    let mut builder = ConfigBuilder::from_opt_file(None)?
        .system_dict(system_dic_path.as_ref());
    for u in user_dict_paths {
        builder = builder.user_dict(u.as_ref());
    }
    JapaneseDictionary::from_cfg(&builder.build())
}

/// Read `SUDACHI_USER_DICT_PATHS` (`:`-separated list of `.dic`
/// paths). Returns an empty vec when the var is unset or empty.
fn env_user_dicts() -> Vec<std::path::PathBuf> {
    std::env::var("SUDACHI_USER_DICT_PATHS")
        .ok()
        .map(|raw| {
            raw.split(':')
                .filter(|s| !s.is_empty())
                .map(std::path::PathBuf::from)
                .collect()
        })
        .unwrap_or_default()
}
