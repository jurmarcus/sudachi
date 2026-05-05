//! Golden tests ported from JL's `DeconjugatorTestsForV5U.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V5U.

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
fn deconjugate_masu_stem_v5_u() {
    assert_golden("言い", "言う", "v5u", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_u() {
    assert_golden("言わない", "言う", "v5u", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_u() {
    assert_golden("言います", "言う", "v5u", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_u() {
    assert_golden("言いましょう", "言う", "v5u", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_u() {
    assert_golden("言いません", "言う", "v5u", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_u() {
    assert_golden("言った", "言う", "v5u", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_u() {
    assert_golden("言わなかった", "言う", "v5u", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_u() {
    assert_golden("言いました", "言う", "v5u", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_u() {
    assert_golden("言いませんでした", "言う", "v5u", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_u() {
    assert_golden("言って", "言う", "v5u", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_u() {
    assert_golden("言わなくて", "言う", "v5u", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_u() {
    assert_golden("言わないで", "言う", "v5u", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_u() {
    assert_golden("言いまして", "言う", "v5u", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_u() {
    assert_golden("言える", "言う", "v5u", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_u() {
    assert_golden("言われる", "言う", "v5u", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_u() {
    assert_golden("言えない", "言う", "v5u", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_u() {
    assert_golden("言われない", "言う", "v5u", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_u() {
    assert_golden("言えた", "言う", "v5u", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_u() {
    assert_golden("言われた", "言う", "v5u", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_u() {
    assert_golden("言えました", "言う", "v5u", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_u() {
    assert_golden("言われました", "言う", "v5u", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_u() {
    assert_golden("言えなかった", "言う", "v5u", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_u() {
    assert_golden("言われなかった", "言う", "v5u", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_u() {
    assert_golden("言えませんでした", "言う", "v5u", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_u() {
    assert_golden("言われませんでした", "言う", "v5u", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_u() {
    assert_golden("言えます", "言う", "v5u", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_u() {
    assert_golden("言われます", "言う", "v5u", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_u() {
    assert_golden("言えません", "言う", "v5u", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_u() {
    assert_golden("言われません", "言う", "v5u", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_u() {
    assert_golden("言え", "言う", "v5u", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_u() {
    assert_golden("言うな", "言う", "v5u", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_u() {
    assert_golden("言いなさい", "言う", "v5u", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_u() {
    assert_golden("言ってください", "言う", "v5u", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_u() {
    assert_golden("言わないでください", "言う", "v5u", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_u() {
    assert_golden("言おう", "言う", "v5u", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_u() {
    assert_golden("言お", "言う", "v5u", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_u() {
    assert_golden("言いましょう", "言う", "v5u", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_u() {
    assert_golden("言えば", "言う", "v5u", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_u() {
    assert_golden("言わなければ", "言う", "v5u", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_u() {
    assert_golden("言ったら", "言う", "v5u", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_u() {
    assert_golden("言ったらば", "言う", "v5u", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_u() {
    assert_golden("言わなかったら", "言う", "v5u", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_u() {
    assert_golden("言わせる", "言う", "v5u", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_u() {
    assert_golden("言わせない", "言う", "v5u", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_u() {
    assert_golden("言わせん", "言う", "v5u", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_u() {
    assert_golden("言わせます", "言う", "v5u", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_u() {
    assert_golden("言わします", "言う", "v5u", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_u() {
    assert_golden("言わせません", "言う", "v5u", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_u() {
    assert_golden("言わせた", "言う", "v5u", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_u() {
    assert_golden("言わせなかった", "言う", "v5u", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_u() {
    assert_golden("言わせました", "言う", "v5u", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_u() {
    assert_golden("言わせませんでした", "言う", "v5u", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_u() {
    assert_golden("言わせられる", "言う", "v5u", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_u() {
    assert_golden("言わせられない", "言う", "v5u", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_u() {
    assert_golden("言わせられます", "言う", "v5u", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_u() {
    assert_golden("言わせられません", "言う", "v5u", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_u() {
    assert_golden("言いたい", "言う", "v5u", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_u() {
    assert_golden("言いたくありません", "言う", "v5u", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_u() {
    assert_golden("言いたくありませんでした", "言う", "v5u", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_u() {
    assert_golden("言いたくない", "言う", "v5u", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_u() {
    assert_golden("言いたかった", "言う", "v5u", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_u() {
    assert_golden("言いたくなかった", "言う", "v5u", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_u() {
    assert_golden("言っている", "言う", "v5u", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_u() {
    assert_golden("言っていない", "言う", "v5u", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_u() {
    assert_golden("言っていた", "言う", "v5u", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_u() {
    assert_golden("言っていなかった", "言う", "v5u", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_u() {
    assert_golden("言っています", "言う", "v5u", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_u() {
    assert_golden("言っていません", "言う", "v5u", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_u() {
    assert_golden("言っていました", "言う", "v5u", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_u() {
    assert_golden("言っていませんでした", "言う", "v5u", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_u() {
    assert_golden("言ってる", "言う", "v5u", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_u() {
    assert_golden("言ってない", "言う", "v5u", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_u() {
    assert_golden("言ってた", "言う", "v5u", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_u() {
    assert_golden("言ってなかった", "言う", "v5u", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_u() {
    assert_golden("言ってます", "言う", "v5u", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_u() {
    assert_golden("言ってません", "言う", "v5u", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_u() {
    assert_golden("言ってました", "言う", "v5u", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_u() {
    assert_golden("言ってません", "言う", "v5u", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_u() {
    assert_golden("言ってませんでした", "言う", "v5u", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_u() {
    assert_golden("言ってしまう", "言う", "v5u", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_u() {
    assert_golden("言ってもう", "言う", "v5u", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_u() {
    assert_golden("言ってしまわない", "言う", "v5u", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_u() {
    assert_golden("言ってしまった", "言う", "v5u", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_u() {
    assert_golden("言ってしまわなかった", "言う", "v5u", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_u() {
    assert_golden("言ってしまって", "言う", "v5u", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_u() {
    assert_golden("言ってしまえば", "言う", "v5u", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_u() {
    assert_golden("言ってしまわなければ", "言う", "v5u", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_u() {
    assert_golden("言ってしまわなかったら", "言う", "v5u", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_u() {
    assert_golden("言ってしまったら", "言う", "v5u", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_u() {
    assert_golden("言ってしまおう", "言う", "v5u", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_u() {
    assert_golden("言ってしまいます", "言う", "v5u", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_u() {
    assert_golden("言ってしまいません", "言う", "v5u", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_u() {
    assert_golden("言ってしまいました", "言う", "v5u", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_u() {
    assert_golden("言ってしまいませんでした", "言う", "v5u", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_u() {
    assert_golden("言ってしまえる", "言う", "v5u", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_u() {
    assert_golden("言ってしまわれる", "言う", "v5u", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_u() {
    assert_golden("言ってしまわせる", "言う", "v5u", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_u() {
    assert_golden("言っちゃう", "言う", "v5u", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_u() {
    assert_golden("言っちゃわない", "言う", "v5u", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_u() {
    assert_golden("言っちゃった", "言う", "v5u", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_u() {
    assert_golden("言っちゃわなかった", "言う", "v5u", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_u() {
    assert_golden("言っちゃって", "言う", "v5u", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_u() {
    assert_golden("言っちゃえば", "言う", "v5u", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_u() {
    assert_golden("言っちゃわなければ", "言う", "v5u", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_u() {
    assert_golden("言っちゃわなかったら", "言う", "v5u", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_u() {
    assert_golden("言っちゃおう", "言う", "v5u", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_u() {
    assert_golden("言っちゃえる", "言う", "v5u", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_u() {
    assert_golden("言っておく", "言う", "v5u", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_u() {
    assert_golden("言っておかない", "言う", "v5u", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_u() {
    assert_golden("言っておいた", "言う", "v5u", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_u() {
    assert_golden("言っておかなかった", "言う", "v5u", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_u() {
    assert_golden("言っておいて", "言う", "v5u", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_u() {
    assert_golden("言っておけば", "言う", "v5u", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_u() {
    assert_golden("言っておいたら", "言う", "v5u", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_u() {
    assert_golden("言っておこう", "言う", "v5u", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_u() {
    assert_golden("言っておける", "言う", "v5u", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_u() {
    assert_golden("言っておかれる", "言う", "v5u", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_u() {
    assert_golden("言っとく", "言う", "v5u", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_u() {
    assert_golden("言っとかない", "言う", "v5u", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_u() {
    assert_golden("言っといた", "言う", "v5u", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_u() {
    assert_golden("言っとかなかった", "言う", "v5u", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_u() {
    assert_golden("言っといて", "言う", "v5u", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_u() {
    assert_golden("言っとけば", "言う", "v5u", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_u() {
    assert_golden("言っといたら", "言う", "v5u", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_u() {
    assert_golden("言っとこう", "言う", "v5u", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_u() {
    assert_golden("言っとける", "言う", "v5u", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_u() {
    assert_golden("言っとかれる", "言う", "v5u", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_u() {
    assert_golden("言ってある", "言う", "v5u", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_u() {
    assert_golden("言ってあった", "言う", "v5u", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_u() {
    assert_golden("言ってあって", "言う", "v5u", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_u() {
    assert_golden("言ってあったら", "言う", "v5u", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_u() {
    assert_golden("言ってあれば", "言う", "v5u", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_u() {
    assert_golden("言っていく", "言う", "v5u", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_u() {
    assert_golden("言っていかない", "言う", "v5u", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_u() {
    assert_golden("言っていった", "言う", "v5u", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_u() {
    assert_golden("言っていかなかった", "言う", "v5u", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_u() {
    assert_golden("言っていって", "言う", "v5u", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_u() {
    assert_golden("言っていこう", "言う", "v5u", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_u() {
    assert_golden("言っていける", "言う", "v5u", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_u() {
    assert_golden("言っていかれる", "言う", "v5u", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_u() {
    assert_golden("言っていかせる", "言う", "v5u", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_u() {
    assert_golden("言ってくる", "言う", "v5u", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_u() {
    assert_golden("言ってこない", "言う", "v5u", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_u() {
    assert_golden("言ってきた", "言う", "v5u", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_u() {
    assert_golden("言ってこなかった", "言う", "v5u", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_u() {
    assert_golden("言ってきて", "言う", "v5u", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_u() {
    assert_golden("言ってくれば", "言う", "v5u", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_u() {
    assert_golden("言ってきたら", "言う", "v5u", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_u() {
    assert_golden("言ってこられる", "言う", "v5u", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_u() {
    assert_golden("言ってこさせる", "言う", "v5u", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_u() {
    assert_golden("言いながら", "言う", "v5u", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_u() {
    assert_golden("言いすぎる", "言う", "v5u", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_u() {
    assert_golden("言いそう", "言う", "v5u", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_u() {
    assert_golden("言わぬ", "言う", "v5u", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_u() {
    assert_golden("言わず", "言う", "v5u", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_u() {
    assert_golden("言わずに", "言う", "v5u", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_u() {
    assert_golden("言ったり", "言う", "v5u", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_u() {
    assert_golden("言わなかったり", "言う", "v5u", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_u() {
    assert_golden("言わん", "言う", "v5u", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_u() {
    assert_golden("言わんかった", "言う", "v5u", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_u() {
    assert_golden("言わざる", "言う", "v5u", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_u() {
    assert_golden("言えよう", "言う", "v5u", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_u() {
    assert_golden("言えよ", "言う", "v5u", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_u() {
    assert_golden("言えろ", "言う", "v5u", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_u() {
    assert_golden("言えて", "言う", "v5u", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_u() {
    assert_golden("言えたら", "言う", "v5u", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_u() {
    assert_golden("言えれば", "言う", "v5u", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_u() {
    assert_golden("言えられる", "言う", "v5u", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_u() {
    assert_golden("言えさせる", "言う", "v5u", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_u() {
    assert_golden("言ってあげる", "言う", "v5u", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_u() {
    assert_golden("言ってあげられる", "言う", "v5u", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_u() {
    assert_golden("言っておる", "言う", "v5u", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_u() {
    assert_golden("言っておらない", "言う", "v5u", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_u() {
    assert_golden("言っておらん", "言う", "v5u", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_u() {
    assert_golden("言っておった", "言う", "v5u", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_u() {
    assert_golden("言っておらなかった", "言う", "v5u", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_u() {
    assert_golden("言っております", "言う", "v5u", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_u() {
    assert_golden("言っておりません", "言う", "v5u", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_u() {
    assert_golden("言っておりました", "言う", "v5u", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_u() {
    assert_golden("言っておりませんでした", "言う", "v5u", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_u() {
    assert_golden("言っておって", "言う", "v5u", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_u() {
    assert_golden("言っておろう", "言う", "v5u", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_u() {
    assert_golden("言っておれる", "言う", "v5u", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_u() {
    assert_golden("言っておられる", "言う", "v5u", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_u() {
    assert_golden("言っとる", "言う", "v5u", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_u() {
    assert_golden("言っとらない", "言う", "v5u", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_u() {
    assert_golden("言っとらん", "言う", "v5u", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_u() {
    assert_golden("言っとった", "言う", "v5u", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_u() {
    assert_golden("言っとらなかった", "言う", "v5u", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_u() {
    assert_golden("言っとります", "言う", "v5u", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_u() {
    assert_golden("言っとりません", "言う", "v5u", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_u() {
    assert_golden("言っとりました", "言う", "v5u", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_u() {
    assert_golden("言っとりませんでした", "言う", "v5u", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_u() {
    assert_golden("言っとって", "言う", "v5u", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_u() {
    assert_golden("言っとろう", "言う", "v5u", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_u() {
    assert_golden("言っとれる", "言う", "v5u", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_u() {
    assert_golden("言っとられる", "言う", "v5u", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_u() {
    assert_golden("言わす", "言う", "v5u", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_u() {
    assert_golden("言っては", "言う", "v5u", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_u() {
    assert_golden("言っちゃ", "言う", "v5u", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_u() {
    assert_golden("言わなきゃ", "言う", "v5u", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_u() {
    assert_golden("言っちまう", "言う", "v5u", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_u() {
    assert_golden("言っちゃう", "言う", "v5u", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_u() {
    assert_golden("言っていらっしゃる", "言う", "v5u", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_u() {
    assert_golden("言っていらっしゃらない", "言う", "v5u", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_u() {
    assert_golden("言いつつ", "言う", "v5u", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_u() {
    assert_golden("言ってくれる", "言う", "v5u", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_u() {
    assert_golden("言ってくれない", "言う", "v5u", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_u() {
    assert_golden("言ってくれます", "言う", "v5u", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_u() {
    assert_golden("言ってくれません", "言う", "v5u", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_u() {
    assert_golden("言ってくれ", "言う", "v5u", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_u() {
    assert_golden("言わへん", "言う", "v5u", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_u() {
    assert_golden("言わへんかった", "言う", "v5u", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_u() {
    assert_golden("言わひん", "言う", "v5u", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_u() {
    assert_golden("言わひんかった", "言う", "v5u", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_u() {
    assert_golden("言わさない", "言う", "v5u", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_u() {
    assert_golden("言いましたら", "言う", "v5u", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_u() {
    assert_golden("言いになる", "言う", "v5u", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_u() {
    assert_golden("言いなさる", "言う", "v5u", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_u() {
    assert_golden("言いはる", "言う", "v5u", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_u() {
    assert_golden("言いなさるな", "言う", "v5u", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_u() {
    assert_golden("言うまい", "言う", "v5u", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_u() {
    assert_golden("言いますまい", "言う", "v5u", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_u() {
    assert_golden("言わば", "言う", "v5u", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_u() {
    assert_golden("言わねば", "言う", "v5u", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_u() {
    assert_golden("言わにゃ", "言う", "v5u", "～colloquial negative conditional");
}

// ─── Verb-producing aux on renyou base (added 2026-05-06) ────────────

#[test]
fn deconjugate_aux_hajimeru_past_v5_u() {
    assert_golden("買い始めた", "買う", "v5u", "～start V-ing→past");
}

#[test]
fn deconjugate_aux_tsuzukeru_teiru_v5_u() {
    assert_golden("買い続けている", "買う", "v5u", "～continue V-ing→teiru");
}

#[test]
fn deconjugate_aux_te_morau_past_v5_u() {
    assert_golden("買ってもらった", "買う", "v5u", "～have someone do→past");
}
