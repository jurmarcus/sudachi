//! `FixReadingAmbiguity` — Resolve kanji-homograph reading
//! ambiguities using contextual cues from neighbouring morphemes.
//!
//! ## Reading reassignments ported in this commit
//!
//! - 表 (ヒョウ) → オモテ when followed by directional へ/に AND
//!   not preceded by a Noun (e.g., 表へ出る "go outside" vs
//!   メニュー表 "menu chart").
//! - 何 (ナン) → ナニ when at end-of-sentence OR followed by を/が/も.
//! - 一日 / １日 / 1日 (ツイタチ) → イチニチ unless preceded by a
//!   morpheme ending in 月 (i.e. inside a date X月一日).
//! - 禍 (カ) → ワザワイ standalone (カ only valid in compounds like
//!   コロナ禍 / 戦禍).
//! - 私 (シ) → ワタシ standalone, AND reclassify as Pronoun.
//! - 寒気 (カンキ) → サムケ when followed by が + する (chills, not
//!   cold air).
//! - 後 (ゴ) → アト when followed by Numeral / 数詞 sub-POS.
//! - 次 (ジ) Prefix → ツギ Noun (ジ only valid in 次回/次期/次男).
//! - 何時 (ナンドキ) → ナンジ (modern usage).
//! - 長 (チョウ) Suffix → Noun (the "chief/head" reading; archaic
//!   なが mapping bypassed).
//! - 隙 (ヒマ) → スキ (modern reading).
//! - 弄う dict form → 弄る (replace イラ → イジ in reading).
//!
//! Deferred:
//! - 角 (カド) → かど/つの/かく — needs 4-way contextual
//!   disambiguation with prev/next/prev-2/next-2 inspection.
//!
//! Ported from
//! [Sirush/Jiten Disambiguation.cs `FixReadingAmbiguity`](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Stages/MorphologicalAnalyser.Disambiguation.cs).

use crate::lookup::Lexicon;
use crate::stage::{Phase, Stage};
use crate::token::{Morpheme, Pos};

pub const NAME: &str = "disambiguation_fix_reading_ambiguity";

pub fn stage() -> Stage {
    Stage::always(NAME, Phase::Disambiguation, apply)
}

