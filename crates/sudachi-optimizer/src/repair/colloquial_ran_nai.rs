//! `RepairColloquialRanNai` — Merge colloquial らん + negative
//! (ない / ねえ / ねぇ / ねー) into a single Auxiliary morpheme when
//! preceded by a te/de form.
//!
//! Sudachi tokenises らん as an adverb, which prevents
//! `combine::inflections` from gluing it to the preceding verb.
//! Jiten's deconjugator already has the rule `らんない → られない`
//! (n-slang), so this repair just needs to produce a single
//! mergeable morpheme that the deconjugator can recognise.
//!
//! Example: 付き合っ + て + らん + ない → 付き合っ + て + らんない (Auxiliary).
//!
//! ## Output
//!
//! Replace the `(らん, next)` pair with a single morpheme:
//! - surface: `らん` + next.surface
//! - dict / normalized form: られない
//! - reading: `ラン` + next.reading_form
//! - POS: Auxiliary (助動詞)
//!
//! Ported from
//! [Sirush/Jiten RepairStages.cs `RepairColloquialRanNai`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.RepairStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_colloquial_ran_nai";

const NEGATIVES: &[&str] = &["ない", "ねえ", "ねぇ", "ねー"];

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::TEXT_RAN, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 3 {
        return morphemes;
    }

    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut i = 0;
    while i < morphemes.len() {
        let current = &morphemes[i];

        let next = morphemes.get(i + 1);
        let prev = result.last();

        let is_ran = current.surface == "らん";
        let next_is_negative = next.is_some_and(|n| NEGATIVES.contains(&n.surface.as_str()));
        let prev_is_te_de_context = prev.is_some_and(|p| {
            matches!(p.pos, Pos::Particle) && (p.surface == "て" || p.surface == "で")
                || p.surface.ends_with('て')
                || p.surface.ends_with('で')
        });

        if is_ran && next_is_negative && prev_is_te_de_context {
            let next = next.unwrap();
            let merged_surface = format!("らん{}", next.surface);
            let merged_reading = format!("ラン{}", next.reading_form);
            let mut merged = Morpheme::synthesize(
                merged_surface,
                merged_reading,
                "られない",
                vec!["助動詞".into()],
                current.char_range.start..next.char_range.end,
            );
            merged.normalized_form = "られない".to_string();
            merged.record_rule(NAME);
            result.push(merged);
            i += 2; // consume current + next
            continue;
        }

        result.push(current.clone());
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    fn synth(
        surface: &str,
        dict: &str,
        pos_top: &str,
        char_range: std::ops::Range<usize>,
    ) -> Morpheme {
        Morpheme::synthesize(surface, surface, dict, vec![pos_top.into()], char_range)
    }

    #[test]
    fn merges_ran_nai_after_te_particle() {
        // 付き合っ + て + らん + ない → 付き合っ + て + らんない (Auxiliary).
        let tsukiatte = synth("付き合っ", "付き合う", "動詞", 0..4);
        let te = synth("て", "て", "助詞", 4..5);
        let ran = synth("らん", "らん", "副詞", 5..7);
        let nai = synth("ない", "ない", "助動詞", 7..9);
        let out = apply(vec![tsukiatte, te, ran, nai], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["付き合っ", "て", "らんない"]);
        assert!(matches!(out[2].pos, Pos::Auxiliary));
        assert_eq!(out[2].dictionary_form, "られない");
        assert!(out[2].applied_rules.contains(&NAME));
    }

    #[test]
    fn merges_ran_nee_after_de_particle() {
        let aru = synth("飛んで", "飛ぶ", "動詞", 0..3);
        let ran = synth("らん", "らん", "副詞", 3..5);
        let nee = synth("ねえ", "ねえ", "助動詞", 5..7);
        let out = apply(vec![aru, ran, nee], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["飛んで", "らんねえ"]);
        assert!(matches!(out[1].pos, Pos::Auxiliary));
    }

    #[test]
    fn does_not_merge_without_te_de_context() {
        // らん after a noun → not a colloquial negative pattern.
        let school = synth("学校", "学校", "名詞", 0..2);
        let ran = synth("らん", "らん", "副詞", 2..4);
        let nai = synth("ない", "ない", "助動詞", 4..6);
        let out = apply(vec![school, ran, nai], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["学校", "らん", "ない"]);
    }

    #[test]
    fn does_not_merge_when_next_is_not_negative() {
        let prev = synth("行って", "行く", "動詞", 0..3);
        let ran = synth("らん", "らん", "副詞", 3..5);
        let other = synth("だ", "だ", "助動詞", 5..6);
        let out = apply(vec![prev, ran, other], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["行って", "らん", "だ"]);
    }
}
