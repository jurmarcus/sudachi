//! Golden tests ported from JL's `DeconjugatorTestsForV5US.cs`.
//! 231 test cases proving deconjugator output matches
//! JL's expectations for class V5US.

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
fn deconjugate_masu_stem_v5_us() {
    assert_golden("問い", "問う", "v5u-s", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_us() {
    assert_golden("問わない", "問う", "v5u-s", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_us() {
    assert_golden("問います", "問う", "v5u-s", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_us() {
    assert_golden("問いましょう", "問う", "v5u-s", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_us() {
    assert_golden("問いません", "問う", "v5u-s", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_us() {
    assert_golden("問った", "問う", "v5u-s", "～past");
}

#[test]
fn deconjugate_plain_past_affirmative_2_v5_us() {
    assert_golden("問うた", "問う", "v5u-s", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_us() {
    assert_golden("問わなかった", "問う", "v5u-s", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_us() {
    assert_golden("問いました", "問う", "v5u-s", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_us() {
    assert_golden("問いませんでした", "問う", "v5u-s", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_us() {
    assert_golden("問って", "問う", "v5u-s", "～te");
}

#[test]
fn deconjugate_plain_te_form_affirmative_2_v5_us() {
    assert_golden("問うて", "問う", "v5u-s", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_us() {
    assert_golden("問わなくて", "問う", "v5u-s", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_us() {
    assert_golden("問わないで", "問う", "v5u-s", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_us() {
    assert_golden("問いまして", "問う", "v5u-s", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_us() {
    assert_golden("問える", "問う", "v5u-s", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_us() {
    assert_golden("問われる", "問う", "v5u-s", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_us() {
    assert_golden("問えない", "問う", "v5u-s", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_us() {
    assert_golden("問われない", "問う", "v5u-s", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_us() {
    assert_golden("問えた", "問う", "v5u-s", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_us() {
    assert_golden("問われた", "問う", "v5u-s", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_us() {
    assert_golden("問えました", "問う", "v5u-s", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_us() {
    assert_golden("問われました", "問う", "v5u-s", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_us() {
    assert_golden("問えなかった", "問う", "v5u-s", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_us() {
    assert_golden("問われなかった", "問う", "v5u-s", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_us() {
    assert_golden("問えませんでした", "問う", "v5u-s", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_us() {
    assert_golden("問われませんでした", "問う", "v5u-s", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_us() {
    assert_golden("問えます", "問う", "v5u-s", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_us() {
    assert_golden("問われます", "問う", "v5u-s", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_us() {
    assert_golden("問えません", "問う", "v5u-s", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_us() {
    assert_golden("問われません", "問う", "v5u-s", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_us() {
    assert_golden("問え", "問う", "v5u-s", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_us() {
    assert_golden("問うな", "問う", "v5u-s", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_us() {
    assert_golden("問いなさい", "問う", "v5u-s", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_us() {
    assert_golden("問ってください", "問う", "v5u-s", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_us() {
    assert_golden("問わないでください", "問う", "v5u-s", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_us() {
    assert_golden("問おう", "問う", "v5u-s", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_us() {
    assert_golden("問お", "問う", "v5u-s", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_us() {
    assert_golden("問いましょう", "問う", "v5u-s", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_us() {
    assert_golden("問えば", "問う", "v5u-s", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_us() {
    assert_golden("問わなければ", "問う", "v5u-s", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_us() {
    assert_golden("問ったら", "問う", "v5u-s", "～conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_2_v5_us() {
    assert_golden("問うたら", "問う", "v5u-s", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_us() {
    assert_golden("問ったらば", "問う", "v5u-s", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_us() {
    assert_golden("問わなかったら", "問う", "v5u-s", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_us() {
    assert_golden("問わせる", "問う", "v5u-s", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_us() {
    assert_golden("問わせない", "問う", "v5u-s", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_us() {
    assert_golden("問わせん", "問う", "v5u-s", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_us() {
    assert_golden("問わせます", "問う", "v5u-s", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_us() {
    assert_golden("問わします", "問う", "v5u-s", "～short causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_negative_v5_us() {
    assert_golden("問わしません", "問う", "v5u-s", "～short causative→polite negative");
}

#[test]
fn deconjugate_polite_causative_negative_v5_us() {
    assert_golden("問わせません", "問う", "v5u-s", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_us() {
    assert_golden("問わせた", "問う", "v5u-s", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_us() {
    assert_golden("問わせなかった", "問う", "v5u-s", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_us() {
    assert_golden("問わせました", "問う", "v5u-s", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_us() {
    assert_golden("問わせませんでした", "問う", "v5u-s", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_us() {
    assert_golden("問わせられる", "問う", "v5u-s", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_us() {
    assert_golden("問わせられない", "問う", "v5u-s", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_us() {
    assert_golden("問わせられます", "問う", "v5u-s", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_us() {
    assert_golden("問わせられません", "問う", "v5u-s", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_us() {
    assert_golden("問いたい", "問う", "v5u-s", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_us() {
    assert_golden("問いたくありません", "問う", "v5u-s", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_us() {
    assert_golden("問いたくありませんでした", "問う", "v5u-s", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_us() {
    assert_golden("問いたくない", "問う", "v5u-s", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_us() {
    assert_golden("問いたかった", "問う", "v5u-s", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_us() {
    assert_golden("問いたくなかった", "問う", "v5u-s", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_us() {
    assert_golden("問っている", "問う", "v5u-s", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_us() {
    assert_golden("問っていない", "問う", "v5u-s", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_us() {
    assert_golden("問っていた", "問う", "v5u-s", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_us() {
    assert_golden("問っていなかった", "問う", "v5u-s", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_us() {
    assert_golden("問っています", "問う", "v5u-s", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_us() {
    assert_golden("問っていません", "問う", "v5u-s", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_us() {
    assert_golden("問っていました", "問う", "v5u-s", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_us() {
    assert_golden("問っていませんでした", "問う", "v5u-s", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_us() {
    assert_golden("問ってる", "問う", "v5u-s", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_us() {
    assert_golden("問ってない", "問う", "v5u-s", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_us() {
    assert_golden("問ってた", "問う", "v5u-s", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_us() {
    assert_golden("問ってなかった", "問う", "v5u-s", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_us() {
    assert_golden("問ってます", "問う", "v5u-s", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_us() {
    assert_golden("問ってません", "問う", "v5u-s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_us() {
    assert_golden("問ってました", "問う", "v5u-s", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_us() {
    assert_golden("問ってません", "問う", "v5u-s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_us() {
    assert_golden("問ってませんでした", "問う", "v5u-s", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_us() {
    assert_golden("問ってしまう", "問う", "v5u-s", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_us() {
    assert_golden("問ってもう", "問う", "v5u-s", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_us() {
    assert_golden("問ってしまわない", "問う", "v5u-s", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_us() {
    assert_golden("問ってしまった", "問う", "v5u-s", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_us() {
    assert_golden("問ってしまわなかった", "問う", "v5u-s", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_us() {
    assert_golden("問ってしまって", "問う", "v5u-s", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_us() {
    assert_golden("問ってしまえば", "問う", "v5u-s", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_us() {
    assert_golden("問ってしまわなければ", "問う", "v5u-s", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_us() {
    assert_golden("問ってしまわなかったら", "問う", "v5u-s", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_us() {
    assert_golden("問ってしまったら", "問う", "v5u-s", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_us() {
    assert_golden("問ってしまおう", "問う", "v5u-s", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_us() {
    assert_golden("問ってしまいます", "問う", "v5u-s", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_us() {
    assert_golden("問ってしまいません", "問う", "v5u-s", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_us() {
    assert_golden("問ってしまいました", "問う", "v5u-s", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_us() {
    assert_golden("問ってしまいませんでした", "問う", "v5u-s", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_us() {
    assert_golden("問ってしまえる", "問う", "v5u-s", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_us() {
    assert_golden("問ってしまわれる", "問う", "v5u-s", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_us() {
    assert_golden("問ってしまわせる", "問う", "v5u-s", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_us() {
    assert_golden("問っちゃう", "問う", "v5u-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_us() {
    assert_golden("問っちゃわない", "問う", "v5u-s", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_us() {
    assert_golden("問っちゃった", "問う", "v5u-s", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_us() {
    assert_golden("問っちゃわなかった", "問う", "v5u-s", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_us() {
    assert_golden("問っちゃって", "問う", "v5u-s", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_us() {
    assert_golden("問っちゃえば", "問う", "v5u-s", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_us() {
    assert_golden("問っちゃわなければ", "問う", "v5u-s", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_us() {
    assert_golden("問っちゃわなかったら", "問う", "v5u-s", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_us() {
    assert_golden("問っちゃおう", "問う", "v5u-s", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_us() {
    assert_golden("問っちゃえる", "問う", "v5u-s", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_us() {
    assert_golden("問っておく", "問う", "v5u-s", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_us() {
    assert_golden("問っておかない", "問う", "v5u-s", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_us() {
    assert_golden("問っておいた", "問う", "v5u-s", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_us() {
    assert_golden("問っておかなかった", "問う", "v5u-s", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_us() {
    assert_golden("問っておいて", "問う", "v5u-s", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_us() {
    assert_golden("問っておけば", "問う", "v5u-s", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_us() {
    assert_golden("問っておいたら", "問う", "v5u-s", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_us() {
    assert_golden("問っておこう", "問う", "v5u-s", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_us() {
    assert_golden("問っておける", "問う", "v5u-s", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_us() {
    assert_golden("問っておかれる", "問う", "v5u-s", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_us() {
    assert_golden("問っとく", "問う", "v5u-s", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_us() {
    assert_golden("問っとかない", "問う", "v5u-s", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_us() {
    assert_golden("問っといた", "問う", "v5u-s", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_us() {
    assert_golden("問っとかなかった", "問う", "v5u-s", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_us() {
    assert_golden("問っといて", "問う", "v5u-s", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_us() {
    assert_golden("問っとけば", "問う", "v5u-s", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_us() {
    assert_golden("問っといたら", "問う", "v5u-s", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_us() {
    assert_golden("問っとこう", "問う", "v5u-s", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_us() {
    assert_golden("問っとける", "問う", "v5u-s", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_us() {
    assert_golden("問っとかれる", "問う", "v5u-s", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_us() {
    assert_golden("問ってある", "問う", "v5u-s", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_us() {
    assert_golden("問ってあった", "問う", "v5u-s", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_us() {
    assert_golden("問ってあって", "問う", "v5u-s", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_us() {
    assert_golden("問ってあったら", "問う", "v5u-s", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_us() {
    assert_golden("問ってあれば", "問う", "v5u-s", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_us() {
    assert_golden("問っていく", "問う", "v5u-s", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_us() {
    assert_golden("問っていかない", "問う", "v5u-s", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_us() {
    assert_golden("問っていった", "問う", "v5u-s", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_us() {
    assert_golden("問っていかなかった", "問う", "v5u-s", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_us() {
    assert_golden("問っていって", "問う", "v5u-s", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_us() {
    assert_golden("問っていこう", "問う", "v5u-s", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_us() {
    assert_golden("問っていける", "問う", "v5u-s", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_us() {
    assert_golden("問っていかれる", "問う", "v5u-s", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_us() {
    assert_golden("問っていかせる", "問う", "v5u-s", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_us() {
    assert_golden("問ってくる", "問う", "v5u-s", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_us() {
    assert_golden("問ってこない", "問う", "v5u-s", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_us() {
    assert_golden("問ってきた", "問う", "v5u-s", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_us() {
    assert_golden("問ってこなかった", "問う", "v5u-s", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_us() {
    assert_golden("問ってきて", "問う", "v5u-s", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_us() {
    assert_golden("問ってくれば", "問う", "v5u-s", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_us() {
    assert_golden("問ってきたら", "問う", "v5u-s", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_us() {
    assert_golden("問ってこられる", "問う", "v5u-s", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_us() {
    assert_golden("問ってこさせる", "問う", "v5u-s", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_us() {
    assert_golden("問いながら", "問う", "v5u-s", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_us() {
    assert_golden("問いすぎる", "問う", "v5u-s", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_us() {
    assert_golden("問いそう", "問う", "v5u-s", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_us() {
    assert_golden("問わぬ", "問う", "v5u-s", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_us() {
    assert_golden("問わず", "問う", "v5u-s", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_us() {
    assert_golden("問わずに", "問う", "v5u-s", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_us() {
    assert_golden("問ったり", "問う", "v5u-s", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_2_v5_us() {
    assert_golden("問うたり", "問う", "v5u-s", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_us() {
    assert_golden("問わなかったり", "問う", "v5u-s", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_us() {
    assert_golden("問わん", "問う", "v5u-s", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_us() {
    assert_golden("問わんかった", "問う", "v5u-s", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_us() {
    assert_golden("問わざる", "問う", "v5u-s", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_us() {
    assert_golden("問えよう", "問う", "v5u-s", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_us() {
    assert_golden("問えよ", "問う", "v5u-s", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_us() {
    assert_golden("問えろ", "問う", "v5u-s", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_us() {
    assert_golden("問えて", "問う", "v5u-s", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_us() {
    assert_golden("問えたら", "問う", "v5u-s", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_us() {
    assert_golden("問えれば", "問う", "v5u-s", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_us() {
    assert_golden("問えられる", "問う", "v5u-s", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_us() {
    assert_golden("問えさせる", "問う", "v5u-s", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_us() {
    assert_golden("問ってあげる", "問う", "v5u-s", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_us() {
    assert_golden("問ってあげられる", "問う", "v5u-s", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_us() {
    assert_golden("問っておる", "問う", "v5u-s", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_us() {
    assert_golden("問っておらない", "問う", "v5u-s", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_us() {
    assert_golden("問っておらん", "問う", "v5u-s", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_us() {
    assert_golden("問っておった", "問う", "v5u-s", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_us() {
    assert_golden("問っておらなかった", "問う", "v5u-s", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_us() {
    assert_golden("問っております", "問う", "v5u-s", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_us() {
    assert_golden("問っておりません", "問う", "v5u-s", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_us() {
    assert_golden("問っておりました", "問う", "v5u-s", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_us() {
    assert_golden("問っておりませんでした", "問う", "v5u-s", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_us() {
    assert_golden("問っておって", "問う", "v5u-s", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_us() {
    assert_golden("問っておろう", "問う", "v5u-s", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_us() {
    assert_golden("問っておれる", "問う", "v5u-s", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_us() {
    assert_golden("問っておられる", "問う", "v5u-s", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_us() {
    assert_golden("問っとる", "問う", "v5u-s", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_us() {
    assert_golden("問っとらない", "問う", "v5u-s", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_us() {
    assert_golden("問っとらん", "問う", "v5u-s", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_us() {
    assert_golden("問っとった", "問う", "v5u-s", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_us() {
    assert_golden("問っとらなかった", "問う", "v5u-s", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_us() {
    assert_golden("問っとります", "問う", "v5u-s", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_us() {
    assert_golden("問っとりません", "問う", "v5u-s", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_us() {
    assert_golden("問っとりました", "問う", "v5u-s", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_us() {
    assert_golden("問っとりませんでした", "問う", "v5u-s", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_us() {
    assert_golden("問っとって", "問う", "v5u-s", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_us() {
    assert_golden("問っとろう", "問う", "v5u-s", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_us() {
    assert_golden("問っとれる", "問う", "v5u-s", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_us() {
    assert_golden("問っとられる", "問う", "v5u-s", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_us() {
    assert_golden("問わす", "問う", "v5u-s", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_us() {
    assert_golden("問っては", "問う", "v5u-s", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_us() {
    assert_golden("問っちゃ", "問う", "v5u-s", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_us() {
    assert_golden("問わなきゃ", "問う", "v5u-s", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_us() {
    assert_golden("問っちまう", "問う", "v5u-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_us() {
    assert_golden("問っちゃう", "問う", "v5u-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_us() {
    assert_golden("問っていらっしゃる", "問う", "v5u-s", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_us() {
    assert_golden("問っていらっしゃらない", "問う", "v5u-s", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_us() {
    assert_golden("問いつつ", "問う", "v5u-s", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_us() {
    assert_golden("問ってくれる", "問う", "v5u-s", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_us() {
    assert_golden("問ってくれない", "問う", "v5u-s", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_us() {
    assert_golden("問ってくれます", "問う", "v5u-s", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_us() {
    assert_golden("問ってくれません", "問う", "v5u-s", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_us() {
    assert_golden("問ってくれ", "問う", "v5u-s", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_us() {
    assert_golden("問わへん", "問う", "v5u-s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_us() {
    assert_golden("問わへんかった", "問う", "v5u-s", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_us() {
    assert_golden("問わひん", "問う", "v5u-s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_us() {
    assert_golden("問わひんかった", "問う", "v5u-s", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_us() {
    assert_golden("問わさない", "問う", "v5u-s", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_us() {
    assert_golden("問いましたら", "問う", "v5u-s", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_us() {
    assert_golden("問いになる", "問う", "v5u-s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_us() {
    assert_golden("問いなさる", "問う", "v5u-s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_us() {
    assert_golden("問いはる", "問う", "v5u-s", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_us() {
    assert_golden("問いなさるな", "問う", "v5u-s", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_us() {
    assert_golden("問うまい", "問う", "v5u-s", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_us() {
    assert_golden("問いますまい", "問う", "v5u-s", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_us() {
    assert_golden("問わば", "問う", "v5u-s", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_us() {
    assert_golden("問わねば", "問う", "v5u-s", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_us() {
    assert_golden("問わにゃ", "問う", "v5u-s", "～colloquial negative conditional");
}