pub fn apply(mut morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
    let n = morphemes.len();
    for i in 0..n {
        let mut changed = false;

        let surface = morphemes[i].surface.clone();
        let reading = morphemes[i].reading_form.clone();

        // 表 → オモテ when followed by へ/に, not preceded by Noun.
        if surface == "表" && reading == "ヒョウ" && i + 1 < n {
            let nxt = &morphemes[i + 1];
            let prev_noun = i > 0 && matches!(morphemes[i - 1].pos, Pos::Noun);
            if (nxt.surface == "へ" || nxt.surface == "に") && !prev_noun {
                morphemes[i].reading_form = "オモテ".to_string();
                changed = true;
            }
        }

        // 何 → ナニ at sentence end or before を/が/も.
        if surface == "何" && reading == "ナン" {
            let next_triggers = morphemes
                .get(i + 1)
                .map(|n| matches!(n.surface.as_str(), "を" | "が" | "も"))
                .unwrap_or(true);
            if next_triggers {
                morphemes[i].reading_form = "ナニ".to_string();
                changed = true;
            }
        }

        // 一日 / 1日 → イチニチ unless preceded by X月.
        if matches!(surface.as_str(), "一日" | "１日" | "1日") && reading == "ツイタチ" {
            let preceded_by_month = i > 0 && morphemes[i - 1].surface.ends_with('月');
            if !preceded_by_month {
                morphemes[i].reading_form = "イチニチ".to_string();
                changed = true;
            }
        }

        // 禍 → ワザワイ standalone.
        if surface == "禍" && reading == "カ" {
            morphemes[i].reading_form = "ワザワイ".to_string();
            changed = true;
        }

        // 私 → ワタシ standalone, reclassify as Pronoun.
        if surface == "私" && reading == "シ" {
            morphemes[i].reading_form = "ワタシ".to_string();
            morphemes[i].pos = Pos::Pronoun;
            morphemes[i].part_of_speech = vec!["代名詞".into()];
            changed = true;
        }

        // 寒気 → サムケ when が + する follows.
        if surface == "寒気" && reading == "カンキ" && i + 2 < n {
            if morphemes[i + 1].surface == "が" && morphemes[i + 2].dictionary_form == "する" {
                morphemes[i].reading_form = "サムケ".to_string();
                changed = true;
            }
        }

        // 後 → アト when followed by Numeral / 数詞 sub-POS.
        if surface == "後" && reading == "ゴ" && i + 1 < n {
            let nxt = &morphemes[i + 1];
            let nxt_is_numeral = nxt.part_of_speech.iter().any(|p| p == "数詞");
            if nxt_is_numeral {
                morphemes[i].reading_form = "アト".to_string();
                changed = true;
            }
        }

        // 次 Prefix → ツギ Noun.
        if surface == "次" && reading == "ジ" && matches!(morphemes[i].pos, Pos::Prefix) {
            morphemes[i].reading_form = "ツギ".to_string();
            morphemes[i].pos = Pos::Noun;
            morphemes[i].part_of_speech = vec!["名詞".into()];
            changed = true;
        }

        // 何時 → ナンジ.
        if surface == "何時" && reading == "ナンドキ" {
            morphemes[i].reading_form = "ナンジ".to_string();
            changed = true;
        }

        // 長 Suffix チョウ → Noun.
        if surface == "長" && reading == "チョウ" && matches!(morphemes[i].pos, Pos::Suffix) {
            morphemes[i].pos = Pos::Noun;
            morphemes[i].part_of_speech = vec!["名詞".into()];
            changed = true;
        }

        // 隙 → スキ.
        if surface == "隙" && reading == "ヒマ" {
            morphemes[i].reading_form = "スキ".to_string();
            changed = true;
        }

        // 弄う → 弄る (replace イラ → イジ in reading).
        if morphemes[i].dictionary_form == "弄う" {
            morphemes[i].dictionary_form = "弄る".to_string();
            morphemes[i].normalized_form = "弄る".to_string();
            morphemes[i].reading_form = morphemes[i].reading_form.replace("イラ", "イジ");
            changed = true;
        }

        if changed {
            morphemes[i].record_rule(NAME);
        }
    }
    morphemes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;

    fn synth(
        surface: &str,
        reading: &str,
        pos_top: &str,
        char_range: std::ops::Range<usize>,
    ) -> Morpheme {
        Morpheme::synthesize(
            surface,
            reading,
            surface,
            vec![pos_top.into()],
            char_range,
        )
    }

    /// Direct port of Jiten PipelineStageTests.cs
    /// `DisambiguationStage_ShouldAdjustReadingByContext`.
    #[test]
    fn omote_disambiguates_when_followed_by_directional_he() {
        let omote = synth("表", "ヒョウ", "名詞", 0..1);
        let he = synth("へ", "へ", "助詞", 1..2);
        let out = apply(vec![omote, he], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "オモテ");
    }

    #[test]
    fn nani_disambiguates_at_sentence_end() {
        let nan = synth("何", "ナン", "代名詞", 0..1);
        let out = apply(vec![nan], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "ナニ");
    }

    #[test]
    fn ichinichi_disambiguates_when_no_month_predecessor() {
        let one_day = synth("一日", "ツイタチ", "名詞", 0..2);
        let out = apply(vec![one_day], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "イチニチ");
    }

    #[test]
    fn keeps_tsuitachi_after_month() {
        let three_month = synth("3月", "サンガツ", "名詞", 0..2);
        let one_day = synth("一日", "ツイタチ", "名詞", 2..4);
        let out = apply(vec![three_month, one_day], &EmptyLexicon);
        assert_eq!(out[1].reading_form, "ツイタチ");
    }

    #[test]
    fn watashi_disambiguates_and_reclassifies() {
        let watashi = synth("私", "シ", "名詞", 0..1);
        let out = apply(vec![watashi], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "ワタシ");
        assert!(matches!(out[0].pos, Pos::Pronoun));
    }

    #[test]
    fn samuke_disambiguates_when_ga_suru_follows() {
        let kanki = synth("寒気", "カンキ", "名詞", 0..2);
        let ga = synth("が", "が", "助詞", 2..3);
        let mut suru = synth("する", "スル", "動詞", 3..5);
        suru.dictionary_form = "する".to_string();
        let out = apply(vec![kanki, ga, suru], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "サムケ");
    }
}
