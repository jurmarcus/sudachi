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
}
