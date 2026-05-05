//! [`Stage`] — one named step in the optimizer pipeline.
//!
//! A stage is a function from `Vec<Morpheme>` to `Vec<Morpheme>` plus
//! metadata: a stable name for the audit trail, a [`Phase`] for
//! ordering, and a [`MorphemeFeatures`] gate so stages whose
//! triggering features aren't present in the current morpheme stream
//! get skipped.

use crate::lookup::Lexicon;
use crate::token::Morpheme;
use crate::token_features::MorphemeFeatures;

/// The phase of optimization a stage belongs to. Stages run in this
/// order — Split → Repair → Combine → Cleanup → Disambiguation —
/// with multiple passes interleaved per the canonical pipeline
/// (see [`crate::pipeline::canonical_stages`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Split,
    Repair,
    Combine,
    Cleanup,
    Disambiguation,
}

/// The boxed closure shape stored inside [`Stage::process`]. Lifted
/// to a type alias so the field doesn't trip clippy's
/// `type_complexity` lint and so it can be referenced by name.
type StageProcess =
    Box<dyn Fn(Vec<Morpheme>, &dyn Lexicon) -> Vec<Morpheme> + Send + Sync>;

/// One named transformation. The closure form (`process`) is
/// type-erased so all stages can live in a single `Vec<Stage>`
/// regardless of where they came from.
pub struct Stage {
    pub name: &'static str,
    pub phase: Phase,
    pub required_features: MorphemeFeatures,
    process: StageProcess,
}

impl Stage {
    /// Build a stage from a stable name, phase, optional feature
    /// gate, and a closure.
    pub fn new<F>(
        name: &'static str,
        phase: Phase,
        required_features: MorphemeFeatures,
        process: F,
    ) -> Self
    where
        F: Fn(Vec<Morpheme>, &dyn Lexicon) -> Vec<Morpheme> + Send + Sync + 'static,
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
        F: Fn(Vec<Morpheme>, &dyn Lexicon) -> Vec<Morpheme> + Send + Sync + 'static,
    {
        Self::new(name, phase, MorphemeFeatures::empty(), process)
    }

    /// Apply the stage to a morpheme stream.
    pub fn apply(&self, morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
        (self.process)(morphemes, lexicon)
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
