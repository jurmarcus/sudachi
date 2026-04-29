//! `SplitTawakeNoun` — Split たわけ (misanalysed as 戯け noun or
//! たわける verb) into た (past auxiliary) + わけ (noun) when
//! preceded by a verb stem, auxiliary, i-adjective, particle,
//! adverb, or the geminate っ.
//!
//! Sudachi frequently fuses た + わけ into たわけ after verb stems:
//! してたわけ → してた + わけ, あったわけ → あっ + た + わけ.
//! Legitimate uses of たわけ ("fool" — 戯け) follow nouns, prefixes,
//! or adnominals and are left intact.
//!
//! Ported from
//! [Sirush/Jiten SplitStages.cs `SplitTawakeNoun`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.SplitStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "split_tawake_noun";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Split, MorphemeFeatures::TEXT_TAWAKE, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 2);

    for (i, m) in morphemes.iter().enumerate() {
        let candidate = m.surface == "たわけ"
            && (m.dictionary_form == "たわけ" || m.dictionary_form == "たわける")
            && i > 0;
        if !candidate {
            result.push(m.clone());
            continue;
        }
        let prev = &morphemes[i - 1];

        // Verb-context predecessors: verb / aux / i-adj / particle /
        // adverb, OR the geminate っ supplementary symbol.
        let after_verb_context = matches!(
            prev.pos,
            Pos::Verb | Pos::Auxiliary | Pos::Adjective | Pos::Particle | Pos::Adverb,
        ) || (matches!(prev.pos, Pos::Symbol) && prev.surface == "っ");

        if !after_verb_context {
            result.push(m.clone());
            continue;
        }

        let begin = m.char_range.start;
        let end = m.char_range.end;
        let mut ta = Morpheme::synthesize(
            "た",
            "た",
            "た",
            vec!["助動詞".into()],
            begin..begin + 1,
        );
        ta.record_rule(NAME);
        let mut wake = Morpheme::synthesize(
            "わけ",
            "わけ",
            "わけ",
            vec!["名詞".into()],
            begin + 1..end,
        );
        wake.record_rule(NAME);
        result.push(ta);
        result.push(wake);
    }

    result
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
    fn splits_after_verb_predecessor() {
        // してた (Sudachi may emit as one verb morpheme) + たわけ →
        // してた + た + わけ.
        let shi_te_ta = synth("してた", "する", "動詞", 0..3);
        let tawake = synth("たわけ", "たわけ", "名詞", 3..6);
        let out = apply(vec![shi_te_ta, tawake], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["してた", "た", "わけ"]);
        assert!(matches!(out[1].pos, Pos::Auxiliary));
        assert!(matches!(out[2].pos, Pos::Noun));
        assert!(out[1].applied_rules.contains(&NAME));
    }

    #[test]
    fn splits_after_geminate_tsu() {
        // あっ + たわけ → あっ + た + わけ. The っ predecessor is
        // tagged 補助記号 by Sudachi.
        let tsu = synth("っ", "っ", "補助記号", 0..1);
        let tawake = synth("たわけ", "たわけ", "名詞", 1..4);
        let out = apply(vec![tsu, tawake], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["っ", "た", "わけ"]);
    }

    #[test]
    fn does_not_split_after_noun() {
        // 大 + たわけ → legitimate "fool" reading; leave intact.
        let prev = synth("大", "大", "名詞", 0..1);
        let tawake = synth("たわけ", "たわけ", "名詞", 1..4);
        let out = apply(vec![prev, tawake], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["大", "たわけ"]);
        assert!(out[1].applied_rules.is_empty());
    }

    #[test]
    fn does_not_split_when_no_predecessor() {
        let tawake = synth("たわけ", "たわけ", "名詞", 0..3);
        let out = apply(vec![tawake], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["たわけ"]);
    }

    #[test]
    fn does_not_split_when_dict_form_is_unrelated() {
        // たわけ surface but a dict form not in {たわけ, たわける} →
        // shape-mismatch, leave alone.
        let prev = synth("行っ", "行く", "動詞", 0..2);
        let tawake = synth("たわけ", "別物", "名詞", 2..5);
        let out = apply(vec![prev, tawake], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["行っ", "たわけ"]);
    }
}
