//! `CombineVerbDependant` — Run four sub-passes to merge various
//! verb-dependant patterns:
//!
//! 1. **Dependants** (`combine_verb_dependants`): merge a Dependant
//!    sub-POS morpheme onto a preceding Verb (excluding おる, and
//!    skipping when next.surface == current.surface).
//! 2. **PossibleDependants**: merge curated dependent verbs (得る,
//!    しまう, こなす, いく, 貰う, いる, ない, だす, etc.) onto
//!    preceding Verb when conditions hold.
//! 3. **Suru chains** (`combine_verb_dependants_suru`): merge
//!    suru-noun + する/す verb forms (e.g., 勉強+する → 勉強する).
//! 4. **Te-iru chains** (`combine_verb_dependants_teiru`): merge
//!    Verb + て + te-form auxiliary chains (deferred to a
//!    follow-up — needs careful 3-token handling).
//!
//! ## Status of port
//!
//! Implemented: passes 1 (Dependants), 2 (PossibleDependants without
//! lexicon-driven 付く branch), 3 (Suru-noun chains).
//!
//! Deferred: pass 4 (Te-iru chains, needs auxiliary registry to know
//! which te-form auxiliaries qualify), and the lexicon-driven 付く
//! branch in pass 2.
//!
//! Ported from
//! [Sirush/Jiten Helpers/MorphologicalAnalyser.CombineHelpers.cs](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Helpers/MorphologicalAnalyser.CombineHelpers.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};

pub const NAME: &str = "combine_verb_dependant";

const POSSIBLE_DEPENDANT_DICT_FORMS: &[&str] = &[
    "得る", "しまう", "こなす", "いく", "貰う", "いる", "ない", "だす",
];

pub fn stage() -> Stage {
    Stage::always(NAME, Phase::Combine, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let after_dependants = combine_dependants(morphemes);
    let after_possible = combine_possible_dependants(after_dependants);
    combine_suru_chains(after_possible)
}

fn combine_dependants(morphemes: Vec<Morpheme>) -> Vec<Morpheme> {
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut current = morphemes[0].clone();
    for next in morphemes.into_iter().skip(1) {
        let next_is_dependant = next
            .part_of_speech
            .iter()
            .skip(1)
            .any(|p| p == "非自立可能");
        if next_is_dependant
            && matches!(current.pos, Pos::Verb)
            && next.dictionary_form != "おる"
            && next.surface != current.surface
        {
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

fn combine_possible_dependants(morphemes: Vec<Morpheme>) -> Vec<Morpheme> {
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut current = morphemes[0].clone();
    for next in morphemes.into_iter().skip(1) {
        let is_classical_wa_row_te = next.dictionary_form.ends_with('う')
            && next.surface.ends_with("いて");
        let in_curated_set = POSSIBLE_DEPENDANT_DICT_FORMS.contains(&next.dictionary_form.as_str());
        let suru_after_past = next.dictionary_form == "する"
            && (current.surface.ends_with('た') || current.surface.ends_with('だ'));
        let next_is_dependant = next
            .part_of_speech
            .iter()
            .skip(1)
            .any(|p| p == "非自立可能");

        if next_is_dependant
            && matches!(current.pos, Pos::Verb)
            && !current.surface.ends_with("たり")
            && next.surface != current.surface
            && !is_classical_wa_row_te
            && (in_curated_set || suru_after_past)
        {
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

fn combine_suru_chains(morphemes: Vec<Morpheme>) -> Vec<Morpheme> {
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut i = 0;
    while i < morphemes.len() {
        let cur = &morphemes[i];
        if i + 1 < morphemes.len() {
            let next = &morphemes[i + 1];
            let is_modern_suru = next.dictionary_form == "する"
                && next.surface != "する"
                && next.surface != "しない"
                && !next.surface.ends_with("すぎ")
                && !next.surface.ends_with("過ぎ");
            let is_literary_suru = next.dictionary_form == "す" && next.normalized_form == "為る";
            let is_suru_noun = cur
                .part_of_speech
                .iter()
                .skip(1)
                .any(|p| p == "サ変可能");
            if is_suru_noun && (is_modern_suru || is_literary_suru) {
                let mut merged = cur.clone();
                merged.surface.push_str(&next.surface);
                merged.reading_form.push_str(&next.reading_form);
                merged.char_range = cur.char_range.start..next.char_range.end;
                merged.record_rule(NAME);
                out.push(merged);
                i += 2;
                continue;
            }
        }
        out.push(cur.clone());
        i += 1;
    }
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
    fn merges_te_iru_simple_dependant_pattern() {
        // Verb + いる (Dependant sub-POS) → merge.
        let nete = synth("寝て", "寝る", &["動詞"], 0..2);
        let iru = synth("いる", "いる", &["動詞", "非自立可能"], 2..4);
        let out = apply(vec![nete, iru], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "寝ている");
    }

    #[test]
    fn merges_suru_after_suru_noun() {
        let benkyou = synth("勉強", "勉強", &["名詞", "サ変可能"], 0..2);
        let suru = synth("し", "する", &["動詞"], 2..3);
        let out = apply(vec![benkyou, suru], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "勉強し");
    }

    #[test]
    fn does_not_merge_oru_dependant() {
        let nete = synth("寝て", "寝る", &["動詞"], 0..2);
        let oru = synth("おる", "おる", &["動詞", "非自立可能"], 2..4);
        let out = apply(vec![nete, oru], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_when_next_is_same_as_current() {
        // Guard against repeated identical surfaces.
        let mut a = synth("いる", "いる", &["動詞"], 0..2);
        a.pos = Pos::Verb;
        let mut b = synth("いる", "いる", &["動詞", "非自立可能"], 2..4);
        b.pos = Pos::Verb;
        let out = apply(vec![a, b], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }
}
