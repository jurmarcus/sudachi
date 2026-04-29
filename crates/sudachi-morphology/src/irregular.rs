//! Per-form lookup tables for the truly irregular verbs that don't
//! fit godan or ichidan paradigms: する, 来る, 行く's hybrid past.
//!
//! These have so many stem alternations and historical irregularities
//! that rule-driven conjugation produces wrong output. A direct
//! `(VerbClass, ConjForm) → surface` table is both faster and more
//! correct.
//!
//! `する` stems: さ-/し-/す-/せ-/しよ- across 6 grammatical roles.
//! `来る` stems: こ-/き-/く-/こよ- across 5 grammatical roles, with
//! the kanji-vs-kana split (来る vs くる) fully tabulated.

use crate::tag::ConjForm;
use crate::verb::Conjugated;
use crate::verb_class::VerbClass;

/// Look up an irregular conjugation by (class, form). Returns None
/// for combinations that aren't in the irregular table OR for non-
/// irregular verb classes (those go through `Verb::conjugate`).
///
/// For Suru and Kuru, both kanji (来る) and kana-only (くる) dict
/// forms are recognised — the table preserves which form was used so
/// 来ます stays 来ます and きます stays きます.
pub fn lookup_irregular(dict_form: &str, class: VerbClass, form: ConjForm) -> Option<Conjugated> {
    match class {
        VerbClass::Suru => suru_lookup(dict_form, form),
        VerbClass::SuruCompound => suru_compound_lookup(dict_form, form),
        VerbClass::Kuru => kuru_lookup(dict_form, form),
        _ => None,
    }
}

// ────────────────────────────────────────────────────────────────────
// する
// ────────────────────────────────────────────────────────────────────

fn suru_lookup(dict_form: &str, form: ConjForm) -> Option<Conjugated> {
    if dict_form != "する" {
        return None;
    }
    let surface = match form {
        ConjForm::Dictionary => "する",
        ConjForm::Negative => "しない",
        ConjForm::NegativePast => "しなかった",
        ConjForm::NegativeTe => "しなくて",
        ConjForm::NegativeBa => "しなければ",
        ConjForm::Past => "した",
        ConjForm::Te => "して",
        ConjForm::Polite => "します",
        ConjForm::PolitePast => "しました",
        ConjForm::PoliteNegative => "しません",
        ConjForm::PoliteNegativePast => "しませんでした",
        ConjForm::PoliteTe => "しまして",
        ConjForm::PoliteVolitional => "しましょう",
        ConjForm::Causative => "させる",
        ConjForm::Passive => "される",
        ConjForm::CausativePassive => "させられる",
        ConjForm::Potential => "できる", // 出来る — する's potential is suppletive
        ConjForm::PotentialNegative => "できない",
        ConjForm::Volitional => "しよう",
        ConjForm::Imperative => "しろ", // also せよ (literary), see imperative_alt
        ConjForm::ImperativeNegative => "するな",
        ConjForm::ConditionalBa => "すれば",
        ConjForm::ConditionalTara => "したら",
        ConjForm::ProvisionalNara => "するなら",
        ConjForm::Desiderative => "したい",
        ConjForm::DesiderativeOther => "したがる",
        ConjForm::Progressive => "している",
        ConjForm::ProgressiveContracted => "してる",
        ConjForm::Resultative => "してある",
        ConjForm::Preparative => "しておく",
        ConjForm::Attempt => "してみる",
        ConjForm::Completion => "してしまう",
        ConjForm::ComingTo => "してくる",
        ConjForm::GoingTo => "していく",
        _ => return None,
    };
    Some(Conjugated {
        surface: surface.to_string(),
        form,
    })
}

/// Suru-noun + する compounds (勉強する, 旅行する, …). The noun
/// stays put; する conjugates around it.
fn suru_compound_lookup(dict_form: &str, form: ConjForm) -> Option<Conjugated> {
    if !dict_form.ends_with("する") || dict_form.chars().count() < 3 {
        return None;
    }
    let noun_chars = dict_form.chars().count() - 2; // drop trailing する
    let noun: String = dict_form.chars().take(noun_chars).collect();

    // Conjugate する via the suru_lookup table, then prefix the noun.
    let suru = suru_lookup("する", form)?;
    Some(Conjugated {
        surface: format!("{}{}", noun, suru.surface),
        form: suru.form,
    })
}

