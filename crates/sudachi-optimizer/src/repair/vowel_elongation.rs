//! `RepairVowelElongation` — Repair morphemes broken by elongated
//! vowels (the choonpu mark `ー`).
//!
//! Three independent passes live in this stage:
//!
//! 1. **Strip trailing `ー` from particles/conjunctions** (e.g., `けどー → けど`)
//!    when the body is all hiragana.
//! 2. **Re-classify katakana with trailing `ー` as hiragana particle**
//!    when the body is katakana and the hiragana equivalent is in
//!    [`KNOWN_PARTICLES_AND_CONJUNCTIONS`] (e.g., `ケドー → けど`).
//! 3. **Strip internal `ー` from hiragana morphemes** whose normalized
//!    form contains kanji (e.g., `なーい → ない`, normalized 無い).
//!
//! ## Status of port
//!
//! Implemented: passes 1, 2, 3 — they don't depend on the
//! deconjugator.
//!
//! Deferred (need Jiten's Deconjugator):
//! - Pattern 0–4: kanji-stem + hiragana + ー → godan volitional;
//!   noun + んー filler merge; るう ending → ru-verb + elongation;
//!   etc. These require deconjugation lookup to validate the
//!   verb-form candidate.
//!
//! Ported from
//! [Sirush/Jiten RepairStages.cs `RepairVowelElongation`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.RepairStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_vowel_elongation";

const KNOWN_PARTICLES_AND_CONJUNCTIONS: &[&str] = &[
    "けど", "けども", "けれど", "けれども", "ので", "のに", "から", "まで",
];

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::LONG_VOWEL_MARK, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    morphemes.into_iter().map(repair_one).collect()
}

fn repair_one(mut m: Morpheme) -> Morpheme {
    // Pass 1: strip trailing ー from hiragana particles / conjunctions.
    if m.surface.chars().count() >= 2
        && m.surface.ends_with('ー')
        && matches!(m.pos, Pos::Particle | Pos::Conjunction)
    {
        let body: String = m.surface.chars().take(m.surface.chars().count() - 1).collect();
        if body.chars().all(is_hiragana) {
            m.surface = body;
            m.record_rule(NAME);
            return m;
        }
    }

    // Pass 2: katakana with trailing ー that maps to a known
    // particle/conjunction → re-classify as Conjunction.
    if m.surface.chars().count() >= 2
        && m.surface.ends_with('ー')
        && matches!(m.pos, Pos::Noun)
    {
        let body: String = m.surface.chars().take(m.surface.chars().count() - 1).collect();
        if !body.is_empty() && body.chars().all(is_katakana) {
            let hiragana = katakana_to_hiragana(&body);
            if KNOWN_PARTICLES_AND_CONJUNCTIONS.contains(&hiragana.as_str()) {
                m.surface = hiragana.clone();
                m.dictionary_form = hiragana.clone();
                m.normalized_form = hiragana;
                m.reading_form = body;
                m.pos = Pos::Conjunction;
                m.part_of_speech = vec!["接続詞".into()];
                m.record_rule(NAME);
                return m;
            }
        }
    }

    // Pass 3: strip internal ー from hiragana morphemes whose
    // normalized form contains kanji. Trailing ー is a colloquial
    // form marker (preserve); only internal ー gets stripped.
    if m.surface.contains('ー') && !m.surface.ends_with('ー') {
        let chars_only_hiragana_or_bar = m
            .surface
            .chars()
            .all(|c| c == 'ー' || is_hiragana(c));
        if chars_only_hiragana_or_bar {
            let stripped: String = m.surface.chars().filter(|c| *c != 'ー').collect();
            let normalized_has_kanji = m.normalized_form.chars().any(is_kanji);
            // Either the normalized form has kanji (proves this is a
            // real word), or stripping coincidentally matches the
            // normalized form (proves ー was just an elongation
            // artefact).
            if !stripped.is_empty()
                && stripped != m.surface
                && (normalized_has_kanji || stripped == m.normalized_form)
            {
                m.surface = stripped.clone();
                m.dictionary_form = m.dictionary_form.replace('ー', "");
                m.reading_form = m.reading_form.replace('ー', "");
                m.record_rule(NAME);
                return m;
            }
        }
    }

    m
}

fn is_hiragana(c: char) -> bool {
    ('\u{3041}'..='\u{309F}').contains(&c)
}

fn is_katakana(c: char) -> bool {
    ('\u{30A0}'..='\u{30FF}').contains(&c)
}

fn is_kanji(c: char) -> bool {
    ('\u{4E00}'..='\u{9FFF}').contains(&c)
}

fn katakana_to_hiragana(s: &str) -> String {
    s.chars()
        .map(|c| {
            if ('\u{30A1}'..='\u{30F6}').contains(&c) {
                char::from_u32(c as u32 - 0x60).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    fn synth(
        surface: &str,
        dict: &str,
        normalized: &str,
        reading: &str,
        pos_top: &str,
    ) -> Morpheme {
        let mut m = Morpheme::synthesize(
            surface,
            reading,
            dict,
            vec![pos_top.into()],
            0..surface.chars().count(),
        );
        m.normalized_form = normalized.to_string();
        m
    }

    /// Direct port of Jiten PipelineStageTests.cs
    /// `RepairVowelElongation_StripsInternalChoonpu_WhenNormalizedHasKanji`.
    #[test]
    fn strips_internal_choonpu_when_normalized_has_kanji() {
        let m = synth("なーい", "なーい", "無い", "なーい", "形容詞");
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "ない");
        assert_eq!(out[0].dictionary_form, "ない");
        assert_eq!(out[0].reading_form, "ない");
    }

    /// Direct port of Jiten
    /// `RepairVowelElongation_PreservesTrailingChoonpu`.
    #[test]
    fn preserves_trailing_choonpu() {
        // Trailing ー is a colloquial marker — leave alone.
        let m = synth("すげー", "すごい", "凄い", "すげー", "形容詞");
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "すげー");
    }

    /// Direct port of Jiten
    /// `RepairVowelElongation_PreservesKanaOnlyNormalized`.
    #[test]
    fn preserves_kana_only_normalized() {
        // おーい has kana-only normalized form (おおい) — it's a
        // real interjection, not an elongation artefact.
        let mut m = synth("おーい", "おーい", "おおい", "おーい", "感動詞");
        m.pos = Pos::Interjection;
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "おーい");
    }

    /// Direct port of Jiten
    /// `RepairVowelElongation_PreservesKatakanaTokens`.
    #[test]
    fn preserves_katakana_tokens() {
        // Katakana ー is a standard long-vowel mark — never strip.
        let m = synth("スーパー", "スーパー", "スーパー", "スーパー", "名詞");
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "スーパー");
    }

    /// Direct port of Jiten
    /// `RepairVowelElongation_StripsMultipleInternalChoonpu`.
    #[test]
    fn strips_multiple_internal_choonpu() {
        let m = synth("なーーい", "なーーい", "無い", "なーーい", "形容詞");
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "ない");
    }

    #[test]
    fn strips_trailing_choonpu_from_particle() {
        let mut m = synth("けどー", "けど", "けど", "けどー", "助詞");
        m.pos = Pos::Particle;
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out[0].surface, "けど");
    }

    #[test]
    fn reclassifies_katakana_kedo_as_conjunction() {
        // ケドー (Sudachi may produce as Noun) → けど (Conjunction).
        let m = synth("ケドー", "ケドー", "ケドー", "ケドー", "名詞");
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out[0].surface, "けど");
        assert!(matches!(out[0].pos, Pos::Conjunction));
        assert_eq!(out[0].reading_form, "ケド");
    }
}
