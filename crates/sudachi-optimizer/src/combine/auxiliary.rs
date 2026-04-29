//! `CombineAuxiliary` — Merge Auxiliary morphemes (た, ます, ている,
//! etc.) onto preceding Verb / Adjective / NaAdjective / Auxiliary
//! morphemes, with a long blacklist of dictionary forms / surfaces
//! that must NOT merge.
//!
//! ## Status of port
//!
//! **Deferred — needs Jiten's `Deconjugator`.**
//!
//! The C# original (`CombineStages.cs ~100 lines`) has two paths:
//! 1. The copula-である merge path (で + ある → である) — could be
//!    ported standalone, but is small enough to roll into the full
//!    port later.
//! 2. The main auxiliary-merge path with a 20-clause blacklist
//!    (skip if dict form ∈ {らしい, べし, む, ようだ, …}, skip if
//!    surface ∈ {なら, なる, だろう, なのだ, …}, special
//!    deconjugator-validated だ-merge for past-tense verbs ending
//!    in ん via `IsValidNdaPastTense`).
//!
//! The `IsValidNdaPastTense` helper relies on the Deconjugator. The
//! verb-stem-existence check (`VerbDictFormExistsInLookup`) uses a
//! lexicon callback we already have, but the bulk of the validation
//! is deconjugator-driven.
//!
//! Until the Deconjugator is ported, this rule is a no-op
//! registered in the canonical pipeline. Original C# at
//! [Sirush/Jiten CombineStages.cs `CombineAuxiliary`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).
//!
//! TODO: port Jiten's `Deconjugator` and re-implement this stage.

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::Morpheme;
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_auxiliary";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::empty(), apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    // No-op until Deconjugator is ported. See module docs.
    morphemes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    #[test]
    fn no_op_until_deconjugator_lands() {
        let m = Morpheme::synthesize("食べ", "タベ", "食べる", vec!["動詞".into()], 0..2);
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert!(out[0].applied_rules.is_empty());
    }
}
