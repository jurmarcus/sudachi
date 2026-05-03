//! [`VerbClass`] — every distinct conjugation paradigm in modern
//! Japanese plus the classical residues that JMdict still carries.
//!
//! Variant names are the **linguistic descriptor** of what the class
//! IS — `Ichidan`, `GodanBu`, `GodanKuIku`, `Suru` — not the
//! abbreviated codes JMdict uses internally (v1, v5b, v5k-s, vs-i).
//! The codes live in the `serde(rename = "...")` attributes so JSON
//! interop with JMdict / Yomichan-derived data works without
//! polluting the API surface.
//!
//! ## Why this many variants
//!
//! Japanese verb conjugation is mostly regular — give Sudachi a verb
//! ending and it can conjugate it. But there are ~12 systematic
//! godan classes (one per consonant) plus ~6 special-case classes
//! that conjugate slightly differently and CANNOT be folded together
//! without producing wrong output. Examples:
//!
//! - `GodanKu` (書く → 書いた) and `GodanKuIku` (行く → 行った)
//!   share the kana ending but differ in past tense.
//! - `GodanRu` (走る → 走らない) and `GodanRuAru` (ある → ない)
//!   differ in negative form.
//! - `GodanU` (買う → 買った) and `GodanUSpecial` (請う → 請うた)
//!   differ in past tense.
//!
//! Conflating these is the #1 source of bugs in conjugation libraries.
//! Keeping the distinctions explicit at the type level prevents that.

use serde::{Deserialize, Serialize};

/// Every conjugation paradigm in modern Japanese, plus classical
/// residues JMdict catalogues. Maps 1:1 onto JMdict's verb POS tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerbClass {
    // ─── Ichidan (一段) ──────────────────────────────────────────────
    /// 一段動詞 — drops stem-final る before any suffix.
    /// Examples: 食べる, 見る, 起きる, 寝る.
    #[serde(rename = "v1")]
    Ichidan,

    /// 一段動詞 -しる irregular — くれる. Imperative is くれ
    /// (irregular; regular ichidan would give くれろ).
    #[serde(rename = "v1-s")]
    IchidanKureru,

    // ─── Godan (五段) — by consonant row ─────────────────────────────
    /// 五段動詞 -ぶ. Past form: んだ. Examples: 飛ぶ, 遊ぶ, 学ぶ.
    #[serde(rename = "v5b")]
    GodanBu,

    /// 五段動詞 -ぐ. Past form: いだ. Examples: 泳ぐ, 急ぐ, 脱ぐ.
    #[serde(rename = "v5g")]
    GodanGu,

    /// 五段動詞 -く. Past form: いた. Examples: 書く, 聞く, 歩く.
    #[serde(rename = "v5k")]
    GodanKu,

    /// 五段動詞 -く 行く-class — 行く only. Past is irregular いった
    /// (not いいた as a regular GodanKu would predict).
    #[serde(rename = "v5k-s")]
    GodanKuIku,

    /// 五段動詞 -む. Past form: んだ. Examples: 飲む, 読む, 住む.
    #[serde(rename = "v5m")]
    GodanMu,

    /// 五段動詞 -ぬ. Past form: んだ. Only verb in this class: 死ぬ.
    #[serde(rename = "v5n")]
    GodanNu,

    /// 五段動詞 -る. Past form: った. Examples: 走る, 知る, 座る,
    /// 切る, 帰る, 入る (those last three are godan despite ichidan-
    /// looking endings).
    #[serde(rename = "v5r")]
    GodanRu,

    /// 五段動詞 -る ある-class — irregular negative ない (not
    /// あらない). Examples: ある, ござる, なさる, くださる, おっしゃる.
    #[serde(rename = "v5r-i")]
    GodanRuAru,

    /// 五段動詞 -す. Past form: した. Examples: 話す, 出す, 押す.
    #[serde(rename = "v5s")]
    GodanSu,

    /// 五段動詞 -つ. Past form: った. Examples: 持つ, 立つ, 待つ.
    #[serde(rename = "v5t")]
    GodanTsu,

    /// 五段動詞 -う. Past form: った. Examples: 買う, 言う, 思う.
    #[serde(rename = "v5u")]
    GodanU,

    /// 五段動詞 -う 請う/問う-class — past is うた (not った).
    /// Examples: 請う → 請うた, 問う → 問うた.
    #[serde(rename = "v5u-s")]
    GodanUSpecial,

    /// 五段動詞 -aる group — humble/honorific verbs with irregular
    /// polite ます form (なさいます, not なさります). Members:
    /// なさる, くださる, ござる, おっしゃる, いらっしゃる.
    #[serde(rename = "v5aru")]
    GodanAru,

    // ─── Irregular ───────────────────────────────────────────────────
    /// する — most irregular verb in the language. Stem changes
    /// across さ-/し-/す-/せ- depending on the form.
    #[serde(rename = "vs-i")]
    Suru,

    /// する compound (suru-noun + する). Same conjugation as Suru
    /// with the noun prefixed. Examples: 勉強する, 旅行する.
    #[serde(rename = "vs-s")]
    SuruCompound,

    /// 来る — irregular. Stem changes こ-/き-/く- across forms.
    #[serde(rename = "vk")]
    Kuru,

    /// -ずる alternate of -じる ichidan verbs. Examples: 信ずる
    /// (= 信じる), 感ずる (= 感じる).
    #[serde(rename = "vz")]
    Zuru,

    // ─── Classical residues JMdict still tags ───────────────────────
    /// Irregular nu verb (classical). Only entries: 死ぬ in some
    /// classical contexts. Modern usage: GodanNu.
    #[serde(rename = "vn")]
    NuVerbClassical,

    /// Irregular ru verb (classical), plain form ends in -り.
    /// Examples: あり, 居り (modern います).
    #[serde(rename = "vr")]
    RuVerbClassical,

    /// Yodan (四段) -る — classical pre-Edo conjugation, replaced by
    /// godan in modern Japanese. JMdict still tags some classical
    /// entries with this.
    #[serde(rename = "v4r")]
    YodanRu,
}

