//! [`Optimizer`] — public entry point for layer (2) cleanup.
//!
//! Cheap to construct, cheap to clone (the pipeline is owned by
//! reference internally so `with_pipeline` just swaps the field).
//! Stateless across calls — each `optimize()` is independent.

use std::sync::Arc;

use crate::document::Document;
use crate::lookup::{EmptyLexicon, Lexicon};
use crate::pipeline::{Pipeline, optimize};

/// KWJA-side post-processor. Wraps a [`Pipeline`] and runs it over
/// any [`Document`] handed in.
///
/// ## Construction
///
/// `Optimizer::new()` builds with the canonical [`Pipeline::analysis`]
/// preset. Override with [`Optimizer::with_pipeline`] for tests or
/// specialised consumers.
///
/// ## Concurrency
///
/// `Optimizer` is `Clone + Send + Sync` — the inner pipeline is
/// `Arc`'d. Cheap to share across threads.
#[derive(Clone)]
pub struct Optimizer {
    pipeline: Arc<Pipeline>,
}

impl Optimizer {
    /// Build an optimizer with the default canonical pipeline.
    pub fn new() -> Self {
        Self {
            pipeline: Arc::new(Pipeline::analysis()),
        }
    }

    /// Override the default pipeline. Useful for tests
    /// ([`Pipeline::empty`]) and for specialised consumers that want
    /// a custom stage subset.
    pub fn with_pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipeline = Arc::new(pipeline);
        self
    }

    /// Run the pipeline over `doc`. Uses [`EmptyLexicon`] — for
    /// vocab-aware pipelines, use [`Optimizer::optimize_with`].
    pub fn optimize(&self, doc: Document) -> Document {
        self.optimize_with(doc, &EmptyLexicon)
    }

    /// Run the pipeline over `doc` with `lexicon` providing optional
    /// vocab knowledge.
    pub fn optimize_with<L: Lexicon>(&self, doc: Document, lexicon: &L) -> Document {
        optimize(doc, &self.pipeline, lexicon)
    }

    /// Run the pipeline against a `&mut Document`, mutating it in
    /// place. Convenience for callers who already own a `&mut`
    /// (e.g. iterating over a `Vec<ParseItem>` and mutating each
    /// `Tree(doc)` variant).
    ///
    /// Internally uses `std::mem::replace` with a placeholder empty
    /// `Document` to satisfy the take-ownership signature of the
    /// pipeline runner. The placeholder is dropped immediately, so
    /// there's no observable side-effect — but a panic between the
    /// `replace` and the assignment back would leave `doc` empty.
    /// Stages don't panic in normal operation; if you've supplied a
    /// pipeline that can panic, use the take-ownership API instead.
    pub fn optimize_in_place(&self, doc: &mut Document) {
        self.optimize_in_place_with(doc, &EmptyLexicon);
    }

    /// In-place variant of [`Optimizer::optimize_with`].
    pub fn optimize_in_place_with<L: Lexicon>(&self, doc: &mut Document, lexicon: &L) {
        let placeholder = Document {
            sentences: Vec::new(),
            discourse_relations: Vec::new(),
        };
        let owned = std::mem::replace(doc, placeholder);
        *doc = self.optimize_with(owned, lexicon);
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_optimizer_uses_canonical_pipeline() {
        let opt = Optimizer::default();
        // Should run without panicking on an empty document.
        let doc = Document {
            sentences: vec![],
            discourse_relations: vec![],
        };
        let out = opt.optimize(doc);
        assert!(out.sentences.is_empty());
    }

    #[test]
    fn with_empty_pipeline_is_identity() {
        let opt = Optimizer::new().with_pipeline(Pipeline::empty());
        let doc = Document {
            sentences: vec![],
            discourse_relations: vec![],
        };
        let out = opt.optimize(doc);
        assert!(out.sentences.is_empty());
    }

    #[test]
    fn in_place_runs_canonical_pipeline() {
        // Build a doc with one bad NE entry that the canonical
        // pipeline (filter_ne) should drop, plus one good entry it
        // should keep.
        use crate::document::{BasePhrase, KeyValue, Sentence};

        let mut doc = Document {
            sentences: vec![Sentence {
                text: "test".into(),
                phrases: vec![],
                base_phrases: vec![BasePhrase {
                    id: 0,
                    surface: "test".into(),
                    head: -1,
                    dep_type: "D".into(),
                    morphemes: vec![],
                    features: vec![
                        // bad: pure-hiragana PERSON → drop
                        KeyValue {
                            key: "NE".into(),
                            value: "PERSON:やまだ".into(),
                        },
                        // good: real kanji name → keep
                        KeyValue {
                            key: "NE".into(),
                            value: "PERSON:山田".into(),
                        },
                    ],
                    relations: vec![],
                }],
                morphemes: vec![],
            }],
            discourse_relations: vec![],
        };

        let opt = Optimizer::new();
        opt.optimize_in_place(&mut doc);

        let features = &doc.sentences[0].base_phrases[0].features;
        assert_eq!(features.len(), 1, "expected one NE feature to survive");
        assert_eq!(features[0].value, "PERSON:山田");
    }

    #[test]
    fn in_place_with_empty_pipeline_is_identity() {
        use crate::document::{BasePhrase, KeyValue, Sentence};

        let original_features = vec![KeyValue {
            key: "NE".into(),
            value: "PERSON:やまだ".into(),
        }];
        let mut doc = Document {
            sentences: vec![Sentence {
                text: "test".into(),
                phrases: vec![],
                base_phrases: vec![BasePhrase {
                    id: 0,
                    surface: "test".into(),
                    head: -1,
                    dep_type: "D".into(),
                    morphemes: vec![],
                    features: original_features.clone(),
                    relations: vec![],
                }],
                morphemes: vec![],
            }],
            discourse_relations: vec![],
        };

        Optimizer::new()
            .with_pipeline(Pipeline::empty())
            .optimize_in_place(&mut doc);

        // No stages → no change.
        assert_eq!(doc.sentences[0].base_phrases[0].features, original_features);
    }
}
