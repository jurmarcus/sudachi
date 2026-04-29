//! `SplitCompoundAuxiliaryVerbs` — Split compound verbs that Sudachi
//! emits as a single token when their right side is an auxiliary verb.
//!
//! Sudachi's UniDic frequently treats compounds like し終わる, 食べ続ける,
//! 書き始める as single morphemes. JMDict doesn't carry these compounds
//! as entries, but it does carry their components — so for vocab
//! resolution we need to split them apart.
//!
//! ## Algorithm
//!
//! For each verb morpheme with a dictionary form ≥ 3 chars, iterate
//! over [`AUXILIARY_VERBS`] looking for a suffix match. If matched:
//!
//! 1. Skip the split if the [`Lexicon`] reports the compound exists
//!    as a dictionary entry (e.g., 滲み出す is a real word; splitting
//!    would lose the entry-level reading disambiguation).
//! 2. Compute the main-verb prefix length from the dictionary form.
//! 3. Verify the surface form is long enough AND that the auxiliary
//!    surface starts with the registered stem ([`auxiliary_verb_stem`]).
//! 4. Emit two morphemes: main verb (Verb POS) + auxiliary
//!    (Verb POS, dependant flag via `非自立可能` sub-POS).
//!
//! Ported from
//! [Sirush/Jiten SplitStages.cs `SplitCompoundAuxiliaryVerbs`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.SplitStages.cs).

