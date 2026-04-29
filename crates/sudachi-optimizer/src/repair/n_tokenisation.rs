//! `RepairNTokenisation` — Repair ん particle / copula tokenisation
//! issues. Two phases live in this stage.
//!
//! ## Phase 1 — Split compound tokens that Sudachi over-grouped
//!
//! - Tokens starting with ん whose remainder is in
//!   [`N_COMPOUND_SUFFIXES`](crate::data::N_COMPOUND_SUFFIXES) →
//!   split into ん + remainder.
//! - Tokens starting with だ when the previous result token ends in
//!   ん, AND the remainder is in
//!   [`DA_COMPOUND_SUFFIXES`](crate::data::DA_COMPOUND_SUFFIXES) →
//!   split into だ + remainder.
//! - そうだ misclassified as Adverb → split into そう (Auxiliary,
//!   AuxVerbStem sub-POS) + だ.
//!
//! ## Phase 2 — Recombine verb stems with ん
//!
//! Combine pairs like 読ん + だ → 読んだ (verb past tense). Jiten
//! validates the merge with its Deconjugator; without that we use a
//! structural heuristic:
//!
//! - current ends in ん (length > 1, not standalone ん)
//! - current is not a na-adjective (form-checking: dict form
//!   doesn't end in ん itself)
//! - current is not a Suffix
//! - next is だ or で
//!
//! When all hold, replace `(current, next)` with one Verb morpheme
//! `current + next`. This is more permissive than Jiten's
//! deconjugator-validated merge but covers the common cases (eg.
//! 読ん+だ, 飲ん+で, 死ん+だ).
//!
//! ## Status of port
//!
//! Implemented: Phase 1 fully + Phase 2's simple-combine path.
//!
//! Deferred (need Jiten's Deconjugator):
//! - Standalone ん look-back combine (multi-token verb stems)
//! - ませ + ん → ません merge with verb-stem chaining
//! - HasCompoundLookup-driven noun + ん + だ → verb past tense
//!
//! Ported from
//! [Sirush/Jiten RepairStages.cs `RepairNTokenisation`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.RepairStages.cs).

use crate::data::{DA_COMPOUND_SUFFIXES, N_COMPOUND_SUFFIXES};
use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_n_tokenisation";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::empty(), apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    // Phase 1 can split a single morpheme (んだ → ん + だ; そうだ → そう + だ).
    // Phase 2 needs at least 2 morphemes to look ahead but a no-op
    // pass on a 1-morpheme list is fine.
    let split = phase_one_split(morphemes);
    phase_two_recombine(split)
}

fn phase_one_split(morphemes: Vec<Morpheme>) -> Vec<Morpheme> {
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 4);

    for m in morphemes {
        let chars: Vec<char> = m.surface.chars().collect();

        // Split: token starts with ん, remainder is an N-compound suffix.
        if chars.len() > 1 && chars[0] == 'ん' {
            let remainder: String = chars[1..].iter().collect();
            let remainder_matches = N_COMPOUND_SUFFIXES.contains(&remainder.as_str())
                || N_COMPOUND_SUFFIXES.iter().any(|s| remainder.starts_with(s));
            if remainder_matches {
                let begin = m.char_range.start;
                let dict_for_n = if matches!(m.pos, Pos::Interjection) { "の" } else { "" };
                let mut n_token = make_n_token(dict_for_n, begin..begin + 1);
                n_token.record_rule(NAME);
                out.push(n_token);

                let mut remainder_tok = m.clone();
                remainder_tok.surface = remainder.clone();
                remainder_tok.dictionary_form = remainder.clone();
                remainder_tok.normalized_form = remainder.clone();
                remainder_tok.reading_form = remainder;
                remainder_tok.char_range = (begin + 1)..m.char_range.end;
                remainder_tok.record_rule(NAME);
                out.push(remainder_tok);
                continue;
            }
        }

        // Split: token starts with だ when prev (in `out`) ends with ん,
        // remainder in DA_COMPOUND_SUFFIXES.
        if chars.len() > 1
            && chars[0] == 'だ'
            && !out.is_empty()
            && (out.last().unwrap().surface == "ん" || out.last().unwrap().surface.ends_with('ん'))
        {
            let remainder: String = chars[1..].iter().collect();
            if DA_COMPOUND_SUFFIXES.contains(&remainder.as_str()) {
                let begin = m.char_range.start;
                let mut da_token = make_da_token(begin..begin + 1);
                da_token.record_rule(NAME);
                out.push(da_token);

                let mut remainder_tok = m.clone();
                remainder_tok.surface = remainder.clone();
                remainder_tok.dictionary_form = remainder.clone();
                remainder_tok.normalized_form = remainder.clone();
                remainder_tok.reading_form = remainder;
                remainder_tok.char_range = (begin + 1)..m.char_range.end;
                remainder_tok.record_rule(NAME);
                out.push(remainder_tok);
                continue;
            }
        }

        // Split そうだ adverb → そう (Auxiliary, AuxVerbStem) + だ.
        if m.surface == "そうだ" && matches!(m.pos, Pos::Adverb) {
            let begin = m.char_range.start;
            let end = m.char_range.end;
            let mut sou = Morpheme::synthesize(
                "そう",
                "そう",
                "そう",
                vec!["助動詞".into(), "助動詞語幹".into()],
                begin..begin + 2,
            );
            sou.record_rule(NAME);
            out.push(sou);
            let mut da = make_da_token((begin + 2)..end);
            da.record_rule(NAME);
            out.push(da);
            continue;
        }

        out.push(m);
    }

    out
}

