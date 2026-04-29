//! `RepairColloquialNegativeNee` — Recombine the colloquial negative
//! ねえ (= ない) when Sudachi has split it into ね (particle) + え
//! (interjection) after a te/de form.
//!
//! Sudachi's UniDic regularly splits 入ってねえのに into
//! 入っ + て + ね + え + のに instead of recognising ねえ as the
//! casual contraction of ない. After this repair, the ね+え pair
//! collapses into a single Auxiliary morpheme dictionary form ない.
//!
//! ## Trigger
//!
//! When current is え (Interjection), result has ≥ 2 tokens, the
//! previous one is ね (Particle), AND the one before is either:
//! - a Particle te/で, OR
//! - a Verb whose surface ends in て or で (already merged by an
//!   earlier RepairN pass).
//!
//! ## Output
//!
//! Replace the ね morpheme with a single ねえ morpheme: Auxiliary,
//! dict form ない, normalized form ない, reading ネエ. Drop the え.
//!
//! Ported from
//! [Sirush/Jiten RepairStages.cs `RepairColloquialNegativeNee`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.RepairStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_colloquial_negative_nee";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::INTERJECTION, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 3 {
        return morphemes;
    }

    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len());

    for current in morphemes {
        let trigger = current.surface == "え"
            && matches!(current.pos, Pos::Interjection)
            && result.len() >= 2
            && {
                let last = &result[result.len() - 1];
                let last_minus_one = &result[result.len() - 2];

                let prev_is_ne = last.surface == "ね" && matches!(last.pos, Pos::Particle);

                let context_is_te_or_de = match last_minus_one.pos {
                    Pos::Particle => {
                        last_minus_one.surface == "て" || last_minus_one.surface == "で"
                    }
                    Pos::Verb => {
                        last_minus_one.surface.ends_with('て')
                            || last_minus_one.surface.ends_with('で')
                    }
                    _ => false,
                };

                prev_is_ne && context_is_te_or_de
            };

        if trigger {
            // Replace ね with the single ねえ Auxiliary morpheme.
            let prev_idx = result.len() - 1;
            let mut nee = result[prev_idx].clone();
            nee.surface = "ねえ".to_string();
            nee.dictionary_form = "ない".to_string();
            nee.normalized_form = "ない".to_string();
            nee.reading_form = "ネエ".to_string();
            nee.pos = Pos::Auxiliary;
            nee.part_of_speech = vec!["助動詞".into()];
            nee.char_range = nee.char_range.start..current.char_range.end;
            nee.record_rule(NAME);
            result[prev_idx] = nee;
            // Drop the え; don't push current.
            continue;
        }

        result.push(current);
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

    /// Direct port of Jiten PipelineStageTests.cs
    /// `RepairStage_ColloquialNegativeNee_RecombinesSplitNeE_AfterTeParticle`.
    #[test]
    fn recombines_split_nee_after_te_particle() {
        let haitta = synth("入っ", "入る", "動詞", 0..2);
        let te = synth("て", "て", "助詞", 2..3);
        let ne = synth("ね", "ね", "助詞", 3..4);
        let mut e = synth("え", "え", "感動詞", 4..5);
        e.pos = Pos::Interjection;
        let out = apply(vec![haitta, te, ne, e], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["入っ", "て", "ねえ"]);
        assert!(matches!(out[2].pos, Pos::Auxiliary));
        assert_eq!(out[2].dictionary_form, "ない");
    }

    /// Direct port of Jiten PipelineStageTests.cs
    /// `RepairStage_ColloquialNegativeNee_RecombinesSplitNeE_AfterMergedDeForm`.
    #[test]
    fn recombines_split_nee_after_merged_de_form() {
        let nonde = synth("飲んで", "飲む", "動詞", 0..3);
        let ne = synth("ね", "ね", "助詞", 3..4);
        let mut e = synth("え", "え", "感動詞", 4..5);
        e.pos = Pos::Interjection;
        let out = apply(vec![nonde, ne, e], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["飲んで", "ねえ"]);
        assert!(matches!(out[1].pos, Pos::Auxiliary));
        assert_eq!(out[1].dictionary_form, "ない");
    }

    /// Direct port of Jiten PipelineStageTests.cs
    /// `RepairStage_ColloquialNegativeNee_DoesNotRecombine_WithoutTeDeContext`.
    #[test]
    fn does_not_recombine_without_te_de_context() {
        let school = synth("学校", "学校", "名詞", 0..2);
        let ne = synth("ね", "ね", "助詞", 2..3);
        let mut e = synth("え", "え", "感動詞", 3..4);
        e.pos = Pos::Interjection;
        let out = apply(vec![school, ne, e], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["学校", "ね", "え"]);
    }

    #[test]
    fn does_not_recombine_when_too_few_morphemes() {
        let mut e = synth("え", "え", "感動詞", 0..1);
        e.pos = Pos::Interjection;
        let out = apply(vec![e.clone()], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "え");
    }
}
