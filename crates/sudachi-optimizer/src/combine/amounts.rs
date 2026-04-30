//! `CombineAmounts` — Glue a numeric / amount morpheme onto a known
//! counter sequel.
//!
//! Sudachi sometimes splits 数詞 + 助数詞 pairs that are stored in
//! the dictionary as a single entry (e.g., 3人 / 一日 / 五千円).
//! Jiten ships a 800+ entry hand-curated `AmountCombinations` table
//! of `(left, right)` tuples that should always merge.
//!
//! ## Status of port
//!
//! Implemented: rule logic + a small representative subset of
//! `AmountCombinations`.
//!
//! Deferred: full table import (~830 entries). Adding more entries
//! is mechanical — paste tuples into [`AMOUNT_COMBINATIONS`].
//!
//! Trigger:
//! - Current morpheme has 数詞 OR 助数詞 sub-POS.
//! - `(current.surface, next.surface)` is in [`AMOUNT_COMBINATIONS`].
//!
//! Output: merge surface, drop next; force POS to Noun.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineAmounts`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs)
//! + [Data/AmountCombinations.cs](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Data/AmountCombinations.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_amounts";

/// Representative subset of Jiten's full table. Each tuple is a
/// `(left_surface, right_surface)` pair that should always merge.
/// The full ~830-entry table awaits a follow-up commit.
const AMOUNT_COMBINATIONS: &[(&str, &str)] = &[
    ("１", "つ"),
    ("２", "つ"),
    ("３", "つ"),
    ("４", "つ"),
    ("５", "つ"),
    ("一", "つ"),
    ("二", "つ"),
    ("三", "つ"),
    ("一", "日"),
    ("二", "日"),
    ("一", "人"),
    ("二", "人"),
    ("三", "人"),
    ("１", "人"),
    ("２", "人"),
    ("３", "人"),
    ("十", "分"),
    ("半", "分"),
    ("一", "回"),
    ("二", "回"),
    ("三", "回"),
    ("一", "階"),
    ("二", "階"),
    ("三", "階"),
    ("一", "個"),
    ("二", "個"),
    ("三", "個"),
];

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::NUMERIC_AMOUNT, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut current = morphemes[0].clone();
    for next in morphemes.into_iter().skip(1) {
        let current_is_amount = current
            .part_of_speech
            .iter()
            .skip(1)
            .any(|p| p == "数詞" || p == "助数詞" || p == "助数詞可能");
        let pair_known = AMOUNT_COMBINATIONS
            .iter()
            .any(|(l, r)| *l == current.surface && *r == next.surface);
        if current_is_amount && pair_known {
            current.surface.push_str(&next.surface);
            current.reading_form.push_str(&next.reading_form);
            current.dictionary_form_reading.push_str(&next.dictionary_form_reading);
            current.char_range = current.char_range.start..next.char_range.end;
            // The merged amount is a counter expression (三人, 一日,
            // 五千円) — its own vocab entry, not an inflected form of
            // the head numeral. Update dict_form + normalized_form
            // so vocab lookup finds the compound counter rather than
            // the bare numeral. Same rationale as the particle fix
            // for には, とは, etc.
            current.dictionary_form = current.surface.clone();
            current.normalized_form = current.surface.clone();
            current.pos = Pos::Noun;
            current.part_of_speech = vec!["名詞".into()];
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
    fn merges_known_amount_counter_pair() {
        let three = synth("３", "３", &["名詞", "数詞"], 0..1);
        let people = synth("人", "人", &["接尾辞"], 1..2);
        let out = apply(vec![three, people], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "３人");
        assert!(matches!(out[0].pos, Pos::Noun));
    }

    #[test]
    fn does_not_merge_when_pair_not_in_table() {
        let three = synth("３", "３", &["名詞", "数詞"], 0..1);
        let unknown = synth("ZZZ", "ZZZ", &["接尾辞"], 1..4);
        let out = apply(vec![three, unknown], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_when_left_not_amount() {
        let school = synth("学校", "学校", &["名詞"], 0..2);
        let people = synth("人", "人", &["接尾辞"], 2..3);
        let out = apply(vec![school, people], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }
}
