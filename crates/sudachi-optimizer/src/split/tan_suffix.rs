//! `SplitTanSuffix` — Split たん(suffix) + だ/です into
//! [prev + た] + ん + だ/です for the explanatory のだ pattern.
//!
//! Sudachi sometimes tokenises たんだ as たん(suffix) + だ(auxiliary),
//! e.g., イッ(noun) + たん(suffix) + だ(aux) instead of the correct
//! イッた + んだ. After this split, `repair::process_special_cases`
//! will combine the new ん + だ into んだ (explanatory のだ).
//!
//! ## Algorithm (per Jiten)
//!
//! For a triple (prev, たん[suffix], だ/です[aux]) where `result` is
//! non-empty:
//! - Split if `prev` ends in て or で (clear te-form past-tense
//!   candidate), OR
//! - Split if Jiten's deconjugator says `prev + た` is a verb past
//!   tense.
//!
//! When splitting:
//! 1. Replace the previous morpheme with `prev + た`.
//! 2. Emit ん (Particle, 準体助詞 sub-POS) at the split point.
//! 3. Emit だ/です untouched (the next iteration handles it).
//!
//! ## Deconjugator port deferred
//!
//! The "verb past tense" branch needs a port of Jiten's Deconjugator
//! (Jiten.Parser/Deconjugator.cs ~16KB + a deconjugation table). Until
//! that lands, this stage only handles the te/de case — which catches
//! the common cases (eg. イッた + んだ, 食べた + んだ) where Sudachi
//! produced イッ ending in っ-then-た as a separate morpheme that
//! ends in て / で precedence form. The exotic deconjugator-only
//! cases pass through unchanged for now.
//!
//! TODO(deconjugator): port `Jiten.Parser.Deconjugator` and re-enable
//! the verb-past-tense branch.
//!
//! Ported from
//! [Sirush/Jiten SplitStages.cs `SplitTanSuffix`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.SplitStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "split_tan_suffix";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Split, MorphemeFeatures::TEXT_TAN_SUFFIX, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }

    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 2);

    let mut i = 0;
    while i < morphemes.len() {
        let m = &morphemes[i];

        // Pattern guard: m must be たん(suffix), there must be a
        // next morpheme and it must be a だ/です auxiliary, AND
        // result must already have a prev morpheme.
        let is_tan_suffix = m.surface == "たん" && matches!(m.pos, Pos::Suffix);
        let next_is_da_or_desu = i + 1 < morphemes.len() && {
            let next = &morphemes[i + 1];
            matches!(next.pos, Pos::Auxiliary)
                && (next.dictionary_form == "だ" || next.dictionary_form == "です")
        };
        if !is_tan_suffix || !next_is_da_or_desu || result.is_empty() {
            result.push(m.clone());
            i += 1;
            continue;
        }

        // Decide whether to split. Two paths:
        //   1. prev ends in て/で (clear past-tense candidate; cheap
        //      check).
        //   2. lexicon's morphology oracle says prev+た is a valid
        //      verb past tense (catches cases like 食べ + たんだ
        //      where prev doesn't end in te/de).
        let prev = result.last().unwrap();
        let prev_last_char = prev.surface.chars().last();
        let prev_ends_in_te_de = matches!(prev_last_char, Some('て') | Some('で'));
        let prev_plus_ta_is_verb_past = if !prev_ends_in_te_de {
            let candidate = format!("{}た", prev.surface);
            lexicon.is_valid_verb_past(&candidate)
        } else {
            false
        };
        let should_split = prev_ends_in_te_de || prev_plus_ta_is_verb_past;

        if !should_split {
            result.push(m.clone());
            i += 1;
            continue;
        }

        // 1. Replace prev with prev + た.
        let prev_idx = result.len() - 1;
        let prev = &mut result[prev_idx];
        prev.surface.push('た');
        prev.char_range = prev.char_range.start..(m.char_range.start + 1);
        prev.record_rule(NAME);

        // 2. Emit ん at the split point.
        let split_start = m.char_range.start + 1;
        let split_end = m.char_range.end;
        let mut nn = Morpheme::synthesize(
            "ん",
            "ん",
            "の",
            vec!["助詞".into(), "準体助詞".into()],
            split_start..split_end,
        );
        nn.normalized_form = "ん".to_string();
        nn.record_rule(NAME);
        result.push(nn);

        // The だ/です auxiliary at i+1 passes through unchanged on
        // the next iteration.
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    fn synth(surface: &str, dict: &str, pos_top: &str, char_range: std::ops::Range<usize>) -> Morpheme {
        Morpheme::synthesize(surface, surface, dict, vec![pos_top.into()], char_range)
    }

    fn synth_sub(
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
    fn splits_after_te_form_predecessor() {
        // 行って + たん + だ → 行ってた + ん + だ.
        let prev = synth("行って", "行く", "動詞", 0..3);
        let tan = synth("たん", "たん", "接尾辞", 3..5);
        let da = synth("だ", "だ", "助動詞", 5..6);
        let out = apply(vec![prev, tan, da], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["行ってた", "ん", "だ"]);
        assert!(out[0].applied_rules.contains(&NAME));
        assert!(matches!(out[1].pos, Pos::Particle));
    }

    #[test]
    fn splits_after_de_form_predecessor() {
        // 飛んで + たん + です → 飛んでた + ん + です.
        let prev = synth("飛んで", "飛ぶ", "動詞", 0..3);
        let tan = synth("たん", "たん", "接尾辞", 3..5);
        let desu = synth("です", "です", "助動詞", 5..7);
        let out = apply(vec![prev, tan, desu], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["飛んでた", "ん", "です"]);
    }

    #[test]
    fn splits_after_non_te_de_predecessor_when_morphology_validates() {
        // 食べ + たん + だ → 食べた + ん + だ. Prev doesn't end in
        // て/で, but the morphology oracle confirms 食べた is a
        // valid v1 past tense → split.
        let prev = synth("食べ", "食べる", "動詞", 0..2);
        let tan = synth_sub("たん", "たん", &["接尾辞"], 2..4);
        let da = synth("だ", "だ", "助動詞", 4..5);
        let out = apply(vec![prev, tan, da], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["食べた", "ん", "だ"]);
    }

    #[test]
    fn no_split_when_strict_lexicon_rejects_verb_past() {
        // With a strict (vocab-grounded) lexicon, 猫 + たん + だ
        // doesn't split because 猫た isn't a known verb past.
        struct StrictLexicon;
        impl Lexicon for StrictLexicon {
            fn is_valid_verb_past(&self, surface: &str) -> bool {
                ["食べた", "書いた", "読んだ"].contains(&surface)
            }
        }
        let prev = synth("猫", "猫", "名詞", 0..1);
        let tan = synth_sub("たん", "たん", &["接尾辞"], 1..3);
        let da = synth("だ", "だ", "助動詞", 3..4);
        let out = apply(vec![prev, tan, da], &StrictLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["猫", "たん", "だ"]);
    }

    #[test]
    fn no_split_when_next_is_not_da_or_desu() {
        // たん followed by something that isn't だ/です (e.g., ね)
        // is left alone.
        let prev = synth("行って", "行く", "動詞", 0..3);
        let tan = synth_sub("たん", "たん", &["接尾辞"], 3..5);
        let ne = synth("ね", "ね", "助詞", 5..6);
        let out = apply(vec![prev, tan, ne], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["行って", "たん", "ね"]);
    }

    #[test]
    fn no_split_when_no_predecessor() {
        // たん at index 0 → result is empty when we look at it →
        // pattern guard fails, no split.
        let tan = synth_sub("たん", "たん", &["接尾辞"], 0..2);
        let da = synth("だ", "だ", "助動詞", 2..3);
        let out = apply(vec![tan, da], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["たん", "だ"]);
    }
}
