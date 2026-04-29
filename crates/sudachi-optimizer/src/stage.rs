//! [`Stage`] — one named step in the optimizer pipeline.
//!
//! A stage is a function from `Vec<OptimizerToken>` to
//! `Vec<OptimizerToken>` plus metadata: a stable name for the audit
//! trail, a [`StageGroup`] for ordering, and a [`TokenFeatures`] gate
//! so we can skip stages whose triggering features aren't present in
//! the current token stream.

use crate::lookup::OptimizerLookup;
use crate::token::OptimizerToken;
use crate::token_features::TokenFeatures;

/// Pipeline grouping. Stages run in this order:
/// Split → Repair → Combine → Cleanup → Disambiguation, with multiple
/// passes interleaved per the Jiten convention (Pipeline.cs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StageGroup {
    Split,
    Repair,
    Combine,
    Cleanup,
    Disambiguation,
}

/// One named transformation. The closure form (`process`) is type-
/// erased so all stages can live in a single `Vec<Stage>` regardless
/// of where they came from.
pub struct Stage {
    pub name: &'static str,
    pub group: StageGroup,
    pub required_features: TokenFeatures,
    process: Box<
        dyn Fn(Vec<OptimizerToken>, &dyn OptimizerLookup) -> Vec<OptimizerToken>
            + Send
            + Sync,
    >,
}

impl Stage {
    /// Build a stage from a stable name, group, optional feature
    /// gate, and a closure.
    pub fn new<F>(
        name: &'static str,
        group: StageGroup,
        required_features: TokenFeatures,
        process: F,
    ) -> Self
    where
        F: Fn(Vec<OptimizerToken>, &dyn OptimizerLookup) -> Vec<OptimizerToken>
            + Send
            + Sync
            + 'static,
    {
        Self {
            name,
            group,
            required_features,
            process: Box::new(process),
        }
    }

    /// Convenience for stages with no required features.
    pub fn always<F>(name: &'static str, group: StageGroup, process: F) -> Self
    where
        F: Fn(Vec<OptimizerToken>, &dyn OptimizerLookup) -> Vec<OptimizerToken>
            + Send
            + Sync
            + 'static,
    {
        Self::new(name, group, TokenFeatures::empty(), process)
    }

    /// Apply the stage to a token stream.
    pub fn apply(&self, tokens: Vec<OptimizerToken>, lookup: &dyn OptimizerLookup) -> Vec<OptimizerToken> {
        (self.process)(tokens, lookup)
    }
}

impl std::fmt::Debug for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stage")
            .field("name", &self.name)
            .field("group", &self.group)
            .field("required_features", &self.required_features)
            .finish()
    }
}
