//! `SplitTatteParticle` — Split tatte/datte particle when miscategorised.
//!
//! Two distinct splits live in this stage (Jiten lumps them together
//! because both are surface-form rewrites of the same morpheme shape):
//!
//! 1. **だな misparsed as 棚 (shelf)** → split into だ (copula) + な
//!    (sentence-ending particle). Sudachi sometimes lemmatises だな
//!    as the noun 棚; we recognise the misparse via the
//!    `normalized_form == "棚"` guard.
//!
//! 2. **たって/だって as a conjunctive particle following a verb stem**
//!    → split into た/だ (past auxiliary) + って (quotative particle).
//!    Only when preceded by a verb / i-adjective / auxiliary in a
//!    stem form. Without this split, the past-tense conditional
//!    (eg. 出たって — "even if I went out") gets glued to the wrong
//!    morpheme by downstream Combine stages.
//!
//! Ported from
//! [Sirush/Jiten SplitStages.cs `SplitTatteParticle`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.SplitStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "split_tatte_particle";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Split, MorphemeFeatures::TEXT_TATTE, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 2);

    for (i, m) in morphemes.iter().enumerate() {
        // Case 1: だな misparsed as 棚 → split into だ + な.
        if m.surface == "だな"
            && matches!(m.pos, Pos::Noun)
            && m.normalized_form == "棚"
        {
            let begin = m.char_range.start;
            let end = m.char_range.end;
            let mut da =
                Morpheme::synthesize("だ", "だ", "だ", vec!["助動詞".into()], begin..begin + 1);
            da.record_rule(NAME);
            let mut na = Morpheme::synthesize(
                "な",
                "な",
                "な",
                vec!["助詞".into(), "終助詞".into()],
                begin + 1..end,
            );
            na.record_rule(NAME);
            result.push(da);
            result.push(na);
            continue;
        }

        // Case 2: たって/だって conjunctive particle after verb-like
        // stem → split into past-marker + quotative particle.
        if i > 0
            && matches!(m.pos, Pos::Particle)
            && m.part_of_speech.iter().skip(1).any(|p| p == "接続助詞")
            && (m.surface == "たって" || m.surface == "だって")
        {
            let prev = &morphemes[i - 1];
            let prev_is_stem =
                matches!(prev.pos, Pos::Verb | Pos::Adjective | Pos::Auxiliary);
            if prev_is_stem {
                let past_marker = if m.surface == "たって" { "た" } else { "だ" };
                let begin = m.char_range.start;
                let end = m.char_range.end;
                let mut past = Morpheme::synthesize(
                    past_marker,
                    past_marker,
                    past_marker,
                    vec!["助動詞".into()],
                    begin..begin + 1,
                );
                past.record_rule(NAME);
                let mut tte = Morpheme::synthesize(
                    "って",
                    "って",
                    "って",
                    vec!["助詞".into(), "接続助詞".into()],
                    begin + 1..end,
                );
                tte.record_rule(NAME);
                result.push(past);
                result.push(tte);
                continue;
            }
        }

        result.push(m.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    /// Build a verb morpheme (as Sudachi/UniDic would emit).
    fn verb(surface: &str, dict: &str, reading: &str) -> Morpheme {
        Morpheme::synthesize(
            surface,
            reading,
            dict,
            vec!["動詞".into()],
            0..surface.chars().count(),
        )
    }

    /// Build a conjunctive-particle morpheme (たって/だって shape).
    fn conj_particle(surface: &str) -> Morpheme {
        let n = surface.chars().count();
        Morpheme::synthesize(
            surface,
            surface,
            surface,
            vec!["助詞".into(), "接続助詞".into()],
            0..n,
        )
    }

    /// Build a noun morpheme with an explicit normalized form.
    fn noun(surface: &str, dict: &str, normalized: &str) -> Morpheme {
        let n = surface.chars().count();
        let mut m = Morpheme::synthesize(surface, surface, dict, vec!["名詞".into()], 0..n);
        m.normalized_form = normalized.to_string();
        m
    }

    /// Ported from
    /// [Jiten PipelineStageTests.cs `SplitStage_ShouldSplitTatteParticle`](https://github.com/Sirush/Jiten/blob/master/Jiten.Tests/PipelineStageTests.cs).
    #[test]
    fn splits_tatte_after_verb_into_ta_and_tte() {
        let mut deru = verb("出", "出る", "で");
        deru.char_range = 0..1;
        let mut tatte = conj_particle("たって");
        tatte.char_range = 1..4;
        let input = vec![deru, tatte];

        let out = apply(input, &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["出", "た", "って"]);
        assert!(matches!(out[1].pos, Pos::Auxiliary));
        assert!(matches!(out[2].pos, Pos::Particle));
        assert!(out[1].applied_rules.contains(&NAME));
    }

    #[test]
    fn splits_datte_after_verb_into_da_and_tte() {
        let mut yon = verb("読ん", "読む", "よん");
        yon.char_range = 0..2;
        let mut datte = conj_particle("だって");
        datte.char_range = 2..5;
        let input = vec![yon, datte];

        let out = apply(input, &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["読ん", "だ", "って"]);
        assert!(matches!(out[1].pos, Pos::Auxiliary));
        assert!(matches!(out[2].pos, Pos::Particle));
    }

    #[test]
    fn does_not_split_tatte_when_no_preceding_verb_stem() {
        let standalone = conj_particle("たって");
        let out = apply(vec![standalone.clone()], &EmptyLexicon);

        // Without a verb-like predecessor the rule must leave it alone.
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "たって");
        assert!(out[0].applied_rules.is_empty());
    }

    #[test]
    fn does_not_split_tatte_after_noun() {
        let book = Morpheme::synthesize("本", "ほん", "本", vec!["名詞".into()], 0..1);
        let mut tatte = conj_particle("たって");
        tatte.char_range = 1..4;
        let out = apply(vec![book, tatte], &EmptyLexicon);

        // After a noun, たって should remain untouched.
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["本", "たって"]);
    }

    #[test]
    fn splits_dana_misparsed_as_tana_into_da_and_na() {
        let dana = noun("だな", "棚", "棚");
        let out = apply(vec![dana], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["だ", "な"]);
        assert!(matches!(out[0].pos, Pos::Auxiliary));
        assert!(matches!(out[1].pos, Pos::Particle));
    }

    #[test]
    fn does_not_split_legitimate_dana_noun() {
        // 棚 alone (without surface だな) should be untouched.
        // Verifies the guards combine correctly: surface AND
        // normalized_form must both indicate the misparse.
        let shelf = noun("棚", "棚", "棚");
        let out = apply(vec![shelf], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "棚");
    }
}
