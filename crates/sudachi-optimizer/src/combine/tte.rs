//! `CombineTte` — Glue together morphemes Sudachi split as `Xっ` + `て…`
//! into a single morpheme `Xって…`.
//!
//! Sudachi sometimes emits the geminate-て pattern (走った, 思って) as
//! two morphemes when it should be one — e.g., 走っ (Verb) + た
//! (Auxiliary) instead of 走った. This stage merges any pair where
//! the previous morpheme ends in っ and the next starts with て.
//!
//! ## Behaviour
//!
//! - Append next.surface onto current.surface.
//! - Append next.reading_form onto current.reading_form.
//! - Extend current.char_range to next.char_range.end.
//! - POS, dict form, and normalized form remain those of the
//!   first morpheme — the merged token's "head" identity is the
//!   verb stem, the て-cluster is conjugation tail.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineTte`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::Morpheme;
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_tte";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::ENDS_WITH_TSU, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }

    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut current = morphemes[0].clone();

    for next in morphemes.into_iter().skip(1) {
        if current.surface.ends_with('っ') && next.surface.starts_with('て') {
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
        reading: &str,
        pos_top: &str,
        char_range: std::ops::Range<usize>,
    ) -> Morpheme {
        Morpheme::synthesize(
            surface,
            reading,
            surface,
            vec![pos_top.into()],
            char_range,
        )
    }

    #[test]
    fn merges_t_te_pair() {
        let hashitta = synth("走っ", "ハシッ", "動詞", 0..2);
        let te = synth("て", "テ", "助詞", 2..3);
        let out = apply(vec![hashitta, te], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "走って");
        assert_eq!(out[0].reading_form, "ハシッテ");
        assert_eq!(out[0].char_range, 0..3);
        assert!(out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn merges_chained_t_te_pairs() {
        // 走っ + ても (te-particle compound).
        let hashitta = synth("走っ", "ハシッ", "動詞", 0..2);
        let temo = synth("ても", "テモ", "助詞", 2..4);
        let out = apply(vec![hashitta, temo], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "走っても");
    }

    #[test]
    fn does_not_merge_when_prev_does_not_end_in_tsu() {
        let nonde = synth("飲ん", "ノン", "動詞", 0..2);
        let de = synth("で", "デ", "助詞", 2..3);
        let out = apply(vec![nonde, de], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_when_next_does_not_start_with_te() {
        let hashitta = synth("走っ", "ハシッ", "動詞", 0..2);
        let other = synth("る", "ル", "動詞", 2..3);
        let out = apply(vec![hashitta, other], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn passes_through_single_morpheme_input() {
        let m = synth("猫", "ネコ", "名詞", 0..1);
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "猫");
    }
}
