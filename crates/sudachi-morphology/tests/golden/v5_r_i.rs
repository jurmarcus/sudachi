//! Golden tests ported from JL's `DeconjugatorTestsForV5RI.cs`.
//! 230 test cases proving deconjugator output matches
//! JL's expectations for class V5RI.

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
fn deconjugate_masu_stem_v5_ri() {
    assert_golden("有り", "有る", "v5r-i", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_ri() {
    assert_golden("ない", "有る", "v5r-i", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_ri() {
    assert_golden("有ります", "有る", "v5r-i", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_ri() {
    assert_golden("有りましょう", "有る", "v5r-i", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_ri() {
    assert_golden("有りません", "有る", "v5r-i", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_ri() {
    assert_golden("有った", "有る", "v5r-i", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_ri() {
    assert_golden("なかった", "有る", "v5r-i", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_ri() {
    assert_golden("有りました", "有る", "v5r-i", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_ri() {
    assert_golden("有りませんでした", "有る", "v5r-i", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_ri() {
    assert_golden("有って", "有る", "v5r-i", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_ri() {
    assert_golden("なくて", "有る", "v5r-i", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_ri() {
    assert_golden("ないで", "有る", "v5r-i", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_ri() {
    assert_golden("有りまして", "有る", "v5r-i", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_ri() {
    assert_golden("有れる", "有る", "v5r-i", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_ri() {
    assert_golden("有られる", "有る", "v5r-i", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_ri() {
    assert_golden("有れない", "有る", "v5r-i", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_ri() {
    assert_golden("有られない", "有る", "v5r-i", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_ri() {
    assert_golden("有れた", "有る", "v5r-i", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_ri() {
    assert_golden("有られた", "有る", "v5r-i", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_ri() {
    assert_golden("有れました", "有る", "v5r-i", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_ri() {
    assert_golden("有られました", "有る", "v5r-i", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_ri() {
    assert_golden("有れなかった", "有る", "v5r-i", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_ri() {
    assert_golden("有られなかった", "有る", "v5r-i", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_ri() {
    assert_golden("有れませんでした", "有る", "v5r-i", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_ri() {
    assert_golden("有られませんでした", "有る", "v5r-i", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_ri() {
    assert_golden("有れます", "有る", "v5r-i", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_ri() {
    assert_golden("有られます", "有る", "v5r-i", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_ri() {
    assert_golden("有れません", "有る", "v5r-i", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_ri() {
    assert_golden("有られません", "有る", "v5r-i", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_ri() {
    assert_golden("有れ", "有る", "v5r-i", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_ri() {
    assert_golden("有るな", "有る", "v5r-i", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_ri() {
    assert_golden("有りなさい", "有る", "v5r-i", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_ri() {
    assert_golden("有ってください", "有る", "v5r-i", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_ri() {
    assert_golden("ないでください", "有る", "v5r-i", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_ri() {
    assert_golden("有ろう", "有る", "v5r-i", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_ri() {
    assert_golden("有ろ", "有る", "v5r-i", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_ri() {
    assert_golden("有りましょう", "有る", "v5r-i", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_ri() {
    assert_golden("有れば", "有る", "v5r-i", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_ri() {
    assert_golden("なければ", "有る", "v5r-i", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_ri() {
    assert_golden("有ったら", "有る", "v5r-i", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_ri() {
    assert_golden("有ったらば", "有る", "v5r-i", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_ri() {
    assert_golden("なかったら", "有る", "v5r-i", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_ri() {
    assert_golden("有らせる", "有る", "v5r-i", "～causative");
}

#[test]
fn deconjugate_plain_causative_passive_affirmative_v5_ri() {
    assert_golden("有らされる", "有る", "v5r-i", "～causative passive");
}

#[test]
fn deconjugate_polite_causative_passive_affirmative_v5_ri() {
    assert_golden("有らされます", "有る", "v5r-i", "～causative passive→polite");
}

#[test]
fn deconjugate_plain_causative_passive_negative_v5_ri() {
    assert_golden("有らされない", "有る", "v5r-i", "～causative passive→negative");
}

#[test]
fn deconjugate_polite_causative_passive_negative_v5_ri() {
    assert_golden("有らされません", "有る", "v5r-i", "～causative passive→polite negative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_ri() {
    assert_golden("有らせない", "有る", "v5r-i", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_ri() {
    assert_golden("有らせん", "有る", "v5r-i", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_ri() {
    assert_golden("有らせます", "有る", "v5r-i", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_ri() {
    assert_golden("有らします", "有る", "v5r-i", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_ri() {
    assert_golden("有らせません", "有る", "v5r-i", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_ri() {
    assert_golden("有らせた", "有る", "v5r-i", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_ri() {
    assert_golden("有らせなかった", "有る", "v5r-i", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_ri() {
    assert_golden("有らせました", "有る", "v5r-i", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_ri() {
    assert_golden("有らせませんでした", "有る", "v5r-i", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_ri() {
    assert_golden("有らせられる", "有る", "v5r-i", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_ri() {
    assert_golden("有らせられない", "有る", "v5r-i", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_ri() {
    assert_golden("有らせられます", "有る", "v5r-i", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_ri() {
    assert_golden("有らせられません", "有る", "v5r-i", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_ri() {
    assert_golden("有りたい", "有る", "v5r-i", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_ri() {
    assert_golden("有りたくありません", "有る", "v5r-i", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_ri() {
    assert_golden("有りたくありませんでした", "有る", "v5r-i", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_ri() {
    assert_golden("有りたくない", "有る", "v5r-i", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_ri() {
    assert_golden("有りたかった", "有る", "v5r-i", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_ri() {
    assert_golden("有りたくなかった", "有る", "v5r-i", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_ri() {
    assert_golden("有っている", "有る", "v5r-i", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_ri() {
    assert_golden("有っていない", "有る", "v5r-i", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_ri() {
    assert_golden("有っていた", "有る", "v5r-i", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_ri() {
    assert_golden("有っていなかった", "有る", "v5r-i", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_ri() {
    assert_golden("有っています", "有る", "v5r-i", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_ri() {
    assert_golden("有っていません", "有る", "v5r-i", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_ri() {
    assert_golden("有っていました", "有る", "v5r-i", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_ri() {
    assert_golden("有っていませんでした", "有る", "v5r-i", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_ri() {
    assert_golden("有ってる", "有る", "v5r-i", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_ri() {
    assert_golden("有ってない", "有る", "v5r-i", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_ri() {
    assert_golden("有ってた", "有る", "v5r-i", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_ri() {
    assert_golden("有ってなかった", "有る", "v5r-i", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_ri() {
    assert_golden("有ってます", "有る", "v5r-i", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_ri() {
    assert_golden("有ってません", "有る", "v5r-i", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_ri() {
    assert_golden("有ってました", "有る", "v5r-i", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_ri() {
    assert_golden("有ってません", "有る", "v5r-i", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_ri() {
    assert_golden("有ってませんでした", "有る", "v5r-i", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_ri() {
    assert_golden("有ってしまう", "有る", "v5r-i", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_ri() {
    assert_golden("有ってもう", "有る", "v5r-i", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_ri() {
    assert_golden("有ってしまわない", "有る", "v5r-i", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_ri() {
    assert_golden("有ってしまった", "有る", "v5r-i", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_ri() {
    assert_golden("有ってしまわなかった", "有る", "v5r-i", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_ri() {
    assert_golden("有ってしまって", "有る", "v5r-i", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_ri() {
    assert_golden("有ってしまえば", "有る", "v5r-i", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_ri() {
    assert_golden("有ってしまわなければ", "有る", "v5r-i", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_ri() {
    assert_golden("有ってしまわなかったら", "有る", "v5r-i", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_ri() {
    assert_golden("有ってしまったら", "有る", "v5r-i", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_ri() {
    assert_golden("有ってしまおう", "有る", "v5r-i", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_ri() {
    assert_golden("有ってしまいます", "有る", "v5r-i", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_ri() {
    assert_golden("有ってしまいません", "有る", "v5r-i", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_ri() {
    assert_golden("有ってしまいました", "有る", "v5r-i", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_ri() {
    assert_golden("有ってしまいませんでした", "有る", "v5r-i", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_ri() {
    assert_golden("有ってしまえる", "有る", "v5r-i", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_ri() {
    assert_golden("有ってしまわれる", "有る", "v5r-i", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_ri() {
    assert_golden("有ってしまわせる", "有る", "v5r-i", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_ri() {
    assert_golden("有っちゃう", "有る", "v5r-i", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_ri() {
    assert_golden("有っちゃわない", "有る", "v5r-i", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_ri() {
    assert_golden("有っちゃった", "有る", "v5r-i", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_ri() {
    assert_golden("有っちゃわなかった", "有る", "v5r-i", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_ri() {
    assert_golden("有っちゃって", "有る", "v5r-i", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_ri() {
    assert_golden("有っちゃえば", "有る", "v5r-i", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_ri() {
    assert_golden("有っちゃわなければ", "有る", "v5r-i", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_ri() {
    assert_golden("有っちゃわなかったら", "有る", "v5r-i", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_ri() {
    assert_golden("有っちゃおう", "有る", "v5r-i", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_ri() {
    assert_golden("有っちゃえる", "有る", "v5r-i", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_ri() {
    assert_golden("有っておく", "有る", "v5r-i", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_ri() {
    assert_golden("有っておかない", "有る", "v5r-i", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_ri() {
    assert_golden("有っておいた", "有る", "v5r-i", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_ri() {
    assert_golden("有っておかなかった", "有る", "v5r-i", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_ri() {
    assert_golden("有っておいて", "有る", "v5r-i", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_ri() {
    assert_golden("有っておけば", "有る", "v5r-i", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_ri() {
    assert_golden("有っておいたら", "有る", "v5r-i", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_ri() {
    assert_golden("有っておこう", "有る", "v5r-i", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_ri() {
    assert_golden("有っておける", "有る", "v5r-i", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_ri() {
    assert_golden("有っておかれる", "有る", "v5r-i", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_ri() {
    assert_golden("有っとく", "有る", "v5r-i", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_ri() {
    assert_golden("有っとかない", "有る", "v5r-i", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_ri() {
    assert_golden("有っといた", "有る", "v5r-i", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_ri() {
    assert_golden("有っとかなかった", "有る", "v5r-i", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_ri() {
    assert_golden("有っといて", "有る", "v5r-i", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_ri() {
    assert_golden("有っとけば", "有る", "v5r-i", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_ri() {
    assert_golden("有っといたら", "有る", "v5r-i", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_ri() {
    assert_golden("有っとこう", "有る", "v5r-i", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_ri() {
    assert_golden("有っとける", "有る", "v5r-i", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_ri() {
    assert_golden("有っとかれる", "有る", "v5r-i", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_ri() {
    assert_golden("有ってある", "有る", "v5r-i", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_ri() {
    assert_golden("有ってあった", "有る", "v5r-i", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_ri() {
    assert_golden("有ってあって", "有る", "v5r-i", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_ri() {
    assert_golden("有ってあったら", "有る", "v5r-i", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_ri() {
    assert_golden("有ってあれば", "有る", "v5r-i", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_ri() {
    assert_golden("有っていく", "有る", "v5r-i", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_ri() {
    assert_golden("有っていかない", "有る", "v5r-i", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_ri() {
    assert_golden("有っていった", "有る", "v5r-i", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_ri() {
    assert_golden("有っていかなかった", "有る", "v5r-i", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_ri() {
    assert_golden("有っていって", "有る", "v5r-i", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_ri() {
    assert_golden("有っていこう", "有る", "v5r-i", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_ri() {
    assert_golden("有っていける", "有る", "v5r-i", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_ri() {
    assert_golden("有っていかれる", "有る", "v5r-i", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_ri() {
    assert_golden("有っていかせる", "有る", "v5r-i", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_ri() {
    assert_golden("有ってくる", "有る", "v5r-i", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_ri() {
    assert_golden("有ってこない", "有る", "v5r-i", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_ri() {
    assert_golden("有ってきた", "有る", "v5r-i", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_ri() {
    assert_golden("有ってこなかった", "有る", "v5r-i", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_ri() {
    assert_golden("有ってきて", "有る", "v5r-i", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_ri() {
    assert_golden("有ってくれば", "有る", "v5r-i", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_ri() {
    assert_golden("有ってきたら", "有る", "v5r-i", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_ri() {
    assert_golden("有ってこられる", "有る", "v5r-i", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_ri() {
    assert_golden("有ってこさせる", "有る", "v5r-i", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_ri() {
    assert_golden("有りながら", "有る", "v5r-i", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_ri() {
    assert_golden("有りすぎる", "有る", "v5r-i", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_ri() {
    assert_golden("有りそう", "有る", "v5r-i", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_ri() {
    assert_golden("有らぬ", "有る", "v5r-i", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_ri() {
    assert_golden("有らず", "有る", "v5r-i", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_ri() {
    assert_golden("有らずに", "有る", "v5r-i", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_ri() {
    assert_golden("有ったり", "有る", "v5r-i", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_ri() {
    assert_golden("なかったり", "有る", "v5r-i", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_ri() {
    assert_golden("有らん", "有る", "v5r-i", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_ri() {
    assert_golden("有らんかった", "有る", "v5r-i", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_ri() {
    assert_golden("有らざる", "有る", "v5r-i", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_ri() {
    assert_golden("有れよう", "有る", "v5r-i", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_ri() {
    assert_golden("有れよ", "有る", "v5r-i", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_ri() {
    assert_golden("有れろ", "有る", "v5r-i", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_ri() {
    assert_golden("有れて", "有る", "v5r-i", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_ri() {
    assert_golden("有れたら", "有る", "v5r-i", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_ri() {
    assert_golden("有れれば", "有る", "v5r-i", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_ri() {
    assert_golden("有れられる", "有る", "v5r-i", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_ri() {
    assert_golden("有れさせる", "有る", "v5r-i", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_ri() {
    assert_golden("有ってあげる", "有る", "v5r-i", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_ri() {
    assert_golden("有ってあげられる", "有る", "v5r-i", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_ri() {
    assert_golden("有っておる", "有る", "v5r-i", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_ri() {
    assert_golden("有っておらない", "有る", "v5r-i", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_ri() {
    assert_golden("有っておらん", "有る", "v5r-i", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_ri() {
    assert_golden("有っておった", "有る", "v5r-i", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_ri() {
    assert_golden("有っておらなかった", "有る", "v5r-i", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_ri() {
    assert_golden("有っております", "有る", "v5r-i", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_ri() {
    assert_golden("有っておりません", "有る", "v5r-i", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_ri() {
    assert_golden("有っておりました", "有る", "v5r-i", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_ri() {
    assert_golden("有っておりませんでした", "有る", "v5r-i", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_ri() {
    assert_golden("有っておって", "有る", "v5r-i", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_ri() {
    assert_golden("有っておろう", "有る", "v5r-i", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_ri() {
    assert_golden("有っておれる", "有る", "v5r-i", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_ri() {
    assert_golden("有っておられる", "有る", "v5r-i", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_ri() {
    assert_golden("有っとる", "有る", "v5r-i", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_ri() {
    assert_golden("有っとらない", "有る", "v5r-i", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_ri() {
    assert_golden("有っとらん", "有る", "v5r-i", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_ri() {
    assert_golden("有っとった", "有る", "v5r-i", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_ri() {
    assert_golden("有っとらなかった", "有る", "v5r-i", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_ri() {
    assert_golden("有っとります", "有る", "v5r-i", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_ri() {
    assert_golden("有っとりません", "有る", "v5r-i", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_ri() {
    assert_golden("有っとりました", "有る", "v5r-i", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_ri() {
    assert_golden("有っとりませんでした", "有る", "v5r-i", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_ri() {
    assert_golden("有っとって", "有る", "v5r-i", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_ri() {
    assert_golden("有っとろう", "有る", "v5r-i", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_ri() {
    assert_golden("有っとれる", "有る", "v5r-i", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_ri() {
    assert_golden("有っとられる", "有る", "v5r-i", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_ri() {
    assert_golden("有らす", "有る", "v5r-i", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_ri() {
    assert_golden("有っては", "有る", "v5r-i", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_ri() {
    assert_golden("有っちゃ", "有る", "v5r-i", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_ri() {
    assert_golden("なきゃ", "有る", "v5r-i", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_ri() {
    assert_golden("有っちまう", "有る", "v5r-i", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_ri() {
    assert_golden("有っちゃう", "有る", "v5r-i", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_ri() {
    assert_golden("有っていらっしゃる", "有る", "v5r-i", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_ri() {
    assert_golden("有っていらっしゃらない", "有る", "v5r-i", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_ri() {
    assert_golden("有りつつ", "有る", "v5r-i", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_ri() {
    assert_golden("有ってくれる", "有る", "v5r-i", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_ri() {
    assert_golden("有ってくれない", "有る", "v5r-i", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_ri() {
    assert_golden("有ってくれます", "有る", "v5r-i", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_ri() {
    assert_golden("有ってくれません", "有る", "v5r-i", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_ri() {
    assert_golden("有ってくれ", "有る", "v5r-i", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_ri() {
    assert_golden("有らへん", "有る", "v5r-i", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_ri() {
    assert_golden("有らへんかった", "有る", "v5r-i", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_ri() {
    assert_golden("有らへん", "有る", "v5r-i", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_ri() {
    assert_golden("有らへんかった", "有る", "v5r-i", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_ri() {
    assert_golden("有らさない", "有る", "v5r-i", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_ri() {
    assert_golden("有りましたら", "有る", "v5r-i", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_ri() {
    assert_golden("有りになる", "有る", "v5r-i", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_ri() {
    assert_golden("有りなさる", "有る", "v5r-i", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_ri() {
    assert_golden("有りはる", "有る", "v5r-i", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_ri() {
    assert_golden("有りなさるな", "有る", "v5r-i", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_ri() {
    assert_golden("有るまい", "有る", "v5r-i", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_ri() {
    assert_golden("有りますまい", "有る", "v5r-i", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_ri() {
    assert_golden("有らば", "有る", "v5r-i", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_ri() {
    assert_golden("有らねば", "有る", "v5r-i", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_ri() {
    assert_golden("有らにゃ", "有る", "v5r-i", "～colloquial negative conditional");
}
