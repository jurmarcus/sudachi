//! `RepairOrphanedAuxiliary` — Recover verb stems that Sudachi
//! glued to a preceding noun, leaving the verb's auxiliary or
//! godan-ending dangling.
//!
//! Sudachi sometimes mis-tokenises a noun + verb compound by
//! attaching the verb's stem chars to the noun (eg. ふりかえ + ら +
//! れる instead of ふりかえる + られる). The auxiliary or godan-ending
//! morpheme that comes next is then "orphaned". This stage detects
//! the orphan and reconstructs the original verb by transferring
//! kanji from the noun.
//!
//! ## Algorithm (per Jiten)
//!
//! Requires a [`Lexicon`] that can confirm dictionary entries.
//! Without one, the rule is a no-op (we'd be guessing at vocab).
//!
//! For each morpheme starting at index ≥ 1:
//! 1. Identify orphan candidates:
//!    - **OrphanedAuxiliary**: Auxiliary with dict form in
//!      [`VERB_INDICATING_AUXILIARIES`] (passive/causative/potential).
//!    - **OrphanedVerbEnding**: 1-char surface in
//!      [`GODAN_VERB_ENDINGS`] (one of る/す/つ/く/ぐ/む/ぶ/ぬ/う).
//! 2. The previous morpheme must be a Noun of length ≥ 2.
//! 3. Slide a window of size 1..min(prev.len-1, 3) over the noun's
//!    tail. For each window:
//!    - Require the window to contain at least one kanji (filter
//!      out hiragana-only false matches).
//!    - For OrphanedVerbEnding: candidate dict form = window + orphan.
//!    - For OrphanedAuxiliary: try each godan ending as the
//!      candidate dict form.
//!    - If [`Lexicon::has_compound_entry`] confirms BOTH the
//!      candidate dict form AND the noun remainder are real
//!      entries → split: replace prev with the remainder, emit a
//!      Verb morpheme `(window + orphan, dict form)`.
//!
//! Ported from
//! [Sirush/Jiten RepairStages.cs `RepairOrphanedAuxiliary`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.RepairStages.cs).

use crate::data::{GODAN_VERB_ENDINGS, VERB_INDICATING_AUXILIARIES};
use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_orphaned_auxiliary";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::empty(), apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }

    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 2);
    for (i, m) in morphemes.iter().enumerate() {
        if i == 0 {
            result.push(m.clone());
            continue;
        }

        let is_orphaned_aux = matches!(m.pos, Pos::Auxiliary)
            && VERB_INDICATING_AUXILIARIES.contains(&m.dictionary_form.as_str());
        let is_orphaned_ending = !is_orphaned_aux
            && m.surface.chars().count() == 1
            && GODAN_VERB_ENDINGS.contains(&m.surface.as_str());

        if !is_orphaned_aux && !is_orphaned_ending {
            result.push(m.clone());
            continue;
        }

        let Some(prev_borrow) = result.last() else {
            result.push(m.clone());
            continue;
        };
        let prev_chars: Vec<char> = prev_borrow.surface.chars().collect();
        if !matches!(prev_borrow.pos, Pos::Noun) || prev_chars.len() < 2 {
            result.push(m.clone());
            continue;
        }
        // Clone prev so we can release the immutable borrow on result
        // before mutating it inside apply_split.
        let prev_owned = prev_borrow.clone();
        drop(prev_borrow);

        let max_window = (prev_chars.len() - 1).min(3);
        let mut repaired = false;

        for w in 1..=max_window {
            let stem: String = prev_chars[prev_chars.len() - w..].iter().collect();
            if !stem.chars().any(is_kanji) {
                continue;
            }
            let remainder: String = prev_chars[..prev_chars.len() - w].iter().collect();

            let did_split = if is_orphaned_ending {
                let dict_form = format!("{}{}", stem, m.surface);
                if lexicon.has_compound_entry(&dict_form) == Some(true)
                    && lexicon.has_compound_entry(&remainder) == Some(true)
                {
                    apply_split(&mut result, prev_owned.clone(), m, &remainder, &stem, &dict_form);
                    true
                } else {
                    false
                }
            } else {
                let mut split_done = false;
                for ending in GODAN_VERB_ENDINGS {
                    let dict_form = format!("{}{}", stem, ending);
                    if lexicon.has_compound_entry(&dict_form) == Some(true)
                        && lexicon.has_compound_entry(&remainder) == Some(true)
                    {
                        apply_split(&mut result, prev_owned.clone(), m, &remainder, &stem, &dict_form);
                        split_done = true;
                        break;
                    }
                }
                split_done
            };

            if did_split {
                repaired = true;
                break;
            }
        }

        if !repaired {
            result.push(m.clone());
        }
    }

    result
}

