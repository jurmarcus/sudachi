//! `CombineAuxiliary` — Merge Auxiliary morphemes (た, ます, ている,
//! etc.) onto preceding Verb / Adjective / NaAdjective / Auxiliary
//! morphemes, with a long blacklist of dictionary forms / surfaces
//! that must NOT merge.
//!
//! Two paths run per pair:
//!
//! 1. **Copula `である` merge**: when prev is `で` (with dict form
//!    だ — the copula te-form) and current is ある, merge into である.
//! 2. **Generic auxiliary merge**: when current is Auxiliary and
//!    passes a 20-clause negative filter list, append onto previous
//!    Verb / Adjective / NaAdjective / Auxiliary morpheme. Special
//!    case: bare `だ` only merges when prev ends in ん AND the
//!    morphology oracle confirms `prev + だ` is a valid verb past.
//!
//! Plus a fallback: Expression + た merges when prev surface ends
//! in て or で (catches Sudachi misclassifications of verb te-forms
//! as Expression that the past tense should still glue onto).
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineAuxiliary`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "combine_auxiliary";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Combine, MorphemeFeatures::empty(), apply)
}

pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    out.push(morphemes[0].clone());

    for current in morphemes.into_iter().skip(1) {
        let prev_idx = out.len() - 1;
        let mut combined = false;

        if !matches!(current.pos, Pos::Auxiliary) {
            // Copula である merge: prev is で (cop te-form, dict
            // form だ) + current is ある → である.
            let prev = &out[prev_idx];
            if prev.surface == "で"
                && prev.dictionary_form == "だ"
                && (current.dictionary_form == "ある" || current.dictionary_form == "有る")
            {
                let prev_owned = prev.clone();
                let mut merged = prev_owned;
                merged.surface.push_str(&current.surface);
                merged.reading_form.push_str(&current.reading_form);
                merged.dictionary_form_reading.push_str(&current.dictionary_form_reading);
                merged.char_range = merged.char_range.start..current.char_range.end;
                merged.pos = current.pos;
                merged.part_of_speech = current.part_of_speech.clone();
                merged.dictionary_form = "である".to_string();
                // Self-consistency: when we promote dict_form to a new
                // compound identity (である is its own copula entry,
                // distinct from the で + ある components), normalized
                // form must follow.
                merged.normalized_form = "である".to_string();
                merged.record_rule(NAME);
                out[prev_idx] = merged;
                continue;
            }
            out.push(current);
            continue;
        }

        // Current is Auxiliary. Try the generic merge. Clone the
        // prev so we can release its immutable borrow before the
        // mutable index update inside the if-block.
        let prev_owned = out[prev_idx].clone();
        let prev_compatible = matches!(
            prev_owned.pos,
            Pos::Verb | Pos::Adjective | Pos::AdjectivalNoun | Pos::Auxiliary
        );

        if prev_compatible && passes_blacklist(&prev_owned, &current, lexicon) {
            let mut merged = prev_owned.clone();
            merged.surface.push_str(&current.surface);
            merged.reading_form.push_str(&current.reading_form);
            merged.char_range = merged.char_range.start..current.char_range.end;
            merged.record_rule(NAME);
            out[prev_idx] = merged;
            combined = true;
        }

        // Fallback: Expression + た merges when prev surface ends
        // in て/で (catches the te-form misclassification case).
        if !combined
            && matches!(prev_owned.pos, Pos::Other)
            && current.dictionary_form == "た"
        {
            let prev_last_char = prev_owned.surface.chars().last();
            if matches!(prev_last_char, Some('て') | Some('で')) {
                let mut merged = prev_owned.clone();
                merged.surface.push_str(&current.surface);
                merged.reading_form.push_str(&current.reading_form);
                merged.char_range = merged.char_range.start..current.char_range.end;
                merged.record_rule(NAME);
                out[prev_idx] = merged;
                combined = true;
            }
        }

        if !combined {
            out.push(current);
        }
    }
    out
}

/// The 20-clause negative filter list from Jiten's CombineAuxiliary.
/// Returns true when the merge should proceed, false to skip.
fn passes_blacklist(prev: &Morpheme, current: &Morpheme, lexicon: &dyn Lexicon) -> bool {
    // Surface-based exclusions.
    if matches!(
        current.surface.as_str(),
        "な" | "に" | "なら" | "なる" | "だろう" | "で" | "や" | "やろ" | "やしない"
            | "し" | "なのだ" | "だろ" | "ハズ"
    ) {
        return false;
    }
    if current.surface.starts_with("なん") {
        return false;
    }
    // Dict-form-based exclusions.
    if matches!(
        current.dictionary_form.as_str(),
        "らしい" | "べし" | "む" | "ごとし" | "如し" | "ようだ" | "やがる" | "たり" | "筈"
    ) {
        return false;
    }
    // です special: only allowed when prev is Verb AND current is でし/でした
    // (the politeness-attaching form). Block all other です attachments.
    if current.dictionary_form == "です" {
        let prev_is_verb = matches!(prev.pos, Pos::Verb);
        let current_is_deshi_form = matches!(current.surface.as_str(), "でし" | "でした");
        if !(prev_is_verb && current_is_deshi_form) {
            return false;
        }
    }
    // じゃ (with dict form だ) — block (it's a topic-condition
    // contraction, not a genuine だ attachment).
    if current.surface == "じゃ" && current.dictionary_form == "だ" {
        return false;
    }
    // Bare だ: only merge when prev ends in ん AND the morphology
    // oracle confirms `prev + だ` is a valid verb past tense.
    // Without this guard we'd misclassify negative-ん + だ
    // (slurred neg) as past tense.
    if current.surface == "だ" {
        let prev_ends_in_n = prev.surface.ends_with('ん');
        if !prev_ends_in_n {
            return false;
        }
        let candidate = format!("{}{}", prev.surface, current.surface);
        if !lexicon.is_valid_verb_past(&candidate) {
            return false;
        }
    }
    true
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
    fn merges_ta_after_verb() {
        // 食べ + た → 食べた (typical past tense merge).
        let mut tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        tabe.pos = Pos::Verb;
        let mut ta = synth("た", "た", &["助動詞"], 2..3);
        ta.pos = Pos::Auxiliary;
        let out = apply(vec![tabe, ta], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "食べた");
        assert!(out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn merges_masu_after_verb() {
        let mut tabe = synth("食べ", "食べる", &["動詞"], 0..2);
        tabe.pos = Pos::Verb;
        let mut masu = synth("ます", "ます", &["助動詞"], 2..4);
        masu.pos = Pos::Auxiliary;
        let out = apply(vec![tabe, masu], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "食べます");
    }

    #[test]
    fn merges_dearu_copula_chain() {
        // で (Particle, dict form だ) + ある (Verb) → である.
        let mut de = synth("で", "だ", &["助詞"], 0..1);
        de.pos = Pos::Particle;
        let mut aru = synth("ある", "ある", &["動詞"], 1..3);
        aru.pos = Pos::Verb;
        let out = apply(vec![de, aru], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "である");
        assert_eq!(out[0].dictionary_form, "である");
        assert!(matches!(out[0].pos, Pos::Verb));
    }

    #[test]
    fn does_not_merge_blacklisted_surface_na() {
        // な must not merge with preceding adjective.
        let mut adj = synth("好き", "好き", &["形状詞"], 0..2);
        adj.pos = Pos::AdjectivalNoun;
        let mut na = synth("な", "だ", &["助動詞"], 2..3);
        na.pos = Pos::Auxiliary;
        let out = apply(vec![adj, na], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_rashii_dict_form() {
        let mut tabe = synth("食べる", "食べる", &["動詞"], 0..3);
        tabe.pos = Pos::Verb;
        let mut rashii = synth("らしい", "らしい", &["助動詞"], 3..6);
        rashii.pos = Pos::Auxiliary;
        let out = apply(vec![tabe, rashii], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn does_not_merge_darou_surface() {
        let mut tabe = synth("食べる", "食べる", &["動詞"], 0..3);
        tabe.pos = Pos::Verb;
        let mut darou = synth("だろう", "だ", &["助動詞"], 3..6);
        darou.pos = Pos::Auxiliary;
        let out = apply(vec![tabe, darou], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn merges_bare_da_after_n_only_when_morphology_validates() {
        // 飲ん + だ should merge (飲んだ is valid v5m past).
        let mut yon = synth("飲ん", "飲む", &["動詞"], 0..2);
        yon.pos = Pos::Verb;
        let mut da = synth("だ", "だ", &["助動詞"], 2..3);
        da.pos = Pos::Auxiliary;
        let out = apply(vec![yon, da], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "飲んだ");
    }

    #[test]
    fn does_not_merge_bare_da_when_morphology_rejects() {
        // 知ら + ん + だ — last morpheme is だ following ん, but
        // 知らん isn't a verb past (it's slurred negative). Strict
        // lexicon should reject the merge.
        struct StrictLexicon;
        impl Lexicon for StrictLexicon {
            fn is_valid_verb_past(&self, surface: &str) -> bool {
                ["飲んだ", "読んだ", "死んだ"].contains(&surface)
            }
        }
        // We pass [知らん, だ] — prev is "知らん" (ends in ん), da
        // is the auxiliary. With strict lexicon, 知らんだ isn't a
        // valid past → no merge.
        let mut shiran = synth("知らん", "知る", &["動詞"], 0..3);
        shiran.pos = Pos::Verb;
        let mut da = synth("だ", "だ", &["助動詞"], 3..4);
        da.pos = Pos::Auxiliary;
        let out = apply(vec![shiran, da], &StrictLexicon);
        assert_eq!(out.len(), 2, "should not merge: {:?}", out);
    }

    #[test]
    fn does_not_merge_after_noun() {
        // Auxiliary after a Noun shouldn't trigger this rule.
        let school = synth("学校", "学校", &["名詞"], 0..2);
        let mut da = synth("だ", "だ", &["助動詞"], 2..3);
        da.pos = Pos::Auxiliary;
        let out = apply(vec![school, da], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }
}
