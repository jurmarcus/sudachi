//! `CombineSuffix` — Glue specific suffix morphemes onto the
//! preceding morpheme.
//!
//! Targets a curated set of dictionary forms that Sudachi reliably
//! emits as separate suffix tokens but that should be merged for
//! vocab resolution:
//!
//! - `っこ` (e.g., 行きっこ) — onto any morpheme.
//! - `さ` (nominaliser, e.g., 大きさ) — onto any morpheme.
//! - `がる` (auxiliary, e.g., 怖がる) — onto any morpheme.
//! - `ぶり` / `振り` — onto i-adjective whose surface lacks い but
//!   whose dict form ends in い (e.g., 久しぶり after 久し+い stem).
//! - `ら` (pluraliser) — onto a Pronoun other than 貴様
//!   (e.g., 彼ら, 我ら).
//!
//! Plus a special-case fallback: `がったり` (Sudachi misparses as an
//! Adverb after an i-adjective stem) → merge.
//!
//! Note: honorific suffixes (さん, くん, ちゃん, 様, 殿, 氏) are NOT
//! in the merge list — they have their own dedicated handling
//! elsewhere.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineSuffix`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_suffix";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::SUFFIX, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut current = morphemes[0].clone();
    for (i, next) in morphemes.iter().enumerate().skip(1) {
        let prev_at_i_minus_one = &morphemes[i - 1];
        let next_is_target_suffix = matches!(next.pos, Pos::Suffix)
            || next.part_of_speech.iter().skip(1).any(|p| p == "接尾辞");

        let dict_match = next_is_target_suffix
            && (next.dictionary_form == "っこ"
                || next.dictionary_form == "さ"
                || next.dictionary_form == "がる"
                || ((next.dictionary_form == "ぶり" || next.dictionary_form == "振り")
                    && matches!(current.pos, Pos::Adjective)
                    && !current.surface.ends_with('い')
                    && current.dictionary_form.ends_with('い'))
                || (next.dictionary_form == "ら"
                    && matches!(prev_at_i_minus_one.pos, Pos::Pronoun)
                    && prev_at_i_minus_one.surface != "貴様"));

        let gattari_special = matches!(next.pos, Pos::Adverb)
            && next.surface == "がったり"
            && matches!(current.pos, Pos::Adjective)
            && !current.surface.ends_with('い')
            && current.dictionary_form.ends_with('い');

        if dict_match || gattari_special {
            // Snapshot `current.surface` BEFORE the merge so we can
            // build the new lemma as `prev_surface + suffix_dict_form`.
            // Without this, when combine_inflections has already chained
            // additional conjugation onto the suffix (がる + れて + いる
            // → surface=がられている, dict=がる), promoting `current.surface
            // .clone()` would set the lemma to the whole conjugated
            // surface (`可愛がられている`) instead of the lexeme
            // (`可愛がる`).
            let prev_surface = current.surface.clone();
            current.surface.push_str(&next.surface);
            current.reading_form.push_str(&next.reading_form);
            current.dictionary_form_reading.push_str(&next.dictionary_form_reading);
            current.char_range = current.char_range.start..next.char_range.end;
            // For nominalising / verbalising suffixes, the merged
            // result is a NEW vocab entry, not a form of the head
            // morpheme. Update dict_form + normalized_form so vocab
            // matchers find e.g. 大きさ (id=26823, term=大きさ) rather
            // than the i-adj entry 大きい.
            //
            // Which dict_match cases produce new vocab vs verb-style
            // conjugation:
            // - さ      : 大きい → 大きさ        (new noun)
            // - がる    : 怖い → 怖がる         (new godan-r verb)
            // - ぶり/振り : 久しい → 久しぶり    (new noun/expression)
            // - ら      : 彼 → 彼ら            (new plural pronoun)
            // - っこ    : 行き → 行きっこ      (verbal idiom; Jiten
            //            keeps it under the verb)
            //
            // Promote dict/normalized for the four nominalising cases.
            // Leave っこ alone (it's an aspectual idiom, not a new
            // lexeme).
            let is_promoting_suffix = matches!(
                next.dictionary_form.as_str(),
                "さ" | "がる" | "ぶり" | "振り" | "ら"
            );
            if is_promoting_suffix {
                let new_lemma = format!("{prev_surface}{}", next.dictionary_form);
                current.dictionary_form = new_lemma.clone();
                current.normalized_form = new_lemma;
            }
            current.record_rule(NAME);
        } else {
            out.push(current);
            current = next.clone();
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
    fn merges_sa_suffix_after_iadjective_stem() {
        let oki = synth("大き", "大きい", &["形容詞"], 0..2);
        let sa = synth("さ", "さ", &["接尾辞"], 2..3);
        let out = apply(vec![oki, sa], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "大きさ");
    }

    #[test]
    fn merges_garu_suffix() {
        let kowa = synth("怖", "怖い", &["形容詞"], 0..1);
        let garu = synth("がる", "がる", &["接尾辞"], 1..3);
        let out = apply(vec![kowa, garu], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "怖がる");
    }

    #[test]
    fn merges_ra_pluraliser_after_pronoun() {
        let kare = synth("彼", "彼", &["代名詞"], 0..1);
        let ra = synth("ら", "ら", &["接尾辞"], 1..2);
        let out = apply(vec![kare, ra], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "彼ら");
    }

    #[test]
    fn does_not_merge_ra_after_kisama_pronoun() {
        let kisama = synth("貴様", "貴様", &["代名詞"], 0..2);
        let ra = synth("ら", "ら", &["接尾辞"], 2..3);
        let out = apply(vec![kisama, ra], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_unrelated_suffix() {
        // さん is honorific suffix — not in the merge list.
        let tanaka = synth("田中", "田中", &["名詞"], 0..2);
        let san = synth("さん", "さん", &["接尾辞"], 2..4);
        let out = apply(vec![tanaka, san], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }
}
