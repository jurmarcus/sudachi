//! `CombinePrefixes` — Glue Prefix morphemes onto the following
//! content word when the [`Lexicon`] confirms the combined surface
//! is a real dictionary entry.
//!
//! Examples (with a populated lexicon):
//! - お + 茶 → お茶 (kana prefix → noun)
//! - 不 + 自由 → 不自由 (kanji prefix → na-adjective)
//! - 再 + 開 → 再開 (kanji prefix → noun)
//!
//! ## Trigger
//!
//! - Current is Prefix.
//! - Next is content word: Noun / AdjectivalNoun / Adverb, OR (when
//!   prefix is kanji) Verb / Adjective.
//! - [`Lexicon::has_compound_entry`] returns `Some(true)` for the
//!   combined surface.
//!
//! ## Status of port
//!
//! The basic "merge if lexicon confirms" branch is implemented.
//! With [`EmptyLexicon`] the rule is a no-op (no info → don't
//! manufacture compounds we can't verify).
//!
//! Deferred:
//! - PrefixCombineExclusions table (a hard list of compounds we
//!   shouldn't merge even when the lexicon confirms).
//! - Reading-based fallback (Sudachi's reading differs from surface
//!   for colloquial/contracted forms).
//! - Partial combination (prefix + part of next when next isn't a
//!   real word — needs HasCompoundLookup partial-match logic).
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombinePrefixes`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_prefixes";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::PREFIX, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut i = 0;
    while i < morphemes.len() {
        let cur = &morphemes[i];
        if matches!(cur.pos, Pos::Prefix) && i + 1 < morphemes.len() {
            let next = &morphemes[i + 1];
            let is_kanji_prefix = cur.surface.chars().any(is_kanji);
            let is_content_word = matches!(
                next.pos,
                Pos::Noun | Pos::AdjectivalNoun | Pos::Adverb
            ) || (is_kanji_prefix && matches!(next.pos, Pos::Verb | Pos::Adjective));
            if is_content_word {
                let combined = format!("{}{}", cur.surface, next.surface);
                if lexicon.has_compound_entry(&combined) == Some(true) {
                    let mut merged = next.clone();
                    merged.surface = combined.clone();
                    merged.dictionary_form = combined.clone();
                    merged.normalized_form = combined;
                    merged.char_range = cur.char_range.start..next.char_range.end;
                    // Kanji prefix + verb/adj normally produces a noun.
                    if is_kanji_prefix && matches!(next.pos, Pos::Verb | Pos::Adjective) {
                        merged.pos = Pos::Noun;
                        merged.part_of_speech = vec!["名詞".into()];
                    }
                    merged.record_rule(NAME);
                    out.push(merged);
                    i += 2;
                    continue;
                }
            }
        }
        out.push(cur.clone());
        i += 1;
    }
    out
}

fn is_kanji(c: char) -> bool {
    ('\u{4E00}'..='\u{9FAF}').contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;
    use std::collections::HashSet;

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

    struct KnownCompounds(HashSet<&'static str>);
    impl Lexicon for KnownCompounds {
        fn has_compound_entry(&self, term: &str) -> Option<bool> {
            Some(self.0.contains(term))
        }
    }

    #[test]
    fn merges_kana_prefix_o_with_noun_when_lexicon_confirms() {
        let mut o = synth("お", "お", &["接頭辞"], 0..1);
        o.pos = Pos::Prefix;
        let cha = synth("茶", "茶", &["名詞"], 1..2);
        let lex = KnownCompounds(HashSet::from(["お茶"]));
        let out = apply(vec![o, cha], &lex);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "お茶");
    }

    #[test]
    fn no_op_when_lexicon_returns_no_info() {
        let mut o = synth("お", "お", &["接頭辞"], 0..1);
        o.pos = Pos::Prefix;
        let cha = synth("茶", "茶", &["名詞"], 1..2);
        let out = apply(vec![o, cha], &EmptyLexicon);
        assert_eq!(out.len(), 2, "EmptyLexicon → no merge");
    }

    #[test]
    fn does_not_merge_kana_prefix_with_verb() {
        // Kana prefix お only combines with content nouns/na-adj/adv.
        let mut o = synth("お", "お", &["接頭辞"], 0..1);
        o.pos = Pos::Prefix;
        let taberu = synth("食べる", "食べる", &["動詞"], 1..4);
        let lex = KnownCompounds(HashSet::from(["お食べる"]));
        let out = apply(vec![o, taberu], &lex);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn merges_kanji_prefix_with_verb() {
        let mut sai = synth("再", "再", &["接頭辞"], 0..1);
        sai.pos = Pos::Prefix;
        let kai = synth("開", "開く", &["動詞"], 1..2);
        let lex = KnownCompounds(HashSet::from(["再開"]));
        let out = apply(vec![sai, kai], &lex);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "再開");
        assert!(matches!(out[0].pos, Pos::Noun));
    }
}
