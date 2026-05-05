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
///
/// Adjective classes ([`VerbClass::IAdjective`] /
/// [`VerbClass::IAdjectiveIrregular`]) are dispatched here too so the
/// unified `Verb::conjugate` entry point can handle them. The actual
/// conjugation is delegated to [`crate::adjective::IAdjective`],
/// whose `conjugate` method already handles the いい/良い irregular
/// case via its `stem` accessor.
pub fn lookup_irregular(dict_form: &str, class: VerbClass, form: ConjForm) -> Option<Conjugated> {
    match class {
        VerbClass::Suru => suru_lookup(dict_form, form),
        VerbClass::SuruCompound => suru_compound_lookup(dict_form, form),
        VerbClass::SuruNoun => suru_noun_lookup(dict_form, form),
        VerbClass::Kuru => kuru_lookup(dict_form, form),
        VerbClass::IAdjective | VerbClass::IAdjectiveIrregular => {
            crate::adjective::IAdjective::new(dict_form).conjugate(form)
        }
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
///
/// Bare `する` falls through to [`suru_lookup`] so that this function
/// is safe to call as a single dispatch entry for `vs-i` POS entries
/// (which include both compounds AND the bare verb する itself).
fn suru_compound_lookup(dict_form: &str, form: ConjForm) -> Option<Conjugated> {
    if dict_form == "する" {
        return suru_lookup(dict_form, form);
    }
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

/// Bare suru-noun input (vs POS). The dict form is the noun alone —
/// e.g. `勉強`, `閉扉`, `介在`. Conjugation appends a する-paradigm
/// tail to the noun via [`suru_lookup`].
///
/// Distinct from [`suru_compound_lookup`], which expects the dict
/// form to already end in する. JMdict tags both shapes (some
/// entries with `vs` plus a bare-noun headword, others with `vs-i`
/// plus an X+する headword), so both paths exist.
fn suru_noun_lookup(noun: &str, form: ConjForm) -> Option<Conjugated> {
    if noun.is_empty() {
        return None;
    }
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
    // Bare 来る or くる — direct table hit.
    if dict_form == "来る" || dict_form == "くる" {
        return kuru_bare_lookup(dict_form, form);
    }
    // Compound (X+来る / X+くる, e.g. お釣りが来る, 戻ってくる).
    // Strip the trailing 来る/くる, conjugate the bare verb, then
    // re-prepend the prefix unchanged.
    if let Some(prefix) = dict_form.strip_suffix("来る") {
        let bare = kuru_bare_lookup("来る", form)?;
        return Some(Conjugated {
            surface: format!("{}{}", prefix, bare.surface),
            form: bare.form,
        });
    }
    if let Some(prefix) = dict_form.strip_suffix("くる") {
        let bare = kuru_bare_lookup("くる", form)?;
        return Some(Conjugated {
            surface: format!("{}{}", prefix, bare.surface),
            form: bare.form,
        });
    }
    None
}

fn kuru_bare_lookup(dict_form: &str, form: ConjForm) -> Option<Conjugated> {
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
        // Kuru class but dict form isn't 来る/くる/X+来る/X+くる.
        assert!(lookup_irregular("行く", VerbClass::Kuru, ConjForm::Past).is_none());
    }

    // ─── Suru-noun (vs POS, bare-noun input) ────────────────────────

    #[test]
    fn benkyou_suru_noun_paradigm() {
        // The dict_form is just the noun — the suru tail is implicit.
        check_irregular("勉強", VerbClass::SuruNoun, ConjForm::Dictionary, "勉強する");
        check_irregular("勉強", VerbClass::SuruNoun, ConjForm::Negative, "勉強しない");
        check_irregular("勉強", VerbClass::SuruNoun, ConjForm::Past, "勉強した");
        check_irregular("勉強", VerbClass::SuruNoun, ConjForm::Te, "勉強して");
        check_irregular("勉強", VerbClass::SuruNoun, ConjForm::Polite, "勉強します");
        check_irregular(
            "勉強",
            VerbClass::SuruNoun,
            ConjForm::PolitePast,
            "勉強しました",
        );
        check_irregular(
            "勉強",
            VerbClass::SuruNoun,
            ConjForm::Causative,
            "勉強させる",
        );
        check_irregular("勉強", VerbClass::SuruNoun, ConjForm::Passive, "勉強される");
        check_irregular("勉強", VerbClass::SuruNoun, ConjForm::Potential, "勉強できる");
    }

    #[test]
    fn suru_noun_empty_returns_none() {
        assert!(lookup_irregular("", VerbClass::SuruNoun, ConjForm::Past).is_none());
    }

    // ─── Suru-compound bare-する fallback ────────────────────────────

    #[test]
    fn suru_compound_handles_bare_suru() {
        // vs-i POS sometimes tags bare する itself; the from_jmdict
        // remap sends those to SuruCompound. The fallback inside
        // suru_compound_lookup must hand off to suru_lookup for
        // dict_form == "する".
        check_irregular("する", VerbClass::SuruCompound, ConjForm::Past, "した");
        check_irregular("する", VerbClass::SuruCompound, ConjForm::Polite, "します");
        check_irregular("する", VerbClass::SuruCompound, ConjForm::Causative, "させる");
    }

    // ─── Kuru compounds (X+来る / X+くる) ───────────────────────────

    #[test]
    fn kuru_compound_kanji_paradigm() {
        // お釣りが来る — preserve the prefix, conjugate only the
        // trailing 来る.
        check_irregular(
            "お釣りが来る",
            VerbClass::Kuru,
            ConjForm::Negative,
            "お釣りが来ない",
        );
        check_irregular(
            "お釣りが来る",
            VerbClass::Kuru,
            ConjForm::Past,
            "お釣りが来た",
        );
        check_irregular(
            "お釣りが来る",
            VerbClass::Kuru,
            ConjForm::Polite,
            "お釣りが来ます",
        );
    }

    #[test]
    fn kuru_compound_kana_paradigm() {
        check_irregular(
            "戻ってくる",
            VerbClass::Kuru,
            ConjForm::Negative,
            "戻ってこない",
        );
        check_irregular(
            "戻ってくる",
            VerbClass::Kuru,
            ConjForm::Past,
            "戻ってきた",
        );
        check_irregular(
            "戻ってくる",
            VerbClass::Kuru,
            ConjForm::Te,
            "戻ってきて",
        );
        check_irregular(
            "戻ってくる",
            VerbClass::Kuru,
            ConjForm::Polite,
            "戻ってきます",
        );
    }

    // ─── Adjective dispatch ─────────────────────────────────────────

    #[test]
    fn adjective_dispatch_regular_iadj() {
        check_irregular("高い", VerbClass::IAdjective, ConjForm::Past, "高かった");
        check_irregular("高い", VerbClass::IAdjective, ConjForm::Negative, "高くない");
        check_irregular(
            "高い",
            VerbClass::IAdjective,
            ConjForm::NegativePast,
            "高くなかった",
        );
        check_irregular("高い", VerbClass::IAdjective, ConjForm::Te, "高くて");
        check_irregular(
            "高い",
            VerbClass::IAdjective,
            ConjForm::ConditionalBa,
            "高ければ",
        );
        check_irregular("高い", VerbClass::IAdjective, ConjForm::Polite, "高いです");
    }

    #[test]
    fn adjective_dispatch_irregular_ii() {
        // いい / 良い is tagged adj-ix in JMdict — the irregular case
        // uses the よ-stem (良かった, not いかった).
        check_irregular(
            "いい",
            VerbClass::IAdjectiveIrregular,
            ConjForm::Past,
            "よかった",
        );
        check_irregular(
            "いい",
            VerbClass::IAdjectiveIrregular,
            ConjForm::Negative,
            "よくない",
        );
        check_irregular(
            "良い",
            VerbClass::IAdjectiveIrregular,
            ConjForm::Past,
            "良かった",
        );
    }
}
