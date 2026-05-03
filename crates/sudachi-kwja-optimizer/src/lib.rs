//! # sudachi-kwja-optimizer
//!
//! Document-tree rewriter that fixes known KWJA mis-tagging.
//!
//! KWJA's structural analysis (bunsetsu / BP / dependency / NE /
//! features) is mostly correct when fed clean Sudachi+optimizer
//! morphemes, but has known failure modes — over-tagged NE spans,
//! malformed BIO sequences, inconsistent feature label spellings, low-
//! confidence multi-label noise. This crate runs a sequence of small,
//! named transformations on the raw KWJA Document tree to produce a
//! "post-optimised" tree better-suited for the comprehension layer
//! downstream.
//!
//! ```text
//! KWJA raw Document → OPTIMIZER PIPELINE → post-optimised Document
//!                          ↓
//!                 applied_rules tracked per pipeline run
//! ```
//!
//! ## Layer (2) in the comprehension pipeline
//!
//! See `COMPREHENSION_PIPELINE.md` at the workspace root. This crate
//! is the (2) layer:
//!
//! - **(1)** `sudachi-optimizer` — Sudachi morpheme cleanup
//! - **(2)** `sudachi-kwja-optimizer` — KWJA tree cleanup (this crate)
//! - **(3)** `jisho-core` — comprehension layer combining (1) and (2)
//!   with jisho-specific data
//!
//! Layer (2) is **mechanical**: rules don't depend on jisho-specific
//! data (vocab table, learner state, etc.). Cleanups that need vocab
//! corroboration (like KWJA reading drift gated on a vocab-table hit)
//! are (3) hybrid rules, not (2) optimizer rules.
//!
//! ## Phases
//!
//! | Phase     | Purpose                                                  |
//! |-----------|----------------------------------------------------------|
//! | Filter    | Remove low-confidence / spurious annotations             |
//! | Validate  | Check structural invariants (BIO sequences, dep arcs)    |
//! | Normalize | Canonicalise feature label spellings                     |
//!
//! ## Quick start
//!
//! ```ignore
//! use sudachi_kwja_optimizer::{Optimizer, Pipeline};
//! use sudachi_kwja::document::Document;
//!
//! let raw_doc: Document = /* from sudachi-kwja Pipeline::parse_morphemes */;
//! let optimizer = Optimizer::new().with_pipeline(Pipeline::analysis());
//! let clean_doc = optimizer.optimize(raw_doc);
//! ```
//!
//! ## Adding a new rule
//!
//! 1. Pick a phase subdirectory (`filter/`, `validate/`, `normalize/`).
//! 2. Add a new `<rule_name>.rs` file (one rule per file).
//! 3. Define `pub fn apply(doc: Document, lexicon: &dyn Lexicon)
//!    -> Document`.
//! 4. Register it in [`pipeline::canonical_stages`] with the right
//!    [`Phase`] and [`DocumentFeatures`] gate.
//! 5. Write unit tests in the same file.

pub mod doc_features;
pub mod lookup;
pub mod pipeline;
pub mod stage;

pub mod filter;
pub mod normalize;
pub mod validate;

mod optimizer;

pub use doc_features::DocumentFeatures;
pub use lookup::{EmptyLexicon, Lexicon};
pub use optimizer::Optimizer;
pub use pipeline::{Pipeline, optimize};
pub use stage::{Phase, Stage};

/// Re-export the KWJA Document tree types this crate operates on.
/// Consumers don't need a direct `sudachi-kwja` dep just to wire up
/// the optimizer; importing through this gateway mirrors how other
/// crates reach upstream `sudachi` through `sudachi_optimizer::sudachi`.
pub mod document {
    pub use sudachi_kwja::document::tree::{
        BasePhrase, Document, KeyValue, Morpheme, Phrase, Relation, Sentence,
    };
}
