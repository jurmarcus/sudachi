//! `CombineAuxiliaryVerbStem` — Glue an auxiliary verb stem (そう
//! etc., 助動詞語幹 sub-POS) onto the preceding Verb / Adjective /
//! Noun / "い-ending suffix".
//!
//! Example: 食べ + そう → 食べそう (Verb + Auxiliary stem).
//!
//! ## Trigger
//!
//! - Current morpheme has 助動詞語幹 sub-POS.
//! - Current surface is NOT one of {ように, よう, ようです, みたい}
//!   — those have their own dedicated handling.
//! - Previous morpheme is Verb / Adjective / Noun, OR is a Suffix
//!   whose dict form ends in い (handles adjectival suffixes like
//!   やすい/にくい/づらい — stems やす/にく/づら).
//!
//! Append onto previous in-place.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineAuxiliaryVerbStem`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_auxiliary_verb_stem";

const SKIP_SURFACES: &[&str] = &["ように", "よう", "ようです", "みたい"];

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::AUX_VERB_STEM, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    out.push(morphemes[0].clone());

    for current in morphemes.into_iter().skip(1) {
        let is_aux_stem = current
            .part_of_speech
            .iter()
            .skip(1)
            .any(|p| p == "助動詞語幹")
            && !SKIP_SURFACES.contains(&current.surface.as_str());

        let prev_compatible = match out.last() {
            Some(p) => {
                let is_adjectival_suffix =
                    matches!(p.pos, Pos::Suffix) && p.dictionary_form.ends_with('い');
                matches!(p.pos, Pos::Verb | Pos::Adjective | Pos::Noun) || is_adjectival_suffix
            }
            None => false,
        };

        if is_aux_stem && prev_compatible {
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
    fn merges_sou_after_verb() {
        let tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        let sou = synth("そう", "そう", &["助動詞", "助動詞語幹"], 2..4);
        let out = apply(vec![tabe, sou], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "食べそう");
    }

    #[test]
    fn merges_sou_after_adjective() {
        let omoshiroi = synth("面白", "面白い", &["形容詞"], 0..2);
        let sou = synth("そう", "そう", &["助動詞", "助動詞語幹"], 2..4);
        let out = apply(vec![omoshiroi, sou], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "面白そう");
    }

    #[test]
    fn merges_sou_after_iending_adjectival_suffix() {
        // やす (suffix, dict やすい) + そう → やすそう.
        let mut yasu = synth("やす", "やすい", &["接尾辞"], 0..2);
        yasu.pos = Pos::Suffix;
        let sou = synth("そう", "そう", &["助動詞", "助動詞語幹"], 2..4);
        let out = apply(vec![yasu, sou], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "やすそう");
    }

    #[test]
    fn does_not_merge_skip_surfaces() {
        // ように has its own handling — skip.
        let tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        let youni = synth("ように", "ように", &["助動詞", "助動詞語幹"], 2..4);
        let out = apply(vec![tabe, youni], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_after_particle() {
        let to = synth("と", "と", &["助詞"], 0..1);
        let sou = synth("そう", "そう", &["助動詞", "助動詞語幹"], 1..3);
        let out = apply(vec![to, sou], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }
}
