//! `RepairCompoundNounSuffix` — rewrite the lemma of a Sudachi-merged
//! noun-suffix compound (`留学中`, `災害時`, `ダンス部`) to its head when
//! the head is a vocab entry but the compound isn't.
//!
//! ## Why
//!
//! UniDic registers many noun-suffix compounds as single tokens
//! (`留学中`, `災害時`, `ダンス部`, `宿泊代`, `塵出し`, `不燃塵`,
//! `新機種`, …). When the compound itself isn't a vocab entry,
//! downstream's dict_form-based vocab lookup misses — even though
//! the head IS a vocab entry.
//!
//! Same shape as [`repair_honorific_lemma`] but for noun-forming
//! suffixes on the trailing side. Surface stays as the merged form;
//! only `dict_form` / `normalized_form` are normalised to the head so
//! lookups land on it.
//!
//! ## Suffix list
//!
//! Curated to common semantic-modifier suffixes that don't form a
//! distinct lexeme:
//!
//! | Suffix | Meaning | Example |
//! |--------|---------|---------|
//! | `中`   | during / mid- | 留学中, 工事中, 通話中 |
//! | `時`   | at the time of | 災害時, 緊急時, 雨天時 |
//! | `代`   | charge / cost | 宿泊代, 食費代, 交通代 |
//! | `部`   | department / club | ダンス部, 経理部, 営業部 |
//! | `機`   | machine | 洗濯機 (when not its own entry) |
//! | `出し` | act of putting out | 塵出し, ゴミ出し, 引き出し |
//!
//! Single-char suffixes (中/時/代/部/機) are checked at the trailing
//! char; the multi-char `出し` is checked as a string suffix.
//!
//! ## Trigger
//!
//! - Token is a single Noun (or AdjectivalNoun).
//! - Surface ends with one of the registered suffixes.
//! - [`Lexicon::has_compound_entry`] returns NOT `Some(true)` for the
//!   compound (compound not confirmed in vocab).
//! - [`Lexicon::has_compound_entry`] returns `Some(true)` for the
//!   suffix-stripped head.
//!
//! When all three hold, rewrite `dict_form` / `normalized_form` to
//! the head. Otherwise no-op.

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_compound_noun_suffix";

/// Single-char suffixes. Chars not strings — easy boundary check.
const SINGLE_CHAR_SUFFIXES: &[char] = &['中', '時', '代', '部', '機'];

/// Multi-char suffixes. String suffix check.
const MULTI_CHAR_SUFFIXES: &[&str] = &["出し"];

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::empty(), apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    morphemes
        .into_iter()
        .map(|m| repair_one(m, lexicon))
        .collect()
}

fn repair_one(mut m: Morpheme, lexicon: &dyn Lexicon) -> Morpheme {
    if !matches!(m.pos, Pos::Noun | Pos::AdjectivalNoun) {
        return m;
    }
    // Compound already vocab? — keep the catalog entry.
    if lexicon.has_compound_entry(&m.dictionary_form) == Some(true) {
        return m;
    }

    // Try multi-char suffixes first (e.g., `出し` — would otherwise
    // collide with single-char `し` if we ever added that).
    let stripped = strip_multi_char_suffix(&m.surface)
        .or_else(|| strip_single_char_suffix(&m.surface));

    let Some(head) = stripped else {
        return m;
    };
    if head.is_empty() {
        return m;
    }
    if lexicon.has_compound_entry(&head) != Some(true) {
        return m;
    }
    m.dictionary_form = head.clone();
    m.normalized_form = head;
    m.record_rule(NAME);
    m
}

fn strip_multi_char_suffix(surface: &str) -> Option<String> {
    for suffix in MULTI_CHAR_SUFFIXES {
        if surface.len() > suffix.len() && surface.ends_with(suffix) {
            let head = &surface[..surface.len() - suffix.len()];
            return Some(head.to_string());
        }
    }
    None
}

fn strip_single_char_suffix(surface: &str) -> Option<String> {
    let last = surface.chars().last()?;
    if !SINGLE_CHAR_SUFFIXES.contains(&last) {
        return None;
    }
    let chars: Vec<char> = surface.chars().collect();
    if chars.len() < 2 {
        return None;
    }
    Some(chars[..chars.len() - 1].iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;
    use std::collections::HashSet;

    fn synth(surface: &str, dict: &str) -> Morpheme {
        let mut m = Morpheme::synthesize(
            surface,
            "",
            dict,
            vec!["名詞".into(), "普通名詞".into(), "一般".into()],
            0..surface.chars().count(),
        );
        m.normalized_form = dict.to_string();
        m.pos = Pos::Noun;
        m
    }

    struct PartialLex(HashSet<&'static str>);
    impl Lexicon for PartialLex {
        fn has_compound_entry(&self, t: &str) -> Option<bool> {
            Some(self.0.contains(t))
        }
    }

    #[test]
    fn rewrites_single_char_suffix_when_head_in_vocab() {
        let lex = PartialLex(HashSet::from(["留学", "災害", "ダンス"]));
        for (surface, expected_head) in [
            ("留学中", "留学"),
            ("災害時", "災害"),
            ("ダンス部", "ダンス"),
        ] {
            let m = synth(surface, surface);
            let out = repair_one(m, &lex);
            assert_eq!(out.dictionary_form, expected_head, "{surface}");
            assert_eq!(out.surface, surface);
            assert!(out.applied_rules.contains(&NAME), "{surface}");
        }
    }

    #[test]
    fn rewrites_multi_char_suffix() {
        let lex = PartialLex(HashSet::from(["塵"]));
        let m = synth("塵出し", "塵出し");
        let out = repair_one(m, &lex);
        assert_eq!(out.dictionary_form, "塵");
    }

    #[test]
    fn keeps_compound_lemma_when_compound_in_vocab() {
        let lex = PartialLex(HashSet::from(["留学中", "留学"]));
        let m = synth("留学中", "留学中");
        let out = repair_one(m, &lex);
        assert_eq!(out.dictionary_form, "留学中");
        assert!(!out.applied_rules.contains(&NAME));
    }

    #[test]
    fn no_op_when_head_not_in_vocab() {
        let lex = PartialLex(HashSet::from([]));
        let m = synth("留学中", "留学中");
        let out = repair_one(m, &lex);
        assert_eq!(out.dictionary_form, "留学中");
    }

    #[test]
    fn no_op_with_empty_lexicon() {
        let m = synth("留学中", "留学中");
        let out = repair_one(m, &EmptyLexicon);
        assert_eq!(out.dictionary_form, "留学中");
    }

    #[test]
    fn ignores_non_noun() {
        let lex = PartialLex(HashSet::from(["留学"]));
        let mut m = synth("留学中", "留学中");
        m.pos = Pos::Verb;
        let out = repair_one(m, &lex);
        assert_eq!(out.dictionary_form, "留学中");
    }
}
