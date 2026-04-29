//! [`ConjForm`] — every conjugation operation modern Japanese has.
//!
//! A flat enum because flat enums are the right representation for a
//! closed taxonomy. Compound forms (polite-negative-past) are
//! expressed as their own variants when they're idiomatic and as
//! chains when they're compositional.
//!
//! Forms divide into:
//!
//! - **Base forms** — Dictionary, Te, Tara, Ba, Nara, Volitional,
//!   Imperative, Conditional, Provisional. These are stand-alone.
//! - **Polarity / tense modifiers** — Negative, Past, NegativePast.
//! - **Politeness modifiers** — Polite, PoliteNegative, PolitePast,
//!   PoliteNegativePast, PoliteVolitional.
//! - **Voice modifiers** — Causative, Passive, CausativePassive,
//!   Potential, Honorific (passive form used as honorific).
//! - **Auxiliary chains** — Desiderative (たい), DesiderativeOther
//!   (たがる), Progressive (ている), Resultative (てある),
//!   Preparative (ておく), Attempt (てみる), Completion (てしまう), …
//! - **Conditional / hypothetical** — ConditionalBa, ConditionalTara,
//!   ProvisionalNara, NegativeConditional (なければ).
//! - **Compound modal** — Should (べき), Must (なければならない), …
//! - **Stems** — only meaningful internally; not user-facing in
//!   forward conjugation (used by deconjugator's intermediate forms).

use serde::{Deserialize, Serialize};

/// Every conjugation operation in modern Japanese, plus a handful of
/// classical ones JMdict still represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConjForm {
    // ─── Base forms ─────────────────────────────────────────────────
    /// 終止形 — dictionary form (食べる, 走る, ある).
    Dictionary,
    /// 連用形 — masu-stem (食べ-, 走り-). Combines with ます/たい/etc.
    StemRenyou,
    /// 未然形 — nai-stem (食べ-, 走ら-). Combines with ない/れる/せる.
    StemMizen,
    /// 已然形 — e-stem (食べれ-, 走れ-). Combines with ば.
    StemIzen,
    /// 命令形 — imperative-stem (食べろ, 走れ).
    StemMeirei,

    // ─── Negation ───────────────────────────────────────────────────
    /// Plain negative — ない.
    Negative,
    /// Plain negative past — なかった.
    NegativePast,
    /// Negative te-form — なくて.
    NegativeTe,
    /// Negative ba-conditional — なければ.
    NegativeBa,
    /// Classical negative — ぬ.
    NegativeNu,
    /// Classical negative continuative — ず.
    NegativeZu,
    /// Negative continuative without ni — ずに / ないで.
    NegativeWithoutDoing,

    // ─── Past ───────────────────────────────────────────────────────
    /// Plain past — た / だ.
    Past,
    /// Past tara-conditional — たら.
    PastTara,
    /// Past ri-form (alternation) — たり / だり.
    PastTari,

    // ─── Te-form ────────────────────────────────────────────────────
    /// Plain te-form — て / で.
    Te,

    // ─── Polite (ます-form) ─────────────────────────────────────────
    /// Polite — ます.
    Polite,
    /// Polite past — ました.
    PolitePast,
    /// Polite negative — ません.
    PoliteNegative,
    /// Polite negative past — ませんでした.
    PoliteNegativePast,
    /// Polite te-form — まして.
    PoliteTe,
    /// Polite volitional — ましょう.
    PoliteVolitional,

    // ─── Voice ──────────────────────────────────────────────────────
    /// Causative — せる / させる.
    Causative,
    /// Causative-passive — せられる / させられる.
    CausativePassive,
    /// Short causative — す / さす (colloquial).
    CausativeShort,
    /// Passive — れる / られる.
    Passive,
    /// Honorific (= passive form used as honorific) — れる / られる.
    /// Same surface as Passive; tagged distinctly when context shows
    /// honorific intent.
    Honorific,
    /// Potential — ichidan: られる, godan: e-row + る (書ける, 読める).
    Potential,
    /// Negative potential — られない / 書けない.
    PotentialNegative,

    // ─── Volitional / probable ──────────────────────────────────────
    /// Volitional — おう / よう (行こう, 食べよう).
    Volitional,
    /// Negative volitional — まい (行くまい).
    VolitionalNegative,
    /// Conjectural — だろう (verb+だろう).
    Conjectural,
    /// Polite conjectural — でしょう.
    ConjecturalPolite,

    // ─── Imperative ─────────────────────────────────────────────────
    /// Imperative — godan e-row, ichidan ろ (走れ, 食べろ).
    Imperative,
    /// Negative imperative — verb + な (行くな).
    ImperativeNegative,

    // ─── Conditional / hypothetical ─────────────────────────────────
    /// Conditional ba — えば (走れば).
    ConditionalBa,
    /// Conditional tara — たら (走ったら).
    ConditionalTara,
    /// Provisional nara — なら (行くなら).
    ProvisionalNara,

    // ─── Desiderative ───────────────────────────────────────────────
    /// First-person desiderative — たい (食べたい).
    Desiderative,
    /// Third-person desiderative — たがる (食べたがる).
    DesiderativeOther,

    // ─── Te-form auxiliary chains (compound predicates) ──────────────
    /// Progressive / resultative state — ている.
    Progressive,
    /// Progressive contracted — てる.
    ProgressiveContracted,
    /// Existence with intent — てある.
    Resultative,
    /// Preparation — ておく (とく).
    Preparative,
    /// Attempt — てみる.
    Attempt,
    /// Completion / regret — てしまう (ちゃう / じゃう).
    Completion,
    /// Movement-toward — てくる.
    ComingTo,
    /// Movement-away — ていく.
    GoingTo,
    /// Receiving favour — てもらう / ていただく.
    Receiving,
    /// Giving favour outward — てくれる / てくださる.
    GivingOutward,
    /// Giving favour to other — てあげる / てやる / てさしあげる.
    GivingToOther,

    // ─── Modal compounds ────────────────────────────────────────────
    /// Obligation — なければならない / なきゃ.
    Obligation,
    /// Permission — てもいい.
    Permission,
    /// Prohibition — てはいけない / ちゃいけない.
    Prohibition,
    /// Recommendation — べき / べし.
    Recommendation,
    /// Hearsay — そうだ (after dictionary form).
    Hearsay,
    /// Polite hearsay — そうです (after dictionary form).
    HearsayPolite,
    /// Appearance — そうだ (after stem for verbs / kanji-only stem
    /// for adjectives — drops trailing い/だ).
    Appearance,
    /// Polite appearance — そうです (after stem).
    AppearancePolite,
    /// Looks like — verb dict + みたいだ ("seems like").
    SeemsLike,
    /// Polite SeemsLike — みたいです.
    SeemsLikePolite,
    /// Reportedly — verb dict + らしい ("apparently / I hear").
    Reportedly,

    // ─── Honorific / humble (keigo registers as full forms) ─────────
    /// Honorific construction — お〜になる.
    HonorificOninaru,
    /// Humble construction — お〜する / お〜いたす.
    HumbleOsuru,

    // ─── Misc / compound endings ────────────────────────────────────
    /// Explanatory — のだ / んだ.
    Explanatory,
    /// Explanatory polite — のです / んです.
    ExplanatoryPolite,
    /// Quotative + try — てみよう.
    AttemptVolitional,

    // ─── Stem-only (deconjugator intermediate forms) ────────────────
    /// Bare e-stem (potential / ba root). Internal use only.
    StemE,
    /// Bare a-stem (negative / passive / causative root). Internal.
    StemA,
    /// Bare i-stem (masu / tai root). Internal.
    StemI,
    /// Bare o-stem (volitional root). Internal.
    StemO,
}

