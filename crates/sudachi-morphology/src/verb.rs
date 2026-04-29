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
use crate::HonorificPrefix;

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

    /// Movement-toward — te-form + くる (歩いてくる).
    pub fn coming_to(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "くる"),
            form: ConjForm::ComingTo,
        }
    }

    /// Movement-away — te-form + いく (歩いていく).
    pub fn going_to(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "いく"),
            form: ConjForm::GoingTo,
        }
    }

    /// Receiving favour — te-form + もらう (教えてもらう).
    pub fn receiving(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "もらう"),
            form: ConjForm::Receiving,
        }
    }

    /// Giving favour outward — te-form + くれる (教えてくれる).
    pub fn giving_outward(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "くれる"),
            form: ConjForm::GivingOutward,
        }
    }

    /// Giving favour to other — te-form + あげる (教えてあげる).
    pub fn giving_to_other(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "あげる"),
            form: ConjForm::GivingToOther,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Volitional / conjectural extensions
    // ─────────────────────────────────────────────────────────────────

    /// Negative volitional — verb + まい (行くまい). Plain dict form
    /// + まい is the modern usage; classical needed stem inflections.
    pub fn volitional_negative(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "まい"),
            form: ConjForm::VolitionalNegative,
        }
    }

    /// Conjectural — verb + だろう (行くだろう, 食べるだろう).
    pub fn conjectural(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "だろう"),
            form: ConjForm::Conjectural,
        }
    }

    /// Polite conjectural — verb + でしょう (行くでしょう).
    pub fn conjectural_polite(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "でしょう"),
            form: ConjForm::ConjecturalPolite,
        }
    }

    /// Volitional + attempt — てみよう (食べてみよう "let me try
    /// eating"). Composed: te-form + みよう.
    pub fn attempt_volitional(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "みよう"),
            form: ConjForm::AttemptVolitional,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Voice extensions
    // ─────────────────────────────────────────────────────────────────

    /// Short causative — godan: a-stem + す (書かす, 読ます). For
    /// ichidan the same as full causative + reduction (食べさす).
    pub fn causative_short(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "さす")
        } else {
            append(&self.stem_mizen(), "す")
        };
        Conjugated {
            surface,
            form: ConjForm::CausativeShort,
        }
    }

    /// Honorific — same surface as Passive, semantically distinct
    /// (尊敬語 reading of れる/られる).
    pub fn honorific(&self) -> Conjugated {
        let p = self.passive();
        Conjugated {
            surface: p.surface,
            form: ConjForm::Honorific,
        }
    }

    /// Negative potential — ichidan: stem + られない. godan: e-stem +
    /// ない (書けない, 読めない).
    pub fn potential_negative(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "られない")
        } else {
            append(&self.stem_izen(), "ない")
        };
        Conjugated {
            surface,
            form: ConjForm::PotentialNegative,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Negative variants
    // ─────────────────────────────────────────────────────────────────

    /// Classical negative — a-stem + ぬ (行かぬ, 食べぬ).
    pub fn negative_nu(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "ぬ")
        } else {
            append(&self.stem_mizen(), "ぬ")
        };
        Conjugated {
            surface,
            form: ConjForm::NegativeNu,
        }
    }

    /// Classical negative continuative — a-stem + ず (行かず, 食べず).
    pub fn negative_zu(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "ず")
        } else {
            append(&self.stem_mizen(), "ず")
        };
        Conjugated {
            surface,
            form: ConjForm::NegativeZu,
        }
    }

    /// Negative continuative without doing — a-stem + ずに (行かずに).
    pub fn negative_without_doing(&self) -> Conjugated {
        let surface = if self.class.is_ichidan() {
            append(&self.root(), "ずに")
        } else {
            append(&self.stem_mizen(), "ずに")
        };
        Conjugated {
            surface,
            form: ConjForm::NegativeWithoutDoing,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Past variants
    // ─────────────────────────────────────────────────────────────────

    /// Past tara conditional — alias for [`Self::conditional_tara`].
    /// JMdict / nazeka treats this as a distinct form so we expose
    /// the alias to keep the ConjForm dispatch closed.
    pub fn past_tara(&self) -> Conjugated {
        let mut c = self.conditional_tara();
        c.form = ConjForm::PastTara;
        c
    }

    /// Past tari — past + り (走ったり, 食べたり). Used for listing.
    pub fn past_tari(&self) -> Conjugated {
        let surface = append(&self.past().surface, "り");
        Conjugated {
            surface,
            form: ConjForm::PastTari,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Modal compounds
    // ─────────────────────────────────────────────────────────────────

    /// Obligation — negative_ba + ならない (行かなければならない).
    pub fn obligation(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.negative_ba().surface, "ならない"),
            form: ConjForm::Obligation,
        }
    }

    /// Permission — te-form + もいい (食べてもいい).
    pub fn permission(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "もいい"),
            form: ConjForm::Permission,
        }
    }

    /// Prohibition — te-form + はいけない (食べてはいけない).
    pub fn prohibition(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.te_form().surface, "はいけない"),
            form: ConjForm::Prohibition,
        }
    }

    /// Recommendation — dict + べき (行くべき). べし for classical
    /// usage is just dict + べし; we use べき as the standard modern
    /// attributive form.
    pub fn recommendation(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "べき"),
            form: ConjForm::Recommendation,
        }
    }

    /// Hearsay — dict + そうだ (行くそうだ "I heard he's going").
    /// Distinct from Appearance — Hearsay attaches to dict form,
    /// Appearance to the stem.
    pub fn hearsay(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "そうだ"),
            form: ConjForm::Hearsay,
        }
    }

    /// Polite hearsay — dict + そうです (行くそうです).
    pub fn hearsay_polite(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "そうです"),
            form: ConjForm::HearsayPolite,
        }
    }

    /// Appearance — i-stem + そうだ (行きそうだ "looks like he's
    /// going"). Distinct from Hearsay — see [`Self::hearsay`].
    pub fn appearance(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.stem_renyou(), "そうだ"),
            form: ConjForm::Appearance,
        }
    }

    /// Polite appearance — i-stem + そうです (行きそうです).
    pub fn appearance_polite(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.stem_renyou(), "そうです"),
            form: ConjForm::AppearancePolite,
        }
    }

    /// Looks-like — dict + みたいだ (行くみたいだ "seems like
    /// he's going"). Less formal than Hearsay; speaker's inference.
    pub fn seems_like(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "みたいだ"),
            form: ConjForm::SeemsLike,
        }
    }

    /// Polite seems-like — dict + みたいです.
    pub fn seems_like_polite(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "みたいです"),
            form: ConjForm::SeemsLikePolite,
        }
    }

    /// Reportedly — dict + らしい ("apparently / I hear that").
    /// Slightly more reliable than みたい; based on hearsay.
    pub fn reportedly(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "らしい"),
            form: ConjForm::Reportedly,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Honorific / humble (keigo)
    // ─────────────────────────────────────────────────────────────────

    /// Honorific construction — お + i-stem + になる (お読みになる).
    /// Defaults to the お prefix (wago verbs are the common case).
    /// For Sino-Japanese (kango) verbs, use
    /// [`Self::honorific_oninaru_with_prefix`] with [`HonorificPrefix::Go`].
    pub fn honorific_oninaru(&self) -> Conjugated {
        self.honorific_oninaru_with_prefix(HonorificPrefix::O)
    }

    /// Honorific construction with explicit prefix selection.
    /// Use [`HonorificPrefix::O`] for native (wago) verbs and
    /// [`HonorificPrefix::Go`] for Sino-Japanese (kango) — e.g.,
    /// ご説明になる, ご報告になる.
    pub fn honorific_oninaru_with_prefix(&self, prefix: HonorificPrefix) -> Conjugated {
        Conjugated {
            surface: format!("{}{}になる", prefix.surface(), self.stem_renyou()),
            form: ConjForm::HonorificOninaru,
        }
    }

    /// Humble construction — お + i-stem + する (お読みする).
    /// Defaults to お; see [`Self::humble_osuru_with_prefix`].
    pub fn humble_osuru(&self) -> Conjugated {
        self.humble_osuru_with_prefix(HonorificPrefix::O)
    }

    /// Humble construction with explicit prefix selection.
    /// Use [`HonorificPrefix::Go`] for kango verbs (ご報告する,
    /// ご説明する).
    pub fn humble_osuru_with_prefix(&self, prefix: HonorificPrefix) -> Conjugated {
        Conjugated {
            surface: format!("{}{}する", prefix.surface(), self.stem_renyou()),
            form: ConjForm::HumbleOsuru,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Explanatory のだ / のです
    // ─────────────────────────────────────────────────────────────────

    /// Explanatory — dict + のだ (行くのだ). Casual contraction is
    /// んだ; we emit the long form here.
    pub fn explanatory(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "のだ"),
            form: ConjForm::Explanatory,
        }
    }

    /// Polite explanatory — dict + のです.
    pub fn explanatory_polite(&self) -> Conjugated {
        Conjugated {
            surface: append(&self.dict_form, "のです"),
            form: ConjForm::ExplanatoryPolite,
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Stems (exposed via ConjForm dispatch)
    // ─────────────────────────────────────────────────────────────────

    /// Mizen-stem (未然形) — bare a-stem (or ichidan root). Used by
    /// negative / passive / causative chains.
    pub fn mizen_stem(&self) -> Conjugated {
        Conjugated {
            surface: self.stem_mizen(),
            form: ConjForm::StemMizen,
        }
    }

    /// Renyou-stem (連用形) — bare i-stem (or ichidan root). Used
    /// by polite / tai / appearance chains. Same as the masu-stem.
    pub fn renyou_stem(&self) -> Conjugated {
        Conjugated {
            surface: self.stem_renyou(),
            form: ConjForm::StemRenyou,
        }
    }

    /// Izen-stem (已然形) — bare e-stem (or ichidan root). Used by
    /// conditional ば and potential.
    pub fn izen_stem(&self) -> Conjugated {
        Conjugated {
            surface: self.stem_izen(),
            form: ConjForm::StemIzen,
        }
    }

    /// Meirei-stem (命令形) — surface of imperative.
    pub fn meirei_stem(&self) -> Conjugated {
        let mut c = self.imperative();
        c.form = ConjForm::StemMeirei;
        c
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
            ConjForm::ComingTo => self.coming_to(),
            ConjForm::GoingTo => self.going_to(),
            ConjForm::Receiving => self.receiving(),
            ConjForm::GivingOutward => self.giving_outward(),
            ConjForm::GivingToOther => self.giving_to_other(),
            ConjForm::VolitionalNegative => self.volitional_negative(),
            ConjForm::Conjectural => self.conjectural(),
            ConjForm::ConjecturalPolite => self.conjectural_polite(),
            ConjForm::AttemptVolitional => self.attempt_volitional(),
            ConjForm::CausativeShort => self.causative_short(),
            ConjForm::Honorific => self.honorific(),
            ConjForm::PotentialNegative => self.potential_negative(),
            ConjForm::NegativeNu => self.negative_nu(),
            ConjForm::NegativeZu => self.negative_zu(),
            ConjForm::NegativeWithoutDoing => self.negative_without_doing(),
            ConjForm::PastTara => self.past_tara(),
            ConjForm::PastTari => self.past_tari(),
            ConjForm::Obligation => self.obligation(),
            ConjForm::Permission => self.permission(),
            ConjForm::Prohibition => self.prohibition(),
            ConjForm::Recommendation => self.recommendation(),
            ConjForm::Hearsay => self.hearsay(),
            ConjForm::HearsayPolite => self.hearsay_polite(),
            ConjForm::Appearance => self.appearance(),
            ConjForm::AppearancePolite => self.appearance_polite(),
            ConjForm::SeemsLike => self.seems_like(),
            ConjForm::SeemsLikePolite => self.seems_like_polite(),
            ConjForm::Reportedly => self.reportedly(),
            ConjForm::HonorificOninaru => self.honorific_oninaru(),
            ConjForm::HumbleOsuru => self.humble_osuru(),
            ConjForm::Explanatory => self.explanatory(),
            ConjForm::ExplanatoryPolite => self.explanatory_polite(),
            ConjForm::StemMizen => self.mizen_stem(),
            ConjForm::StemRenyou => self.renyou_stem(),
            ConjForm::StemIzen => self.izen_stem(),
            ConjForm::StemMeirei => self.meirei_stem(),
            // ConjForm::StemA, StemE, StemI, StemO are deconjugator-internal
            // intermediate forms; not exposed via forward Verb dispatch.
            ConjForm::StemA | ConjForm::StemE | ConjForm::StemI | ConjForm::StemO => return None,
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

    // ─── Te-aux chains (新) ───────────────────────────────────────────

    #[test]
    fn te_chain_directional_and_giving() {
        check("食べる", VerbClass::Ichidan, ConjForm::ComingTo, "食べてくる");
        check("食べる", VerbClass::Ichidan, ConjForm::GoingTo, "食べていく");
        check("食べる", VerbClass::Ichidan, ConjForm::Receiving, "食べてもらう");
        check("食べる", VerbClass::Ichidan, ConjForm::GivingOutward, "食べてくれる");
        check("食べる", VerbClass::Ichidan, ConjForm::GivingToOther, "食べてあげる");
        check("書く", VerbClass::GodanKu, ConjForm::ComingTo, "書いてくる");
        check("読む", VerbClass::GodanMu, ConjForm::Receiving, "読んでもらう");
    }

    // ─── Volitional / conjectural extensions ─────────────────────────

    #[test]
    fn volitional_and_conjectural_extensions() {
        check("行く", VerbClass::GodanKuIku, ConjForm::VolitionalNegative, "行くまい");
        check("食べる", VerbClass::Ichidan, ConjForm::Conjectural, "食べるだろう");
        check("食べる", VerbClass::Ichidan, ConjForm::ConjecturalPolite, "食べるでしょう");
        check("食べる", VerbClass::Ichidan, ConjForm::AttemptVolitional, "食べてみよう");
    }

    // ─── Voice extensions ────────────────────────────────────────────

    #[test]
    fn causative_short_and_potential_negative() {
        check("書く", VerbClass::GodanKu, ConjForm::CausativeShort, "書かす");
        check("食べる", VerbClass::Ichidan, ConjForm::CausativeShort, "食べさす");
        check("書く", VerbClass::GodanKu, ConjForm::PotentialNegative, "書けない");
        check("読む", VerbClass::GodanMu, ConjForm::PotentialNegative, "読めない");
        check("食べる", VerbClass::Ichidan, ConjForm::PotentialNegative, "食べられない");
    }

    #[test]
    fn honorific_passive_form_distinct_tag() {
        let v = Verb::new("食べる", VerbClass::Ichidan);
        let h = v.honorific();
        let p = v.passive();
        // Surface identical (尊敬語 reading of られる), tag distinct.
        assert_eq!(h.surface, p.surface);
        assert!(matches!(h.form, ConjForm::Honorific));
        assert!(matches!(p.form, ConjForm::Passive));
    }

    // ─── Negative variants ───────────────────────────────────────────

    #[test]
    fn classical_negative_forms() {
        check("行く", VerbClass::GodanKuIku, ConjForm::NegativeNu, "行かぬ");
        check("食べる", VerbClass::Ichidan, ConjForm::NegativeNu, "食べぬ");
        check("行く", VerbClass::GodanKuIku, ConjForm::NegativeZu, "行かず");
        check("行く", VerbClass::GodanKuIku, ConjForm::NegativeWithoutDoing, "行かずに");
    }

    // ─── Past variants ───────────────────────────────────────────────

    #[test]
    fn past_tari_form() {
        check("走る", VerbClass::GodanRu, ConjForm::PastTari, "走ったり");
        check("食べる", VerbClass::Ichidan, ConjForm::PastTari, "食べたり");
        check("読む", VerbClass::GodanMu, ConjForm::PastTari, "読んだり");
    }

    // ─── Modal compounds ─────────────────────────────────────────────

    #[test]
    fn modal_compounds() {
        check("行く", VerbClass::GodanKuIku, ConjForm::Obligation, "行かなければならない");
        check("食べる", VerbClass::Ichidan, ConjForm::Permission, "食べてもいい");
        check("食べる", VerbClass::Ichidan, ConjForm::Prohibition, "食べてはいけない");
        check("行く", VerbClass::GodanKuIku, ConjForm::Recommendation, "行くべき");
    }

    // ─── Hearsay vs appearance ───────────────────────────────────────

    #[test]
    fn hearsay_attaches_to_dict_form() {
        check("行く", VerbClass::GodanKuIku, ConjForm::Hearsay, "行くそうだ");
        check("食べる", VerbClass::Ichidan, ConjForm::Hearsay, "食べるそうだ");
    }

    #[test]
    fn appearance_attaches_to_stem() {
        check("行く", VerbClass::GodanKuIku, ConjForm::Appearance, "行きそうだ");
        check("食べる", VerbClass::Ichidan, ConjForm::Appearance, "食べそうだ");
        check("読む", VerbClass::GodanMu, ConjForm::Appearance, "読みそうだ");
    }

    // ─── Polite hearsay / appearance / seems-like / reportedly ───────

    #[test]
    fn polite_hearsay_and_appearance() {
        check("行く", VerbClass::GodanKuIku, ConjForm::HearsayPolite, "行くそうです");
        check("食べる", VerbClass::Ichidan, ConjForm::HearsayPolite, "食べるそうです");
        check("行く", VerbClass::GodanKuIku, ConjForm::AppearancePolite, "行きそうです");
        check("食べる", VerbClass::Ichidan, ConjForm::AppearancePolite, "食べそうです");
    }

    #[test]
    fn seems_like_and_reportedly() {
        check("行く", VerbClass::GodanKuIku, ConjForm::SeemsLike, "行くみたいだ");
        check("行く", VerbClass::GodanKuIku, ConjForm::SeemsLikePolite, "行くみたいです");
        check("行く", VerbClass::GodanKuIku, ConjForm::Reportedly, "行くらしい");
        check("食べる", VerbClass::Ichidan, ConjForm::Reportedly, "食べるらしい");
    }

    // ─── Keigo constructions ─────────────────────────────────────────

    #[test]
    fn keigo_constructions() {
        check(
            "読む",
            VerbClass::GodanMu,
            ConjForm::HonorificOninaru,
            "お読みになる",
        );
        check("読む", VerbClass::GodanMu, ConjForm::HumbleOsuru, "お読みする");
        check(
            "書く",
            VerbClass::GodanKu,
            ConjForm::HonorificOninaru,
            "お書きになる",
        );
    }

    #[test]
    fn keigo_with_go_prefix_for_kango_verbs() {
        // Sino-Japanese (kango) verbs take ご instead of お.
        let yomu = Verb::new("読む", VerbClass::GodanMu);
        assert_eq!(
            yomu.honorific_oninaru_with_prefix(HonorificPrefix::Go).surface,
            "ご読みになる" // not what real Japanese would use for 読む but
                           // demonstrates the API works
        );
        // Real example: 説明する would be honorific_oninaru as
        // ご説明になる. We can't construct 説明する as a Verb here
        // (it's a SuruCompound), but we can test the prefix API on
        // any verb structurally.
        let kaku = Verb::new("書く", VerbClass::GodanKu);
        assert_eq!(
            kaku.honorific_oninaru_with_prefix(HonorificPrefix::Go).surface,
            "ご書きになる"
        );
        assert_eq!(
            kaku.humble_osuru_with_prefix(HonorificPrefix::Go).surface,
            "ご書きする"
        );
    }

    #[test]
    fn honorific_prefix_default_matches_explicit_o() {
        let v = Verb::new("読む", VerbClass::GodanMu);
        assert_eq!(
            v.honorific_oninaru().surface,
            v.honorific_oninaru_with_prefix(HonorificPrefix::O).surface,
        );
        assert_eq!(
            v.humble_osuru().surface,
            v.humble_osuru_with_prefix(HonorificPrefix::O).surface,
        );
    }

    // ─── Explanatory ─────────────────────────────────────────────────

    #[test]
    fn explanatory_chains() {
        check("行く", VerbClass::GodanKuIku, ConjForm::Explanatory, "行くのだ");
        check(
            "行く",
            VerbClass::GodanKuIku,
            ConjForm::ExplanatoryPolite,
            "行くのです",
        );
        check("食べる", VerbClass::Ichidan, ConjForm::Explanatory, "食べるのだ");
    }

    // ─── Stem accessors via dispatch ─────────────────────────────────

    #[test]
    fn stems_accessible_via_dispatch() {
        check("書く", VerbClass::GodanKu, ConjForm::StemRenyou, "書き");
        check("書く", VerbClass::GodanKu, ConjForm::StemMizen, "書か");
        check("書く", VerbClass::GodanKu, ConjForm::StemIzen, "書け");
        check("書く", VerbClass::GodanKu, ConjForm::StemMeirei, "書け");
        check("食べる", VerbClass::Ichidan, ConjForm::StemRenyou, "食べ");
        check("食べる", VerbClass::Ichidan, ConjForm::StemMeirei, "食べろ");
    }

    #[test]
    fn deconjugator_internal_stems_return_none() {
        // StemA/E/I/O are deconjugator-internal; not exposed forward.
        let v = Verb::new("書く", VerbClass::GodanKu);
        assert!(v.conjugate(ConjForm::StemA).is_none());
        assert!(v.conjugate(ConjForm::StemE).is_none());
        assert!(v.conjugate(ConjForm::StemI).is_none());
        assert!(v.conjugate(ConjForm::StemO).is_none());
    }
}
