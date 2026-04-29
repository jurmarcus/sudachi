//! Forward conjugation for i-adjectives (形容詞) and na-adjectives
//! (形容動詞 / 形状詞).
//!
//! ## i-adjectives (高い, 寒い, おもしろい)
//!
//! All conjugations work by stripping the final い and adding a
//! suffix:
//!
//! | Form | Suffix | Example (高い) |
//! |---|---|---|
//! | Negative | くない | 高くない |
//! | Past | かった | 高かった |
//! | Negative past | くなかった | 高くなかった |
//! | Adverbial | く | 高く |
//! | Te-form | くて | 高くて |
//! | Conditional ば | ければ | 高ければ |
//! | Conditional たら | かったら | 高かったら |
//! | Provisional | なら | 高いなら |
//!
//! Special case: いい/良い is suppletive — uses よ-stem.
//!
//! ## na-adjectives (好き, 静か, 元気)
//!
//! These are nominal — they take the copula だ and act like nouns
//! before particles:
//!
//! | Form | Surface | Example (好き) |
//! |---|---|---|
//! | Predicative (plain) | 〜だ | 好きだ |
//! | Predicative (polite) | 〜です | 好きです |
//! | Attributive | 〜な | 好きな (人) |
//! | Adverbial | 〜に | 好きに |
//! | Te-form | 〜で | 好きで |
//! | Negative | 〜じゃない / 〜ではない | 好きじゃない |
//! | Past | 〜だった | 好きだった |

use crate::tag::ConjForm;
use crate::verb::Conjugated;

/// I-adjective forward conjugation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IAdjective {
    pub dict_form: String,
}

impl IAdjective {
    pub fn new(dict_form: impl Into<String>) -> Self {
        Self {
            dict_form: dict_form.into(),
        }
    }

    /// The stem — dict form minus terminal い. For いい/良い this is
    /// the irregular よ-stem (not い-).
    fn stem(&self) -> String {
        // Special case: いい / 良い → よ-stem.
        if self.dict_form == "いい" {
            return "よ".to_string();
        }
        if self.dict_form == "良い" {
            return "良".to_string();
        }
        // Strip trailing い.
        let chars: Vec<char> = self.dict_form.chars().collect();
        if chars.last() == Some(&'い') {
            chars[..chars.len() - 1].iter().collect()
        } else {
            self.dict_form.clone()
        }
    }

    /// Read-back stem for いい — distinguishes よ-stem from regular
    /// stem because conditional/attributive use いい as-is.
    fn is_ii_special(&self) -> bool {
        self.dict_form == "いい" || self.dict_form == "良い"
    }

    pub fn dictionary(&self) -> Conjugated {
        Conjugated {
            surface: self.dict_form.clone(),
            form: ConjForm::Dictionary,
        }
    }

    pub fn negative(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}くない", self.stem()),
            form: ConjForm::Negative,
        }
    }

    pub fn past(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}かった", self.stem()),
            form: ConjForm::Past,
        }
    }

    pub fn negative_past(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}くなかった", self.stem()),
            form: ConjForm::NegativePast,
        }
    }

    pub fn te_form(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}くて", self.stem()),
            form: ConjForm::Te,
        }
    }

    /// Adverbial form (〜く). E.g., 早く, 高く.
    pub fn adverbial(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}く", self.stem()),
            form: ConjForm::StemRenyou,
        }
    }

    pub fn conditional_ba(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}ければ", self.stem()),
            form: ConjForm::ConditionalBa,
        }
    }

    pub fn conditional_tara(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}かったら", self.stem()),
            form: ConjForm::ConditionalTara,
        }
    }

    /// Provisional uses dict form + なら (no stem change), even for
    /// いい (いいなら, not よなら).
    pub fn provisional_nara(&self) -> Conjugated {
        let base = if self.is_ii_special() {
            self.dict_form.clone()
        } else {
            self.dict_form.clone()
        };
        Conjugated {
            surface: format!("{}なら", base),
            form: ConjForm::ProvisionalNara,
        }
    }

    /// Polite — i-adj + です (no stem change).
    pub fn polite(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}です", self.dict_form),
            form: ConjForm::Polite,
        }
    }

    /// Polite past — past form + です.
    pub fn polite_past(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}かったです", self.stem()),
            form: ConjForm::PolitePast,
        }
    }

    /// Polite negative — くないです (or くありません).
    pub fn polite_negative(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}くないです", self.stem()),
            form: ConjForm::PoliteNegative,
        }
    }

    /// Polite negative past — くなかったです.
    pub fn polite_negative_past(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}くなかったです", self.stem()),
            form: ConjForm::PoliteNegativePast,
        }
    }

    pub fn conjugate(&self, form: ConjForm) -> Option<Conjugated> {
        Some(match form {
            ConjForm::Dictionary => self.dictionary(),
            ConjForm::Negative => self.negative(),
            ConjForm::Past => self.past(),
            ConjForm::NegativePast => self.negative_past(),
            ConjForm::Te => self.te_form(),
            ConjForm::StemRenyou => self.adverbial(),
            ConjForm::ConditionalBa => self.conditional_ba(),
            ConjForm::ConditionalTara => self.conditional_tara(),
            ConjForm::ProvisionalNara => self.provisional_nara(),
            ConjForm::Polite => self.polite(),
            ConjForm::PolitePast => self.polite_past(),
            ConjForm::PoliteNegative => self.polite_negative(),
            ConjForm::PoliteNegativePast => self.polite_negative_past(),
            _ => return None,
        })
    }
}

/// Na-adjective forward conjugation. Stores the bare stem (without
/// trailing な or だ).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaAdjective {
    pub stem: String,
}

