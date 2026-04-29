//! [`Verb`] — the type-based forward conjugation API.
//!
//! Construct a `Verb` from its dictionary form + class and ask for
//! any specific conjugated form via a method.
//!
//! ## Stem terminology
//!
//! Japanese conjugation hangs off four grammatical stems:
//!
//! | Name (English) | Name (Japanese) | Used for |
//! |---|---|---|
//! | nai-stem | 未然形 (mizenkei) | negative, passive, causative, volitional |
//! | masu-stem | 連用形 (renyoukei) | polite, tai, te-form (after sound changes) |
//! | dictionary-stem | 終止形 (shuushikei) | dictionary form, attributive |
//! | ba-stem | 已然形 (izenkei) | conditional ば, potential |
//! | imperative-stem | 命令形 (meireikei) | imperative |
//!
//! For ichidan verbs (drop る then add suffix) all stems are the
//! same — just the dict form minus `る`. For godan verbs, each stem
//! lives in a different vowel row of the gojuuon table.
//!
//! ## Te / past sound change (音便)
//!
//! Godan verbs undergo sound changes when forming te / past:
//!
//! | Class | Stem-i | te-form | past-form |
//! |---|---|---|---|
//! | GodanU | 買い | 買って | 買った |
//! | GodanTsu | 待ち | 待って | 待った |
//! | GodanRu | 走り | 走って | 走った |
//! | GodanKu | 書き | 書いて | 書いた |
//! | GodanGu | 泳ぎ | 泳いで | 泳いだ |
//! | GodanMu | 飲み | 飲んで | 飲んだ |
//! | GodanBu | 飛び | 飛んで | 飛んだ |
//! | GodanNu | 死に | 死んで | 死んだ |
//! | GodanSu | 話し | 話して | 話した |
//! | GodanKuIku (irreg) | 行き | 行って | 行った |
//! | GodanUSpecial (irreg) | 請い | 請うて | 請うた |

use crate::kana::{append, replace_last_char, shift_godan_terminal, split_last_char, VowelRow};
use crate::tag::ConjForm;
use crate::verb_class::VerbClass;

/// A Japanese verb identified by its dictionary form + paradigm
/// class. All conjugation methods return owned `Conjugated` values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verb {
    pub dict_form: String,
    pub class: VerbClass,
}

/// One conjugated form: the surface string plus its derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conjugated {
    pub surface: String,
    pub form: ConjForm,
}

