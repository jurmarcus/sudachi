//! `CombineFinal` — Final pass: merge ば onto Verb predecessors AND
//! glue together SpecialCases2 known compounds.
//!
//! ## ば-merge
//!
//! `Verb + ば` → merged into a single Verb morpheme (the conditional
//! form). Other particles are left to earlier Combine stages.
//!
//! ## SpecialCases2 lookup (deferred)
//!
//! Jiten consults a 100+ entry hand-curated table of (token1, token2,
//! optional POS) tuples and merges any pair found there (e.g.,
//! じゃ+ない → じゃない Expression, だ+けど → だけど Conjunction).
//! Porting the full SpecialCases2 table is its own follow-up commit
//! since it carries 100+ entries that need careful Pos-mapping.
//!
//! Implemented here: ば-merge only.
//!
//! TODO: port SpecialCases2 table into `crate::data` and wire
//! through here.
//!
//! Ported from
//! [Sirush/Jiten CombineStages.cs `CombineFinal`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.CombineStages.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};

pub const NAME: &str = "combine_final";

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
        if i + 1 < morphemes.len() {
            let next = &morphemes[i + 1];
            if next.surface == "ば" && matches!(cur.pos, Pos::Verb) {
                let mut merged = cur.clone();
                merged.surface.push_str(&next.surface);
                merged.reading_form.push_str(&next.reading_form);
                merged.char_range = cur.char_range.start..next.char_range.end;
                merged.record_rule(NAME);
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

    #[test]
    fn merges_ba_after_verb() {
        let yare = synth("やれ", "やる", &["動詞"], 0..2);
        let ba = synth("ば", "ば", &["助詞"], 2..3);
        let out = apply(vec![yare, ba], &EmptyLexicon);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "やれば");
        assert!(matches!(out[0].pos, Pos::Verb));
    }

    #[test]
    fn does_not_merge_ba_after_noun() {
        let school = synth("学校", "学校", &["名詞"], 0..2);
        let ba = synth("ば", "ば", &["助詞"], 2..3);
        let out = apply(vec![school, ba], &EmptyLexicon);
        assert_eq!(out.len(), 2);
    }
}
