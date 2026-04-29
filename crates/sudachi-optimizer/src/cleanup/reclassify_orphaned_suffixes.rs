//! `ReclassifyOrphanedSuffixes` — Reclassify Suffix morphemes that
//! didn't merge with a host (after the Combine phase ran) as
//! standalone CommonNouns, except for honorifics, じまい, and
//! adjectival/na-adjectival suffix sub-types.
//!
//! ## Algorithm (per Jiten)
//!
//! For each Suffix morpheme at index ≥ 1:
//! - Skip honorific dict forms: じまい, 仕舞い, ちゃん, さん, くん,
//!   様, 殿, 氏 (always person-title suffixes; never reclassified).
//! - Skip if predecessor is Noun / Numeral / Prefix / Pronoun /
//!   Suffix (legitimately attached).
//! - Skip if first sub-POS is 形容詞的 (Adjectival) or 形状詞的
//!   (NaAdjectiveLike) — those keep their suffix POS so the parser
//!   routes them through the verb/adj branch.
//! - Otherwise → reclassify as Noun (Pos::Noun, top POS 名詞), drop
//!   sub-POS sections, clear reading.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `ReclassifyOrphanedSuffixes`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "cleanup_reclassify_orphaned_suffixes";

const HONORIFIC_DICT_FORMS: &[&str] =
    &["じまい", "仕舞い", "ちゃん", "さん", "くん", "様", "殿", "氏"];

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Cleanup, MorphemeFeatures::SUFFIX, apply)
}

pub fn apply(mut morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }

    for i in 1..morphemes.len() {
        if !matches!(morphemes[i].pos, Pos::Suffix) {
            continue;
        }
        if HONORIFIC_DICT_FORMS.contains(&morphemes[i].dictionary_form.as_str()) {
            continue;
        }
        let prev_pos = morphemes[i - 1].pos;
        if matches!(
            prev_pos,
            Pos::Noun | Pos::Pronoun | Pos::Prefix | Pos::Suffix
        ) {
            continue;
        }
        // Skip adjectival sub-types — handled by the parser's
        // adj branch, not as standalone nouns.
        if morphemes[i]
            .part_of_speech
            .iter()
            .skip(1)
            .any(|p| p == "形容詞的" || p == "形状詞的")
        {
            continue;
        }

        morphemes[i].pos = Pos::Noun;
        morphemes[i].part_of_speech = vec!["名詞".into()];
        morphemes[i].reading_form = String::new();
        morphemes[i].record_rule(NAME);
    }

    morphemes
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
    fn reclassifies_orphaned_suffix_after_verb() {
        // Verb predecessor → suffix didn't legitimately attach,
        // reclassify as Noun.
        let mut suffix = synth("もの", "もの", &["接尾辞"], 1..3);
        suffix.pos = Pos::Suffix;
        let verb = synth("行く", "行く", &["動詞"], 0..1);
        let out = apply(vec![verb, suffix], &EmptyLexicon);
        assert_eq!(out.len(), 2);
        assert!(matches!(out[1].pos, Pos::Noun));
    }

    #[test]
    fn keeps_honorific_san_classified_as_suffix() {
        let mut san = synth("さん", "さん", &["接尾辞"], 2..4);
        san.pos = Pos::Suffix;
        let tanaka = synth("田中", "田中", &["名詞"], 0..2);
        let out = apply(vec![tanaka, san], &EmptyLexicon);
        assert!(matches!(out[1].pos, Pos::Suffix));
    }

    #[test]
    fn keeps_suffix_when_prev_is_noun() {
        let mut other = synth("っぽい", "っぽい", &["接尾辞"], 2..5);
        other.pos = Pos::Suffix;
        let kodomo = synth("子供", "子供", &["名詞"], 0..2);
        let out = apply(vec![kodomo, other], &EmptyLexicon);
        assert!(matches!(out[1].pos, Pos::Suffix));
    }

    #[test]
    fn keeps_adjectival_subpos_suffix() {
        let mut suffix = synth("らしい", "らしい", &["接尾辞", "形容詞的"], 1..4);
        suffix.pos = Pos::Suffix;
        let verb = synth("行く", "行く", &["動詞"], 0..1);
        let out = apply(vec![verb, suffix], &EmptyLexicon);
        assert!(matches!(out[1].pos, Pos::Suffix));
    }
}