impl Verb {
    /// Construct a verb. Caller is responsible for getting `class`
    /// right (typically from JMdict POS tags).
    pub fn new(dict_form: impl Into<String>, class: VerbClass) -> Self {
        Self {
            dict_form: dict_form.into(),
            class,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Stem accessors
    // ─────────────────────────────────────────────────────────────────

    /// Dictionary stem — for ichidan, the part before る; for godan,
    /// the entire dict form. Used as the prefix for almost every
    /// conjugation operation.
    fn root(&self) -> String {
        if self.class.is_ichidan() {
            // Drop the trailing る.
            split_last_char(&self.dict_form)
                .map(|(prefix, _)| prefix)
                .unwrap_or_default()
        } else {
            // Godan / irregular: the dict form minus its terminal kana
            // is the consonant-less root.
            split_last_char(&self.dict_form)
                .map(|(prefix, _)| prefix)
                .unwrap_or_default()
        }
    }

    /// nai-stem (未然形). Used as base for negative, passive,
    /// causative, volitional.
    pub fn stem_mizen(&self) -> String {
        if self.class.is_ichidan() {
            self.root()
        } else if let Some(terminal) = self.class.terminal_kana() {
            let a_kana = shift_godan_terminal(terminal, VowelRow::A)
                .expect("terminal_kana should always be a valid godan ending");
            append(&self.root(), &a_kana.to_string())
        } else {
            // Irregulars handled in irregular.rs
            String::new()
        }
    }

    /// masu-stem (連用形). Used for polite, tai, ren'youkei
    /// compounds.
    pub fn stem_renyou(&self) -> String {
        if self.class.is_ichidan() {
            self.root()
        } else if let Some(terminal) = self.class.terminal_kana() {
            let i_kana = shift_godan_terminal(terminal, VowelRow::I)
                .expect("terminal_kana should always be a valid godan ending");
            append(&self.root(), &i_kana.to_string())
        } else {
            String::new()
        }
    }

    /// e-stem (已然形). Used for ba-conditional and potential.
    pub fn stem_izen(&self) -> String {
        if self.class.is_ichidan() {
            self.root()
        } else if let Some(terminal) = self.class.terminal_kana() {
            let e_kana = shift_godan_terminal(terminal, VowelRow::E)
                .expect("terminal_kana should always be a valid godan ending");
            append(&self.root(), &e_kana.to_string())
        } else {
            String::new()
        }
    }

    /// o-stem (used for volitional おう/よう).
    pub fn stem_o(&self) -> String {
        if self.class.is_ichidan() {
            self.root()
        } else if let Some(terminal) = self.class.terminal_kana() {
            let o_kana = shift_godan_terminal(terminal, VowelRow::O)
                .expect("terminal_kana should always be a valid godan ending");
            append(&self.root(), &o_kana.to_string())
        } else {
            String::new()
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Dictionary form (passthrough)
    // ─────────────────────────────────────────────────────────────────

    /// Dictionary (citation) form.
    pub fn dictionary(&self) -> Conjugated {
        Conjugated {
            surface: self.dict_form.clone(),
            form: ConjForm::Dictionary,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Negative
    // ─────────────────────────────────────────────────────────────────

    /// Plain negative — ない. ichidan: stem + ない. godan: a-stem +
    /// ない. Special-cases ある → ない.
    pub fn negative(&self) -> Conjugated {
        let surface = match self.class {
            VerbClass::GodanRuAru => {
                // ある → ない (irregular). Other v5r-i verbs (ござる
                // etc.) follow regular godan: ござら + ない.
                if self.dict_form == "ある" {
                    "ない".to_string()
                } else {
                    append(&self.stem_mizen(), "ない")
                }
            }
            _ => append(&self.stem_mizen(), "ない"),
        };
        Conjugated {
            surface,
            form: ConjForm::Negative,
        }
    }

    /// Plain negative past — なかった. Built from negative by
    /// applying i-adj past rule (い → かった).
    pub fn negative_past(&self) -> Conjugated {
        let neg = self.negative().surface;
        // Strip trailing い from なかった etc., then apply かった.
        let past = if neg.ends_with('い') {
            let (prefix, _) = split_last_char(&neg).unwrap();
            append(&prefix, "かった")
        } else {
            // Shouldn't happen for well-formed verbs.
            append(&neg, "かった")
        };
        Conjugated {
            surface: past,
            form: ConjForm::NegativePast,
        }
    }

    /// Negative te-form — なくて.
    pub fn negative_te(&self) -> Conjugated {
        let neg = self.negative().surface;
        let surface = if neg.ends_with('い') {
            let (prefix, _) = split_last_char(&neg).unwrap();
            append(&prefix, "くて")
        } else {
            append(&neg, "くて")
        };
        Conjugated {
            surface,
            form: ConjForm::NegativeTe,
        }
    }

    /// Negative ba-conditional — なければ.
    pub fn negative_ba(&self) -> Conjugated {
        let neg = self.negative().surface;
        let surface = if neg.ends_with('い') {
            let (prefix, _) = split_last_char(&neg).unwrap();
            append(&prefix, "ければ")
        } else {
            append(&neg, "ければ")
        };
        Conjugated {
            surface,
            form: ConjForm::NegativeBa,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Past / Te-form (音便 sound changes)
    // ─────────────────────────────────────────────────────────────────

    /// Plain past — た / だ.
    pub fn past(&self) -> Conjugated {
        let surface = self.te_or_past_inner(PastOrTe::Past);
        Conjugated {
            surface,
            form: ConjForm::Past,
        }
    }

    /// Plain te-form — て / で.
    pub fn te_form(&self) -> Conjugated {
        let surface = self.te_or_past_inner(PastOrTe::Te);
        Conjugated {
            surface,
            form: ConjForm::Te,
        }
    }

    fn te_or_past_inner(&self, kind: PastOrTe) -> String {
        let (te, past) = match self.class {
            // Ichidan: stem + て / た.
            VerbClass::Ichidan | VerbClass::IchidanKureru => {
                return append(&self.root(), kind.suffix("て", "た"));
            }
            VerbClass::Zuru => {
                // 信ずる → 信じて / 信じた (becomes ichidan in modern usage).
                let (prefix, _) = split_last_char(&self.dict_form).unwrap();
                let prefix = if prefix.ends_with('ず') {
                    replace_last_char(&prefix, 'じ')
                } else {
                    prefix
                };
                return append(&prefix, kind.suffix("て", "た"));
            }
            VerbClass::GodanU | VerbClass::GodanTsu | VerbClass::GodanRu | VerbClass::GodanRuAru
            | VerbClass::GodanAru => ("って", "った"),
            VerbClass::GodanKu => ("いて", "いた"),
            VerbClass::GodanKuIku => ("って", "った"), // 行く → 行って / 行った (irregular)
            VerbClass::GodanGu => ("いで", "いだ"),
            VerbClass::GodanMu | VerbClass::GodanBu | VerbClass::GodanNu => ("んで", "んだ"),
            VerbClass::GodanSu => ("して", "した"),
            VerbClass::GodanUSpecial => ("うて", "うた"),
            VerbClass::NuVerbClassical => ("んで", "んだ"),
            VerbClass::RuVerbClassical | VerbClass::YodanRu => ("って", "った"),
            // Irregulars (Suru, Kuru) handled in irregular module.
            _ => return String::new(),
        };
        let suffix = kind.suffix(te, past);
        // Special-case 行く/行きます: drop last char of dict form, add った/って.
        if matches!(self.class, VerbClass::GodanKuIku) {
            return append(&self.root(), suffix);
        }
        // Normal godan: drop the terminal kana, add the te/past suffix.
        append(&self.root(), suffix)
    }

    // ─────────────────────────────────────────────────────────────────
    // Polite (ます-form)
    // ─────────────────────────────────────────────────────────────────

    /// Polite — ます.
    pub fn polite(&self) -> Conjugated {
        // GodanAru irregular: なさる → なさいます (stem-i is なさい).
        let stem = if matches!(self.class, VerbClass::GodanAru) {
            self.godan_aru_polite_stem()
        } else {
            self.stem_renyou()
        };
        Conjugated {
            surface: append(&stem, "ます"),
            form: ConjForm::Polite,
        }
    }

    /// Polite past — ました.
    pub fn polite_past(&self) -> Conjugated {
        let stem = if matches!(self.class, VerbClass::GodanAru) {
            self.godan_aru_polite_stem()
        } else {
            self.stem_renyou()
        };
        Conjugated {
            surface: append(&stem, "ました"),
            form: ConjForm::PolitePast,
        }
    }

    /// Polite negative — ません.
    pub fn polite_negative(&self) -> Conjugated {
        let stem = if matches!(self.class, VerbClass::GodanAru) {
            self.godan_aru_polite_stem()
        } else {
            self.stem_renyou()
        };
        Conjugated {
            surface: append(&stem, "ません"),
            form: ConjForm::PoliteNegative,
        }
    }

    /// Polite negative past — ませんでした.
    pub fn polite_negative_past(&self) -> Conjugated {
        let stem = if matches!(self.class, VerbClass::GodanAru) {
            self.godan_aru_polite_stem()
        } else {
            self.stem_renyou()
        };
        Conjugated {
            surface: append(&stem, "ませんでした"),
            form: ConjForm::PoliteNegativePast,
        }
    }

    /// Polite te-form — まして.
    pub fn polite_te(&self) -> Conjugated {
        let stem = self.stem_renyou();
        Conjugated {
            surface: append(&stem, "まして"),
            form: ConjForm::PoliteTe,
        }
    }

    /// Polite volitional — ましょう.
    pub fn polite_volitional(&self) -> Conjugated {
        let stem = self.stem_renyou();
        Conjugated {
            surface: append(&stem, "ましょう"),
            form: ConjForm::PoliteVolitional,
        }
    }

    /// GodanAru polite stem: なさる → なさい, くださる → ください,
    /// ござる → ござい (the standard godan-i would give なさり,
    /// くださり — wrong).
    fn godan_aru_polite_stem(&self) -> String {
        // Replace the terminal る with い.
        replace_last_char(&self.dict_form, 'い')
    }

    // ─────────────────────────────────────────────────────────────────
    // Voice — Causative / Passive / Potential
    // ─────────────────────────────────────────────────────────────────

    /// Causative — godan: a-stem + せる, ichidan: stem + させる.
    pub fn causative(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "させる")
        } else {
            append(&self.stem_mizen(), "せる")
        };
        Conjugated {
            surface,
            form: ConjForm::Causative,
        }
    }

    /// Passive — godan: a-stem + れる, ichidan: stem + られる.
    pub fn passive(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "られる")
        } else {
            append(&self.stem_mizen(), "れる")
        };
        Conjugated {
            surface,
            form: ConjForm::Passive,
        }
    }

    /// Causative-passive — godan: a-stem + せられる (or short: a-stem
    /// + される), ichidan: stem + させられる.
    pub fn causative_passive(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "させられる")
        } else {
            append(&self.stem_mizen(), "せられる")
        };
        Conjugated {
            surface,
            form: ConjForm::CausativePassive,
        }
    }

    /// Potential — ichidan: stem + られる. godan: e-stem + る
    /// (書く → 書ける, 読む → 読める).
    pub fn potential(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "られる")
        } else {
            append(&self.stem_izen(), "る")
        };
        Conjugated {
            surface,
            form: ConjForm::Potential,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Volitional / imperative / conditionals
    // ─────────────────────────────────────────────────────────────────

    /// Plain volitional — ichidan: stem + よう, godan: o-stem + う.
    pub fn volitional(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "よう")
        } else {
            append(&self.stem_o(), "う")
        };
        Conjugated {
            surface,
            form: ConjForm::Volitional,
        }
    }

