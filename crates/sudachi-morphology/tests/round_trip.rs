//! Round-trip symmetry tests — the integration-level proof that
//! forward conjugation and backward deconjugation agree.
//!
//! For each (verb, form) pair, we:
//! 1. Forward: `Verb::conjugate(form)` produces a surface string.
//! 2. Backward: `deconjugate(surface)` produces candidate base forms.
//! 3. Assert: the original verb's `(dict_form, jmdict_tag)` appears
//!    among the deconjugation candidates.
//!
//! If a round-trip fails, either:
//! - The forward conjugator produced a wrong surface (bug in
//!   `Verb::conjugate`), or
//! - The backward rule table doesn't recognise the surface (bug in
//!   the rule data or the deconjugator algorithm).
//!
//! Both halves of the symmetry are testable from one assertion.

use sudachi_morphology::{deconjugate, ConjForm, Verb, VerbClass};

/// Forward-conjugate then deconjugate; assert the original verb is
/// among the candidates.
fn round_trip(dict: &str, class: VerbClass, form: ConjForm) {
    let v = Verb::new(dict, class);
    let conjugated = v
        .conjugate(form)
        .unwrap_or_else(|| panic!("forward conjugate({:?}) returned None for {}", form, dict));
    let candidates = deconjugate(&conjugated.surface);

    let expected_tag = class.jmdict_tag();
    let matches: Vec<_> = candidates
        .iter()
        .filter(|f| {
            f.text == dict
                && f.tags.last().map(String::as_str) == Some(expected_tag)
        })
        .collect();

    assert!(
        !matches.is_empty(),
        "round-trip failed: {} ({:?}) → {} ({:?}) → no candidate ({}, {}).\n\
         Got candidates: {:?}",
        dict,
        class,
        conjugated.surface,
        form,
        dict,
        expected_tag,
        candidates
            .iter()
            .map(|c| (c.text.as_str(), c.tags.last().map(String::as_str)))
            .collect::<Vec<_>>(),
    );
}

// ─── Ichidan ─────────────────────────────────────────────────────────

#[test]
fn ichidan_taberu_round_trip_all_basic_forms() {
    for form in [
        ConjForm::Past,
        ConjForm::Negative,
        ConjForm::NegativePast,
        ConjForm::Te,
        ConjForm::Polite,
        ConjForm::PolitePast,
    ] {
        round_trip("食べる", VerbClass::Ichidan, form);
    }
}

// ─── Godan classes ───────────────────────────────────────────────────

#[test]
fn godan_kaku_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("書く", VerbClass::GodanKu, form);
    }
}

#[test]
fn godan_oyogu_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("泳ぐ", VerbClass::GodanGu, form);
    }
}

#[test]
fn godan_hashiru_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("走る", VerbClass::GodanRu, form);
    }
}

#[test]
fn godan_kau_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("買う", VerbClass::GodanU, form);
    }
}

#[test]
fn godan_motsu_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("持つ", VerbClass::GodanTsu, form);
    }
}

#[test]
fn godan_nomu_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("飲む", VerbClass::GodanMu, form);
    }
}

#[test]
fn godan_tobu_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("飛ぶ", VerbClass::GodanBu, form);
    }
}

#[test]
fn godan_shinu_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("死ぬ", VerbClass::GodanNu, form);
    }
}

#[test]
fn godan_hanasu_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("話す", VerbClass::GodanSu, form);
    }
}

// ─── Irregulars ──────────────────────────────────────────────────────

#[test]
fn iku_irregular_past_round_trip() {
    round_trip("行く", VerbClass::GodanKuIku, ConjForm::Past);
    round_trip("行く", VerbClass::GodanKuIku, ConjForm::Te);
}

#[test]
fn suru_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("する", VerbClass::Suru, form);
    }
}

#[test]
fn kuru_kanji_round_trip() {
    for form in [ConjForm::Past, ConjForm::Negative, ConjForm::Te, ConjForm::Polite] {
        round_trip("来る", VerbClass::Kuru, form);
    }
}

#[test]
fn aru_irregular_negative_round_trip() {
    // ある + Past → あった → deconjugate back to ある (v5r-i).
    round_trip("ある", VerbClass::GodanRuAru, ConjForm::Past);
    round_trip("ある", VerbClass::GodanRuAru, ConjForm::Polite);
    // Negative is irregular (ない). Since this becomes adj-i, the
    // deconjugator can't recover ある from ない without context.
    // That's expected behavior — exclude from this test.
}

// ─── Voice / chain forms ────────────────────────────────────────────

#[test]
fn ichidan_causative_round_trip() {
    round_trip("食べる", VerbClass::Ichidan, ConjForm::Causative);
}

#[test]
fn ichidan_passive_round_trip() {
    round_trip("食べる", VerbClass::Ichidan, ConjForm::Passive);
}

#[test]
fn ichidan_potential_round_trip() {
    round_trip("食べる", VerbClass::Ichidan, ConjForm::Potential);
}

#[test]
fn godan_potential_round_trip() {
    // 書ける round-trips to 書く (v5k).
    round_trip("書く", VerbClass::GodanKu, ConjForm::Potential);
    round_trip("走る", VerbClass::GodanRu, ConjForm::Potential);
}

// ─── Conditional forms ──────────────────────────────────────────────

#[test]
fn conditional_ba_round_trip() {
    round_trip("食べる", VerbClass::Ichidan, ConjForm::ConditionalBa);
    round_trip("書く", VerbClass::GodanKu, ConjForm::ConditionalBa);
}

#[test]
fn conditional_tara_round_trip() {
    round_trip("食べる", VerbClass::Ichidan, ConjForm::ConditionalTara);
    round_trip("書く", VerbClass::GodanKu, ConjForm::ConditionalTara);
}

// ─── Volitional / imperative ────────────────────────────────────────

#[test]
fn volitional_round_trip() {
    round_trip("食べる", VerbClass::Ichidan, ConjForm::Volitional);
    round_trip("書く", VerbClass::GodanKu, ConjForm::Volitional);
}

#[test]
fn imperative_round_trip() {
    round_trip("食べる", VerbClass::Ichidan, ConjForm::Imperative);
    round_trip("書く", VerbClass::GodanKu, ConjForm::Imperative);
}

// ─── Desiderative ───────────────────────────────────────────────────

#[test]
fn desiderative_round_trip() {
    round_trip("食べる", VerbClass::Ichidan, ConjForm::Desiderative);
    round_trip("書く", VerbClass::GodanKu, ConjForm::Desiderative);
}