fn apply_split(
    result: &mut Vec<Morpheme>,
    mut noun: Morpheme,
    orphan: &Morpheme,
    remainder: &str,
    stem: &str,
    dict_form: &str,
) {
    let stem_len = stem.chars().count();
    let noun_orig_end = noun.char_range.end;

    // Truncate the noun: drop the last stem_len chars from surface
    // and from any of the form fields that end with stem.
    noun.surface = remainder.to_string();
    noun.char_range = noun.char_range.start..(noun.char_range.end - stem_len);
    if noun.dictionary_form.ends_with(stem) {
        let new_len = noun.dictionary_form.chars().count() - stem_len;
        noun.dictionary_form = noun.dictionary_form.chars().take(new_len).collect();
    }
    if noun.normalized_form.ends_with(stem) {
        let new_len = noun.normalized_form.chars().count() - stem_len;
        noun.normalized_form = noun.normalized_form.chars().take(new_len).collect();
    }
    noun.record_rule(NAME);

    // Emit the recovered verb.
    let mut verb = Morpheme::synthesize(
        format!("{}{}", stem, orphan.surface),
        "",
        dict_form,
        vec!["動詞".into()],
        (noun_orig_end - stem_len)..orphan.char_range.end,
    );
    verb.normalized_form = dict_form.to_string();
    verb.record_rule(NAME);

    // Replace the noun in result with the truncated version, then
    // append the verb.
    let last_idx = result.len() - 1;
    result[last_idx] = noun;
    result.push(verb);
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
        pos_top: &str,
        char_range: std::ops::Range<usize>,
    ) -> Morpheme {
        Morpheme::synthesize(surface, surface, dict, vec![pos_top.into()], char_range)
    }

    /// Lexicon with a fixed set of "known" dictionary entries; all
    /// other queries return Some(false).
    struct KnownEntries(HashSet<&'static str>);
    impl Lexicon for KnownEntries {
        fn has_compound_entry(&self, term: &str) -> Option<bool> {
            Some(self.0.contains(term))
        }
    }

    #[test]
    fn no_op_when_lexicon_returns_no_info() {
        // EmptyLexicon → None for all queries → rule must not fire.
        let noun = synth("ふりかえ", "ふりかえ", "名詞", 0..4);
        let mut aux = synth("られる", "られる", "助動詞", 4..7);
        aux.pos = Pos::Auxiliary;
        let out = apply(vec![noun, aux], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["ふりかえ", "られる"]);
    }

    #[test]
    fn no_op_when_morpheme_count_lt_2() {
        let noun = synth("ふりかえ", "ふりかえ", "名詞", 0..4);
        let out = apply(vec![noun], &EmptyLexicon);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn does_not_fire_for_non_orphan_morphemes() {
        let noun = synth("学校", "学校", "名詞", 0..2);
        let other = synth("です", "です", "助動詞", 2..4);
        let lex = KnownEntries(HashSet::from(["学校"]));
        let out = apply(vec![noun, other], &lex);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["学校", "です"]);
    }
}
