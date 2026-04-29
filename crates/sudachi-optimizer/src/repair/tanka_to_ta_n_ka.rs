//! `RepairTankaToTaNKa` — Repair たんか misparsed as 短歌 ("tanka"
//! poem) noun when it should be た + ん + か (past + のだ particle
//! + question particle, e.g. 行ったんか — "did you go?").
//!
//! ## Algorithm (per Jiten)
//!
//! For each `たんか` noun morpheme:
//! 1. Skip if followed by を (real noun usage like 短歌を吐く).
//! 2. Skip if preceded by を or の (real noun marker).
//! 3. Otherwise, examine the previous content morpheme (skipping
//!    punctuation):
//!    - **Pattern (te/de)**: prev ends in て or で → split, replace
//!      prev with prev+た, emit ん + か.
//!    - **Pattern (verb past tense)**: deconjugator says prev+た is
//!      a valid verb past tense → split.
//!    - **Pattern (Kansai もう)**: prev = もう, before-prev ends in
//!      て/で → 3-way merge (verbて + もう + た → verbてもうた).
//!    - **Pattern (Kansai しもた)**: prev = も, before-prev = し,
//!      before-before ends in て/で → 4-way merge.
//!
//! ## Status of port
//!
//! Implemented: te/de pattern + the negative guards (を/の around).
//!
//! Deferred (require Jiten's Deconjugator and/or Kansai-specific
//! merge logic):
//! - verb-past-tense pattern (needs Deconjugator)
//! - Kansai もう pattern (uncommon, complex 3-way merge)
//! - Kansai しもた pattern (uncommon, complex 4-way merge)
//!
//! TODO: port these once the Deconjugator lands.
//!
//! Ported from
//! [Sirush/Jiten RepairStages.cs `RepairTankaToTaNKa`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.RepairStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_tanka_to_ta_n_ka";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::TEXT_TANKA, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 4);

    for (i, m) in morphemes.iter().enumerate() {
        if !matches!(m.pos, Pos::Noun) || m.surface != "たんか" {
            result.push(m.clone());
            continue;
        }

        // Guard: followed by を — real noun usage.
        if i + 1 < morphemes.len() && morphemes[i + 1].surface == "を" {
            result.push(m.clone());
            continue;
        }
        // Guard: preceded by を or の — real noun usage.
        if let Some(prev_in_result) = result.last() {
            if prev_in_result.surface == "を" || prev_in_result.surface == "の" {
                result.push(m.clone());
                continue;
            }
        }

        // Find the previous content morpheme (skip punctuation).
        let prev_idx = result
            .iter()
            .enumerate()
            .rev()
            .find(|(_, p)| !matches!(p.pos, Pos::Symbol))
            .map(|(idx, _)| idx);
        let Some(prev_idx) = prev_idx else {
            result.push(m.clone());
            continue;
        };
        let prev = &result[prev_idx];

        // Pattern (te/de): prev ends in て/で → split.
        let prev_ends_in_te_de = prev
            .surface
            .chars()
            .last()
            .is_some_and(|c| c == 'て' || c == 'で');

        // Pattern (verb past tense): deconjugator says prev+た is a
        // valid verb past tense. Use the lexicon's morphology
        // oracle to validate without baking conjugation rules
        // into the optimizer.
        let prev_plus_ta_is_verb_past = if !prev_ends_in_te_de {
            let candidate = format!("{}た", prev.surface);
            lexicon.is_valid_verb_past(&candidate)
        } else {
            false
        };

        if !prev_ends_in_te_de && !prev_plus_ta_is_verb_past {
            // Other patterns (Kansai もう / しもた) still deferred —
            // they need multi-token merge logic beyond a simple
            // verb-past validation.
            result.push(m.clone());
            continue;
        }

        // Replace prev with prev+た, emit ん + か.
        let begin = m.char_range.start;
        let end = m.char_range.end;

        let prev_owned = result[prev_idx].clone();
        let mut prev_plus_ta = prev_owned;
        prev_plus_ta.surface.push('た');
        prev_plus_ta.char_range = prev_plus_ta.char_range.start..(begin + 1);
        prev_plus_ta.pos = Pos::Verb;
        prev_plus_ta.part_of_speech = vec!["動詞".into()];
        prev_plus_ta.record_rule(NAME);
        result[prev_idx] = prev_plus_ta;

        let mut nn = Morpheme::synthesize(
            "ん",
            "ん",
            "の",
            vec!["助詞".into(), "準体助詞".into()],
            (begin + 1)..(begin + 2),
        );
        nn.normalized_form = "ん".to_string();
        nn.record_rule(NAME);
        result.push(nn);

        let mut ka = Morpheme::synthesize(
            "か",
            "か",
            "か",
            vec!["助詞".into()],
            (begin + 2)..end,
        );
        ka.record_rule(NAME);
        result.push(ka);
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
    fn splits_after_te_form_predecessor() {
        // 怖がって + たんか → 怖がってた + ん + か.
        let prev = synth("怖がって", "怖がる", "動詞", 0..4);
        let tanka = synth("たんか", "短歌", "名詞", 4..7);
        let out = apply(vec![prev, tanka], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["怖がってた", "ん", "か"]);
        assert!(matches!(out[0].pos, Pos::Verb));
        assert!(matches!(out[1].pos, Pos::Particle));
        assert!(matches!(out[2].pos, Pos::Particle));
    }

    #[test]
    fn skips_when_followed_by_wo() {
        // 短歌 followed by を → real noun, leave alone.
        let tanka = synth("たんか", "短歌", "名詞", 0..3);
        let wo = synth("を", "を", "助詞", 3..4);
        let out = apply(vec![tanka, wo], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["たんか", "を"]);
    }

    #[test]
    fn skips_when_preceded_by_no() {
        // のたんか → 〜の + 短歌 (real noun phrase).
        let no = synth("の", "の", "助詞", 0..1);
        let tanka = synth("たんか", "短歌", "名詞", 1..4);
        let out = apply(vec![no, tanka], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["の", "たんか"]);
    }

    #[test]
    fn splits_after_verb_past_validated_predecessor() {
        // 云う stem 云 + たんか. After this rule, prev becomes 云った
        // and we should split into 云った + ん + か since the
        // morphology oracle confirms 云った is a valid verb past.
        // Note: my deconjugator's coverage of v5u verbs makes this
        // path active for prev = 云っ (te-form) which already ends
        // in っ → wait, no, that ends in て. Let me pick a stem that
        // doesn't end in て/で but that prev+た is recognised as
        // valid past.
        //
        // 食べ + たんか: 食べた is a valid v1 past tense.
        //   - te/de check: 食べ ends in べ — NOT te/de
        //   - past check: lexicon.is_valid_verb_past("食べた") → true
        //   - → split
        let prev = synth("食べ", "食べる", "動詞", 0..2);
        let tanka = synth("たんか", "短歌", "名詞", 2..5);
        let out = apply(vec![prev, tanka], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["食べた", "ん", "か"]);
    }

    #[test]
    fn skips_when_strict_lexicon_rejects_verb_past() {
        // 猫 + たんか — 猫た isn't a real verb past, but the
        // morphology library's broad rule-matching does accept it
        // (false positive). Real consumers ground their validator
        // against a vocab catalog. Demonstrate with a strict
        // Lexicon that says "only known verbs are valid".
        struct VocabGroundedLexicon;
        impl Lexicon for VocabGroundedLexicon {
            fn is_valid_verb_past(&self, surface: &str) -> bool {
                // Toy implementation: only known catalog hits.
                ["食べた", "書いた", "読んだ", "走った"].contains(&surface)
            }
        }
        let prev = synth("猫", "猫", "名詞", 0..1);
        let tanka = synth("たんか", "短歌", "名詞", 1..4);
        let out = apply(vec![prev, tanka], &VocabGroundedLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["猫", "たんか"]);
    }

    #[test]
    fn does_not_touch_other_nouns() {
        let other = synth("猫", "猫", "名詞", 0..1);
        let out = apply(vec![other], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "猫");
    }
}
