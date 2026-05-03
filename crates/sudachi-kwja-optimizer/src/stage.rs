//! [`Stage`] — one named step in the KWJA-optimizer pipeline.
//!
//! Mirrors `sudachi_optimizer::stage::Stage` but operates on the KWJA
//! [`Document`] tree instead of a `Vec<Morpheme>`. A stage is a
//! function from `Document` to `Document` plus metadata: a stable
//! name for the audit trail, a [`Phase`] for ordering, and a
//! [`DocumentFeatures`] gate so stages whose triggering features
//! aren't present in the current tree get skipped.

use crate::doc_features::DocumentFeatures;
use crate::document::Document;
use crate::lookup::Lexicon;

/// The phase of optimization a stage belongs to.
///
/// Stages run in this order — Filter → Validate → Normalize — with
/// optional interleaving as the rule set grows. Mirrors
/// `sudachi_optimizer::stage::Phase` in shape; the labels differ
/// because KWJA-side cleanup operations have a different vocabulary
/// (no Split/Combine — KWJA doesn't reorder morphemes; we work with
/// the tree it produced).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    /// Drop spurious annotations (low-confidence NE, noisy
    /// multi-label features, etc.).
    Filter,
    /// Check structural invariants (BIO sequence well-formedness,
    /// dependency-arc sanity, etc.).
    Validate,
    /// Canonicalise label spellings (`敬語=尊敬` vs `敬語=尊敬語`,
    /// etc.).
    Normalize,
}

/// One named transformation. The closure form (`process`) is
/// type-erased so all stages can live in a single `Vec<Stage>`
/// regardless of where they came from.
pub struct Stage {
    pub name: &'static str,
    pub phase: Phase,
    pub required_features: DocumentFeatures,
    process: Box<dyn Fn(Document, &dyn Lexicon) -> Document + Send + Sync>,
}

impl Stage {
    /// Build a stage from a stable name, phase, optional feature
    /// gate, and a closure.
    pub fn new<F>(
        name: &'static str,
        phase: Phase,
        required_features: DocumentFeatures,
        process: F,
    ) -> Self
    where
        F: Fn(Document, &dyn Lexicon) -> Document + Send + Sync + 'static,
    {
        Self {
            name,
            phase,
            required_features,
            process: Box::new(process),
        }
    }

    /// Convenience for stages with no required features.
    pub fn always<F>(name: &'static str, phase: Phase, process: F) -> Self
    where
        F: Fn(Document, &dyn Lexicon) -> Document + Send + Sync + 'static,
    {
        Self::new(name, phase, DocumentFeatures::empty(), process)
    }

    /// Apply the stage to a Document.
    pub fn apply(&self, doc: Document, lexicon: &dyn Lexicon) -> Document {
        (self.process)(doc, lexicon)
    }
}

impl std::fmt::Debug for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stage")
            .field("name", &self.name)
            .field("phase", &self.phase)
            .field("required_features", &self.required_features)
            .finish()
    }
}