use crate::data::{AUXILIARY_VERBS, auxiliary_verb_stem};
use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "split_compound_auxiliary_verbs";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Split, MorphemeFeatures::empty(), apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 4);

    for m in morphemes {
        // Only process verbs with a dictionary form long enough to
        // hold a stem + auxiliary.
        if !matches!(m.pos, Pos::Verb)
            || m.dictionary_form.is_empty()
            || m.dictionary_form.chars().count() < 3
        {
            result.push(m);
            continue;
        }

        // Find a registered auxiliary verb that the dict form ends with.
        let matched_aux = AUXILIARY_VERBS.iter().find(|aux| {
            let aux_chars = aux.chars().count();
            let dict_chars = m.dictionary_form.chars().count();
            dict_chars > aux_chars && m.dictionary_form.ends_with(*aux)
        });

        let Some(matched_aux) = matched_aux else {
            result.push(m);
            continue;
        };

        // Skip the split if the compound is a real dictionary entry —
        // the Lexicon callback knows what's in the vocab catalog.
        if lexicon.has_compound_entry(&m.dictionary_form) {
            result.push(m);
            continue;
        }

        // Compute main-verb prefix length from the dictionary form.
        let aux_chars = matched_aux.chars().count();
        let dict_char_count = m.dictionary_form.chars().count();
        let main_verb_dict_len = dict_char_count - aux_chars;

        // Surface must be long enough to carry the same prefix length.
        let surface_chars: Vec<char> = m.surface.chars().collect();
        if surface_chars.len() <= main_verb_dict_len {
            result.push(m);
            continue;
        }

        // Look up the registered stem for this auxiliary; verify the
        // auxiliary surface portion starts with the stem.
        let Some(aux_stem) = auxiliary_verb_stem(matched_aux) else {
            result.push(m);
            continue;
        };
        let main_verb_surface: String = surface_chars[..main_verb_dict_len].iter().collect();
        let aux_verb_surface: String = surface_chars[main_verb_dict_len..].iter().collect();
        if !aux_verb_surface.starts_with(aux_stem) {
            result.push(m);
            continue;
        }

        // Compute the dictionary form's main-verb prefix.
        let dict_chars: Vec<char> = m.dictionary_form.chars().collect();
        let main_verb_dict: String = dict_chars[..main_verb_dict_len].iter().collect();

        // Build the two output morphemes. Char ranges split at
        // main_verb_dict_len from the original morpheme's start.
        let begin = m.char_range.start;
        let mid = begin + main_verb_dict_len;
        let end = m.char_range.end;

        let mut main = Morpheme::synthesize(
            main_verb_surface,
            "", // reading filled by downstream Sudachi re-lookup if needed
            main_verb_dict,
            vec!["動詞".into()],
            begin..mid,
        );
        main.record_rule(NAME);

        let mut aux = Morpheme::synthesize(
            aux_verb_surface,
            "",
            (*matched_aux).to_string(),
            // 非自立可能 marks the morpheme as a dependant — matches
            // PartOfSpeechSection.PossibleDependant in Jiten.
            vec!["動詞".into(), "非自立可能".into()],
            mid..end,
        );
        aux.record_rule(NAME);

        result.push(main);
        result.push(aux);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    /// Build a verb morpheme with explicit surface, dict form, and
    /// char range.
    fn verb(surface: &str, dict: &str) -> Morpheme {
        let n = surface.chars().count();
        Morpheme::synthesize(surface, "", dict, vec!["動詞".into()], 0..n)
    }

    /// Lexicon implementation that says yes for a fixed compound —
    /// used to test the "skip split when compound is a real entry"
    /// branch.
    struct OneCompound(&'static str);
    impl Lexicon for OneCompound {
        fn has_compound_entry(&self, term: &str) -> bool {
            term == self.0
        }
    }

    #[test]
    fn splits_shi_owatta_into_shi_and_owatta() {
        // し終わっ (dict: し終わる) → し + 終わっ (dict 終わる)
        let m = verb("し終わっ", "し終わる");
        let out = apply(vec![m], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["し", "終わっ"]);
        let dict_forms: Vec<&str> = out.iter().map(|m| m.dictionary_form.as_str()).collect();
        assert_eq!(dict_forms, vec!["し", "終わる"]);
        assert!(matches!(out[0].pos, Pos::Verb));
        assert!(matches!(out[1].pos, Pos::Verb));
        // Dependent marker on the auxiliary half.
        assert!(out[1].part_of_speech.iter().any(|p| p == "非自立可能"));
        assert!(out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn splits_tabe_tsuzukeru_into_tabe_and_tsuzukeru() {
        // 食べ続ける (dict: 食べ続ける) → 食べ + 続ける
        let m = verb("食べ続ける", "食べ続ける");
        let out = apply(vec![m], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["食べ", "続ける"]);
    }

    #[test]
    fn keeps_compound_when_lexicon_recognises_it() {
        // 滲み出す is in JMDict — must not split (would lose the
        // entry-level disambiguation).
        let m = verb("滲み出し", "滲み出す");
        let out = apply(vec![m], &OneCompound("滲み出す"));

        assert_eq!(out.len(), 1, "lexicon-known compound must stay intact");
        assert_eq!(out[0].surface, "滲み出し");
    }

    #[test]
    fn does_not_split_when_aux_stem_does_not_match_surface() {
        // Suppose dict form ends with 出す but surface lacks the
        // expected 出 stem prefix on the auxiliary side. Construct a
        // pathological morpheme where surface == dict (so split
        // surface "_ABCXXX" wouldn't start with "出") to verify the
        // stem guard fires.
        //
        // Here: dict 行き出す, surface 行き出す → prefix 行き (2 chars),
        // suffix 出す → starts with 出, so this WOULD split. To
        // exercise the negative path we pretend the surface dropped
        // the 出 character: surface 行きす, dict 行き出す → suffix す
        // doesn't start with 出. Note this is contrived (Sudachi
        // wouldn't really emit such a mismatch), but the guard
        // exists for safety.
        let m = verb("行きす", "行き出す");
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1, "stem-mismatch must not split");
    }

    #[test]
    fn does_not_split_short_dictionary_forms() {
        // Dict form < 3 chars → never splits regardless of suffix.
        let m = verb("ある", "ある");
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "ある");
    }

    #[test]
    fn does_not_touch_non_verb_morphemes() {
        let noun = Morpheme::synthesize(
            "学生",
            "がくせい",
            "学生",
            vec!["名詞".into()],
            0..2,
        );
        let out = apply(vec![noun], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert!(matches!(out[0].pos, Pos::Noun));
    }

    #[test]
    fn does_not_split_when_no_registered_aux_matches() {
        // 学ぶ ends with ぶ — not in the auxiliary verb list.
        let m = verb("学んだ", "学ぶ");
        let out = apply(vec![m], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "学んだ");
    }
}