impl NaAdjective {
    /// Build from the bare stem form (好き, 静か, not 好きな or 好きだ).
    pub fn new(stem: impl Into<String>) -> Self {
        Self { stem: stem.into() }
    }

    /// Predicative form — 好きだ.
    pub fn dictionary(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}だ", self.stem),
            form: ConjForm::Dictionary,
        }
    }

    /// Polite predicative — 好きです.
    pub fn polite(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}です", self.stem),
            form: ConjForm::Polite,
        }
    }

    /// Attributive — 好きな (used before nouns).
    pub fn attributive(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}な", self.stem),
            form: ConjForm::Dictionary, // attributive shares dict role
        }
    }

    /// Adverbial — 好きに.
    pub fn adverbial(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}に", self.stem),
            form: ConjForm::StemRenyou,
        }
    }

    /// Te-form — 好きで.
    pub fn te_form(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}で", self.stem),
            form: ConjForm::Te,
        }
    }

    /// Negative — 好きじゃない (or 好きではない).
    pub fn negative(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}じゃない", self.stem),
            form: ConjForm::Negative,
        }
    }

    /// Negative formal — 好きではない.
    pub fn negative_formal(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}ではない", self.stem),
            form: ConjForm::Negative,
        }
    }

    /// Past — 好きだった.
    pub fn past(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}だった", self.stem),
            form: ConjForm::Past,
        }
    }

    /// Negative past — 好きじゃなかった.
    pub fn negative_past(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}じゃなかった", self.stem),
            form: ConjForm::NegativePast,
        }
    }

    /// Polite past — 好きでした.
    pub fn polite_past(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}でした", self.stem),
            form: ConjForm::PolitePast,
        }
    }

    /// Polite negative — 好きじゃないです (or 好きじゃありません).
    pub fn polite_negative(&self) -> Conjugated {
        Conjugated {
            surface: format!("{}じゃないです", self.stem),
            form: ConjForm::PoliteNegative,
        }
    }

    pub fn conjugate(&self, form: ConjForm) -> Option<Conjugated> {
        Some(match form {
            ConjForm::Dictionary => self.dictionary(),
            ConjForm::Polite => self.polite(),
            ConjForm::StemRenyou => self.adverbial(),
            ConjForm::Te => self.te_form(),
            ConjForm::Negative => self.negative(),
            ConjForm::Past => self.past(),
            ConjForm::NegativePast => self.negative_past(),
            ConjForm::PolitePast => self.polite_past(),
            ConjForm::PoliteNegative => self.polite_negative(),
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_iadj(dict: &str, form: ConjForm, expected: &str) {
        let adj = IAdjective::new(dict);
        let c = adj.conjugate(form).unwrap();
        assert_eq!(c.surface, expected, "{} {:?}", dict, form);
    }

    fn check_naadj(stem: &str, form: ConjForm, expected: &str) {
        let adj = NaAdjective::new(stem);
        let c = adj.conjugate(form).unwrap();
        assert_eq!(c.surface, expected, "{} {:?}", stem, form);
    }

    #[test]
    fn takai_full_paradigm() {
        check_iadj("高い", ConjForm::Negative, "高くない");
        check_iadj("高い", ConjForm::Past, "高かった");
        check_iadj("高い", ConjForm::NegativePast, "高くなかった");
        check_iadj("高い", ConjForm::Te, "高くて");
        check_iadj("高い", ConjForm::StemRenyou, "高く");
        check_iadj("高い", ConjForm::ConditionalBa, "高ければ");
        check_iadj("高い", ConjForm::ConditionalTara, "高かったら");
        check_iadj("高い", ConjForm::ProvisionalNara, "高いなら");
        check_iadj("高い", ConjForm::Polite, "高いです");
        check_iadj("高い", ConjForm::PolitePast, "高かったです");
        check_iadj("高い", ConjForm::PoliteNegative, "高くないです");
    }

    #[test]
    fn ii_uses_yo_stem_irregularly() {
        // いい → よくない / よかった (not いくない / いかった).
        check_iadj("いい", ConjForm::Negative, "よくない");
        check_iadj("いい", ConjForm::Past, "よかった");
        check_iadj("いい", ConjForm::NegativePast, "よくなかった");
        check_iadj("いい", ConjForm::Te, "よくて");
        // But ProvisionalNara uses dict form: いいなら (not よなら).
        check_iadj("いい", ConjForm::ProvisionalNara, "いいなら");
    }

    #[test]
    fn yoi_kanji_uses_yo_stem() {
        check_iadj("良い", ConjForm::Negative, "良くない");
        check_iadj("良い", ConjForm::Past, "良かった");
    }

    #[test]
    fn na_adj_suki() {
        check_naadj("好き", ConjForm::Dictionary, "好きだ");
        check_naadj("好き", ConjForm::Polite, "好きです");
        check_naadj("好き", ConjForm::Te, "好きで");
        check_naadj("好き", ConjForm::Negative, "好きじゃない");
        check_naadj("好き", ConjForm::Past, "好きだった");
        check_naadj("好き", ConjForm::NegativePast, "好きじゃなかった");
        check_naadj("好き", ConjForm::PolitePast, "好きでした");
        check_naadj("好き", ConjForm::StemRenyou, "好きに");
    }

    #[test]
    fn na_adj_attributive_form() {
        let suki = NaAdjective::new("好き");
        assert_eq!(suki.attributive().surface, "好きな");
    }

    #[test]
    fn na_adj_negative_formal_variant() {
        let suki = NaAdjective::new("好き");
        assert_eq!(suki.negative_formal().surface, "好きではない");
    }
}
