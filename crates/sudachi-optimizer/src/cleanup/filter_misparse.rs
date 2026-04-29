//! `FilterMisparse` — Filter token sequences known to be misparses (logged-but-discarded).
//!
//! **Status:** scaffold (no-op). Body to be ported from
//! [Sirush/Jiten Disambiguation.cs](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/Disambiguation.cs)
//! in a follow-up commit.
//!
//! Method: `FilterMisparse` (search the C# source for the function definition).

use crate::lookup::OptimizerLookup;
use crate::stage::{Stage, StageGroup};
use crate::token::OptimizerToken;
use crate::token_features::TokenFeatures;

/// Stable name used in `applied_rules` and pipeline diagnostics.
/// Snake-case mirror of the Jiten C# method, prefixed by category.
pub const NAME: &str = "cleanup_filter_misparse";

/// Construct the [`Stage`] for the canonical pipeline. Wires `NAME`,
/// the [`StageGroup::Cleanup`] grouping, and the [`TokenFeatures`]
/// gate.
pub fn stage() -> Stage {
    Stage::new(NAME, StageGroup::Cleanup, TokenFeatures::empty(), apply)
}

/// Apply the rule. Currently a no-op — pipeline returns input
/// unchanged. Replace with the ported logic in the next pass.
pub fn apply(
    tokens: Vec<OptimizerToken>,
    _lookup: &dyn OptimizerLookup,
) -> Vec<OptimizerToken> {
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::NoLookup;

    #[test]
    fn no_op_returns_input_unchanged() {
        let toks = vec![OptimizerToken::synthesize(
            "猫",
            "ねこ",
            "猫",
            vec!["名詞".into()],
            0..1,
        )];
        let out = apply(toks, &NoLookup);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "猫");
        assert!(out[0].applied_rules.is_empty(), "no-op stub must not record rule");
    }
}
