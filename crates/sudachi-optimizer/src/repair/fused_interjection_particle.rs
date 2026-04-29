//! `RepairFusedInterjectionParticle` — Repair interjection + particle fused into single morpheme.
//!
//! **Status:** scaffold (no-op). Body to be ported from
//! [Sirush/Jiten RepairStages.cs](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/RepairStages.cs)
//! in a follow-up commit. Search the C# source for `RepairFusedInterjectionParticle` to find
//! the function definition; corresponding Jiten test cases live under
//! `Jiten.Tests/Stages/`.

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::Morpheme;
use crate::token_features::MorphemeFeatures;

/// Stable name used in `Morpheme::applied_rules` and pipeline
/// diagnostics. Snake-case mirror of the Jiten C# method, prefixed
/// by phase.
pub const NAME: &str = "repair_fused_interjection_particle";

/// Construct the [`Stage`] for the canonical pipeline. Wires `NAME`,
/// the [`Phase::Repair`] phase, and the [`MorphemeFeatures`]
/// gate.
pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::INTERJECTION, apply)
}

/// Apply the stage. Currently a no-op — pipeline returns input
/// unchanged. Replace with the ported logic in the next pass.
pub fn apply(
    morphemes: Vec<Morpheme>,
    _lexicon: &dyn Lexicon,
) -> Vec<Morpheme> {
    morphemes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    #[test]
    fn no_op_returns_input_unchanged() {
        let ms = vec![Morpheme::synthesize(
            "猫",
            "ねこ",
            "猫",
            vec!["名詞".into()],
            0..1,
        )];
        let out = apply(ms, &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "猫");
        assert!(out[0].applied_rules.is_empty(), "no-op stub must not record rule");
    }
}