impl VerbClass {
    /// Is this an Ichidan (一段) class — `Ichidan` or `IchidanKureru`?
    /// Used by forward conjugation to decide between "drop る + suffix"
    /// (ichidan) vs "shift vowel + suffix" (godan).
    pub fn is_ichidan(self) -> bool {
        matches!(self, Self::Ichidan | Self::IchidanKureru)
    }

    /// Is this any Godan (五段) class? Returns false for irregulars
    /// and classical variants.
    pub fn is_godan(self) -> bool {
        matches!(
            self,
            Self::GodanBu
                | Self::GodanGu
                | Self::GodanKu
                | Self::GodanKuIku
                | Self::GodanMu
                | Self::GodanNu
                | Self::GodanRu
                | Self::GodanRuAru
                | Self::GodanSu
                | Self::GodanTsu
                | Self::GodanU
                | Self::GodanUSpecial
                | Self::GodanAru
        )
    }

    /// Is this an irregular class needing per-form lookup tables
    /// rather than rule-driven conjugation?
    pub fn is_irregular(self) -> bool {
        matches!(self, Self::Suru | Self::SuruCompound | Self::Kuru | Self::Zuru)
    }

    /// The terminal kana the verb ends with in its dictionary form.
    /// Returns None for ichidan (which always ends in る but the
    /// "consonant" is irrelevant — the る is just a suffix marker).
    pub fn terminal_kana(self) -> Option<char> {
        match self {
            Self::GodanBu => Some('ぶ'),
            Self::GodanGu => Some('ぐ'),
            Self::GodanKu | Self::GodanKuIku => Some('く'),
            Self::GodanMu => Some('む'),
            Self::GodanNu | Self::NuVerbClassical => Some('ぬ'),
            Self::GodanRu | Self::GodanRuAru | Self::GodanAru | Self::YodanRu => Some('る'),
            Self::GodanSu => Some('す'),
            Self::GodanTsu => Some('つ'),
            Self::GodanU | Self::GodanUSpecial => Some('う'),
            Self::Zuru => Some('る'), // ずる
            _ => None,
        }
    }

