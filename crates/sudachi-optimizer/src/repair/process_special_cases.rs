//! `ProcessSpecialCases` — Apply a battery of hand-curated
//! special-case rewrites for known Sudachi misclassifications.
//!
//! Jiten's C# original is a single large switch over ~20 sub-cases
//! ranging from "split this specific katakana sequence" to "look up
//! this exact 2- or 3-token sequence in a SpecialCases table".
//!
//! ## Sub-cases ported in this commit
//!
//! - **ノヨ noun split** → ノ (Particle) + ヨ (Particle). Sudachi
//!   sometimes lemmatises the katakana particle pair ノヨ as a noun
//!   (proper name 乃代).
//! - **でしょう combine**: で (Conjunction|Auxiliary) + しょう (Noun)
//!   → でしょう (Expression, dict form でしょう, normalized です).
//! - **で reclassify**: bare で (Conjunction|Auxiliary) when not
//!   followed by も is just a particle.
//! - **ぬ verb → auxiliary**: ぬ (Verb, normalized 寝る — Sudachi's
//!   archaic 寝-ぬ misanalysis) → ぬ (Auxiliary, normalized ず, the
//!   classical negative).
//! - **なれ pronoun → verb**: なれ (Pronoun, normalized 汝 — Sudachi's
//!   archaic "thou" misanalysis) → なれ (Verb, normalized 成る).
//! - **Expression splits**: 人前で → 人前 + で,  様に → 様 + に,
//!   おけばいい → おけ + ば + いい.
//!
//! ## Sub-cases deferred
//!
//! - SpecialCases2 / SpecialCases3 lookup-table merges (would need
//!   the full table ported into `crate::data` — many entries).
//! - Deconjugator-validated んで/んだ Expression → Verb rebrand.
//! - HasCompoundLookup-driven ちかい split (vocab-aware).
//! - RepairChimauFragments helper (auxiliary fragment recovery,
//!   complex).
//! - The てなん particle split (low-impact).
//!
//! These would each land as their own follow-up commit. Each is
//! small individually but carries a separate dependency surface.
//!
//! Ported from
//! [Sirush/Jiten RepairStages.cs `ProcessSpecialCases`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.RepairStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_process_special_cases";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::empty(), apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.is_empty() {
        return morphemes;
    }

    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 4);
    let mut i = 0;
    while i < morphemes.len() {
        let w = &morphemes[i];
        let next = morphemes.get(i + 1);

        // ノヨ split (Sudachi misclassifies as noun, real text is two particles).
        if matches!(w.pos, Pos::Noun | Pos::AdjectivalNoun) && w.surface == "ノヨ" {
            let begin = w.char_range.start;
            let end = w.char_range.end;
            let mut no = Morpheme::synthesize(
                "ノ",
                "ノ",
                "の",
                vec!["助詞".into()],
                begin..begin + 1,
            );
            no.normalized_form = "の".to_string();
            no.record_rule(NAME);
            let mut yo = Morpheme::synthesize(
                "ヨ",
                "ヨ",
                "よ",
                vec!["助詞".into()],
                begin + 1..end,
            );
            yo.normalized_form = "よ".to_string();
            yo.record_rule(NAME);
            out.push(no);
            out.push(yo);
            i += 1;
            continue;
        }

        // でしょう combine: で(Conj|Aux) + しょう(Noun) → でしょう Expression.
        if (matches!(w.pos, Pos::Conjunction | Pos::Auxiliary) && w.surface == "で")
            && next.is_some_and(|n| n.surface == "しょう" && matches!(n.pos, Pos::Noun))
        {
            let n = next.unwrap();
            let mut deshou = Morpheme::synthesize(
                "でしょう",
                "デショウ",
                "でしょう",
                vec!["連語".into()],
                w.char_range.start..n.char_range.end,
            );
            deshou.normalized_form = "です".to_string();
            // Pos::Other for "Expression" (no dedicated variant).
            deshou.pos = Pos::Other;
            deshou.record_rule(NAME);
            out.push(deshou);
            i += 2;
            continue;
        }

        // で as Conjunction|Auxiliary not followed by も → reclassify as Particle.
        if matches!(w.pos, Pos::Conjunction | Pos::Auxiliary)
            && w.surface == "で"
            && !next.is_some_and(|n| n.surface == "も")
        {
            let mut de = w.clone();
            de.pos = Pos::Particle;
            de.part_of_speech = vec!["助詞".into()];
            de.record_rule(NAME);
            out.push(de);
            i += 1;
            continue;
        }

        // ぬ verb (normalized 寝る) → ぬ auxiliary (normalized ず, classical negative).
        if w.surface == "ぬ" && matches!(w.pos, Pos::Verb) && w.normalized_form == "寝る" {
            let mut nu = w.clone();
            nu.pos = Pos::Auxiliary;
            nu.part_of_speech = vec!["助動詞".into()];
            nu.normalized_form = "ず".to_string();
            nu.record_rule(NAME);
            out.push(nu);
            i += 1;
            continue;
        }

        // なれ pronoun (normalized 汝) → なれ verb (normalized 成る).
        if w.surface == "なれ" && matches!(w.pos, Pos::Pronoun) && w.normalized_form == "汝" {
            let mut nare = w.clone();
            nare.pos = Pos::Verb;
            nare.part_of_speech = vec!["動詞".into()];
            nare.normalized_form = "成る".to_string();
            nare.record_rule(NAME);
            out.push(nare);
            i += 1;
            continue;
        }

        // Expression splits.
        if matches!(w.pos, Pos::Other) || w.part_of_speech.first().is_some_and(|p| p == "連語") {
            if let Some((head, tail, tail_pos_top, tail_reading)) = expression_split(&w.surface) {
                let begin = w.char_range.start;
                let end = w.char_range.end;
                let mid = begin + head.chars().count();
                let mut head_tok = Morpheme::synthesize(
                    head,
                    "",
                    head,
                    vec!["名詞".into()],
                    begin..mid,
                );
                head_tok.record_rule(NAME);
                let mut tail_tok = Morpheme::synthesize(
                    tail,
                    tail_reading,
                    tail,
                    vec![tail_pos_top.into()],
                    mid..end,
                );
                tail_tok.record_rule(NAME);
                out.push(head_tok);
                out.push(tail_tok);
                i += 1;
                continue;
            }

            // おけばいい → おけ (Verb, dict おく) + ば (Particle) + いい (Adjective).
            if w.surface == "おけばいい" {
                let begin = w.char_range.start;
                let end = w.char_range.end;
                let o1 = begin + 2;
                let o2 = begin + 3;
                let mut oke = Morpheme::synthesize(
                    "おけ",
                    "オケ",
                    "おく",
                    vec!["動詞".into()],
                    begin..o1,
                );
                oke.record_rule(NAME);
                let mut ba = Morpheme::synthesize(
                    "ば",
                    "バ",
                    "ば",
                    vec!["助詞".into()],
                    o1..o2,
                );
                ba.record_rule(NAME);
                let mut ii = Morpheme::synthesize(
                    "いい",
                    "イイ",
                    "いい",
                    vec!["形容詞".into()],
                    o2..end,
                );
                ii.record_rule(NAME);
                out.push(oke);
                out.push(ba);
                out.push(ii);
                i += 1;
                continue;
            }
        }

        out.push(w.clone());
        i += 1;
    }

    out
}

