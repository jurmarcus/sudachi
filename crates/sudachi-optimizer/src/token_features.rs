//! [`MorphemeFeatures`] — bitmask of cheap pipeline-stage gating flags.
//!
//! Pipeline orchestration scans the morpheme stream once per pass to
//! compute the set of features present, then skips any stage whose
//! `required_features` aren't represented. That avoids walking the
//! morpheme list for stages that have nothing to do (e.g.,
//! `repair::colloquial_ran_nai` only fires when some morpheme contains
//! "らん", which is rare).
//!
//! Mirrors `TokenFeatures` in
//! [Sirush/Jiten Stages/TokenStage.cs](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/TokenStage.cs).

use crate::token::{Morpheme, Pos};
use bitflags::bitflags;

bitflags! {
    /// Per-stage gating flags. Re-scanned after every stage that
    /// modifies the morpheme stream so newly-introduced features can
    /// activate stages that came earlier in the stage list.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MorphemeFeatures: u32 {
        const PREFIX            = 1 << 0;
        const SUFFIX            = 1 << 1;
        const AUXILIARY         = 1 << 2;
        const INTERJECTION      = 1 << 3;

        const AUX_VERB_STEM     = 1 << 4;
        const CONJ_PARTICLE     = 1 << 5;
        const NUMERIC_AMOUNT    = 1 << 6;
        const ADV_PARTICLE      = 1 << 7;
        const DEPENDANT         = 1 << 8;
        const VERB_LIKE         = 1 << 9;

        const LONG_VOWEL_MARK   = 1 << 10;
        const ENDS_WITH_TSU     = 1 << 11;
        const TEXT_TAN_SUFFIX   = 1 << 12;
        const TEXT_TANKA        = 1 << 13;
        const TEXT_HASA         = 1 << 14;
        const TEXT_TAWAKE       = 1 << 15;
        const TEXT_TATTE        = 1 << 16;
        const TEXT_RAN          = 1 << 17;
    }
}

impl MorphemeFeatures {
    /// Scan a morpheme stream and OR together every per-morpheme
    /// feature. Cheap (one pass, constant work per morpheme).
    pub fn scan(morphemes: &[Morpheme]) -> Self {
        let mut f = Self::empty();
        for m in morphemes {
            // Top-level POS → coarse flags.
            match m.pos {
                Pos::Prefix => f |= Self::PREFIX,
                Pos::Suffix => f |= Self::SUFFIX,
                Pos::Auxiliary => f |= Self::AUXILIARY,
                Pos::Interjection => f |= Self::INTERJECTION,
                _ => {}
            }

            // Sub-POS section flags (part_of_speech[1..] inspection).
            for section in m.part_of_speech.iter().skip(1) {
                f |= section_feature(section);
            }

            // Surface-form flags.
            if m.surface.contains('ー') {
                f |= Self::LONG_VOWEL_MARK;
            }
            if m.surface.ends_with('っ') {
                f |= Self::ENDS_WITH_TSU;
            }

            // Text + POS combinations from Jiten's scanner.
            match m.surface.as_str() {
                "たん" if matches!(m.pos, Pos::Suffix) => {
                    f |= Self::TEXT_TAN_SUFFIX;
                }
                "たんか" if matches!(m.pos, Pos::Noun) => {
                    f |= Self::TEXT_TANKA;
                }
                "はさ" if matches!(m.pos, Pos::Noun) => {
                    f |= Self::TEXT_HASA;
                }
                "たわけ" => f |= Self::TEXT_TAWAKE,
                "たって" | "だって"
                    if m.part_of_speech.iter().skip(1).any(|p| p == "接続助詞") =>
                {
                    f |= Self::TEXT_TATTE;
                }
                "だな" if matches!(m.pos, Pos::Noun) => {
                    f |= Self::TEXT_TATTE;
                }
                "らん" => f |= Self::TEXT_RAN,
                _ => {}
            }
        }
        f
    }
}

/// Map a Sudachi sub-POS string (anything past `part_of_speech[0]`)
/// to the corresponding feature flag. Returns `empty()` for any
/// section the pipeline doesn't care about.
fn section_feature(section: &str) -> MorphemeFeatures {
    match section {
        "助動詞語幹" => MorphemeFeatures::AUX_VERB_STEM,
        "接続助詞" => MorphemeFeatures::CONJ_PARTICLE,
        "数詞" | "助数詞可能" => MorphemeFeatures::NUMERIC_AMOUNT,
        "副助詞" => MorphemeFeatures::ADV_PARTICLE,
        // 非自立可能 — "possible dependant", attaches to preceding word.
        "非自立可能" => MorphemeFeatures::DEPENDANT,
        // Sudachi's verb-like sub-POS. Loose mapping.
        "動詞的" => MorphemeFeatures::VERB_LIKE,
        // Already covered by the top-level Prefix/Suffix flags but
        // some Sudachi configs put 接尾辞 in part_of_speech[1..] too.
        "接尾辞" => MorphemeFeatures::SUFFIX,
        _ => MorphemeFeatures::empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(surface: &str, part_of_speech: &[&str]) -> Morpheme {
        Morpheme::synthesize(
            surface,
            "",
            surface,
            part_of_speech.iter().map(|s| s.to_string()).collect(),
            0..surface.chars().count(),
        )
    }

    #[test]
    fn scans_prefix_and_suffix() {
        let ms = vec![m("お", &["接頭辞"]), m("さん", &["接尾辞"])];
        let f = MorphemeFeatures::scan(&ms);
        assert!(f.contains(MorphemeFeatures::PREFIX));
        assert!(f.contains(MorphemeFeatures::SUFFIX));
    }

    #[test]
    fn scans_long_vowel_and_ends_with_tsu() {
        let ms = vec![m("ハート", &["名詞"]), m("食べちゃっ", &["動詞"])];
        let f = MorphemeFeatures::scan(&ms);
        assert!(f.contains(MorphemeFeatures::LONG_VOWEL_MARK));
        assert!(f.contains(MorphemeFeatures::ENDS_WITH_TSU));
    }

    #[test]
    fn scans_text_specific_flags() {
        let ms = vec![m("たん", &["接尾辞"]), m("たわけ", &["名詞"])];
        let f = MorphemeFeatures::scan(&ms);
        assert!(f.contains(MorphemeFeatures::TEXT_TAN_SUFFIX));
        assert!(f.contains(MorphemeFeatures::TEXT_TAWAKE));
    }
}