    /// Resolve a JMdict POS tag string to a [`VerbClass`].
    /// Accepts the abbreviated codes (`v1`, `v5b`, `v5k-s`, etc.).
    ///
    /// Special cases beyond the per-variant `#[serde(rename = ...)]`:
    /// - `vs` (bare suru-noun marker) → [`Self::Suru`]. JMdict tags
    ///   noun-verb-suru entries with bare `vs` AND `vs-i`/`vs-s`
    ///   inconsistently; our analyzer treats them as suru-conjugated.
    /// - `vs-c` (classical suru) → [`Self::Suru`]. Modern usage same.
    pub fn from_jmdict(tag: &str) -> Option<Self> {
        match tag {
            "vs" | "vs-c" => return Some(Self::Suru),
            _ => {}
        }
        // serde_json deserialization handles the per-variant rename mapping.
        // Wrap in quotes to make it a JSON string.
        let json = format!("\"{}\"", tag);
        serde_json::from_str(&json).ok()
    }

    /// JMdict POS tag for this class (`v1`, `v5b`, etc.).
    pub fn jmdict_tag(self) -> &'static str {
        match self {
            Self::Ichidan => "v1",
            Self::IchidanKureru => "v1-s",
            Self::GodanBu => "v5b",
            Self::GodanGu => "v5g",
            Self::GodanKu => "v5k",
            Self::GodanKuIku => "v5k-s",
            Self::GodanMu => "v5m",
            Self::GodanNu => "v5n",
            Self::GodanRu => "v5r",
            Self::GodanRuAru => "v5r-i",
            Self::GodanSu => "v5s",
            Self::GodanTsu => "v5t",
            Self::GodanU => "v5u",
            Self::GodanUSpecial => "v5u-s",
            Self::GodanAru => "v5aru",
            Self::Suru => "vs-i",
            Self::SuruCompound => "vs-s",
            Self::Kuru => "vk",
            Self::Zuru => "vz",
            Self::NuVerbClassical => "vn",
            Self::RuVerbClassical => "vr",
            Self::YodanRu => "v4r",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jmdict_round_trip_for_every_variant() {
        let all = [
            VerbClass::Ichidan,
            VerbClass::IchidanKureru,
            VerbClass::GodanBu,
            VerbClass::GodanGu,
            VerbClass::GodanKu,
            VerbClass::GodanKuIku,
            VerbClass::GodanMu,
            VerbClass::GodanNu,
            VerbClass::GodanRu,
            VerbClass::GodanRuAru,
            VerbClass::GodanSu,
            VerbClass::GodanTsu,
            VerbClass::GodanU,
            VerbClass::GodanUSpecial,
            VerbClass::GodanAru,
            VerbClass::Suru,
            VerbClass::SuruCompound,
            VerbClass::Kuru,
            VerbClass::Zuru,
            VerbClass::NuVerbClassical,
            VerbClass::RuVerbClassical,
            VerbClass::YodanRu,
        ];
        for vc in all {
            let tag = vc.jmdict_tag();
            assert_eq!(
                VerbClass::from_jmdict(tag),
                Some(vc),
                "round-trip failed for {:?} via tag {:?}",
                vc,
                tag,
            );
        }
    }

    #[test]
    fn ichidan_godan_irregular_sets_are_disjoint() {
        let all = [
            VerbClass::Ichidan,
            VerbClass::GodanBu,
            VerbClass::GodanRu,
            VerbClass::Suru,
            VerbClass::Kuru,
        ];
        for vc in all {
            let buckets = [vc.is_ichidan(), vc.is_godan(), vc.is_irregular()]
                .iter()
                .filter(|b| **b)
                .count();
            assert!(buckets <= 1, "{:?} matches multiple buckets", vc);
        }
    }

    #[test]
    fn terminal_kana_matches_class() {
        assert_eq!(VerbClass::GodanBu.terminal_kana(), Some('ぶ'));
        assert_eq!(VerbClass::GodanKuIku.terminal_kana(), Some('く'));
        assert_eq!(VerbClass::GodanRuAru.terminal_kana(), Some('る'));
        assert_eq!(VerbClass::Ichidan.terminal_kana(), None);
        assert_eq!(VerbClass::Kuru.terminal_kana(), None);
    }
}
