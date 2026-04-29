//! Golden tests ported from JL's `DeconjugatorTestsForV4R.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V4R.

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
fn deconjugate_masu_stem_v4_r() {
    assert_golden("おじゃり", "おじゃる", "v4r", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v4_r() {
    assert_golden("おじゃらない", "おじゃる", "v4r", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v4_r() {
    assert_golden("おじゃります", "おじゃる", "v4r", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v4_r() {
    assert_golden("おじゃりましょう", "おじゃる", "v4r", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v4_r() {
    assert_golden("おじゃりません", "おじゃる", "v4r", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v4_r() {
    assert_golden("おじゃった", "おじゃる", "v4r", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v4_r() {
    assert_golden("おじゃらなかった", "おじゃる", "v4r", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v4_r() {
    assert_golden("おじゃりました", "おじゃる", "v4r", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v4_r() {
    assert_golden("おじゃりませんでした", "おじゃる", "v4r", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v4_r() {
    assert_golden("おじゃって", "おじゃる", "v4r", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v4_r() {
    assert_golden("おじゃらなくて", "おじゃる", "v4r", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v4_r() {
    assert_golden("おじゃらないで", "おじゃる", "v4r", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v4_r() {
    assert_golden("おじゃりまして", "おじゃる", "v4r", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v4_r() {
    assert_golden("おじゃれる", "おじゃる", "v4r", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v4_r() {
    assert_golden("おじゃられる", "おじゃる", "v4r", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v4_r() {
    assert_golden("おじゃれない", "おじゃる", "v4r", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v4_r() {
    assert_golden("おじゃられない", "おじゃる", "v4r", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v4_r() {
    assert_golden("おじゃれた", "おじゃる", "v4r", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v4_r() {
    assert_golden("おじゃられた", "おじゃる", "v4r", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v4_r() {
    assert_golden("おじゃれました", "おじゃる", "v4r", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v4_r() {
    assert_golden("おじゃられました", "おじゃる", "v4r", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v4_r() {
    assert_golden("おじゃれなかった", "おじゃる", "v4r", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v4_r() {
    assert_golden("おじゃられなかった", "おじゃる", "v4r", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v4_r() {
    assert_golden("おじゃれませんでした", "おじゃる", "v4r", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v4_r() {
    assert_golden("おじゃられませんでした", "おじゃる", "v4r", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v4_r() {
    assert_golden("おじゃれます", "おじゃる", "v4r", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v4_r() {
    assert_golden("おじゃられます", "おじゃる", "v4r", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v4_r() {
    assert_golden("おじゃれません", "おじゃる", "v4r", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v4_r() {
    assert_golden("おじゃられません", "おじゃる", "v4r", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v4_r() {
    assert_golden("おじゃれ", "おじゃる", "v4r", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v4_r() {
    assert_golden("おじゃるな", "おじゃる", "v4r", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v4_r() {
    assert_golden("おじゃりなさい", "おじゃる", "v4r", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v4_r() {
    assert_golden("おじゃってください", "おじゃる", "v4r", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v4_r() {
    assert_golden("おじゃらないでください", "おじゃる", "v4r", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v4_r() {
    assert_golden("おじゃろう", "おじゃる", "v4r", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v4_r() {
    assert_golden("おじゃろ", "おじゃる", "v4r", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v4_r() {
    assert_golden("おじゃりましょう", "おじゃる", "v4r", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v4_r() {
    assert_golden("おじゃれば", "おじゃる", "v4r", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v4_r() {
    assert_golden("おじゃらなければ", "おじゃる", "v4r", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v4_r() {
    assert_golden("おじゃったら", "おじゃる", "v4r", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v4_r() {
    assert_golden("おじゃったらば", "おじゃる", "v4r", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v4_r() {
    assert_golden("おじゃらなかったら", "おじゃる", "v4r", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v4_r() {
    assert_golden("おじゃらせる", "おじゃる", "v4r", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v4_r() {
    assert_golden("おじゃらせない", "おじゃる", "v4r", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v4_r() {
    assert_golden("おじゃらせん", "おじゃる", "v4r", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v4_r() {
    assert_golden("おじゃらせます", "おじゃる", "v4r", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v4_r() {
    assert_golden("おじゃらします", "おじゃる", "v4r", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v4_r() {
    assert_golden("おじゃらせません", "おじゃる", "v4r", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v4_r() {
    assert_golden("おじゃらせた", "おじゃる", "v4r", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v4_r() {
    assert_golden("おじゃらせなかった", "おじゃる", "v4r", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v4_r() {
    assert_golden("おじゃらせました", "おじゃる", "v4r", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v4_r() {
    assert_golden("おじゃらせませんでした", "おじゃる", "v4r", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v4_r() {
    assert_golden("おじゃらせられる", "おじゃる", "v4r", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v4_r() {
    assert_golden("おじゃらせられない", "おじゃる", "v4r", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v4_r() {
    assert_golden("おじゃらせられます", "おじゃる", "v4r", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v4_r() {
    assert_golden("おじゃらせられません", "おじゃる", "v4r", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v4_r() {
    assert_golden("おじゃりたい", "おじゃる", "v4r", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v4_r() {
    assert_golden("おじゃりたくありません", "おじゃる", "v4r", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v4_r() {
    assert_golden("おじゃりたくありませんでした", "おじゃる", "v4r", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v4_r() {
    assert_golden("おじゃりたくない", "おじゃる", "v4r", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v4_r() {
    assert_golden("おじゃりたかった", "おじゃる", "v4r", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v4_r() {
    assert_golden("おじゃりたくなかった", "おじゃる", "v4r", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v4_r() {
    assert_golden("おじゃっている", "おじゃる", "v4r", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v4_r() {
    assert_golden("おじゃっていない", "おじゃる", "v4r", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v4_r() {
    assert_golden("おじゃっていた", "おじゃる", "v4r", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v4_r() {
    assert_golden("おじゃっていなかった", "おじゃる", "v4r", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v4_r() {
    assert_golden("おじゃっています", "おじゃる", "v4r", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v4_r() {
    assert_golden("おじゃっていません", "おじゃる", "v4r", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v4_r() {
    assert_golden("おじゃっていました", "おじゃる", "v4r", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v4_r() {
    assert_golden("おじゃっていませんでした", "おじゃる", "v4r", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v4_r() {
    assert_golden("おじゃってる", "おじゃる", "v4r", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v4_r() {
    assert_golden("おじゃってない", "おじゃる", "v4r", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v4_r() {
    assert_golden("おじゃってた", "おじゃる", "v4r", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v4_r() {
    assert_golden("おじゃってなかった", "おじゃる", "v4r", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v4_r() {
    assert_golden("おじゃってます", "おじゃる", "v4r", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v4_r() {
    assert_golden("おじゃってません", "おじゃる", "v4r", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v4_r() {
    assert_golden("おじゃってました", "おじゃる", "v4r", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v4_r() {
    assert_golden("おじゃってません", "おじゃる", "v4r", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v4_r() {
    assert_golden("おじゃってませんでした", "おじゃる", "v4r", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v4_r() {
    assert_golden("おじゃってしまう", "おじゃる", "v4r", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v4_r() {
    assert_golden("おじゃってもう", "おじゃる", "v4r", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v4_r() {
    assert_golden("おじゃってしまわない", "おじゃる", "v4r", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v4_r() {
    assert_golden("おじゃってしまった", "おじゃる", "v4r", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v4_r() {
    assert_golden("おじゃってしまわなかった", "おじゃる", "v4r", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v4_r() {
    assert_golden("おじゃってしまって", "おじゃる", "v4r", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v4_r() {
    assert_golden("おじゃってしまえば", "おじゃる", "v4r", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v4_r() {
    assert_golden("おじゃってしまわなければ", "おじゃる", "v4r", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v4_r() {
    assert_golden("おじゃってしまわなかったら", "おじゃる", "v4r", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v4_r() {
    assert_golden("おじゃってしまったら", "おじゃる", "v4r", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v4_r() {
    assert_golden("おじゃってしまおう", "おじゃる", "v4r", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v4_r() {
    assert_golden("おじゃってしまいます", "おじゃる", "v4r", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v4_r() {
    assert_golden("おじゃってしまいません", "おじゃる", "v4r", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v4_r() {
    assert_golden("おじゃってしまいました", "おじゃる", "v4r", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v4_r() {
    assert_golden("おじゃってしまいませんでした", "おじゃる", "v4r", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v4_r() {
    assert_golden("おじゃってしまえる", "おじゃる", "v4r", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v4_r() {
    assert_golden("おじゃってしまわれる", "おじゃる", "v4r", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v4_r() {
    assert_golden("おじゃってしまわせる", "おじゃる", "v4r", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v4_r() {
    assert_golden("おじゃっちゃう", "おじゃる", "v4r", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v4_r() {
    assert_golden("おじゃっちゃわない", "おじゃる", "v4r", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v4_r() {
    assert_golden("おじゃっちゃった", "おじゃる", "v4r", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v4_r() {
    assert_golden("おじゃっちゃわなかった", "おじゃる", "v4r", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v4_r() {
    assert_golden("おじゃっちゃって", "おじゃる", "v4r", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v4_r() {
    assert_golden("おじゃっちゃえば", "おじゃる", "v4r", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v4_r() {
    assert_golden("おじゃっちゃわなければ", "おじゃる", "v4r", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v4_r() {
    assert_golden("おじゃっちゃわなかったら", "おじゃる", "v4r", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v4_r() {
    assert_golden("おじゃっちゃおう", "おじゃる", "v4r", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v4_r() {
    assert_golden("おじゃっちゃえる", "おじゃる", "v4r", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v4_r() {
    assert_golden("おじゃっておく", "おじゃる", "v4r", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v4_r() {
    assert_golden("おじゃっておかない", "おじゃる", "v4r", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v4_r() {
    assert_golden("おじゃっておいた", "おじゃる", "v4r", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v4_r() {
    assert_golden("おじゃっておかなかった", "おじゃる", "v4r", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v4_r() {
    assert_golden("おじゃっておいて", "おじゃる", "v4r", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v4_r() {
    assert_golden("おじゃっておけば", "おじゃる", "v4r", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v4_r() {
    assert_golden("おじゃっておいたら", "おじゃる", "v4r", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v4_r() {
    assert_golden("おじゃっておこう", "おじゃる", "v4r", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v4_r() {
    assert_golden("おじゃっておける", "おじゃる", "v4r", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v4_r() {
    assert_golden("おじゃっておかれる", "おじゃる", "v4r", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v4_r() {
    assert_golden("おじゃっとく", "おじゃる", "v4r", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v4_r() {
    assert_golden("おじゃっとかない", "おじゃる", "v4r", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v4_r() {
    assert_golden("おじゃっといた", "おじゃる", "v4r", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v4_r() {
    assert_golden("おじゃっとかなかった", "おじゃる", "v4r", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v4_r() {
    assert_golden("おじゃっといて", "おじゃる", "v4r", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v4_r() {
    assert_golden("おじゃっとけば", "おじゃる", "v4r", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v4_r() {
    assert_golden("おじゃっといたら", "おじゃる", "v4r", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v4_r() {
    assert_golden("おじゃっとこう", "おじゃる", "v4r", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v4_r() {
    assert_golden("おじゃっとける", "おじゃる", "v4r", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v4_r() {
    assert_golden("おじゃっとかれる", "おじゃる", "v4r", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v4_r() {
    assert_golden("おじゃってある", "おじゃる", "v4r", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v4_r() {
    assert_golden("おじゃってあった", "おじゃる", "v4r", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v4_r() {
    assert_golden("おじゃってあって", "おじゃる", "v4r", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v4_r() {
    assert_golden("おじゃってあったら", "おじゃる", "v4r", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v4_r() {
    assert_golden("おじゃってあれば", "おじゃる", "v4r", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v4_r() {
    assert_golden("おじゃっていく", "おじゃる", "v4r", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v4_r() {
    assert_golden("おじゃっていかない", "おじゃる", "v4r", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v4_r() {
    assert_golden("おじゃっていった", "おじゃる", "v4r", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v4_r() {
    assert_golden("おじゃっていかなかった", "おじゃる", "v4r", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v4_r() {
    assert_golden("おじゃっていって", "おじゃる", "v4r", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v4_r() {
    assert_golden("おじゃっていこう", "おじゃる", "v4r", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v4_r() {
    assert_golden("おじゃっていける", "おじゃる", "v4r", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v4_r() {
    assert_golden("おじゃっていかれる", "おじゃる", "v4r", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v4_r() {
    assert_golden("おじゃっていかせる", "おじゃる", "v4r", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v4_r() {
    assert_golden("おじゃってくる", "おじゃる", "v4r", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v4_r() {
    assert_golden("おじゃってこない", "おじゃる", "v4r", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v4_r() {
    assert_golden("おじゃってきた", "おじゃる", "v4r", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v4_r() {
    assert_golden("おじゃってこなかった", "おじゃる", "v4r", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v4_r() {
    assert_golden("おじゃってきて", "おじゃる", "v4r", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v4_r() {
    assert_golden("おじゃってくれば", "おじゃる", "v4r", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v4_r() {
    assert_golden("おじゃってきたら", "おじゃる", "v4r", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v4_r() {
    assert_golden("おじゃってこられる", "おじゃる", "v4r", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v4_r() {
    assert_golden("おじゃってこさせる", "おじゃる", "v4r", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v4_r() {
    assert_golden("おじゃりながら", "おじゃる", "v4r", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v4_r() {
    assert_golden("おじゃりすぎる", "おじゃる", "v4r", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v4_r() {
    assert_golden("おじゃりそう", "おじゃる", "v4r", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v4_r() {
    assert_golden("おじゃらぬ", "おじゃる", "v4r", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v4_r() {
    assert_golden("おじゃらず", "おじゃる", "v4r", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v4_r() {
    assert_golden("おじゃらずに", "おじゃる", "v4r", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v4_r() {
    assert_golden("おじゃったり", "おじゃる", "v4r", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v4_r() {
    assert_golden("おじゃらなかったり", "おじゃる", "v4r", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v4_r() {
    assert_golden("おじゃらん", "おじゃる", "v4r", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v4_r() {
    assert_golden("おじゃらんかった", "おじゃる", "v4r", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v4_r() {
    assert_golden("おじゃらざる", "おじゃる", "v4r", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v4_r() {
    assert_golden("おじゃれよう", "おじゃる", "v4r", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v4_r() {
    assert_golden("おじゃれよ", "おじゃる", "v4r", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v4_r() {
    assert_golden("おじゃれろ", "おじゃる", "v4r", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v4_r() {
    assert_golden("おじゃれて", "おじゃる", "v4r", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v4_r() {
    assert_golden("おじゃれたら", "おじゃる", "v4r", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v4_r() {
    assert_golden("おじゃれれば", "おじゃる", "v4r", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v4_r() {
    assert_golden("おじゃれられる", "おじゃる", "v4r", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v4_r() {
    assert_golden("おじゃれさせる", "おじゃる", "v4r", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v4_r() {
    assert_golden("おじゃってあげる", "おじゃる", "v4r", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v4_r() {
    assert_golden("おじゃってあげられる", "おじゃる", "v4r", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v4_r() {
    assert_golden("おじゃっておる", "おじゃる", "v4r", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v4_r() {
    assert_golden("おじゃっておらない", "おじゃる", "v4r", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v4_r() {
    assert_golden("おじゃっておらん", "おじゃる", "v4r", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v4_r() {
    assert_golden("おじゃっておった", "おじゃる", "v4r", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v4_r() {
    assert_golden("おじゃっておらなかった", "おじゃる", "v4r", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v4_r() {
    assert_golden("おじゃっております", "おじゃる", "v4r", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v4_r() {
    assert_golden("おじゃっておりません", "おじゃる", "v4r", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v4_r() {
    assert_golden("おじゃっておりました", "おじゃる", "v4r", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v4_r() {
    assert_golden("おじゃっておりませんでした", "おじゃる", "v4r", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v4_r() {
    assert_golden("おじゃっておって", "おじゃる", "v4r", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v4_r() {
    assert_golden("おじゃっておろう", "おじゃる", "v4r", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v4_r() {
    assert_golden("おじゃっておれる", "おじゃる", "v4r", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v4_r() {
    assert_golden("おじゃっておられる", "おじゃる", "v4r", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v4_r() {
    assert_golden("おじゃっとる", "おじゃる", "v4r", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v4_r() {
    assert_golden("おじゃっとらない", "おじゃる", "v4r", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v4_r() {
    assert_golden("おじゃっとらん", "おじゃる", "v4r", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v4_r() {
    assert_golden("おじゃっとった", "おじゃる", "v4r", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v4_r() {
    assert_golden("おじゃっとらなかった", "おじゃる", "v4r", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v4_r() {
    assert_golden("おじゃっとります", "おじゃる", "v4r", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v4_r() {
    assert_golden("おじゃっとりません", "おじゃる", "v4r", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v4_r() {
    assert_golden("おじゃっとりました", "おじゃる", "v4r", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v4_r() {
    assert_golden("おじゃっとりませんでした", "おじゃる", "v4r", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v4_r() {
    assert_golden("おじゃっとって", "おじゃる", "v4r", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v4_r() {
    assert_golden("おじゃっとろう", "おじゃる", "v4r", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v4_r() {
    assert_golden("おじゃっとれる", "おじゃる", "v4r", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v4_r() {
    assert_golden("おじゃっとられる", "おじゃる", "v4r", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v4_r() {
    assert_golden("おじゃらす", "おじゃる", "v4r", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v4_r() {
    assert_golden("おじゃっては", "おじゃる", "v4r", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v4_r() {
    assert_golden("おじゃっちゃ", "おじゃる", "v4r", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v4_r() {
    assert_golden("おじゃらなきゃ", "おじゃる", "v4r", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v4_r() {
    assert_golden("おじゃっちまう", "おじゃる", "v4r", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v4_r() {
    assert_golden("おじゃっちゃう", "おじゃる", "v4r", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v4_r() {
    assert_golden("おじゃっていらっしゃる", "おじゃる", "v4r", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v4_r() {
    assert_golden("おじゃっていらっしゃらない", "おじゃる", "v4r", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v4_r() {
    assert_golden("おじゃりつつ", "おじゃる", "v4r", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v4_r() {
    assert_golden("おじゃってくれる", "おじゃる", "v4r", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v4_r() {
    assert_golden("おじゃってくれない", "おじゃる", "v4r", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v4_r() {
    assert_golden("おじゃってくれます", "おじゃる", "v4r", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v4_r() {
    assert_golden("おじゃってくれません", "おじゃる", "v4r", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v4_r() {
    assert_golden("おじゃってくれ", "おじゃる", "v4r", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v4_r() {
    assert_golden("おじゃらへん", "おじゃる", "v4r", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v4_r() {
    assert_golden("おじゃらへんかった", "おじゃる", "v4r", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v4_r() {
    assert_golden("おじゃらひん", "おじゃる", "v4r", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v4_r() {
    assert_golden("おじゃらひんかった", "おじゃる", "v4r", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v4_r() {
    assert_golden("おじゃらさない", "おじゃる", "v4r", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v4_r() {
    assert_golden("おじゃりましたら", "おじゃる", "v4r", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v4_r() {
    assert_golden("おじゃりになる", "おじゃる", "v4r", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v4_r() {
    assert_golden("おじゃりなさる", "おじゃる", "v4r", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v4_r() {
    assert_golden("おじゃりはる", "おじゃる", "v4r", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v4_r() {
    assert_golden("おじゃりなさるな", "おじゃる", "v4r", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v4_r() {
    assert_golden("おじゃるまい", "おじゃる", "v4r", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v4_r() {
    assert_golden("おじゃりますまい", "おじゃる", "v4r", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v4_r() {
    assert_golden("おじゃらば", "おじゃる", "v4r", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v4_r() {
    assert_golden("おじゃらねば", "おじゃる", "v4r", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v4_r() {
    assert_golden("おじゃらにゃ", "おじゃる", "v4r", "～colloquial negative conditional");
}
