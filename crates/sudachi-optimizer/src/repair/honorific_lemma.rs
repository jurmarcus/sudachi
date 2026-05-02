//! `RepairHonorificLemma` — rewrite the lemma of a Sudachi-merged
//! honorific compound (`ご利用`, `ご都合`) to its head when the head
//! is a vocab entry but the compound isn't.
//!
//! ## Why
//!
//! Sudachi UniDic is inconsistent about honorific compounds:
//!
//! - **Split** (`ご家族`, `お茶`): emits `[ご, 家族]` / `[お, 茶]`,
//!   which [`combine_prefixes`](crate::combine::prefixes) merges into
//!   `ご家族` / `お茶` *if the compound is in the lexicon*.
//! - **Merged** (`ご利用`, `ご都合`, `お祭り`): emits a single Noun
//!   token with `dict=ご利用` / `dict=ご都合` / `dict=お祭り`. No rule
//!   rewrites the lemma.
//!
//! For consumers (jisho-core's vocab matcher) doing `dict_form`-based
//! lookup, the merged-noun case fails when the compound isn't a vocab
//! entry — even though the head IS a vocab entry. This rule closes
//! that gap by stripping the honorific prefix from `dict_form` /
//! `normalized_form` for those cases.
//!
//! Surface stays as the merged form (the prefix is real input the user
//! typed); only the *root* is normalised so vocab lookups land on the
//! head's entry.
//!
//! ## Trigger
//!
//! - Token is a single Noun.
//! - Surface starts with `お` or `ご`.
//! - [`Lexicon::has_compound_entry`] returns `Some(false)` / `None` for
//!   the compound surface (compound NOT in vocab).
//! - [`Lexicon::has_compound_entry`] returns `Some(true)` for the
//!   prefix-stripped head.
//!
//! When `has_compound_entry(compound)` returns `Some(true)`, the
//! compound is a real vocab entry (`お茶`, `お母さん`) — keep the lemma
//! pointing at the compound; downstream gets the catalog entry directly.

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_honorific_lemma";

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
    if !matches!(m.pos, Pos::Noun) {
        return m;
    }
    let mut chars = m.surface.chars();
    let first = match chars.next() {
        Some(c) => c,
        None => return m,
    };
    if first != 'お' && first != 'ご' {
        return m;
    }
    let head: String = chars.collect();
    if head.is_empty() {
        return m;
    }
    // If the compound itself is a vocab entry, the catalog entry is
    // more useful than the head — keep the lemma as-is.
    if lexicon.has_compound_entry(&m.dictionary_form) == Some(true) {
        return m;
    }
    // Only rewrite when the head is confirmed in vocab. Some(false) /
    // None on the head means we have no signal that stripping helps,
    // so we don't speculate.
    if lexicon.has_compound_entry(&head) != Some(true) {
        return m;
    }
    m.dictionary_form = head.clone();
    m.normalized_form = head;
    m.record_rule(NAME);
    m
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
    fn rewrites_dict_form_to_head_when_only_head_is_vocab() {
        let lex = PartialLex(HashSet::from(["利用", "都合", "祭り"]));
        let m = synth("ご利用", "ご利用");
        let out = repair_one(m, &lex);
        assert_eq!(out.surface, "ご利用");
        assert_eq!(out.dictionary_form, "利用");
        assert_eq!(out.normalized_form, "利用");
        assert!(out.applied_rules.contains(&NAME));
    }

    #[test]
    fn keeps_compound_lemma_when_compound_is_vocab() {
        let lex = PartialLex(HashSet::from(["お茶", "茶"]));
        let m = synth("お茶", "お茶");
        let out = repair_one(m, &lex);
        // Compound is in vocab — keep the catalog entry.
        assert_eq!(out.dictionary_form, "お茶");
        assert!(!out.applied_rules.contains(&NAME));
    }

    #[test]
    fn no_op_when_head_not_in_vocab() {
        let lex = PartialLex(HashSet::from([]));
        let m = synth("お珍味", "お珍味");
        let out = repair_one(m, &lex);
        assert_eq!(out.dictionary_form, "お珍味");
    }

    #[test]
    fn no_op_when_no_lexicon_signal() {
        let m = synth("ご利用", "ご利用");
        let out = repair_one(m, &EmptyLexicon);
        assert_eq!(out.dictionary_form, "ご利用");
    }

    #[test]
    fn ignores_non_noun() {
        let lex = PartialLex(HashSet::from(["利用"]));
        let mut m = synth("ご利用", "ご利用");
        m.pos = Pos::Verb;
        let out = repair_one(m, &lex);
        assert_eq!(out.dictionary_form, "ご利用");
    }

    #[test]
    fn ignores_surface_not_starting_with_o_or_go() {
        let lex = PartialLex(HashSet::from(["利用"]));
        let m = synth("利用", "利用");
        let out = repair_one(m, &lex);
        assert_eq!(out.dictionary_form, "利用");
    }
}
