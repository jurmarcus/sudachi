//! `FilterMisparse` — Battery of POS reclassifications for surfaces
//! that Sudachi reliably misclassifies.
//!
//! Despite the name, this stage doesn't actually drop any morphemes
//! — it only retags POS. (The "filter" name is Jiten's; we kept it
//! for source-fidelity.)
//!
//! ## Reclassifications
//!
//! - なん / フン / ふん → Prefix
//! - そう → Adverb
//! - おい → Interjection
//! - つ as Suffix → Counter (the 一つ/二つ "things" counter)
//! - Suffix with 助数詞 sub-POS → Counter
//! - 人 as Suffix preceded by Numeral → Counter (the にん counter)
//! - 家 as Suffix followed by case particle (から/を/が/に/で/へ/の/は/も)
//!   → Noun (いえ, not the け suffix)
//! - 山 as Suffix → Noun
//! - だろう / だろ as Auxiliary → "Expression" (Pos::Other) with
//!   dict form = surface
//! - だあ → だ Auxiliary, dict form です
//! - だー → keep surface, force Auxiliary, dict form です
//!
//! ## Deferred
//!
//! - PreMatchedWordId assignments (いかんせん, せん, セン, ノリ, 頚木)
//!   require a word-ID concept we don't yet model.
//!
//! Ported from
//! [Sirush/Jiten Disambiguation.cs `FilterMisparse`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.Disambiguation.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};

pub const NAME: &str = "cleanup_filter_misparse";

const KE_PARTICLES: &[&str] = &["から", "を", "が", "に", "で", "へ", "の", "は", "も"];

pub fn stage() -> Stage {
    Stage::always(NAME, Phase::Cleanup, apply)
}

pub fn apply(mut morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let n = morphemes.len();
    for i in 0..n {
        let surface = morphemes[i].surface.clone();
        let pos = morphemes[i].pos;
        let mut changed = false;

        if matches!(surface.as_str(), "なん" | "フン" | "ふん") {
            morphemes[i].pos = Pos::Prefix;
            morphemes[i].part_of_speech = vec!["接頭辞".into()];
            changed = true;
        }

        if surface == "そう" {
            morphemes[i].pos = Pos::Adverb;
            morphemes[i].part_of_speech = vec!["副詞".into()];
            changed = true;
        }

        if surface == "おい" {
            morphemes[i].pos = Pos::Interjection;
            morphemes[i].part_of_speech = vec!["感動詞".into()];
            changed = true;
        }

        if surface == "つ" && matches!(pos, Pos::Suffix) {
            morphemes[i].pos = Pos::Counter;
            changed = true;
        }

        if matches!(pos, Pos::Suffix)
            && morphemes[i]
                .part_of_speech
                .iter()
                .skip(1)
                .any(|p| p == "助数詞")
        {
            morphemes[i].pos = Pos::Counter;
            changed = true;
        }

        if surface == "人" && matches!(pos, Pos::Suffix) && i > 0 {
            let prev_is_numeral = morphemes[i - 1]
                .part_of_speech
                .iter()
                .any(|p| p == "数詞");
            if prev_is_numeral {
                morphemes[i].pos = Pos::Counter;
                changed = true;
            }
        }

        if surface == "家" && matches!(pos, Pos::Suffix) && i + 1 < n
            && matches!(morphemes[i + 1].pos, Pos::Particle)
                && KE_PARTICLES.contains(&morphemes[i + 1].surface.as_str())
            {
                morphemes[i].pos = Pos::Noun;
                morphemes[i].part_of_speech = vec!["名詞".into()];
                changed = true;
            }

        if surface == "山" && matches!(pos, Pos::Suffix) {
            morphemes[i].pos = Pos::Noun;
            morphemes[i].part_of_speech = vec!["名詞".into()];
            changed = true;
        }

        if matches!(surface.as_str(), "だろう" | "だろ") && matches!(pos, Pos::Auxiliary) {
            morphemes[i].pos = Pos::Other; // Expression
            morphemes[i].part_of_speech = vec!["連語".into()];
            morphemes[i].dictionary_form = surface.clone();
            changed = true;
        }

        if surface == "だあ" {
            morphemes[i].surface = "だ".to_string();
            morphemes[i].dictionary_form = "です".to_string();
            morphemes[i].pos = Pos::Auxiliary;
            morphemes[i].part_of_speech = vec!["助動詞".into()];
            changed = true;
        } else if surface == "だー" {
            morphemes[i].dictionary_form = "です".to_string();
            morphemes[i].pos = Pos::Auxiliary;
            morphemes[i].part_of_speech = vec!["助動詞".into()];
            changed = true;
        }

        if changed {
            morphemes[i].record_rule(NAME);
        }
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
    fn nan_reclassified_as_prefix() {
        let m = synth("なん", "なん", &["代名詞"], 0..2);
        let out = apply(vec![m], &EmptyLexicon);
        assert!(matches!(out[0].pos, Pos::Prefix));
    }

    #[test]
    fn sou_reclassified_as_adverb() {
        let m = synth("そう", "そう", &["助動詞"], 0..2);
        let out = apply(vec![m], &EmptyLexicon);
        assert!(matches!(out[0].pos, Pos::Adverb));
    }

    #[test]
    fn tsu_suffix_reclassified_as_counter() {
        let mut m = synth("つ", "つ", &["接尾辞"], 0..1);
        m.pos = Pos::Suffix;
        let out = apply(vec![m], &EmptyLexicon);
        assert!(matches!(out[0].pos, Pos::Counter));
    }

    #[test]
    fn nin_counter_after_numeral() {
        let three = synth("3", "3", &["名詞", "数詞"], 0..1);
        let mut nin = synth("人", "人", &["接尾辞"], 1..2);
        nin.pos = Pos::Suffix;
        let out = apply(vec![three, nin], &EmptyLexicon);
        assert!(matches!(out[1].pos, Pos::Counter));
    }

    #[test]
    fn ie_noun_when_followed_by_case_particle() {
        let mut ie = synth("家", "家", &["接尾辞"], 0..1);
        ie.pos = Pos::Suffix;
        let kara = synth("から", "から", &["助詞"], 1..3);
        let out = apply(vec![ie, kara], &EmptyLexicon);
        assert!(matches!(out[0].pos, Pos::Noun));
    }

    #[test]
    fn yama_suffix_reclassified_as_noun() {
        let mut yama = synth("山", "山", &["接尾辞"], 0..1);
        yama.pos = Pos::Suffix;
        let out = apply(vec![yama], &EmptyLexicon);
        assert!(matches!(out[0].pos, Pos::Noun));
    }

    #[test]
    fn darou_aux_reclassified_as_expression() {
        let mut m = synth("だろう", "だ", &["助動詞"], 0..3);
        m.pos = Pos::Auxiliary;
        let out = apply(vec![m], &EmptyLexicon);
        assert!(matches!(out[0].pos, Pos::Other));
        assert_eq!(out[0].dictionary_form, "だろう");
    }

    #[test]
    fn daa_normalises_to_da_aux() {
        let m = synth("だあ", "だあ", &["助動詞"], 0..2);
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out[0].surface, "だ");
        assert_eq!(out[0].dictionary_form, "です");
        assert!(matches!(out[0].pos, Pos::Auxiliary));
    }
}