impl ConjForm {
    /// Is this a base form (stand-alone, not a chain endpoint)?
    pub fn is_base(self) -> bool {
        matches!(
            self,
            Self::Dictionary
                | Self::StemRenyou
                | Self::StemMizen
                | Self::StemIzen
                | Self::StemMeirei
        )
    }

    /// Does this form carry a negative meaning?
    pub fn is_negative(self) -> bool {
        matches!(
            self,
            Self::Negative
                | Self::NegativePast
                | Self::NegativeTe
                | Self::NegativeBa
                | Self::NegativeNu
                | Self::NegativeZu
                | Self::NegativeWithoutDoing
                | Self::PoliteNegative
                | Self::PoliteNegativePast
                | Self::PotentialNegative
                | Self::VolitionalNegative
                | Self::ImperativeNegative
                | Self::Prohibition
        )
    }

    /// Does this form carry a past / perfective meaning?
    pub fn is_past(self) -> bool {
        matches!(
            self,
            Self::Past
                | Self::PastTara
                | Self::PastTari
                | Self::NegativePast
                | Self::PolitePast
                | Self::PoliteNegativePast
        )
    }

    /// Is this form polite (ます-register)?
    pub fn is_polite(self) -> bool {
        matches!(
            self,
            Self::Polite
                | Self::PolitePast
                | Self::PoliteNegative
                | Self::PoliteNegativePast
                | Self::PoliteTe
                | Self::PoliteVolitional
                | Self::ConjecturalPolite
                | Self::ExplanatoryPolite
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polarity_classifications_are_consistent() {
        // PoliteNegativePast should be classified as both negative AND past.
        assert!(ConjForm::PoliteNegativePast.is_negative());
        assert!(ConjForm::PoliteNegativePast.is_past());
        assert!(ConjForm::PoliteNegativePast.is_polite());

        // Plain Past is past but not negative.
        assert!(ConjForm::Past.is_past());
        assert!(!ConjForm::Past.is_negative());

        // Dictionary is none of polite/past/negative.
        assert!(!ConjForm::Dictionary.is_polite());
        assert!(!ConjForm::Dictionary.is_past());
        assert!(!ConjForm::Dictionary.is_negative());
    }

    #[test]
    fn stems_are_base_forms() {
        for stem in [
            ConjForm::StemRenyou,
            ConjForm::StemMizen,
            ConjForm::StemIzen,
            ConjForm::StemMeirei,
        ] {
            assert!(stem.is_base(), "{:?} should be a base form", stem);
        }
    }
}