fn expression_split(surface: &str) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    match surface {
        "人前で" => Some(("人前", "で", "助詞", "デ")),
        "様に" => Some(("様", "に", "助詞", "ニ")),
        _ => None,
    }
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

    #[test]
    fn splits_no_yo_katakana_into_two_particles() {
        let noyo = synth("ノヨ", "乃代", "名詞", 0..2);
        let out = apply(vec![noyo], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["ノ", "ヨ"]);
        assert!(out.iter().all(|m| matches!(m.pos, Pos::Particle)));
    }

    #[test]
    fn combines_de_shou_into_deshou_expression() {
        let de = synth("で", "で", "接続詞", 0..1);
        let shou = synth("しょう", "しょう", "名詞", 1..4);
        let out = apply(vec![de, shou], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["でしょう"]);
        assert_eq!(out[0].normalized_form, "です");
    }

    #[test]
    fn reclassifies_bare_de_conjunction_as_particle() {
        let de = synth("で", "で", "接続詞", 0..1);
        let other = synth("猫", "猫", "名詞", 1..2);
        let out = apply(vec![de, other], &EmptyLexicon);
        assert_eq!(out.len(), 2);
        assert!(matches!(out[0].pos, Pos::Particle));
    }

    #[test]
    fn does_not_reclassify_de_when_followed_by_mo() {
        // でも sequence — leave alone.
        let de = synth("で", "で", "接続詞", 0..1);
        let mo = synth("も", "も", "助詞", 1..2);
        let out = apply(vec![de, mo], &EmptyLexicon);
        // First morpheme should NOT be re-classified to Particle.
        assert!(!matches!(out[0].pos, Pos::Particle));
    }

    #[test]
    fn reclassifies_archaic_nu_verb_as_negative_auxiliary() {
        let mut nu = synth("ぬ", "ぬ", "動詞", 0..1);
        nu.normalized_form = "寝る".to_string();
        let out = apply(vec![nu], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert!(matches!(out[0].pos, Pos::Auxiliary));
        assert_eq!(out[0].normalized_form, "ず");
    }

    #[test]
    fn reclassifies_archaic_nare_pronoun_as_naru_verb() {
        let mut nare = synth("なれ", "なれ", "代名詞", 0..2);
        nare.normalized_form = "汝".to_string();
        let out = apply(vec![nare], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert!(matches!(out[0].pos, Pos::Verb));
        assert_eq!(out[0].normalized_form, "成る");
    }

    #[test]
    fn splits_hitomae_de_expression_into_noun_and_particle() {
        let mut hito = synth("人前で", "人前で", "連語", 0..3);
        hito.pos = Pos::Other;
        let out = apply(vec![hito], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["人前", "で"]);
    }

    #[test]
    fn splits_okeba_ii_expression_into_three() {
        let mut o = synth("おけばいい", "おけばいい", "連語", 0..5);
        o.pos = Pos::Other;
        let out = apply(vec![o], &EmptyLexicon);
        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["おけ", "ば", "いい"]);
    }
}
