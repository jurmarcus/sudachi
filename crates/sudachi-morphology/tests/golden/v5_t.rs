//! Golden tests ported from JL's `DeconjugatorTestsForV5T.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V5T.

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
fn deconjugate_masu_stem_v5_t() {
    assert_golden("育ち", "育つ", "v5t", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_t() {
    assert_golden("育たない", "育つ", "v5t", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_t() {
    assert_golden("育ちます", "育つ", "v5t", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_t() {
    assert_golden("育ちましょう", "育つ", "v5t", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_t() {
    assert_golden("育ちません", "育つ", "v5t", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_t() {
    assert_golden("育った", "育つ", "v5t", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_t() {
    assert_golden("育たなかった", "育つ", "v5t", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_t() {
    assert_golden("育ちました", "育つ", "v5t", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_t() {
    assert_golden("育ちませんでした", "育つ", "v5t", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_t() {
    assert_golden("育って", "育つ", "v5t", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_t() {
    assert_golden("育たなくて", "育つ", "v5t", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_t() {
    assert_golden("育たないで", "育つ", "v5t", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_t() {
    assert_golden("育ちまして", "育つ", "v5t", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_t() {
    assert_golden("育てる", "育つ", "v5t", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_t() {
    assert_golden("育たれる", "育つ", "v5t", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_t() {
    assert_golden("育てない", "育つ", "v5t", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_t() {
    assert_golden("育たれない", "育つ", "v5t", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_t() {
    assert_golden("育てた", "育つ", "v5t", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_t() {
    assert_golden("育たれた", "育つ", "v5t", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_t() {
    assert_golden("育てました", "育つ", "v5t", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_t() {
    assert_golden("育たれました", "育つ", "v5t", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_t() {
    assert_golden("育てなかった", "育つ", "v5t", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_t() {
    assert_golden("育たれなかった", "育つ", "v5t", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_t() {
    assert_golden("育てませんでした", "育つ", "v5t", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_t() {
    assert_golden("育たれませんでした", "育つ", "v5t", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_t() {
    assert_golden("育てます", "育つ", "v5t", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_t() {
    assert_golden("育たれます", "育つ", "v5t", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_t() {
    assert_golden("育てません", "育つ", "v5t", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_t() {
    assert_golden("育たれません", "育つ", "v5t", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_t() {
    assert_golden("育て", "育つ", "v5t", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_t() {
    assert_golden("育つな", "育つ", "v5t", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_t() {
    assert_golden("育ちなさい", "育つ", "v5t", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_t() {
    assert_golden("育ってください", "育つ", "v5t", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_t() {
    assert_golden("育たないでください", "育つ", "v5t", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_t() {
    assert_golden("育とう", "育つ", "v5t", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_t() {
    assert_golden("育と", "育つ", "v5t", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_t() {
    assert_golden("育ちましょう", "育つ", "v5t", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_t() {
    assert_golden("育てば", "育つ", "v5t", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_t() {
    assert_golden("育たなければ", "育つ", "v5t", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_t() {
    assert_golden("育ったら", "育つ", "v5t", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_t() {
    assert_golden("育ったらば", "育つ", "v5t", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_t() {
    assert_golden("育たなかったら", "育つ", "v5t", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_t() {
    assert_golden("育たせる", "育つ", "v5t", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_t() {
    assert_golden("育たせない", "育つ", "v5t", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_t() {
    assert_golden("育たせん", "育つ", "v5t", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_t() {
    assert_golden("育たせます", "育つ", "v5t", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_t() {
    assert_golden("育たします", "育つ", "v5t", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_t() {
    assert_golden("育たせません", "育つ", "v5t", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_t() {
    assert_golden("育たせた", "育つ", "v5t", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_t() {
    assert_golden("育たせなかった", "育つ", "v5t", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_t() {
    assert_golden("育たせました", "育つ", "v5t", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_t() {
    assert_golden("育たせませんでした", "育つ", "v5t", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_t() {
    assert_golden("育たせられる", "育つ", "v5t", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_t() {
    assert_golden("育たせられない", "育つ", "v5t", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_t() {
    assert_golden("育たせられます", "育つ", "v5t", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_t() {
    assert_golden("育たせられません", "育つ", "v5t", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_t() {
    assert_golden("育ちたい", "育つ", "v5t", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_t() {
    assert_golden("育ちたくありません", "育つ", "v5t", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_t() {
    assert_golden("育ちたくありませんでした", "育つ", "v5t", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_t() {
    assert_golden("育ちたくない", "育つ", "v5t", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_t() {
    assert_golden("育ちたかった", "育つ", "v5t", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_t() {
    assert_golden("育ちたくなかった", "育つ", "v5t", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_t() {
    assert_golden("育っている", "育つ", "v5t", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_t() {
    assert_golden("育っていない", "育つ", "v5t", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_t() {
    assert_golden("育っていた", "育つ", "v5t", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_t() {
    assert_golden("育っていなかった", "育つ", "v5t", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_t() {
    assert_golden("育っています", "育つ", "v5t", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_t() {
    assert_golden("育っていません", "育つ", "v5t", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_t() {
    assert_golden("育っていました", "育つ", "v5t", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_t() {
    assert_golden("育っていませんでした", "育つ", "v5t", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_t() {
    assert_golden("育ってる", "育つ", "v5t", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_t() {
    assert_golden("育ってない", "育つ", "v5t", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_t() {
    assert_golden("育ってた", "育つ", "v5t", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_t() {
    assert_golden("育ってなかった", "育つ", "v5t", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_t() {
    assert_golden("育ってます", "育つ", "v5t", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_t() {
    assert_golden("育ってません", "育つ", "v5t", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_t() {
    assert_golden("育ってました", "育つ", "v5t", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_t() {
    assert_golden("育ってません", "育つ", "v5t", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_t() {
    assert_golden("育ってませんでした", "育つ", "v5t", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_t() {
    assert_golden("育ってしまう", "育つ", "v5t", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_t() {
    assert_golden("育ってもう", "育つ", "v5t", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_t() {
    assert_golden("育ってしまわない", "育つ", "v5t", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_t() {
    assert_golden("育ってしまった", "育つ", "v5t", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_t() {
    assert_golden("育ってしまわなかった", "育つ", "v5t", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_t() {
    assert_golden("育ってしまって", "育つ", "v5t", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_t() {
    assert_golden("育ってしまえば", "育つ", "v5t", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_t() {
    assert_golden("育ってしまわなければ", "育つ", "v5t", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_t() {
    assert_golden("育ってしまわなかったら", "育つ", "v5t", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_t() {
    assert_golden("育ってしまったら", "育つ", "v5t", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_t() {
    assert_golden("育ってしまおう", "育つ", "v5t", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_t() {
    assert_golden("育ってしまいます", "育つ", "v5t", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_t() {
    assert_golden("育ってしまいません", "育つ", "v5t", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_t() {
    assert_golden("育ってしまいました", "育つ", "v5t", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_t() {
    assert_golden("育ってしまいませんでした", "育つ", "v5t", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_t() {
    assert_golden("育ってしまえる", "育つ", "v5t", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_t() {
    assert_golden("育ってしまわれる", "育つ", "v5t", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_t() {
    assert_golden("育ってしまわせる", "育つ", "v5t", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_t() {
    assert_golden("育っちゃう", "育つ", "v5t", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_t() {
    assert_golden("育っちゃわない", "育つ", "v5t", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_t() {
    assert_golden("育っちゃった", "育つ", "v5t", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_t() {
    assert_golden("育っちゃわなかった", "育つ", "v5t", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_t() {
    assert_golden("育っちゃって", "育つ", "v5t", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_t() {
    assert_golden("育っちゃえば", "育つ", "v5t", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_t() {
    assert_golden("育っちゃわなければ", "育つ", "v5t", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_t() {
    assert_golden("育っちゃわなかったら", "育つ", "v5t", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_t() {
    assert_golden("育っちゃおう", "育つ", "v5t", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_t() {
    assert_golden("育っちゃえる", "育つ", "v5t", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_t() {
    assert_golden("育っておく", "育つ", "v5t", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_t() {
    assert_golden("育っておかない", "育つ", "v5t", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_t() {
    assert_golden("育っておいた", "育つ", "v5t", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_t() {
    assert_golden("育っておかなかった", "育つ", "v5t", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_t() {
    assert_golden("育っておいて", "育つ", "v5t", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_t() {
    assert_golden("育っておけば", "育つ", "v5t", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_t() {
    assert_golden("育っておいたら", "育つ", "v5t", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_t() {
    assert_golden("育っておこう", "育つ", "v5t", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_t() {
    assert_golden("育っておける", "育つ", "v5t", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_t() {
    assert_golden("育っておかれる", "育つ", "v5t", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_t() {
    assert_golden("育っとく", "育つ", "v5t", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_t() {
    assert_golden("育っとかない", "育つ", "v5t", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_t() {
    assert_golden("育っといた", "育つ", "v5t", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_t() {
    assert_golden("育っとかなかった", "育つ", "v5t", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_t() {
    assert_golden("育っといて", "育つ", "v5t", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_t() {
    assert_golden("育っとけば", "育つ", "v5t", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_t() {
    assert_golden("育っといたら", "育つ", "v5t", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_t() {
    assert_golden("育っとこう", "育つ", "v5t", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_t() {
    assert_golden("育っとける", "育つ", "v5t", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_t() {
    assert_golden("育っとかれる", "育つ", "v5t", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_t() {
    assert_golden("育ってある", "育つ", "v5t", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_t() {
    assert_golden("育ってあった", "育つ", "v5t", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_t() {
    assert_golden("育ってあって", "育つ", "v5t", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_t() {
    assert_golden("育ってあったら", "育つ", "v5t", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_t() {
    assert_golden("育ってあれば", "育つ", "v5t", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_t() {
    assert_golden("育っていく", "育つ", "v5t", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_t() {
    assert_golden("育っていかない", "育つ", "v5t", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_t() {
    assert_golden("育っていった", "育つ", "v5t", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_t() {
    assert_golden("育っていかなかった", "育つ", "v5t", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_t() {
    assert_golden("育っていって", "育つ", "v5t", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_t() {
    assert_golden("育っていこう", "育つ", "v5t", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_t() {
    assert_golden("育っていける", "育つ", "v5t", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_t() {
    assert_golden("育っていかれる", "育つ", "v5t", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_t() {
    assert_golden("育っていかせる", "育つ", "v5t", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_t() {
    assert_golden("育ってくる", "育つ", "v5t", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_t() {
    assert_golden("育ってこない", "育つ", "v5t", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_t() {
    assert_golden("育ってきた", "育つ", "v5t", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_t() {
    assert_golden("育ってこなかった", "育つ", "v5t", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_t() {
    assert_golden("育ってきて", "育つ", "v5t", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_t() {
    assert_golden("育ってくれば", "育つ", "v5t", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_t() {
    assert_golden("育ってきたら", "育つ", "v5t", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_t() {
    assert_golden("育ってこられる", "育つ", "v5t", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_t() {
    assert_golden("育ってこさせる", "育つ", "v5t", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_t() {
    assert_golden("育ちながら", "育つ", "v5t", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_t() {
    assert_golden("育ちすぎる", "育つ", "v5t", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_t() {
    assert_golden("育ちそう", "育つ", "v5t", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_t() {
    assert_golden("育たぬ", "育つ", "v5t", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_t() {
    assert_golden("育たず", "育つ", "v5t", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_t() {
    assert_golden("育たずに", "育つ", "v5t", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_t() {
    assert_golden("育ったり", "育つ", "v5t", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_t() {
    assert_golden("育たなかったり", "育つ", "v5t", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_t() {
    assert_golden("育たん", "育つ", "v5t", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_t() {
    assert_golden("育たんかった", "育つ", "v5t", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_t() {
    assert_golden("育たざる", "育つ", "v5t", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_t() {
    assert_golden("育てよう", "育つ", "v5t", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_t() {
    assert_golden("育てよ", "育つ", "v5t", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_t() {
    assert_golden("育てろ", "育つ", "v5t", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_t() {
    assert_golden("育てて", "育つ", "v5t", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_t() {
    assert_golden("育てたら", "育つ", "v5t", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_t() {
    assert_golden("育てれば", "育つ", "v5t", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_t() {
    assert_golden("育てられる", "育つ", "v5t", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_t() {
    assert_golden("育てさせる", "育つ", "v5t", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_t() {
    assert_golden("育ってあげる", "育つ", "v5t", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_t() {
    assert_golden("育ってあげられる", "育つ", "v5t", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_t() {
    assert_golden("育っておる", "育つ", "v5t", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_t() {
    assert_golden("育っておらない", "育つ", "v5t", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_t() {
    assert_golden("育っておらん", "育つ", "v5t", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_t() {
    assert_golden("育っておった", "育つ", "v5t", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_t() {
    assert_golden("育っておらなかった", "育つ", "v5t", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_t() {
    assert_golden("育っております", "育つ", "v5t", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_t() {
    assert_golden("育っておりません", "育つ", "v5t", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_t() {
    assert_golden("育っておりました", "育つ", "v5t", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_t() {
    assert_golden("育っておりませんでした", "育つ", "v5t", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_t() {
    assert_golden("育っておって", "育つ", "v5t", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_t() {
    assert_golden("育っておろう", "育つ", "v5t", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_t() {
    assert_golden("育っておれる", "育つ", "v5t", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_t() {
    assert_golden("育っておられる", "育つ", "v5t", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_t() {
    assert_golden("育っとる", "育つ", "v5t", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_t() {
    assert_golden("育っとらない", "育つ", "v5t", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_t() {
    assert_golden("育っとらん", "育つ", "v5t", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_t() {
    assert_golden("育っとった", "育つ", "v5t", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_t() {
    assert_golden("育っとらなかった", "育つ", "v5t", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_t() {
    assert_golden("育っとります", "育つ", "v5t", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_t() {
    assert_golden("育っとりません", "育つ", "v5t", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_t() {
    assert_golden("育っとりました", "育つ", "v5t", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_t() {
    assert_golden("育っとりませんでした", "育つ", "v5t", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_t() {
    assert_golden("育っとって", "育つ", "v5t", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_t() {
    assert_golden("育っとろう", "育つ", "v5t", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_t() {
    assert_golden("育っとれる", "育つ", "v5t", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_t() {
    assert_golden("育っとられる", "育つ", "v5t", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_t() {
    assert_golden("育たす", "育つ", "v5t", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_t() {
    assert_golden("育っては", "育つ", "v5t", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_t() {
    assert_golden("育っちゃ", "育つ", "v5t", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_t() {
    assert_golden("育たなきゃ", "育つ", "v5t", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_t() {
    assert_golden("育っちまう", "育つ", "v5t", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_t() {
    assert_golden("育っちゃう", "育つ", "v5t", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_t() {
    assert_golden("育っていらっしゃる", "育つ", "v5t", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_t() {
    assert_golden("育っていらっしゃらない", "育つ", "v5t", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_t() {
    assert_golden("育ちつつ", "育つ", "v5t", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_t() {
    assert_golden("育ってくれる", "育つ", "v5t", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_t() {
    assert_golden("育ってくれない", "育つ", "v5t", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_t() {
    assert_golden("育ってくれます", "育つ", "v5t", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_t() {
    assert_golden("育ってくれません", "育つ", "v5t", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_t() {
    assert_golden("育ってくれ", "育つ", "v5t", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_t() {
    assert_golden("育たへん", "育つ", "v5t", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_t() {
    assert_golden("育たへんかった", "育つ", "v5t", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_t() {
    assert_golden("育たひん", "育つ", "v5t", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_t() {
    assert_golden("育たひんかった", "育つ", "v5t", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_t() {
    assert_golden("育たさない", "育つ", "v5t", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_t() {
    assert_golden("育ちましたら", "育つ", "v5t", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_t() {
    assert_golden("育ちになる", "育つ", "v5t", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_t() {
    assert_golden("育ちなさる", "育つ", "v5t", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_t() {
    assert_golden("育ちはる", "育つ", "v5t", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_t() {
    assert_golden("育ちなさるな", "育つ", "v5t", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_t() {
    assert_golden("育つまい", "育つ", "v5t", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_t() {
    assert_golden("育ちますまい", "育つ", "v5t", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_t() {
    assert_golden("育たば", "育つ", "v5t", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_t() {
    assert_golden("育たねば", "育つ", "v5t", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_t() {
    assert_golden("育たにゃ", "育つ", "v5t", "～colloquial negative conditional");
}