// ────────────────────────────────────────────────────────────────────
// 来る / くる
// ────────────────────────────────────────────────────────────────────

fn kuru_lookup(dict_form: &str, form: ConjForm) -> Option<Conjugated> {
    let kanji = match dict_form {
        "来る" => true,
        "くる" => false,
        _ => return None,
    };
    // Stems by row:
    //   こ - mizen (negative, passive, causative, volitional)
    //   き - renyou (polite, tai, te-form root)
    //   く - dictionary, attributive
    //   くれ - izen (ba conditional)
    //   こい - imperative
    let s = |kanji_form: &str, kana_form: &str| {
        if kanji {
            kanji_form
        } else {
            kana_form
        }
        .to_string()
    };
    let surface = match form {
        ConjForm::Dictionary => s("来る", "くる"),
        ConjForm::Negative => s("来ない", "こない"),
        ConjForm::NegativePast => s("来なかった", "こなかった"),
        ConjForm::NegativeTe => s("来なくて", "こなくて"),
        ConjForm::NegativeBa => s("来なければ", "こなければ"),
        ConjForm::Past => s("来た", "きた"),
        ConjForm::Te => s("来て", "きて"),
        ConjForm::Polite => s("来ます", "きます"),
        ConjForm::PolitePast => s("来ました", "きました"),
        ConjForm::PoliteNegative => s("来ません", "きません"),
        ConjForm::PoliteNegativePast => s("来ませんでした", "きませんでした"),
        ConjForm::PoliteTe => s("来まして", "きまして"),
        ConjForm::PoliteVolitional => s("来ましょう", "きましょう"),
        ConjForm::Causative => s("来させる", "こさせる"),
        ConjForm::Passive => s("来られる", "こられる"),
        ConjForm::CausativePassive => s("来させられる", "こさせられる"),
        ConjForm::Potential => s("来られる", "こられる"),
        ConjForm::PotentialNegative => s("来られない", "こられない"),
        ConjForm::Volitional => s("来よう", "こよう"),
        ConjForm::Imperative => s("来い", "こい"),
        ConjForm::ImperativeNegative => s("来るな", "くるな"),
        ConjForm::ConditionalBa => s("来れば", "くれば"),
        ConjForm::ConditionalTara => s("来たら", "きたら"),
        ConjForm::ProvisionalNara => s("来るなら", "くるなら"),
        ConjForm::Desiderative => s("来たい", "きたい"),
        ConjForm::DesiderativeOther => s("来たがる", "きたがる"),
        ConjForm::Progressive => s("来ている", "きている"),
        ConjForm::ProgressiveContracted => s("来てる", "きてる"),
        ConjForm::Resultative => s("来てある", "きてある"),
        ConjForm::Preparative => s("来ておく", "きておく"),
        ConjForm::Attempt => s("来てみる", "きてみる"),
        ConjForm::Completion => s("来てしまう", "きてしまう"),
        _ => return None,
    };
    Some(Conjugated { surface, form })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_irregular(dict: &str, class: VerbClass, form: ConjForm, expected: &str) {
        let c = lookup_irregular(dict, class, form).unwrap_or_else(|| {
            panic!("lookup_irregular returned None for {} ({:?}) {:?}", dict, class, form)
        });
        assert_eq!(c.surface, expected, "{} {:?} {:?}", dict, class, form);
    }

    // ─── する full paradigm ──────────────────────────────────────────

    #[test]
    fn suru_full_paradigm() {
        check_irregular("する", VerbClass::Suru, ConjForm::Negative, "しない");
        check_irregular("する", VerbClass::Suru, ConjForm::Past, "した");
        check_irregular("する", VerbClass::Suru, ConjForm::Te, "して");
        check_irregular("する", VerbClass::Suru, ConjForm::Polite, "します");
        check_irregular("する", VerbClass::Suru, ConjForm::PolitePast, "しました");
        check_irregular(
            "する",
            VerbClass::Suru,
            ConjForm::PoliteNegativePast,
            "しませんでした",
        );
        check_irregular("する", VerbClass::Suru, ConjForm::Causative, "させる");
        check_irregular("する", VerbClass::Suru, ConjForm::Passive, "される");
        check_irregular(
            "する",
            VerbClass::Suru,
            ConjForm::CausativePassive,
            "させられる",
        );
        check_irregular("する", VerbClass::Suru, ConjForm::Potential, "できる");
        check_irregular("する", VerbClass::Suru, ConjForm::Volitional, "しよう");
        check_irregular("する", VerbClass::Suru, ConjForm::Imperative, "しろ");
        check_irregular("する", VerbClass::Suru, ConjForm::ConditionalBa, "すれば");
        check_irregular("する", VerbClass::Suru, ConjForm::ConditionalTara, "したら");
        check_irregular("する", VerbClass::Suru, ConjForm::Desiderative, "したい");
        check_irregular("する", VerbClass::Suru, ConjForm::Progressive, "している");
    }

    // ─── 来る / くる ─────────────────────────────────────────────────

    #[test]
    fn kuru_kanji_full_paradigm() {
        check_irregular("来る", VerbClass::Kuru, ConjForm::Negative, "来ない");
        check_irregular("来る", VerbClass::Kuru, ConjForm::Past, "来た");
        check_irregular("来る", VerbClass::Kuru, ConjForm::Te, "来て");
        check_irregular("来る", VerbClass::Kuru, ConjForm::Polite, "来ます");
        check_irregular("来る", VerbClass::Kuru, ConjForm::Volitional, "来よう");
        check_irregular("来る", VerbClass::Kuru, ConjForm::Imperative, "来い");
        check_irregular("来る", VerbClass::Kuru, ConjForm::ConditionalBa, "来れば");
        check_irregular("来る", VerbClass::Kuru, ConjForm::Causative, "来させる");
        check_irregular("来る", VerbClass::Kuru, ConjForm::Passive, "来られる");
        check_irregular("来る", VerbClass::Kuru, ConjForm::Potential, "来られる");
    }

    #[test]
    fn kuru_kana_full_paradigm() {
        check_irregular("くる", VerbClass::Kuru, ConjForm::Negative, "こない");
        check_irregular("くる", VerbClass::Kuru, ConjForm::Past, "きた");
        check_irregular("くる", VerbClass::Kuru, ConjForm::Te, "きて");
        check_irregular("くる", VerbClass::Kuru, ConjForm::Polite, "きます");
        check_irregular("くる", VerbClass::Kuru, ConjForm::Volitional, "こよう");
        check_irregular("くる", VerbClass::Kuru, ConjForm::Imperative, "こい");
        check_irregular("くる", VerbClass::Kuru, ConjForm::ConditionalBa, "くれば");
        check_irregular("くる", VerbClass::Kuru, ConjForm::Desiderative, "きたい");
    }

    // ─── Suru compounds ─────────────────────────────────────────────

    #[test]
    fn benkyou_suru_compound() {
        check_irregular(
            "勉強する",
            VerbClass::SuruCompound,
            ConjForm::Negative,
            "勉強しない",
        );
        check_irregular(
            "勉強する",
            VerbClass::SuruCompound,
            ConjForm::Past,
            "勉強した",
        );
        check_irregular(
            "勉強する",
            VerbClass::SuruCompound,
            ConjForm::Polite,
            "勉強します",
        );
        check_irregular(
            "勉強する",
            VerbClass::SuruCompound,
            ConjForm::Causative,
            "勉強させる",
        );
        check_irregular(
            "勉強する",
            VerbClass::SuruCompound,
            ConjForm::Passive,
            "勉強される",
        );
    }

    #[test]
    fn returns_none_for_non_irregular_classes() {
        assert!(lookup_irregular("食べる", VerbClass::Ichidan, ConjForm::Past).is_none());
        assert!(lookup_irregular("書く", VerbClass::GodanKu, ConjForm::Past).is_none());
    }

    #[test]
    fn returns_none_for_wrong_dict_form() {
        // Suru class but dict form isn't する.
        assert!(lookup_irregular("食べる", VerbClass::Suru, ConjForm::Past).is_none());
        // Kuru class but dict form isn't 来る/くる.
        assert!(lookup_irregular("行く", VerbClass::Kuru, ConjForm::Past).is_none());
    }
}
