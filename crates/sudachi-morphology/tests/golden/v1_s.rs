//! Golden tests ported from JL's `DeconjugatorTestsForV1S.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V1S.

use sudachi_morphology::deconjugate;
use crate::helper::{format_process, matches_expected};

/// Assert that `deconjugate(input)`, when filtered to forms
/// matching `expected_dict` + `expected_class`, produces the
/// expected process chain string per JL's formatter.
fn assert_golden(
    input: &str,
    expected_dict: &str,
    expected_class: &str,
    expected: &str,
) {
    let forms = deconjugate(input);
    let matches: Vec<_> = forms
        .iter()
        .filter(|f| {
            f.text == expected_dict
                && f.tags.last().map(String::as_str) == Some(expected_class)
        })
        .collect();
    assert!(
        matches_expected(&matches, expected),
        "deconjugate({:?}) for {} ({}) — expected {:?}, got chains: {:?}",
        input,
        expected_dict,
        expected_class,
        expected,
        matches.iter().map(|f| format_process(&f.process)).collect::<Vec<_>>(),
    );
}

#[test]
fn deconjugate_masu_stem_v1_s() {
    assert_golden("呉れ", "呉れる", "v1-s", "～imperative; masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v1_s() {
    assert_golden("呉れない", "呉れる", "v1-s", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v1_s() {
    assert_golden("呉れます", "呉れる", "v1-s", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v1_s() {
    assert_golden("呉れましょう", "呉れる", "v1-s", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v1_s() {
    assert_golden("呉れません", "呉れる", "v1-s", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v1_s() {
    assert_golden("呉れた", "呉れる", "v1-s", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v1_s() {
    assert_golden("呉れなかった", "呉れる", "v1-s", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v1_s() {
    assert_golden("呉れました", "呉れる", "v1-s", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v1_s() {
    assert_golden("呉れませんでした", "呉れる", "v1-s", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v1_s() {
    assert_golden("呉れて", "呉れる", "v1-s", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v1_s() {
    assert_golden("呉れなくて", "呉れる", "v1-s", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v1_s() {
    assert_golden("呉れないで", "呉れる", "v1-s", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v1_s() {
    assert_golden("呉れまして", "呉れる", "v1-s", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_honorific_affirmative_v1_s() {
    assert_golden("呉れられる", "呉れる", "v1-s", "～passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_honorific_negative_v1_s() {
    assert_golden("呉れられない", "呉れる", "v1-s", "～passive/potential/honorific→negative");
}

#[test]
fn deconjugate_plain_past_passive_potential_honorific_affirmative_v1_s() {
    assert_golden("呉れられた", "呉れる", "v1-s", "～passive/potential/honorific→past");
}

#[test]
fn deconjugate_polite_past_passive_potential_honorific_affirmative_v1_s() {
    assert_golden("呉れられました", "呉れる", "v1-s", "～passive/potential/honorific→polite past");
}

#[test]
fn deconjugate_plain_past_passive_potential_honorific_negative_v1_s() {
    assert_golden("呉れられなかった", "呉れる", "v1-s", "～passive/potential/honorific→negative→past");
}

#[test]
fn deconjugate_polite_past_passive_potential_honorific_negative_v1_s() {
    assert_golden("呉れられませんでした", "呉れる", "v1-s", "～passive/potential/honorific→polite past negative");
}

#[test]
fn deconjugate_polite_passive_potential_honorific_affirmative_v1_s() {
    assert_golden("呉れられます", "呉れる", "v1-s", "～passive/potential/honorific→polite");
}

#[test]
fn deconjugate_polite_passive_potential_honorific_negative_v1_s() {
    assert_golden("呉れられません", "呉れる", "v1-s", "～passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_imperative_negative_v1_s() {
    assert_golden("呉れるな", "呉れる", "v1-s", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v1_s() {
    assert_golden("呉れなさい", "呉れる", "v1-s", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v1_s() {
    assert_golden("呉れてください", "呉れる", "v1-s", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v1_s() {
    assert_golden("呉れないでください", "呉れる", "v1-s", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v1_s() {
    assert_golden("呉れよう", "呉れる", "v1-s", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v1_s() {
    assert_golden("呉れよ", "呉れる", "v1-s", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v1_s() {
    assert_golden("呉れましょう", "呉れる", "v1-s", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v1_s() {
    assert_golden("呉れれば", "呉れる", "v1-s", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v1_s() {
    assert_golden("呉れなければ", "呉れる", "v1-s", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v1_s() {
    assert_golden("呉れたら", "呉れる", "v1-s", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v1_s() {
    assert_golden("呉れたらば", "呉れる", "v1-s", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v1_s() {
    assert_golden("呉れなかったら", "呉れる", "v1-s", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v1_s() {
    assert_golden("呉れさせる", "呉れる", "v1-s", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v1_s() {
    assert_golden("呉れさせない", "呉れる", "v1-s", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v1_s() {
    assert_golden("呉れさせん", "呉れる", "v1-s", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v1_s() {
    assert_golden("呉れさせます", "呉れる", "v1-s", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_2_v1_s() {
    assert_golden("呉れさします", "呉れる", "v1-s", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v1_s() {
    assert_golden("呉れさせません", "呉れる", "v1-s", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v1_s() {
    assert_golden("呉れさせた", "呉れる", "v1-s", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v1_s() {
    assert_golden("呉れさせなかった", "呉れる", "v1-s", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v1_s() {
    assert_golden("呉れさせました", "呉れる", "v1-s", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v1_s() {
    assert_golden("呉れさせませんでした", "呉れる", "v1-s", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v1_s() {
    assert_golden("呉れさせられる", "呉れる", "v1-s", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v1_s() {
    assert_golden("呉れさせられない", "呉れる", "v1-s", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v1_s() {
    assert_golden("呉れさせられます", "呉れる", "v1-s", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v1_s() {
    assert_golden("呉れさせられません", "呉れる", "v1-s", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v1_s() {
    assert_golden("呉れたい", "呉れる", "v1-s", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v1_s() {
    assert_golden("呉れたくありません", "呉れる", "v1-s", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v1_s() {
    assert_golden("呉れたくありませんでした", "呉れる", "v1-s", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v1_s() {
    assert_golden("呉れたくない", "呉れる", "v1-s", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v1_s() {
    assert_golden("呉れたかった", "呉れる", "v1-s", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v1_s() {
    assert_golden("呉れたくなかった", "呉れる", "v1-s", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v1_s() {
    assert_golden("呉れている", "呉れる", "v1-s", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v1_s() {
    assert_golden("呉れていない", "呉れる", "v1-s", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v1_s() {
    assert_golden("呉れていた", "呉れる", "v1-s", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v1_s() {
    assert_golden("呉れていなかった", "呉れる", "v1-s", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v1_s() {
    assert_golden("呉れています", "呉れる", "v1-s", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v1_s() {
    assert_golden("呉れていません", "呉れる", "v1-s", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v1_s() {
    assert_golden("呉れていました", "呉れる", "v1-s", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v1_s() {
    assert_golden("呉れていませんでした", "呉れる", "v1-s", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v1_s() {
    assert_golden("呉れてる", "呉れる", "v1-s", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v1_s() {
    assert_golden("呉れてない", "呉れる", "v1-s", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v1_s() {
    assert_golden("呉れてた", "呉れる", "v1-s", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v1_s() {
    assert_golden("呉れてなかった", "呉れる", "v1-s", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v1_s() {
    assert_golden("呉れてます", "呉れる", "v1-s", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v1_s() {
    assert_golden("呉れてません", "呉れる", "v1-s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v1_s() {
    assert_golden("呉れてました", "呉れる", "v1-s", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v1_s() {
    assert_golden("呉れてません", "呉れる", "v1-s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v1_s() {
    assert_golden("呉れてませんでした", "呉れる", "v1-s", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v1_s() {
    assert_golden("呉れてしまう", "呉れる", "v1-s", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v1_s() {
    assert_golden("呉れてもう", "呉れる", "v1-s", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v1_s() {
    assert_golden("呉れてしまわない", "呉れる", "v1-s", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v1_s() {
    assert_golden("呉れてしまった", "呉れる", "v1-s", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v1_s() {
    assert_golden("呉れてしまわなかった", "呉れる", "v1-s", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v1_s() {
    assert_golden("呉れてしまって", "呉れる", "v1-s", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v1_s() {
    assert_golden("呉れてしまえば", "呉れる", "v1-s", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v1_s() {
    assert_golden("呉れてしまわなければ", "呉れる", "v1-s", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v1_s() {
    assert_golden("呉れてしまわなかったら", "呉れる", "v1-s", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v1_s() {
    assert_golden("呉れてしまったら", "呉れる", "v1-s", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v1_s() {
    assert_golden("呉れてしまおう", "呉れる", "v1-s", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v1_s() {
    assert_golden("呉れてしまいます", "呉れる", "v1-s", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v1_s() {
    assert_golden("呉れてしまいません", "呉れる", "v1-s", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v1_s() {
    assert_golden("呉れてしまいました", "呉れる", "v1-s", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v1_s() {
    assert_golden("呉れてしまいませんでした", "呉れる", "v1-s", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v1_s() {
    assert_golden("呉れてしまえる", "呉れる", "v1-s", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v1_s() {
    assert_golden("呉れてしまわれる", "呉れる", "v1-s", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v1_s() {
    assert_golden("呉れてしまわせる", "呉れる", "v1-s", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v1_s() {
    assert_golden("呉れちゃう", "呉れる", "v1-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v1_s() {
    assert_golden("呉れちゃわない", "呉れる", "v1-s", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v1_s() {
    assert_golden("呉れちゃった", "呉れる", "v1-s", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v1_s() {
    assert_golden("呉れちゃわなかった", "呉れる", "v1-s", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v1_s() {
    assert_golden("呉れちゃって", "呉れる", "v1-s", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v1_s() {
    assert_golden("呉れちゃえば", "呉れる", "v1-s", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v1_s() {
    assert_golden("呉れちゃわなければ", "呉れる", "v1-s", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v1_s() {
    assert_golden("呉れちゃわなかったら", "呉れる", "v1-s", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v1_s() {
    assert_golden("呉れちゃおう", "呉れる", "v1-s", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v1_s() {
    assert_golden("呉れちゃえる", "呉れる", "v1-s", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v1_s() {
    assert_golden("呉れておく", "呉れる", "v1-s", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v1_s() {
    assert_golden("呉れておかない", "呉れる", "v1-s", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v1_s() {
    assert_golden("呉れておいた", "呉れる", "v1-s", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v1_s() {
    assert_golden("呉れておかなかった", "呉れる", "v1-s", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v1_s() {
    assert_golden("呉れておいて", "呉れる", "v1-s", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v1_s() {
    assert_golden("呉れておけば", "呉れる", "v1-s", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v1_s() {
    assert_golden("呉れておいたら", "呉れる", "v1-s", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v1_s() {
    assert_golden("呉れておこう", "呉れる", "v1-s", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v1_s() {
    assert_golden("呉れておける", "呉れる", "v1-s", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v1_s() {
    assert_golden("呉れておかれる", "呉れる", "v1-s", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v1_s() {
    assert_golden("呉れとく", "呉れる", "v1-s", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v1_s() {
    assert_golden("呉れとかない", "呉れる", "v1-s", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v1_s() {
    assert_golden("呉れといた", "呉れる", "v1-s", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v1_s() {
    assert_golden("呉れとかなかった", "呉れる", "v1-s", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v1_s() {
    assert_golden("呉れといて", "呉れる", "v1-s", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v1_s() {
    assert_golden("呉れとけば", "呉れる", "v1-s", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v1_s() {
    assert_golden("呉れといたら", "呉れる", "v1-s", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v1_s() {
    assert_golden("呉れとこう", "呉れる", "v1-s", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v1_s() {
    assert_golden("呉れとける", "呉れる", "v1-s", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v1_s() {
    assert_golden("呉れとかれる", "呉れる", "v1-s", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v1_s() {
    assert_golden("呉れてある", "呉れる", "v1-s", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v1_s() {
    assert_golden("呉れてあった", "呉れる", "v1-s", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v1_s() {
    assert_golden("呉れてあって", "呉れる", "v1-s", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v1_s() {
    assert_golden("呉れてあったら", "呉れる", "v1-s", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v1_s() {
    assert_golden("呉れてあれば", "呉れる", "v1-s", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v1_s() {
    assert_golden("呉れていく", "呉れる", "v1-s", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v1_s() {
    assert_golden("呉れていかない", "呉れる", "v1-s", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v1_s() {
    assert_golden("呉れていった", "呉れる", "v1-s", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v1_s() {
    assert_golden("呉れていかなかった", "呉れる", "v1-s", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v1_s() {
    assert_golden("呉れていって", "呉れる", "v1-s", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v1_s() {
    assert_golden("呉れていこう", "呉れる", "v1-s", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v1_s() {
    assert_golden("呉れていける", "呉れる", "v1-s", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v1_s() {
    assert_golden("呉れていかれる", "呉れる", "v1-s", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v1_s() {
    assert_golden("呉れていかせる", "呉れる", "v1-s", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v1_s() {
    assert_golden("呉れてくる", "呉れる", "v1-s", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v1_s() {
    assert_golden("呉れてこない", "呉れる", "v1-s", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v1_s() {
    assert_golden("呉れてきた", "呉れる", "v1-s", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v1_s() {
    assert_golden("呉れてこなかった", "呉れる", "v1-s", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v1_s() {
    assert_golden("呉れてきて", "呉れる", "v1-s", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v1_s() {
    assert_golden("呉れてくれば", "呉れる", "v1-s", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v1_s() {
    assert_golden("呉れてきたら", "呉れる", "v1-s", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v1_s() {
    assert_golden("呉れてこられる", "呉れる", "v1-s", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v1_s() {
    assert_golden("呉れてこさせる", "呉れる", "v1-s", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v1_s() {
    assert_golden("呉れながら", "呉れる", "v1-s", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v1_s() {
    assert_golden("呉れすぎる", "呉れる", "v1-s", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v1_s() {
    assert_golden("呉れそう", "呉れる", "v1-s", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v1_s() {
    assert_golden("呉れぬ", "呉れる", "v1-s", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v1_s() {
    assert_golden("呉れず", "呉れる", "v1-s", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v1_s() {
    assert_golden("呉れずに", "呉れる", "v1-s", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v1_s() {
    assert_golden("呉れたり", "呉れる", "v1-s", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v1_s() {
    assert_golden("呉れなかったり", "呉れる", "v1-s", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v1_s() {
    assert_golden("呉れん", "呉れる", "v1-s", "～slurred; slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v1_s() {
    assert_golden("呉れんかった", "呉れる", "v1-s", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v1_s() {
    assert_golden("呉れざる", "呉れる", "v1-s", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_affirmative_v1_s() {
    assert_golden("呉れれる", "呉れる", "v1-s", "～potential");
}

#[test]
fn deconjugate_polite_non_past_colloquial_potential_affirmative_v1_s() {
    assert_golden("呉れれます", "呉れる", "v1-s", "～potential→polite");
}

#[test]
fn deconjugate_plain_past_colloquial_potential_affirmative_v1_s() {
    assert_golden("呉れれた", "呉れる", "v1-s", "～potential→past");
}

#[test]
fn deconjugate_polite_past_colloquial_potential_affirmative_v1_s() {
    assert_golden("呉れれました", "呉れる", "v1-s", "～potential→polite past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_negative_v1_s() {
    assert_golden("呉れれない", "呉れる", "v1-s", "～potential→negative");
}

#[test]
fn deconjugate_polite_non_past_colloquial_potential_negative_v1_s() {
    assert_golden("呉れれません", "呉れる", "v1-s", "～potential→polite negative");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_volitional_v1_s() {
    assert_golden("呉れれよう", "呉れる", "v1-s", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_colloquial_potential_volitional_v1_s() {
    assert_golden("呉れれよ", "呉れる", "v1-s", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_imperative_v1_s() {
    assert_golden("呉れれろ", "呉れる", "v1-s", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_te_form_v1_s() {
    assert_golden("呉れれて", "呉れる", "v1-s", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_temporal_conditional_v1_s() {
    assert_golden("呉れれたら", "呉れる", "v1-s", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_provisional_conditional_v1_s() {
    assert_golden("呉れれれば", "呉れる", "v1-s", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_passive_potential_v1_s() {
    assert_golden("呉れれられる", "呉れる", "v1-s", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_causative_v1_s() {
    assert_golden("呉れれさせる", "呉れる", "v1-s", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v1_s() {
    assert_golden("呉れてあげる", "呉れる", "v1-s", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v1_s() {
    assert_golden("呉れてあげられる", "呉れる", "v1-s", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v1_s() {
    assert_golden("呉れておる", "呉れる", "v1-s", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v1_s() {
    assert_golden("呉れておらない", "呉れる", "v1-s", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v1_s() {
    assert_golden("呉れておらん", "呉れる", "v1-s", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v1_s() {
    assert_golden("呉れておった", "呉れる", "v1-s", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v1_s() {
    assert_golden("呉れておらなかった", "呉れる", "v1-s", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v1_s() {
    assert_golden("呉れております", "呉れる", "v1-s", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v1_s() {
    assert_golden("呉れておりません", "呉れる", "v1-s", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v1_s() {
    assert_golden("呉れておりました", "呉れる", "v1-s", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v1_s() {
    assert_golden("呉れておりませんでした", "呉れる", "v1-s", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v1_s() {
    assert_golden("呉れておって", "呉れる", "v1-s", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v1_s() {
    assert_golden("呉れておろう", "呉れる", "v1-s", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v1_s() {
    assert_golden("呉れておれる", "呉れる", "v1-s", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v1_s() {
    assert_golden("呉れておられる", "呉れる", "v1-s", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v1_s() {
    assert_golden("呉れとる", "呉れる", "v1-s", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v1_s() {
    assert_golden("呉れとらない", "呉れる", "v1-s", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v1_s() {
    assert_golden("呉れとらん", "呉れる", "v1-s", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v1_s() {
    assert_golden("呉れとった", "呉れる", "v1-s", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v1_s() {
    assert_golden("呉れとらなかった", "呉れる", "v1-s", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v1_s() {
    assert_golden("呉れとります", "呉れる", "v1-s", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v1_s() {
    assert_golden("呉れとりません", "呉れる", "v1-s", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v1_s() {
    assert_golden("呉れとりました", "呉れる", "v1-s", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v1_s() {
    assert_golden("呉れとりませんでした", "呉れる", "v1-s", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v1_s() {
    assert_golden("呉れとって", "呉れる", "v1-s", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v1_s() {
    assert_golden("呉れとろう", "呉れる", "v1-s", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v1_s() {
    assert_golden("呉れとれる", "呉れる", "v1-s", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v1_s() {
    assert_golden("呉れとられる", "呉れる", "v1-s", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v1_s() {
    assert_golden("呉れさす", "呉れる", "v1-s", "～short causative");
}

#[test]
fn deconjugate_plain_non_past_na_v1_s() {
    assert_golden("呉れな", "呉れる", "v1-s", "～casual polite imperative");
}

#[test]
fn deconjugate_topic_or_condition_v1_s() {
    assert_golden("呉れては", "呉れる", "v1-s", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v1_s() {
    assert_golden("呉れちゃ", "呉れる", "v1-s", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v1_s() {
    assert_golden("呉れなきゃ", "呉れる", "v1-s", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v1_s() {
    assert_golden("呉れちまう", "呉れる", "v1-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v1_s() {
    assert_golden("呉れちゃう", "呉れる", "v1-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v1_s() {
    assert_golden("呉れていらっしゃる", "呉れる", "v1-s", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v1_s() {
    assert_golden("呉れていらっしゃらない", "呉れる", "v1-s", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v1_s() {
    assert_golden("呉れつつ", "呉れる", "v1-s", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v1_s() {
    assert_golden("呉れてくれる", "呉れる", "v1-s", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v1_s() {
    assert_golden("呉れてくれない", "呉れる", "v1-s", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v1_s() {
    assert_golden("呉れてくれます", "呉れる", "v1-s", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v1_s() {
    assert_golden("呉れてくれません", "呉れる", "v1-s", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v1_s() {
    assert_golden("呉れてくれ", "呉れる", "v1-s", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v1_s() {
    assert_golden("呉れへん", "呉れる", "v1-s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v1_s() {
    assert_golden("呉れへんかった", "呉れる", "v1-s", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v1_s() {
    assert_golden("呉れひん", "呉れる", "v1-s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v1_s() {
    assert_golden("呉れひんかった", "呉れる", "v1-s", "～negative→ksb→past");
}

#[test]
fn deconjugate_kansaiben_imperative_v1_s() {
    assert_golden("呉れい", "呉れる", "v1-s", "～imperative (ksb)");
}

#[test]
fn deconjugate_contracted_provisional_conditional_rya_v1_s() {
    assert_golden("呉れりゃ", "呉れる", "v1-s", "～provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v1_s() {
    assert_golden("呉れささない", "呉れる", "v1-s", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v1_s() {
    assert_golden("呉れましたら", "呉れる", "v1-s", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v1_s() {
    assert_golden("呉れになる", "呉れる", "v1-s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v1_s() {
    assert_golden("呉れなさる", "呉れる", "v1-s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v1_s() {
    assert_golden("呉れはる", "呉れる", "v1-s", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v1_s() {
    assert_golden("呉れなさるな", "呉れる", "v1-s", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v1_s() {
    assert_golden("呉れまい", "呉れる", "v1-s", "～negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_2_v1_s() {
    assert_golden("呉れるまい", "呉れる", "v1-s", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v1_s() {
    assert_golden("呉れますまい", "呉れる", "v1-s", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v1_s() {
    assert_golden("呉れねば", "呉れる", "v1-s", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v1_s() {
    assert_golden("呉れにゃ", "呉れる", "v1-s", "～colloquial negative conditional");
}
