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
//! - 今日 (コンニチ) → キョウ standalone. Sudachi's UniDic context
//!   selection picks コンニチ in some sentence positions (notably
//!   when 今日 sits between commas as a topic marker), but キョウ is
//!   the everyday "today" reading in essentially all modern contexts.
//!   コンニチ is reserved for formal compounds like 今日において / 今日では,
//!   which are rare enough that an unconditional rewrite is safer
//!   than letting the systematic mis-reading propagate to vocab
//!   matching downstream.
//! - 私 (ワタクシ) → ワタシ. Sudachi stores the formal pronoun
//!   ワタクシ as the lemma reading; everyday speech / writing uses
//!   ワタシ. Same intervention pattern as 今日.
//! - 玩具 (ガング) → オモチャ. Native jukujikun for "toy"; the kango
//!   ガング is rare/literary.
//! - 雪道 (セツドウ) → ユキミチ. Native compound for "snowy road"; the
//!   kango セツドウ is rare.
//! - 日本 (ニッポン) → ニホン. Sudachi defaults to the formal /
//!   official ニッポン; everyday modern usage is ニホン. Affected
//!   ~2% of cards in a 1000-passage audit.
//! - 明日 (アス) → アシタ. アス is more formal / written; アシタ is
//!   conversational. Both valid; アシタ is more common.
//! - 山道 (サンドウ) → ヤマミチ. Same native-vs-kango pattern as 雪道.
//! - 時 (トキ) → ジ when preceded by a Numeral. The "o'clock" counter
//!   reading (5時 = ゴジ) overwhelmingly dominates after a number; UniDic's
//!   default トキ ("time") only fits in standalone contexts. 29 of 44
//!   fixable mismatches in the 8.6k-card readings audit were this
//!   single pattern. Mirror of the 後→アト rule.
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
        if surface == "寒気" && reading == "カンキ" && i + 2 < n
            && morphemes[i + 1].surface == "が" && morphemes[i + 2].dictionary_form == "する" {
                morphemes[i].reading_form = "サムケ".to_string();
                changed = true;
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

        // 時 → ジ when preceded by a Numeral (時 as the "o'clock"
        // counter: 5時 = ゴジ, not ゴトキ). Sudachi's UniDic defaults
        // to トキ ("time") in many of these contexts, but after a
        // numeral the o'clock reading dominates by overwhelming
        // margin in real text. 29 of 44 fixable mismatches in the
        // 8.6k-card audit were this exact pattern (X時 / X時に).
        //
        // Mirror image of the 後→アト rule above (which fires when
        // *followed* by a numeral, e.g. 後10分 = アト).
        if surface == "時" && reading == "トキ" && i > 0 {
            let prev = &morphemes[i - 1];
            let prev_is_numeral = prev.part_of_speech.iter().any(|p| p == "数詞");
            if prev_is_numeral {
                morphemes[i].reading_form = "ジ".to_string();
                if morphemes[i].dictionary_form_reading == "トキ" {
                    morphemes[i].dictionary_form_reading = "ジ".to_string();
                }
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

        // 今日 → キョウ. Sudachi's UniDic picks コンニチ in some
        // contexts (e.g. between commas), but キョウ is the everyday
        // reading in essentially all modern usage. Both reading_form
        // (surface reading) and dictionary_form_reading (lemma
        // reading) are rewritten so downstream vocab matchers and
        // Jitendex queries both see キョウ.
        if surface == "今日" && reading == "コンニチ" {
            morphemes[i].reading_form = "キョウ".to_string();
            if morphemes[i].dictionary_form_reading == "コンニチ" {
                morphemes[i].dictionary_form_reading = "キョウ".to_string();
            }
            changed = true;
        }

        // 私 → ワタシ. Sudachi's UniDic stores ワタクシ as the lemma
        // reading. ワタクシ is correct in highly formal/written
        // contexts only; in everyday speech and modern writing the
        // pronoun reads ワタシ. Same intervention pattern as 今日.
        if surface == "私" && reading == "ワタクシ" {
            morphemes[i].reading_form = "ワタシ".to_string();
            if morphemes[i].dictionary_form_reading == "ワタクシ" {
                morphemes[i].dictionary_form_reading = "ワタシ".to_string();
            }
            changed = true;
        }

        // 玩具 → オモチャ. Sudachi reads as the kango ガング, but
        // the everyday word for "toy" uses the jukujikun reading
        // オモチャ. ガング is rare and mostly literary.
        if surface == "玩具" && reading == "ガング" {
            morphemes[i].reading_form = "オモチャ".to_string();
            if morphemes[i].dictionary_form_reading == "ガング" {
                morphemes[i].dictionary_form_reading = "オモチャ".to_string();
            }
            changed = true;
        }

        // 雪道 → ユキミチ. Sudachi reads as kango セツドウ, but native
        // reading ユキミチ ("snowy road") is what's in everyday use.
        if surface == "雪道" && reading == "セツドウ" {
            morphemes[i].reading_form = "ユキミチ".to_string();
            if morphemes[i].dictionary_form_reading == "セツドウ" {
                morphemes[i].dictionary_form_reading = "ユキミチ".to_string();
            }
            changed = true;
        }

        // 日本 → ニホン. Sudachi reads as ニッポン (formal/historical
        // / official-name reading); modern everyday usage is ニホン
        // by a wide margin. Frequency stats in vocab confirm ニホン
        // is freq=128 vs ニッポン which is unranked.
        if surface == "日本" && reading == "ニッポン" {
            morphemes[i].reading_form = "ニホン".to_string();
            if morphemes[i].dictionary_form_reading == "ニッポン" {
                morphemes[i].dictionary_form_reading = "ニホン".to_string();
            }
            changed = true;
        }

        // 明日 → アシタ. Sudachi reads as アス (more formal /
        // written register); アシタ is the everyday reading. Both
        // valid; アシタ is more common in spoken / casual writing.
        if surface == "明日" && reading == "アス" {
            morphemes[i].reading_form = "アシタ".to_string();
            if morphemes[i].dictionary_form_reading == "アス" {
                morphemes[i].dictionary_form_reading = "アシタ".to_string();
            }
            changed = true;
        }

        // 山道 → ヤマミチ. Sudachi reads as kango サンドウ; native
        // ヤマミチ is the everyday reading. Same pattern as 雪道.
        if surface == "山道" && reading == "サンドウ" {
            morphemes[i].reading_form = "ヤマミチ".to_string();
            if morphemes[i].dictionary_form_reading == "サンドウ" {
                morphemes[i].dictionary_form_reading = "ヤマミチ".to_string();
            }
            changed = true;
        }

        // 富士山 → フジサン. The mountain is overwhelmingly read
        // フジサン (the official / standard reading); フジヤマ is
        // a marginal Anglo-popularised pronunciation almost never
        // used in modern Japanese.
        if surface == "富士山" && reading == "フジヤマ" {
            morphemes[i].reading_form = "フジサン".to_string();
            if morphemes[i].dictionary_form_reading == "フジヤマ" {
                morphemes[i].dictionary_form_reading = "フジサン".to_string();
            }
            changed = true;
        }

        // 二日 → フツカ. Sudachi reads as フタカ (the rare native
        // counter form); フツカ is the everyday "2nd day / 2 days"
        // reading. Same kind of counter-disambiguation as 一日.
        if surface == "二日" && reading == "フタカ" {
            morphemes[i].reading_form = "フツカ".to_string();
            if morphemes[i].dictionary_form_reading == "フタカ" {
                morphemes[i].dictionary_form_reading = "フツカ".to_string();
            }
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

    #[test]
    fn kyou_disambiguates_konnichi_in_both_reading_fields() {
        // Sudachi reads 今日 as コンニチ in some contexts (e.g.
        // 昨日…、今日、補講…). Override to キョウ unconditionally.
        let mut konnichi = synth("今日", "コンニチ", "名詞", 0..2);
        konnichi.dictionary_form_reading = "コンニチ".to_string();
        let out = apply(vec![konnichi], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "キョウ");
        assert_eq!(out[0].dictionary_form_reading, "キョウ");
        assert!(out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn kyou_left_alone_when_already_correct() {
        let kyou = synth("今日", "キョウ", "名詞", 0..2);
        let out = apply(vec![kyou], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "キョウ");
        // Rule didn't fire — kept original value.
        assert!(!out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn watashi_disambiguates_watakushi_lemma() {
        // Sudachi reads 私 as ワタクシ in many contexts; everyday
        // reading is ワタシ.
        let mut watakushi = synth("私", "ワタクシ", "代名詞", 0..1);
        watakushi.dictionary_form_reading = "ワタクシ".to_string();
        let out = apply(vec![watakushi], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "ワタシ");
        assert_eq!(out[0].dictionary_form_reading, "ワタシ");
        assert!(out[0].applied_rules.contains(&NAME));
    }

    #[test]
    fn omocha_disambiguates_gangu() {
        let mut gangu = synth("玩具", "ガング", "名詞", 0..2);
        gangu.dictionary_form_reading = "ガング".to_string();
        let out = apply(vec![gangu], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "オモチャ");
        assert_eq!(out[0].dictionary_form_reading, "オモチャ");
    }

    #[test]
    fn yukimichi_disambiguates_setsudou() {
        let mut setsudou = synth("雪道", "セツドウ", "名詞", 0..2);
        setsudou.dictionary_form_reading = "セツドウ".to_string();
        let out = apply(vec![setsudou], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "ユキミチ");
        assert_eq!(out[0].dictionary_form_reading, "ユキミチ");
    }

    #[test]
    fn nihon_disambiguates_nippon() {
        let mut nippon = synth("日本", "ニッポン", "名詞", 0..2);
        nippon.dictionary_form_reading = "ニッポン".to_string();
        let out = apply(vec![nippon], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "ニホン");
        assert_eq!(out[0].dictionary_form_reading, "ニホン");
    }

    #[test]
    fn ashita_disambiguates_asu() {
        let mut asu = synth("明日", "アス", "名詞", 0..2);
        asu.dictionary_form_reading = "アス".to_string();
        let out = apply(vec![asu], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "アシタ");
        assert_eq!(out[0].dictionary_form_reading, "アシタ");
    }

    #[test]
    fn yamamichi_disambiguates_sandou() {
        let mut sandou = synth("山道", "サンドウ", "名詞", 0..2);
        sandou.dictionary_form_reading = "サンドウ".to_string();
        let out = apply(vec![sandou], &EmptyLexicon);
        assert_eq!(out[0].reading_form, "ヤマミチ");
        assert_eq!(out[0].dictionary_form_reading, "ヤマミチ");
    }

    #[test]
    fn ji_disambiguates_toki_after_numeral() {
        // 5時 = ゴジ, not ゴトキ. Numeral predecessor triggers
        // o'clock reading.
        let mut go = synth("5", "ゴ", "名詞", 0..1);
        go.part_of_speech = vec!["名詞".into(), "数詞".into()];
        let mut toki = synth("時", "トキ", "名詞", 1..2);
        toki.dictionary_form_reading = "トキ".to_string();
        let out = apply(vec![go, toki], &EmptyLexicon);
        assert_eq!(out[1].reading_form, "ジ");
        assert_eq!(out[1].dictionary_form_reading, "ジ");
        assert!(out[1].applied_rules.contains(&NAME));
    }

    #[test]
    fn toki_left_alone_without_numeral() {
        // Standalone 時 (no numeral predecessor) keeps トキ.
        // E.g. "その時に" — context is a demonstrative, not a number.
        let sono = synth("その", "ソノ", "連体詞", 0..2);
        let mut toki = synth("時", "トキ", "名詞", 2..3);
        toki.dictionary_form_reading = "トキ".to_string();
        let out = apply(vec![sono, toki], &EmptyLexicon);
        assert_eq!(out[1].reading_form, "トキ");
        assert!(!out[1].applied_rules.contains(&NAME));
    }
}
