//! `CombineInflections` — Iteratively merge a base inflectable
//! morpheme (Verb / Adjective / NaAdjective / Auxiliary /
//! verb-like Suffix) with following morphemes whenever the
//! deconjugator validates the merged candidate as a real verb /
//! adjective form.
//!
//! ## Status of port
//!
//! **Deferred — needs Jiten's `Deconjugator`.**
//!
//! Every merge in this stage is conditional on a deconjugator
//! lookup proving the candidate is a valid inflected form. Without
//! the deconjugator we'd either:
//! - over-merge and produce fake verb forms (hurts downstream
//!   matching), or
//! - never merge (no value).
//!
//! Until Jiten's `Deconjugator` (16KB + table) is ported into a
//! sibling crate or into this one, the rule is registered in the
//! canonical pipeline but is a no-op. The pipeline shape is
//! preserved so when the deconjugator lands, swapping in the body
//! is a one-file change.
//!
//! Original C# at
//! [Sirush/Jiten CombineStages.cs `CombineInflections`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs)
//! (~300 lines). Each merge candidate calls
//! `deconjugator.Deconjugate(candidate_hiragana)` and checks the
//! returned `DeconjugationForm`s for verb tags / past markers.
//!
//! TODO: port Jiten's `Deconjugator` and re-implement this stage.

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::Morpheme;
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_inflections";

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
        let out = apply(vec![m.clone()], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "食べ");
        assert!(out[0].applied_rules.is_empty());
    }
}
