//! `RepairFusedInterjectionParticle` — Split interjection morphemes
//! that absorbed a trailing sentence-final particle.
//!
//! Sudachi sometimes emits e.g. ごめんなさいね as a single interjection
//! morpheme with no JMDict match; this stage splits it into
//! ごめんなさい (Interjection) + ね (Particle).
//!
//! ## Algorithm (per Jiten)
//!
//! For each Interjection morpheme of length ≥ 3 chars:
//! 1. If the [`Lexicon`] confirms the whole word is a known compound
//!    (`Some(true)`) → keep intact.
//! 2. Otherwise iterate [`SENTENCE_FINAL_PARTICLES`] in
//!    longest-first order. For each particle that the surface ends
//!    with:
//!    - Compute `base = surface - particle`. Require ≥ 2 chars.
//!    - If [`Lexicon`] confirms the base is NOT a known compound
//!      (`Some(false)`) → skip this particle. (Without confirmation
//!      that base is a real interjection, the split would create a
//!      fake morpheme.)
//!    - Otherwise (Some(true) or None) → split, emit
//!      `[base (Interjection), particle (Particle)]`.
//!
//! This length-first ordering matters: `ごめんなさいよね` must split
//! as ごめんなさい + よね, not ごめんなさいよ + ね.
//!
//! Ported from
//! [Sirush/Jiten RepairStages.cs `RepairFusedInterjectionParticle`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.RepairStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_fused_interjection_particle";

/// Sentence-final particles that Sudachi sometimes fuses onto
/// interjections. Length-first ordering is required by the splitting
/// algorithm — see module docs.
const SENTENCE_FINAL_PARTICLES: &[&str] = &[
    "よね", "なあ", "ねえ", "のよ", "もの", "ね", "な", "よ", "さ", "わ", "の",
];

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::INTERJECTION, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 2);

    for m in morphemes {
        let surface_len = m.surface.chars().count();
        if !matches!(m.pos, Pos::Interjection) || surface_len < 3 {
            result.push(m);
            continue;
        }

        if lexicon.has_compound_entry(&m.surface) == Some(true) {
            result.push(m);
            continue;
        }

        // Try the longest-suffix-first ordering. Particles must
        // already be sorted by length; assert here so future edits
        // can't silently break it.
        debug_assert!(
            SENTENCE_FINAL_PARTICLES
                .windows(2)
                .all(|w| w[0].chars().count() >= w[1].chars().count()),
            "SENTENCE_FINAL_PARTICLES must be sorted longest-first"
        );

        let mut split_done = false;
        for particle in SENTENCE_FINAL_PARTICLES {
            if !m.surface.ends_with(particle) {
                continue;
            }
            let particle_len = particle.chars().count();
            let base_len = surface_len - particle_len;
            if base_len < 2 {
                continue;
            }
            // Skip when lexicon explicitly confirms base ISN'T a
            // known interjection — splitting would manufacture a
            // fake morpheme.
            if lexicon.has_compound_entry(&char_prefix(&m.surface, base_len)) == Some(false) {
                continue;
            }

            let base_text = char_prefix(&m.surface, base_len);
            let begin = m.char_range.start;
            let mid = begin + base_len;
            let end = m.char_range.end;

            let mut base_tok = Morpheme::synthesize(
                base_text.clone(),
                base_text.clone(),
                base_text,
                vec!["感動詞".into()],
                begin..mid,
            );
            base_tok.record_rule(NAME);
            let mut prt_tok = Morpheme::synthesize(
                *particle,
                *particle,
                *particle,
                vec!["助詞".into()],
                mid..end,
            );
            prt_tok.record_rule(NAME);
            result.push(base_tok);
            result.push(prt_tok);
            split_done = true;
            break;
        }

        if !split_done {
            result.push(m);
        }
    }

    result
}

/// Take the first `n` chars of `s` as an owned `String`. Avoids
/// byte-index slicing that would crash on multi-byte chars.
fn char_prefix(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    fn synth_interjection(surface: &str, char_range: std::ops::Range<usize>) -> Morpheme {
        let mut m = Morpheme::synthesize(
            surface,
            surface,
            surface,
            vec!["感動詞".into()],
            char_range,
        );
        m.pos = Pos::Interjection;
        m
    }

    /// Direct port of Jiten PipelineStageTests.cs
    /// `RepairStage_FusedInterjectionYone_PreservesMultiCharParticle`.
    /// Without length-first ordering, よね would split as
    /// ごめんなさいよ + ね (wrong).
    #[test]
    fn yone_split_preserves_multi_char_particle() {
        let m = synth_interjection("ごめんなさいよね", 0..8);
        let out = apply(vec![m], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["ごめんなさい", "よね"]);
    }

    /// Direct port of Jiten PipelineStageTests.cs
    /// `RepairStage_FusedInterjectionNoYo_PreservesMultiCharParticle`.
    #[test]
    fn noyo_split_preserves_multi_char_particle() {
        let m = synth_interjection("ごめんなさいのよ", 0..8);
        let out = apply(vec![m], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["ごめんなさい", "のよ"]);
    }

    /// Direct port of Jiten PipelineStageTests.cs
    /// `RepairStage_FusedInterjectionNee_PreservesMultiCharParticle`.
    #[test]
    fn nee_split_preserves_multi_char_particle() {
        let m = synth_interjection("ごめんなさいねえ", 0..8);
        let out = apply(vec![m], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["ごめんなさい", "ねえ"]);
    }

    #[test]
    fn keeps_interjection_when_lexicon_confirms_compound() {
        struct KnownInterjection(&'static str);
        impl Lexicon for KnownInterjection {
            fn has_compound_entry(&self, term: &str) -> Option<bool> {
                if term == self.0 {
                    Some(true)
                } else {
                    Some(false)
                }
            }
        }
        let m = synth_interjection("ごめんなさいね", 0..7);
        let out = apply(vec![m], &KnownInterjection("ごめんなさいね"));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "ごめんなさいね");
    }

    #[test]
    fn does_not_split_short_interjections() {
        // Length < 3 → leave alone.
        let m = synth_interjection("ね", 0..1);
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "ね");
    }

    #[test]
    fn does_not_touch_non_interjection_morphemes() {
        let m = Morpheme::synthesize("学校", "がっこう", "学校", vec!["名詞".into()], 0..2);
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert!(matches!(out[0].pos, Pos::Noun));
    }
}
