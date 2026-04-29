//! `CombineAdverbialParticle` — Glue だり/たり (adverbial particles)
//! onto the preceding verb morpheme.
//!
//! Example: 食べ + たり → 食べたり (Verb).
//!
//! ## Trigger
//!
//! - Current morpheme is a Verb.
//! - Next morpheme has the 副助詞 sub-POS, dict form is `だり` or `たり`.
//!
//! Append next.surface and next.reading_form onto current; extend
//! the char_range. POS / dict form / normalized form remain those
//! of the verb (the particle is conjugation tail, not head).
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineAdverbialParticle`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_adverbial_particle";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::ADV_PARTICLE, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut current = morphemes[0].clone();
    for next in morphemes.into_iter().skip(1) {
        let next_is_dari_tari = next.part_of_speech.iter().skip(1).any(|p| p == "副助詞")
            && (next.dictionary_form == "だり" || next.dictionary_form == "たり");
        if next_is_dari_tari && matches!(current.pos, Pos::Verb) {
            current.surface.push_str(&next.surface);
            current.reading_form.push_str(&next.reading_form);
            current.char_range = current.char_range.start..next.char_range.end;
            current.record_rule(NAME);
        } else {
            out.push(current);
            current = next;
        }
    }
    out.push(current);
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
    fn merges_tari_after_verb() {
        let tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        let tari = synth("たり", "たり", &["助詞", "副助詞"], 2..4);
        let out = apply(vec![tabe, tari], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "食べたり");
        assert!(out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn merges_dari_after_verb() {
        let yon = synth("読ん", "読む", &["動詞"], 0..2);
        let dari = synth("だり", "だり", &["助詞", "副助詞"], 2..4);
        let out = apply(vec![yon, dari], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "読んだり");
    }

    #[test]
    fn does_not_merge_after_noun() {
        let school = synth("学校", "学校", &["名詞"], 0..2);
        let tari = synth("たり", "たり", &["助詞", "副助詞"], 2..4);
        let out = apply(vec![school, tari], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_other_adverbial_particles() {
        let tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        let other = synth("ばかり", "ばかり", &["助詞", "副助詞"], 2..5);
        let out = apply(vec![tabe, other], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }
}
