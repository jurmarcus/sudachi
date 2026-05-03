//! [`Pipeline`] — bundle of stages + the optimizer orchestrator.
//!
//! Mirrors `sudachi_optimizer::pipeline::Pipeline` in shape and
//! semantics: scan document features once, run each stage whose
//! feature gate matches, re-scan only if the document changed
//! shape (added or removed BPs / morphemes).

use crate::doc_features::DocumentFeatures;
use crate::document::Document;
use crate::lookup::{EmptyLexicon, Lexicon};
use crate::stage::Stage;

/// An ordered bundle of optimizer [`Stage`]s.
///
/// Different consumers want different stage subsets. Build a custom
/// pipeline with [`Pipeline::new`], or use the convenience
/// constructors:
///
/// - [`Pipeline::analysis`] — every canonical stage. The default for
///   downstream comprehension consumers.
/// - [`Pipeline::empty`] — no stages. Test fixture / passthrough.
pub struct Pipeline {
    stages: Vec<Stage>,
}

impl Pipeline {
    pub fn new(stages: Vec<Stage>) -> Self {
        Self { stages }
    }

    pub fn empty() -> Self {
        Self { stages: Vec::new() }
    }

    /// Every canonical stage, in pipeline ordering.
    ///
    /// Today this is just the NE filter — the crate is intentionally
    /// minimal at launch. Add stages here as concrete failure cases
    /// emerge in the regression corpus, not speculatively.
    pub fn analysis() -> Self {
        Self::new(canonical_stages())
    }

    /// The underlying stage list. Mostly useful for diagnostics
    /// and tests.
    pub fn stages(&self) -> &[Stage] {
        &self.stages
    }
}

/// Run the optimizer `pipeline` against `doc` using `lexicon` for
/// optional vocab queries. `doc` is consumed; the returned Document
/// is the post-optimisation tree.
///
/// Generic over any [`Lexicon`]; the dyn-cast happens at the stage
/// boundary so each stage's closure sees a uniform `&dyn Lexicon`.
pub fn optimize<L: Lexicon>(mut doc: Document, pipeline: &Pipeline, lexicon: &L) -> Document {
    let lexicon_dyn: &dyn Lexicon = lexicon;
    let mut features = DocumentFeatures::scan(&doc);

    for stage in &pipeline.stages {
        if !stage.required_features.is_empty()
            && (features & stage.required_features).is_empty()
        {
            continue;
        }
        let prev_shape = doc_shape(&doc);
        let next = stage.apply(doc, lexicon_dyn);
        let changed = doc_shape(&next) != prev_shape;
        doc = next;
        if changed {
            features = DocumentFeatures::scan(&doc);
        }
    }
    doc
}

/// Convenience: run with [`EmptyLexicon`].
pub fn optimize_no_lexicon(doc: Document, pipeline: &Pipeline) -> Document {
    optimize(doc, pipeline, &EmptyLexicon)
}

/// Lightweight shape signature used to detect "did the stage
/// actually change the document?" without deep comparison.
///
/// Returns counts at every nesting level. If counts match, we assume
/// shape is unchanged; stages that mutate in place without changing
/// counts (e.g. relabelling a feature value) don't trigger a full
/// re-scan, which is fine — the features they care about are still
/// present after the relabel, and re-scanning wouldn't change the
/// gating decisions for downstream stages.
fn doc_shape(doc: &Document) -> (usize, usize, usize, usize, usize) {
    let n_sentences = doc.sentences.len();
    let n_phrases: usize = doc.sentences.iter().map(|s| s.phrases.len()).sum();
    let n_bps: usize = doc.sentences.iter().map(|s| s.base_phrases.len()).sum();
    let n_features: usize = doc
        .sentences
        .iter()
        .flat_map(|s| s.base_phrases.iter())
        .map(|bp| bp.features.len())
        .sum();
    let n_morphemes: usize = doc.sentences.iter().map(|s| s.morphemes.len()).sum();
    (n_sentences, n_phrases, n_bps, n_features, n_morphemes)
}

// ────────────────────────────────────────────────────────────────────
// Canonical stage list construction
// ────────────────────────────────────────────────────────────────────

/// Build the canonical stage list.
///
/// Today this is just the NE filter — the crate is intentionally
/// minimal at launch. Per `COMPREHENSION_PIPELINE.md` the rule of
/// growth is "add when concrete failure cases emerge in the
/// regression corpus, not speculatively."
pub fn canonical_stages() -> Vec<Stage> {
    use crate::filter;

    vec![filter::ne::stage()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Document;

    #[test]
    fn empty_pipeline_is_identity() {
        let doc = Document {
            sentences: vec![],
            discourse_relations: vec![],
        };
        let out = optimize_no_lexicon(doc, &Pipeline::empty());
        assert!(out.sentences.is_empty());
    }

    #[test]
    fn canonical_pipeline_compiles_and_runs() {
        let pipeline = Pipeline::analysis();
        assert!(!pipeline.stages().is_empty(), "expected at least one stage");
        let doc = Document {
            sentences: vec![],
            discourse_relations: vec![],
        };
        let out = optimize_no_lexicon(doc, &pipeline);
        assert!(out.sentences.is_empty());
    }
}
