//! `CombineConjunctiveParticle` — Glue て/で/ちゃ/ば onto the
//! preceding verb / i-adjective / auxiliary morpheme.
//!
//! ## Trigger
//!
//! - Current morpheme has 接続助詞 sub-POS AND surface ∈ {て, で, ちゃ, ば}.
//! - Previous morpheme is Verb / Adjective (i-adj) / Auxiliary.
//!
//! Append onto previous in-place.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineConjunctiveParticle`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_conjunctive_particle";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::CONJ_PARTICLE, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    out.push(morphemes[0].clone());
    for current in morphemes.into_iter().skip(1) {
        let is_target_particle = current
            .part_of_speech
            .iter()
            .skip(1)
            .any(|p| p == "接続助詞")
            && matches!(current.surface.as_str(), "て" | "で" | "ちゃ" | "ば");
        let prev_compatible = matches!(
            out.last().map(|p| p.pos),
            Some(Pos::Verb) | Some(Pos::Adjective) | Some(Pos::Auxiliary)
        );

        if is_target_particle && prev_compatible {
            let prev = out.last_mut().unwrap();
            prev.surface.push_str(&current.surface);
            prev.reading_form.push_str(&current.reading_form);
            prev.char_range = prev.char_range.start..current.char_range.end;
            prev.record_rule(NAME);
        } else {
            out.push(current);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    fn synth(
        surface: &str,
        dict: &str,
        pos: &[&str],
        char_range: std::ops::Range<usize>,
    ) -> Morpheme {
        Morpheme::synthesize(
            surface,
            surface,
            dict,
            pos.iter().map(|s| s.to_string()).collect(),
            char_range,
        )
    }

    #[test]
    fn merges_te_after_verb() {
        let tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        let te = synth("て", "て", &["助詞", "接続助詞"], 2..3);
        let out = apply(vec![tabe, te], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "食べて");
    }

    #[test]
    fn merges_ba_after_iadjective() {
        let yokere = synth("良けれ", "良い", &["形容詞"], 0..3);
        let ba = synth("ば", "ば", &["助詞", "接続助詞"], 3..4);
        let out = apply(vec![yokere, ba], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "良ければ");
    }

    #[test]
    fn merges_cha_after_verb() {
        let mi = synth("見", "見る", &["動詞"], 0..1);
        let cha = synth("ちゃ", "ちゃ", &["助詞", "接続助詞"], 1..3);
        let out = apply(vec![mi, cha], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "見ちゃ");
    }

    #[test]
    fn does_not_merge_te_after_noun() {
        let school = synth("学校", "学校", &["名詞"], 0..2);
        let te = synth("て", "て", &["助詞", "接続助詞"], 2..3);
        let out = apply(vec![school, te], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_other_conjunctive_particles() {
        let tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        let to = synth("と", "と", &["助詞", "接続助詞"], 2..3);
        let out = apply(vec![tabe, to], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }
}
