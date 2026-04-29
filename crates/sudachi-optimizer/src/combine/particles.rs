//! `CombineParticles` — Glue specific particle pairs and the
//! kamoshire-* expression chain into single morphemes.
//!
//! ## Pairs handled
//!
//! - `に + は` → には
//! - `と + は` → とは
//! - `で + は` → では
//! - `の + に` → のに
//!
//! ## Kamoshire chain
//!
//! `か + も + しれ…` (3 morphemes, last starts with しれ) →
//! single Expression morpheme. Catches かもしれない/かもしれません.
//!
//! ## では-extension
//!
//! After producing では, look one more ahead for ない/無い → emit
//! `ではない` (Expression). If a か follows after that, extend to
//! `ではないか`.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineParticles`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};

pub const NAME: &str = "combine_particles";

pub fn stage() -> Stage {
    Stage::always(NAME, Phase::Combine, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    if morphemes.len() < 2 {
        return morphemes;
    }
    let mut out: Vec<Morpheme> = Vec::with_capacity(morphemes.len());
    let mut i = 0;
    while i < morphemes.len() {
        let cur = &morphemes[i];

        // Kamoshire chain: か + も + しれ* → Expression.
        if i + 2 < morphemes.len()
            && cur.surface == "か"
            && morphemes[i + 1].surface == "も"
            && morphemes[i + 2].surface.starts_with("しれ")
        {
            let third = &morphemes[i + 2];
            let mut merged = cur.clone();
            merged.surface = format!("{}{}{}", cur.surface, morphemes[i + 1].surface, third.surface);
            merged.reading_form = format!(
                "{}{}{}",
                cur.reading_form, morphemes[i + 1].reading_form, third.reading_form
            );
            merged.char_range = cur.char_range.start..third.char_range.end;
            merged.pos = Pos::Other; // Expression
            merged.part_of_speech = vec!["連語".into()];
            merged.record_rule(NAME);
            out.push(merged);
            i += 3;
            continue;
        }

        // Particle pair merges.
        if i + 1 < morphemes.len() {
            let nxt = &morphemes[i + 1];
            let combined = match (cur.surface.as_str(), nxt.surface.as_str()) {
                ("に", "は") => Some("には"),
                ("と", "は") => Some("とは"),
                ("で", "は") => Some("では"),
                ("の", "に") => Some("のに"),
                _ => None,
            };
            if let Some(combined_text) = combined {
                let mut merged = cur.clone();
                merged.surface = combined_text.to_string();
                merged.reading_form = format!("{}{}", cur.reading_form, nxt.reading_form);
                merged.char_range = cur.char_range.start..nxt.char_range.end;
                merged.record_rule(NAME);

                // では-extension: では + ない/無い (+ optional か).
                if combined_text == "では" && i + 2 < morphemes.len() {
                    let look = &morphemes[i + 2];
                    if look.dictionary_form == "ない" || look.dictionary_form == "無い" {
                        merged.surface.push_str(&look.surface);
                        merged.reading_form.push_str(&look.reading_form);
                        merged.char_range = merged.char_range.start..look.char_range.end;
                        merged.dictionary_form = "ではない".to_string();
                        merged.pos = Pos::Other;
                        merged.part_of_speech = vec!["連語".into()];
                        let mut consumed = 3;
                        if i + 3 < morphemes.len() && morphemes[i + 3].surface == "か" {
                            let ka = &morphemes[i + 3];
                            merged.surface.push('か');
                            merged.reading_form.push_str(&ka.reading_form);
                            merged.char_range = merged.char_range.start..ka.char_range.end;
                            merged.dictionary_form = "ではないか".to_string();
                            consumed = 4;
                        }
                        out.push(merged);
                        i += consumed;
                        continue;
                    }
                }

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

    /// Used by Jiten StageDiagnostics_ShouldRecordStageInfo (which
    /// runs CombineParticles on `に + は` → には).
    #[test]
    fn merges_ni_wa_into_ni_wa_compound() {
        let ni = synth("に", "に", &["助詞"], 0..1);
        let wa = synth("は", "は", &["助詞"], 1..2);
        let out = apply(vec![ni, wa], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "には");
    }

    #[test]
    fn merges_kamoshire_chain_into_expression() {
        let ka = synth("か", "か", &["助詞"], 0..1);
        let mo = synth("も", "も", &["助詞"], 1..2);
        let shire = synth("しれ", "しれる", &["動詞"], 2..4);
        let out = apply(vec![ka, mo, shire], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "かもしれ");
    }

    #[test]
    fn extends_dewa_into_dewanai() {
        let de = synth("で", "で", &["助詞"], 0..1);
        let wa = synth("は", "は", &["助詞"], 1..2);
        let nai = synth("ない", "ない", &["助動詞"], 2..4);
        let out = apply(vec![de, wa, nai], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "ではない");
        assert_eq!(out[0].dictionary_form, "ではない");
    }

    #[test]
    fn extends_dewanai_with_trailing_ka() {
        let de = synth("で", "で", &["助詞"], 0..1);
        let wa = synth("は", "は", &["助詞"], 1..2);
        let nai = synth("ない", "ない", &["助動詞"], 2..4);
        let ka = synth("か", "か", &["助詞"], 4..5);
        let out = apply(vec![de, wa, nai, ka], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "ではないか");
        assert_eq!(out[0].dictionary_form, "ではないか");
    }

    #[test]
    fn does_not_merge_unrelated_particles() {
        let mo = synth("も", "も", &["助詞"], 0..1);
        let ga = synth("が", "が", &["助詞"], 1..2);
        let out = apply(vec![mo, ga], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }
}
