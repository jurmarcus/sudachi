//! [`TokenFeatures`] — bitmask of cheap pipeline-stage gating flags.
//!
//! Pipeline orchestration scans the token stream once per pass to
//! compute the set of features present, then skips any stage whose
//! `required_features` aren't represented. That avoids walking the
//! token list for stages that have nothing to do (e.g.,
//! `RepairColloquialRanNai` only fires when some token contains
//! "らん", which is rare).
//!
//! Mirrors `TokenFeatures` in
//! [Sirush/Jiten Stages/TokenStage.cs](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/TokenStage.cs).

use crate::token::{OptimizerToken, SemanticPos};
use bitflags::bitflags;

bitflags! {
    /// Per-rule gating flags. Re-scanned after every stage that
    /// modifies the token stream so newly-introduced features can
    /// activate stages that came earlier in the stage list.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TokenFeatures: u32 {
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

impl TokenFeatures {
    /// Scan a token stream and OR together every per-token feature.
    /// Cheap (one pass, constant work per token).
    pub fn scan(tokens: &[OptimizerToken]) -> Self {
        let mut f = Self::empty();
        for t in tokens {
            // Top-level POS → coarse flags.
            match t.semantic_pos {
                SemanticPos::Prefix => f |= Self::PREFIX,
                SemanticPos::Suffix => f |= Self::SUFFIX,
                SemanticPos::Auxiliary => f |= Self::AUXILIARY,
                SemanticPos::Interjection => f |= Self::INTERJECTION,
                _ => {}
            }

            // Sub-POS section flags (pos[1..] inspection).
            for section in t.pos.iter().skip(1) {
                f |= section_feature(section);
            }

            // Surface-form flags.
            if t.surface.contains('ー') {
                f |= Self::LONG_VOWEL_MARK;
            }
            if t.surface.ends_with('っ') {
                f |= Self::ENDS_WITH_TSU;
            }

            // Text + POS combinations from Jiten's scanner.
            match t.surface.as_str() {
                "たん" if matches!(t.semantic_pos, SemanticPos::Suffix) => {
                    f |= Self::TEXT_TAN_SUFFIX;
                }
                "たんか" if matches!(t.semantic_pos, SemanticPos::Noun) => {
                    f |= Self::TEXT_TANKA;
                }
                "はさ" if matches!(t.semantic_pos, SemanticPos::Noun) => {
                    f |= Self::TEXT_HASA;
                }
                "たわけ" => f |= Self::TEXT_TAWAKE,
                "たって" | "だって"
                    if t.pos
                        .iter()
                        .skip(1)
                        .any(|p| p == "接続助詞") =>
                {
                    f |= Self::TEXT_TATTE;
                }
                "だな" if matches!(t.semantic_pos, SemanticPos::Noun) => {
                    f |= Self::TEXT_TATTE;
                }
                "らん" => f |= Self::TEXT_RAN,
                _ => {}
            }
        }
        f
    }
}

/// Map a Sudachi sub-POS string (anything past `pos[0]`) to the
/// corresponding feature flag. Returns `empty()` for any section the
/// pipeline doesn't care about.
fn section_feature(section: &str) -> TokenFeatures {
    match section {
        // 助動詞語幹 — auxiliary verb stem.
        "助動詞語幹" => TokenFeatures::AUX_VERB_STEM,
        "接続助詞" => TokenFeatures::CONJ_PARTICLE,
        // 数詞 + 助数詞可能 — Sudachi UniDic numeric / counter sub-POS.
        "数詞" | "助数詞可能" => TokenFeatures::NUMERIC_AMOUNT,
        "副助詞" => TokenFeatures::ADV_PARTICLE,
        // 非自立可能 — "possible dependant", attaches to preceding word.
        "非自立可能" => TokenFeatures::DEPENDANT,
        // Sudachi's verb-like sub-POS (e.g., 動詞的). Loose mapping.
        "動詞的" => TokenFeatures::VERB_LIKE,
        // Already covered by the top-level Prefix/Suffix flags but
        // some Sudachi configs put 接尾辞 in pos[1..] too.
        "接尾辞" => TokenFeatures::SUFFIX,
        _ => TokenFeatures::empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::OptimizerToken;

    fn tok(surface: &str, pos: &[&str]) -> OptimizerToken {
        OptimizerToken::synthesize(
            surface,
            "",
            surface,
            pos.iter().map(|s| s.to_string()).collect(),
            0..surface.chars().count(),
        )
    }

    #[test]
    fn scans_prefix_and_suffix() {
        let toks = vec![tok("お", &["接頭辞"]), tok("さん", &["接尾辞"])];
        let f = TokenFeatures::scan(&toks);
        assert!(f.contains(TokenFeatures::PREFIX));
        assert!(f.contains(TokenFeatures::SUFFIX));
    }

    #[test]
    fn scans_long_vowel_and_ends_with_tsu() {
        let toks = vec![tok("ハート", &["名詞"]), tok("食べちゃっ", &["動詞"])];
        let f = TokenFeatures::scan(&toks);
        assert!(f.contains(TokenFeatures::LONG_VOWEL_MARK));
        assert!(f.contains(TokenFeatures::ENDS_WITH_TSU));
    }

    #[test]
    fn scans_text_specific_flags() {
        let toks = vec![tok("たん", &["接尾辞"]), tok("たわけ", &["名詞"])];
        let f = TokenFeatures::scan(&toks);
        assert!(f.contains(TokenFeatures::TEXT_TAN_SUFFIX));
        assert!(f.contains(TokenFeatures::TEXT_TAWAKE));
    }
}
