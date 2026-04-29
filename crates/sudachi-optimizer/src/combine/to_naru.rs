//! `CombineToNaru` — Re-merge と (Particle) + conjugated なる that
//! Sudachi splits when punctuation follows.
//!
//! Example: トラウマとなり、 → Sudachi emits と + なり + 、; this
//! stage merges to となり + 、 (the と+なる "becomes" pattern).
//!
//! ## Trigger
//!
//! - Current is と Particle.
//! - Next is Verb / Auxiliary with dictionary or normalized form
//!   ∈ {なる, 成る}.
//! - Next surface length ≤ 3 chars (filters compound conjugations
//!   that don't fit this pattern).
//! - Previous (in result) is Noun / Pronoun / NaAdjective.
//!
//! Merge into: surface と+next, dict form となる, normalized なる.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineToNaru`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};

pub const NAME: &str = "combine_to_naru";

pub fn stage() -> Stage {
    Stage::always(NAME, Phase::Combine, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut i = 0;
    while i < morphemes.len() {
        let cur = &morphemes[i];
        if matches!(cur.pos, Pos::Particle) && cur.surface == "と" && i + 1 < morphemes.len() {
            let next = &morphemes[i + 1];
            let next_is_naru = matches!(next.pos, Pos::Verb | Pos::Auxiliary)
                && (next.dictionary_form == "なる"
                    || next.dictionary_form == "成る"
                    || next.normalized_form == "なる"
                    || next.normalized_form == "成る");
            if next_is_naru && next.surface.chars().count() <= 3 {
                let prev_is_noun_like = out
                    .last()
                    .map(|p| matches!(p.pos, Pos::Noun | Pos::Pronoun | Pos::AdjectivalNoun))
                    .unwrap_or(false);
                if prev_is_noun_like {
                    let mut merged = next.clone();
                    merged.surface = format!("{}{}", cur.surface, next.surface);
                    merged.reading_form = format!("{}{}", cur.reading_form, next.reading_form);
                    merged.char_range = cur.char_range.start..next.char_range.end;
                    merged.dictionary_form = "となる".to_string();
                    merged.normalized_form = "なる".to_string();
                    merged.record_rule(NAME);
                    out.push(merged);
                    i += 2;
                    continue;
                }
            }
        }
        out.push(cur.clone());
        i += 1;
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
    fn merges_to_naru_after_noun() {
        let trauma = synth("トラウマ", "トラウマ", &["名詞"], 0..4);
        let to = synth("と", "と", &["助詞"], 4..5);
        let nari = synth("なり", "なる", &["動詞"], 5..7);
        let out = apply(vec![trauma, to, nari], &EmptyLexicon);
        assert_eq!(out.len(), 2);
        assert_eq!(out[1].surface, "となり");
        assert_eq!(out[1].dictionary_form, "となる");
    }

    #[test]
    fn does_not_merge_to_naru_after_verb() {
        let prev = synth("食べ", "食べる", &["動詞"], 0..2);
        let to = synth("と", "と", &["助詞"], 2..3);
        let nari = synth("なり", "なる", &["動詞"], 3..5);
        let out = apply(vec![prev, to, nari], &EmptyLexicon);
        assert_eq!(out.len(), 3);
    }

    #[test]
    fn does_not_merge_when_naru_is_too_long() {
        // Naru surface > 3 chars → filtered out.
        let trauma = synth("トラウマ", "トラウマ", &["名詞"], 0..4);
        let to = synth("と", "と", &["助詞"], 4..5);
        let long_naru = synth("なりまし", "なる", &["動詞"], 5..9);
        let out = apply(vec![trauma, to, long_naru], &EmptyLexicon);
        assert_eq!(out.len(), 3);
    }
}