    /// Imperative — ichidan: stem + ろ, godan: e-stem (書け, 走れ).
    /// くれる irregular: くれ.
    pub fn imperative(&self) -> Conjugated {
        let surface = match self.class {
            VerbClass::IchidanKureru => self.root(),
            VerbClass::Ichidan => append(&self.root(), "ろ"),
            _ if self.class.is_godan() => self.stem_izen(),
            _ => String::new(),
        };
        Conjugated {
            surface,
            form: ConjForm::Imperative,
        }
    }

    /// Negative imperative — verb + な (行くな).
    pub fn imperative_negative(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "な"),
            form: ConjForm::ImperativeNegative,
        }
    }

    /// Conditional ば — e-stem + ば (走れば, 食べれば).
    pub fn conditional_ba(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "れば")
        } else {
            append(&self.stem_izen(), "ば")
        };
        Conjugated {
            surface,
            form: ConjForm::ConditionalBa,
        }
    }

    /// Conditional たら — past + ら (走ったら, 食べたら).
    pub fn conditional_tara(&self) -> Conjugated {
        let past = self.past().surface;
        Conjugated {
            surface: append(&past, "ら"),
            form: ConjForm::ConditionalTara,
        }
    }

    /// Provisional なら — dict form + なら (行くなら, 食べるなら).
    pub fn provisional_nara(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "なら"),
            form: ConjForm::ProvisionalNara,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Desiderative
    // ─────────────────────────────────────────────────────────────────

    /// First-person desiderative — i-stem + たい (食べたい, 走りたい).
    pub fn desiderative(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.stem_renyou(), "たい"),
            form: ConjForm::Desiderative,
        }
    }

    /// Third-person desiderative — i-stem + たがる (食べたがる).
    pub fn desiderative_other(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.stem_renyou(), "たがる"),
            form: ConjForm::DesiderativeOther,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Te-form auxiliary chains
    // ─────────────────────────────────────────────────────────────────

    /// Progressive — te-form + いる (食べている, 走っている).
    pub fn progressive(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "いる"),
            form: ConjForm::Progressive,
        }
    }

    /// Progressive (contracted) — te-form + る (食べてる, 走ってる).
    pub fn progressive_contracted(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "る"),
            form: ConjForm::ProgressiveContracted,
        }
    }

    /// Resultative — te-form + ある (置いてある).
    pub fn resultative(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "ある"),
            form: ConjForm::Resultative,
        }
    }

    /// Preparative — te-form + おく (買っておく).
    pub fn preparative(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "おく"),
            form: ConjForm::Preparative,
        }
    }

    /// Attempt — te-form + みる (食べてみる).
    pub fn attempt(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "みる"),
            form: ConjForm::Attempt,
        }
    }

    /// Completion — te-form + しまう (食べてしまう).
    pub fn completion(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "しまう"),
            form: ConjForm::Completion,
        }
    }

    /// Convenience: dispatch to a specific form by tag.
    /// Routes irregular classes (Suru, SuruCompound, Kuru) through
    /// the per-form lookup table; everything else through the
    /// rule-driven methods on this struct.
    pub fn conjugate(&self, form: ConjForm) -> Option<Conjugated> {
        if self.class.is_irregular() {
            return crate::irregular::lookup_irregular(&self.dict_form, self.class, form);
        }
        Some(match form {
            ConjForm::Dictionary => self.dictionary(),
            ConjForm::Negative => self.negative(),
            ConjForm::NegativePast => self.negative_past(),
            ConjForm::NegativeTe => self.negative_te(),
            ConjForm::NegativeBa => self.negative_ba(),
            ConjForm::Past => self.past(),
            ConjForm::Te => self.te_form(),
            ConjForm::Polite => self.polite(),
            ConjForm::PolitePast => self.polite_past(),
            ConjForm::PoliteNegative => self.polite_negative(),
            ConjForm::PoliteNegativePast => self.polite_negative_past(),
            ConjForm::PoliteTe => self.polite_te(),
            ConjForm::PoliteVolitional => self.polite_volitional(),
            ConjForm::Causative => self.causative(),
            ConjForm::Passive => self.passive(),
            ConjForm::CausativePassive => self.causative_passive(),
            ConjForm::Potential => self.potential(),
            ConjForm::Volitional => self.volitional(),
            ConjForm::Imperative => self.imperative(),
            ConjForm::ImperativeNegative => self.imperative_negative(),
            ConjForm::ConditionalBa => self.conditional_ba(),
            ConjForm::ConditionalTara => self.conditional_tara(),
            ConjForm::ProvisionalNara => self.provisional_nara(),
            ConjForm::Desiderative => self.desiderative(),
            ConjForm::DesiderativeOther => self.desiderative_other(),
            ConjForm::Progressive => self.progressive(),
            ConjForm::ProgressiveContracted => self.progressive_contracted(),
            ConjForm::Resultative => self.resultative(),
            ConjForm::Preparative => self.preparative(),
            ConjForm::Attempt => self.attempt(),
            ConjForm::Completion => self.completion(),
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum PastOrTe {
    Past,
    Te,
}

impl PastOrTe {
    fn suffix<'a>(self, te: &'a str, past: &'a str) -> &'a str {
        match self {
            Self::Past => past,
            Self::Te => te,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: build a verb and assert a specific conjugation.
    fn check(dict: &str, class: VerbClass, form: ConjForm, expected: &str) {
        let v = Verb::new(dict, class);
        let c = v.conjugate(form).unwrap_or_else(|| {
            panic!("conjugate({:?}) returned None for {} ({:?})", form, dict, class)
        });
        assert_eq!(
            c.surface, expected,
            "{} ({:?}) {:?}: expected {}, got {}",
            dict, class, form, expected, c.surface
        );
    }

    // ─── Ichidan ─────────────────────────────────────────────────────

    #[test]
    fn ichidan_taberu_full_paradigm() {
        check("食べる", VerbClass::Ichidan, ConjForm::Negative, "食べない");
        check("食べる", VerbClass::Ichidan, ConjForm::Past, "食べた");
        check("食べる", VerbClass::Ichidan, ConjForm::Te, "食べて");
        check("食べる", VerbClass::Ichidan, ConjForm::Polite, "食べます");
        check("食べる", VerbClass::Ichidan, ConjForm::PolitePast, "食べました");
        check("食べる", VerbClass::Ichidan, ConjForm::PoliteNegative, "食べません");
        check("食べる", VerbClass::Ichidan, ConjForm::PoliteNegativePast, "食べませんでした");
        check("食べる", VerbClass::Ichidan, ConjForm::Causative, "食べさせる");
        check("食べる", VerbClass::Ichidan, ConjForm::Passive, "食べられる");
        check("食べる", VerbClass::Ichidan, ConjForm::CausativePassive, "食べさせられる");
        check("食べる", VerbClass::Ichidan, ConjForm::Potential, "食べられる");
        check("食べる", VerbClass::Ichidan, ConjForm::Volitional, "食べよう");
        check("食べる", VerbClass::Ichidan, ConjForm::Imperative, "食べろ");
        check("食べる", VerbClass::Ichidan, ConjForm::ConditionalBa, "食べれば");
        check("食べる", VerbClass::Ichidan, ConjForm::ConditionalTara, "食べたら");
        check("食べる", VerbClass::Ichidan, ConjForm::Desiderative, "食べたい");
        check("食べる", VerbClass::Ichidan, ConjForm::Progressive, "食べている");
        check("食べる", VerbClass::Ichidan, ConjForm::Completion, "食べてしまう");
        check("食べる", VerbClass::Ichidan, ConjForm::NegativePast, "食べなかった");
        check("食べる", VerbClass::Ichidan, ConjForm::NegativeTe, "食べなくて");
        check("食べる", VerbClass::Ichidan, ConjForm::NegativeBa, "食べなければ");
    }

    // ─── Godan classes — one canonical verb each ─────────────────────

    #[test]
    fn godan_kaku_paradigm() {
        check("書く", VerbClass::GodanKu, ConjForm::Negative, "書かない");
        check("書く", VerbClass::GodanKu, ConjForm::Past, "書いた");
        check("書く", VerbClass::GodanKu, ConjForm::Te, "書いて");
        check("書く", VerbClass::GodanKu, ConjForm::Polite, "書きます");
        check("書く", VerbClass::GodanKu, ConjForm::Potential, "書ける");
        check("書く", VerbClass::GodanKu, ConjForm::Volitional, "書こう");
        check("書く", VerbClass::GodanKu, ConjForm::Imperative, "書け");
        check("書く", VerbClass::GodanKu, ConjForm::ConditionalBa, "書けば");
        check("書く", VerbClass::GodanKu, ConjForm::Desiderative, "書きたい");
    }

    #[test]
    fn godan_oyogu_paradigm() {
        check("泳ぐ", VerbClass::GodanGu, ConjForm::Past, "泳いだ");
        check("泳ぐ", VerbClass::GodanGu, ConjForm::Te, "泳いで");
        check("泳ぐ", VerbClass::GodanGu, ConjForm::Negative, "泳がない");
        check("泳ぐ", VerbClass::GodanGu, ConjForm::Volitional, "泳ごう");
    }

    #[test]
    fn godan_hashiru_paradigm() {
        check("走る", VerbClass::GodanRu, ConjForm::Past, "走った");
        check("走る", VerbClass::GodanRu, ConjForm::Te, "走って");
        check("走る", VerbClass::GodanRu, ConjForm::Negative, "走らない");
        check("走る", VerbClass::GodanRu, ConjForm::Polite, "走ります");
        check("走る", VerbClass::GodanRu, ConjForm::Potential, "走れる");
        check("走る", VerbClass::GodanRu, ConjForm::ConditionalBa, "走れば");
    }

    #[test]
    fn godan_kau_paradigm() {
        check("買う", VerbClass::GodanU, ConjForm::Past, "買った");
        check("買う", VerbClass::GodanU, ConjForm::Te, "買って");
        check("買う", VerbClass::GodanU, ConjForm::Negative, "買わない");
        check("買う", VerbClass::GodanU, ConjForm::Volitional, "買おう");
    }

    #[test]
    fn godan_motsu_paradigm() {
        check("持つ", VerbClass::GodanTsu, ConjForm::Past, "持った");
        check("持つ", VerbClass::GodanTsu, ConjForm::Te, "持って");
        check("持つ", VerbClass::GodanTsu, ConjForm::Negative, "持たない");
    }

    #[test]
    fn godan_nomu_paradigm() {
        check("飲む", VerbClass::GodanMu, ConjForm::Past, "飲んだ");
        check("飲む", VerbClass::GodanMu, ConjForm::Te, "飲んで");
        check("飲む", VerbClass::GodanMu, ConjForm::Negative, "飲まない");
    }

    #[test]
    fn godan_tobu_paradigm() {
        check("飛ぶ", VerbClass::GodanBu, ConjForm::Past, "飛んだ");
        check("飛ぶ", VerbClass::GodanBu, ConjForm::Te, "飛んで");
    }

    #[test]
    fn godan_shinu_paradigm() {
        check("死ぬ", VerbClass::GodanNu, ConjForm::Past, "死んだ");
        check("死ぬ", VerbClass::GodanNu, ConjForm::Te, "死んで");
    }

    #[test]
    fn godan_hanasu_paradigm() {
        check("話す", VerbClass::GodanSu, ConjForm::Past, "話した");
        check("話す", VerbClass::GodanSu, ConjForm::Te, "話して");
        check("話す", VerbClass::GodanSu, ConjForm::Negative, "話さない");
    }

    // ─── Special cases ───────────────────────────────────────────────

    #[test]
    fn godan_ku_iku_irregular_past() {
        // 行く's past is 行った (not 行いた as regular GodanKu would give).
        check("行く", VerbClass::GodanKuIku, ConjForm::Past, "行った");
        check("行く", VerbClass::GodanKuIku, ConjForm::Te, "行って");
    }

    #[test]
    fn godan_ru_aru_irregular_negative() {
        // ある's negative is ない (not あらない).
        check("ある", VerbClass::GodanRuAru, ConjForm::Negative, "ない");
        // ある's other forms are regular godan-r.
        check("ある", VerbClass::GodanRuAru, ConjForm::Past, "あった");
        check("ある", VerbClass::GodanRuAru, ConjForm::Polite, "あります");
    }

    #[test]
    fn godan_aru_polite_irregular() {
        // なさる → なさいます (irregular i-stem).
        check("なさる", VerbClass::GodanAru, ConjForm::Polite, "なさいます");
        check("くださる", VerbClass::GodanAru, ConjForm::Polite, "くださいます");
        check("おっしゃる", VerbClass::GodanAru, ConjForm::Polite, "おっしゃいます");
    }

    #[test]
    fn ichidan_kureru_imperative_irregular() {
        // くれる's imperative is くれ (not くれろ).
        check("くれる", VerbClass::IchidanKureru, ConjForm::Imperative, "くれ");
    }
}
