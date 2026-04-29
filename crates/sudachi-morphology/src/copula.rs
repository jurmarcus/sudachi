//! Copula paradigms — だ, です, である, and the explanatory のだ /
//! んだ chain.
//!
//! The copula is the most-used "verb" in Japanese but isn't really
//! a verb — it's a topic-marking equation operator. Three principal
//! forms cover virtually all modern usage:
//!
//! | Register | Affirmative | Negative |
//! |---|---|---|
//! | Plain | だ | じゃない / ではない |
//! | Polite | です | じゃありません / ではありません |
//! | Literary / formal | である | ではない |
//!
//! Plus the explanatory のだ chain (= "the explanation is that …"):
//! のだ / んだ → のです / んです → 〜のですが … etc.

use crate::tag::ConjForm;
use crate::verb::Conjugated;

/// Copula register — pick which paradigm to draw from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopulaForm {
    /// Plain だ paradigm.
    Da,
    /// Polite です paradigm.
    Desu,
    /// Literary / formal である paradigm.
    Dearu,
}

/// Look up a copula conjugation. The `form` argument selects the
/// paradigm; the `conj` argument selects which form within that
/// paradigm.
pub fn conjugate_copula(register: CopulaForm, conj: ConjForm) -> Option<Conjugated> {
    let surface = match (register, conj) {
        // ─── だ paradigm ──────────────────────────────────────────
        (CopulaForm::Da, ConjForm::Dictionary) => "だ",
        (CopulaForm::Da, ConjForm::Negative) => "じゃない",
        (CopulaForm::Da, ConjForm::Past) => "だった",
        (CopulaForm::Da, ConjForm::NegativePast) => "じゃなかった",
        (CopulaForm::Da, ConjForm::Te) => "で",
        (CopulaForm::Da, ConjForm::ConditionalBa) => "なら",
        (CopulaForm::Da, ConjForm::ConditionalTara) => "だったら",
        (CopulaForm::Da, ConjForm::ProvisionalNara) => "なら",
        (CopulaForm::Da, ConjForm::Conjectural) => "だろう",

        // ─── です paradigm ────────────────────────────────────────
        (CopulaForm::Desu, ConjForm::Dictionary) => "です",
        (CopulaForm::Desu, ConjForm::Negative) => "じゃないです",
        (CopulaForm::Desu, ConjForm::Past) => "でした",
        (CopulaForm::Desu, ConjForm::NegativePast) => "じゃなかったです",
        (CopulaForm::Desu, ConjForm::Te) => "でして",
        (CopulaForm::Desu, ConjForm::Conjectural) => "でしょう",
        (CopulaForm::Desu, ConjForm::PoliteNegative) => "じゃありません",
        (CopulaForm::Desu, ConjForm::PoliteNegativePast) => "じゃありませんでした",

        // ─── である paradigm ──────────────────────────────────────
        (CopulaForm::Dearu, ConjForm::Dictionary) => "である",
        (CopulaForm::Dearu, ConjForm::Negative) => "ではない",
        (CopulaForm::Dearu, ConjForm::Past) => "であった",
        (CopulaForm::Dearu, ConjForm::NegativePast) => "ではなかった",
        (CopulaForm::Dearu, ConjForm::Te) => "であって",
        (CopulaForm::Dearu, ConjForm::Polite) => "であります",
        (CopulaForm::Dearu, ConjForm::PolitePast) => "でありました",
        (CopulaForm::Dearu, ConjForm::PoliteNegative) => "ではありません",

        _ => return None,
    };
    Some(Conjugated {
        surface: surface.to_string(),
        form: conj,
    })
}

/// Explanatory のだ / のです chain. Builds an の/ん nominaliser
/// followed by the chosen copula form.
///
/// Plain: のだ / んだ
/// Polite: のです / んです
/// Past: のだった / のでした
/// Negative: のではない / のじゃない
pub fn conjugate_explanatory(contracted: bool, register: CopulaForm, conj: ConjForm) -> Option<Conjugated> {
    let prefix = if contracted { "ん" } else { "の" };
    let copula = conjugate_copula(register, conj)?;
    Some(Conjugated {
        surface: format!("{}{}", prefix, copula.surface),
        form: ConjForm::Explanatory,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(register: CopulaForm, form: ConjForm, expected: &str) {
        let c = conjugate_copula(register, form).unwrap();
        assert_eq!(c.surface, expected, "{:?} {:?}", register, form);
    }

    #[test]
    fn da_paradigm() {
        check(CopulaForm::Da, ConjForm::Dictionary, "だ");
        check(CopulaForm::Da, ConjForm::Negative, "じゃない");
        check(CopulaForm::Da, ConjForm::Past, "だった");
        check(CopulaForm::Da, ConjForm::NegativePast, "じゃなかった");
        check(CopulaForm::Da, ConjForm::Te, "で");
        check(CopulaForm::Da, ConjForm::ConditionalTara, "だったら");
        check(CopulaForm::Da, ConjForm::ProvisionalNara, "なら");
        check(CopulaForm::Da, ConjForm::Conjectural, "だろう");
    }

    #[test]
    fn desu_paradigm() {
        check(CopulaForm::Desu, ConjForm::Dictionary, "です");
        check(CopulaForm::Desu, ConjForm::Past, "でした");
        check(CopulaForm::Desu, ConjForm::Conjectural, "でしょう");
        check(
            CopulaForm::Desu,
            ConjForm::PoliteNegative,
            "じゃありません",
        );
        check(
            CopulaForm::Desu,
            ConjForm::PoliteNegativePast,
            "じゃありませんでした",
        );
    }

    #[test]
    fn dearu_paradigm() {
        check(CopulaForm::Dearu, ConjForm::Dictionary, "である");
        check(CopulaForm::Dearu, ConjForm::Negative, "ではない");
        check(CopulaForm::Dearu, ConjForm::Past, "であった");
        check(CopulaForm::Dearu, ConjForm::Polite, "であります");
        check(
            CopulaForm::Dearu,
            ConjForm::PoliteNegative,
            "ではありません",
        );
    }

    #[test]
    fn explanatory_chain() {
        // のだ chain.
        let c = conjugate_explanatory(false, CopulaForm::Da, ConjForm::Dictionary).unwrap();
        assert_eq!(c.surface, "のだ");
        let c = conjugate_explanatory(false, CopulaForm::Desu, ConjForm::Dictionary).unwrap();
        assert_eq!(c.surface, "のです");
        // んだ chain (contracted).
        let c = conjugate_explanatory(true, CopulaForm::Da, ConjForm::Dictionary).unwrap();
        assert_eq!(c.surface, "んだ");
        let c = conjugate_explanatory(true, CopulaForm::Desu, ConjForm::Dictionary).unwrap();
        assert_eq!(c.surface, "んです");
        let c = conjugate_explanatory(false, CopulaForm::Da, ConjForm::Past).unwrap();
        assert_eq!(c.surface, "のだった");
        let c = conjugate_explanatory(false, CopulaForm::Desu, ConjForm::Past).unwrap();
        assert_eq!(c.surface, "のでした");
    }
}