fn phase_two_recombine(split: Vec<Morpheme>) -> Vec<Morpheme> {
    let mut result: Vec<Morpheme> = Vec::with_capacity(split.len());
    let mut i = 0;
    while i < split.len() {
        let current = &split[i];
        let next = split.get(i + 1);

        // Heuristic: current ends in ん (longer than 1 char), is not a
        // na-adjective (dict form doesn't end in ん) and not a Suffix,
        // next is だ or で → combine into a Verb past/te-form.
        if current.surface.chars().count() > 1
            && current.surface.ends_with('ん')
            && current.surface != "ん"
            && !matches!(current.pos, Pos::Suffix | Pos::AdjectivalNoun)
            && !current.dictionary_form.ends_with('ん')
            && next.is_some_and(|n| n.surface == "だ" || n.surface == "で")
        {
            let next = next.unwrap();
            let combined_surface = format!("{}{}", current.surface, next.surface);
            let combined_reading = format!("{}{}", current.reading_form, next.reading_form);
            let mut merged = current.clone();
            merged.surface = combined_surface.clone();
            merged.normalized_form = combined_surface;
            merged.reading_form = combined_reading;
            merged.pos = Pos::Verb;
            merged.part_of_speech = vec!["動詞".into()];
            merged.char_range = current.char_range.start..next.char_range.end;
            merged.record_rule(NAME);
            result.push(merged);
            i += 2;
            continue;
        }

        result.push(current.clone());
        i += 1;
    }
    result
}

fn make_n_token(dict_form: &str, char_range: std::ops::Range<usize>) -> Morpheme {
    let mut m = Morpheme::synthesize(
        "ん",
        "ん",
        dict_form,
        vec!["助動詞".into()],
        char_range,
    );
    m.normalized_form = "ん".to_string();
    m
}

fn make_da_token(char_range: std::ops::Range<usize>) -> Morpheme {
    Morpheme::synthesize("だ", "だ", "だ", vec!["助動詞".into()], char_range)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    fn synth(
        surface: &str,
        dict: &str,
        pos_top: &str,
        char_range: std::ops::Range<usize>,
    ) -> Morpheme {
        Morpheme::synthesize(surface, surface, dict, vec![pos_top.into()], char_range)
    }

    /// Direct port of Jiten PipelineStageTests.cs
    /// `RepairStage_ShouldRecoverNdaPastTense_FromCorpusCase`.
    #[test]
    fn recovers_nda_past_tense_from_corpus_case() {
        // Input: 読ん(verb, 読む, よん) + だけど(conjunction, だけど, だけど)
        // Expected: 読んだ(verb) + けど
        let yon = synth("読ん", "読む", "動詞", 0..2);
        let dakedo = synth("だけど", "だけど", "接続詞", 2..5);
        let out = apply(vec![yon, dakedo], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["読んだ", "けど"]);
        assert!(matches!(out[0].pos, Pos::Verb));
    }

    #[test]
    fn splits_nda_when_starts_with_n() {
        // んだ as one token → ん + だ.
        let nda = synth("んだ", "んだ", "感動詞", 0..2);
        let out = apply(vec![nda], &EmptyLexicon);
        // Phase 1 splits, Phase 2 won't recombine (current is ん
        // standalone after split, < 2 chars).
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["ん", "だ"]);
    }

    #[test]
    fn splits_souda_adverb_into_sou_and_da() {
        let souda = synth("そうだ", "そうだ", "副詞", 0..3);
        let out = apply(vec![souda], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["そう", "だ"]);
        assert!(matches!(out[0].pos, Pos::Auxiliary));
    }

    #[test]
    fn does_not_combine_suffix_san_with_da() {
        // さん + だ → must NOT combine (さん is honorific suffix).
        let mut san = synth("さん", "さん", "接尾辞", 0..2);
        san.pos = Pos::Suffix;
        let da = synth("だ", "だ", "助動詞", 2..3);
        let out = apply(vec![san, da], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["さん", "だ"]);
    }

    #[test]
    fn does_not_combine_when_dict_form_ends_in_n() {
        // たくさん is a noun whose dict form ends in ん itself —
        // adding +で must NOT make it a verb.
        let takusan = synth("たくさん", "たくさん", "名詞", 0..4);
        let de = synth("で", "で", "助詞", 4..5);
        let out = apply(vec![takusan, de], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["たくさん", "で"]);
    }
}
