//! `RepairHasaNoun` — Repair the はさ misclassification.
//!
//! Sudachi's UniDic occasionally classifies the particle pair はさ as
//! a single noun (e.g. inside 「これはさ」 — "this is just …"). This
//! stage splits the noun into two particles: は (topic) + さ
//! (sentence-final emphasis).
//!
//! Ported from
//! [Sirush/Jiten RepairStages.cs `RepairHasaNoun`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.RepairStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};
use crate::token_features::MorphemeFeatures;

pub const NAME: &str = "repair_hasa_noun";

pub fn stage() -> Stage {
    Stage::new(NAME, Phase::Repair, MorphemeFeatures::TEXT_HASA, apply)
}

pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let mut result: Vec<Morpheme> = Vec::with_capacity(morphemes.len() + 1);

    for m in morphemes {
        if m.surface != "はさ" || !matches!(m.pos, Pos::Noun) {
            result.push(m);
            continue;
        }
        let begin = m.char_range.start;
        let end = m.char_range.end;

        let mut wa = Morpheme::synthesize(
            "は",
            "は",
            "は",
            vec!["助詞".into()],
            begin..begin + 1,
        );
        wa.record_rule(NAME);
        let mut sa = Morpheme::synthesize(
            "さ",
            "さ",
            "さ",
            vec!["助詞".into()],
            begin + 1..end,
        );
        sa.record_rule(NAME);
        result.push(wa);
        result.push(sa);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    #[test]
    fn splits_hasa_noun_into_two_particles() {
        let hasa = Morpheme::synthesize("はさ", "はさ", "はさ", vec!["名詞".into()], 0..2);
        let out = apply(vec![hasa], &EmptyLexicon);

        let surfaces: Vec<&str> = out.iter().map(|m| m.surface.as_str()).collect();
        assert_eq!(surfaces, vec!["は", "さ"]);
        assert!(matches!(out[0].pos, Pos::Particle));
        assert!(matches!(out[1].pos, Pos::Particle));
        assert!(out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn does_not_touch_hasa_with_non_noun_pos() {
        // If Sudachi classified はさ as something other than a noun
        // (unlikely but defend against it), leave alone.
        let hasa = Morpheme::synthesize("はさ", "はさ", "はさ", vec!["助詞".into()], 0..2);
        let out = apply(vec![hasa], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "はさ");
    }

    #[test]
    fn does_not_touch_other_nouns() {
        let other = Morpheme::synthesize("猫", "ねこ", "猫", vec!["名詞".into()], 0..1);
        let out = apply(vec![other], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "猫");
    }
}
