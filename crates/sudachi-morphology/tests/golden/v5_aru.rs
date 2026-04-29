//! Golden tests ported from JL's `DeconjugatorTestsForV5Aru.cs`.
//! 220 test cases proving deconjugator output matches
//! JL's expectations for class V5Aru.

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
fn deconjugate_masu_stem_v5_aru() {
    assert_golden("仰い", "仰る", "v5aru", "～imperative; masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_aru() {
    assert_golden("仰らない", "仰る", "v5aru", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_aru() {
    assert_golden("仰います", "仰る", "v5aru", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_aru() {
    assert_golden("仰いましょう", "仰る", "v5aru", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_aru() {
    assert_golden("仰いません", "仰る", "v5aru", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_aru() {
    assert_golden("仰った", "仰る", "v5aru", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_aru() {
    assert_golden("仰らなかった", "仰る", "v5aru", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_aru() {
    assert_golden("仰いました", "仰る", "v5aru", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_aru() {
    assert_golden("仰いませんでした", "仰る", "v5aru", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_aru() {
    assert_golden("仰って", "仰る", "v5aru", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_aru() {
    assert_golden("仰らなくて", "仰る", "v5aru", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_aru() {
    assert_golden("仰らないで", "仰る", "v5aru", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_aru() {
    assert_golden("仰いまして", "仰る", "v5aru", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_aru() {
    assert_golden("仰れる", "仰る", "v5aru", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_aru() {
    assert_golden("仰られる", "仰る", "v5aru", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_aru() {
    assert_golden("仰れない", "仰る", "v5aru", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_honorific_negative_v5_aru() {
    assert_golden("仰られない", "仰る", "v5aru", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_aru() {
    assert_golden("仰れた", "仰る", "v5aru", "～potential→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_aru() {
    assert_golden("仰れました", "仰る", "v5aru", "～potential→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_aru() {
    assert_golden("仰れなかった", "仰る", "v5aru", "～potential→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_aru() {
    assert_golden("仰れませんでした", "仰る", "v5aru", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_aru() {
    assert_golden("仰れます", "仰る", "v5aru", "～potential→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_aru() {
    assert_golden("仰れません", "仰る", "v5aru", "～potential→polite negative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_aru() {
    assert_golden("仰るな", "仰る", "v5aru", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_aru() {
    assert_golden("仰いなさい", "仰る", "v5aru", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_aru() {
    assert_golden("仰ってください", "仰る", "v5aru", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_aru() {
    assert_golden("仰らないでください", "仰る", "v5aru", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_aru() {
    assert_golden("仰ろう", "仰る", "v5aru", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_aru() {
    assert_golden("仰ろ", "仰る", "v5aru", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_aru() {
    assert_golden("仰いましょう", "仰る", "v5aru", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_aru() {
    assert_golden("仰れば", "仰る", "v5aru", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_aru() {
    assert_golden("仰らなければ", "仰る", "v5aru", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_aru() {
    assert_golden("仰ったら", "仰る", "v5aru", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_aru() {
    assert_golden("仰ったらば", "仰る", "v5aru", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_aru() {
    assert_golden("仰らなかったら", "仰る", "v5aru", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_aru() {
    assert_golden("仰らせる", "仰る", "v5aru", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_aru() {
    assert_golden("仰らせない", "仰る", "v5aru", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_aru() {
    assert_golden("仰らせん", "仰る", "v5aru", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_aru() {
    assert_golden("仰らせます", "仰る", "v5aru", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_aru() {
    assert_golden("仰らします", "仰る", "v5aru", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_aru() {
    assert_golden("仰らせません", "仰る", "v5aru", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_aru() {
    assert_golden("仰らせた", "仰る", "v5aru", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_aru() {
    assert_golden("仰らせなかった", "仰る", "v5aru", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_aru() {
    assert_golden("仰らせました", "仰る", "v5aru", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_aru() {
    assert_golden("仰らせませんでした", "仰る", "v5aru", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_aru() {
    assert_golden("仰らせられる", "仰る", "v5aru", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_aru() {
    assert_golden("仰らせられない", "仰る", "v5aru", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_aru() {
    assert_golden("仰らせられます", "仰る", "v5aru", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_aru() {
    assert_golden("仰らせられません", "仰る", "v5aru", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_aru() {
    assert_golden("仰りたい", "仰る", "v5aru", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_aru() {
    assert_golden("仰りたくありません", "仰る", "v5aru", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_aru() {
    assert_golden("仰りたくありませんでした", "仰る", "v5aru", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_aru() {
    assert_golden("仰りたくない", "仰る", "v5aru", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_aru() {
    assert_golden("仰りたかった", "仰る", "v5aru", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_aru() {
    assert_golden("仰りたくなかった", "仰る", "v5aru", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_aru() {
    assert_golden("仰っている", "仰る", "v5aru", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_aru() {
    assert_golden("仰っていない", "仰る", "v5aru", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_aru() {
    assert_golden("仰っていた", "仰る", "v5aru", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_aru() {
    assert_golden("仰っていなかった", "仰る", "v5aru", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_aru() {
    assert_golden("仰っています", "仰る", "v5aru", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_aru() {
    assert_golden("仰っていません", "仰る", "v5aru", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_aru() {
    assert_golden("仰っていました", "仰る", "v5aru", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_aru() {
    assert_golden("仰っていませんでした", "仰る", "v5aru", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_aru() {
    assert_golden("仰ってる", "仰る", "v5aru", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_aru() {
    assert_golden("仰ってない", "仰る", "v5aru", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_aru() {
    assert_golden("仰ってた", "仰る", "v5aru", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_aru() {
    assert_golden("仰ってなかった", "仰る", "v5aru", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_aru() {
    assert_golden("仰ってます", "仰る", "v5aru", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_aru() {
    assert_golden("仰ってません", "仰る", "v5aru", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_aru() {
    assert_golden("仰ってました", "仰る", "v5aru", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_aru() {
    assert_golden("仰ってません", "仰る", "v5aru", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_aru() {
    assert_golden("仰ってませんでした", "仰る", "v5aru", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_aru() {
    assert_golden("仰ってしまう", "仰る", "v5aru", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_aru() {
    assert_golden("仰ってもう", "仰る", "v5aru", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_aru() {
    assert_golden("仰ってしまわない", "仰る", "v5aru", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_aru() {
    assert_golden("仰ってしまった", "仰る", "v5aru", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_aru() {
    assert_golden("仰ってしまわなかった", "仰る", "v5aru", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_aru() {
    assert_golden("仰ってしまって", "仰る", "v5aru", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_aru() {
    assert_golden("仰ってしまえば", "仰る", "v5aru", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_aru() {
    assert_golden("仰ってしまわなければ", "仰る", "v5aru", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_aru() {
    assert_golden("仰ってしまわなかったら", "仰る", "v5aru", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_aru() {
    assert_golden("仰ってしまったら", "仰る", "v5aru", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_aru() {
    assert_golden("仰ってしまおう", "仰る", "v5aru", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_aru() {
    assert_golden("仰ってしまいます", "仰る", "v5aru", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_aru() {
    assert_golden("仰ってしまいません", "仰る", "v5aru", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_aru() {
    assert_golden("仰ってしまいました", "仰る", "v5aru", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_aru() {
    assert_golden("仰ってしまいませんでした", "仰る", "v5aru", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_aru() {
    assert_golden("仰ってしまえる", "仰る", "v5aru", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_aru() {
    assert_golden("仰ってしまわれる", "仰る", "v5aru", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_aru() {
    assert_golden("仰ってしまわせる", "仰る", "v5aru", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_aru() {
    assert_golden("仰っちゃう", "仰る", "v5aru", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_aru() {
    assert_golden("仰っちゃわない", "仰る", "v5aru", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_aru() {
    assert_golden("仰っちゃった", "仰る", "v5aru", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_aru() {
    assert_golden("仰っちゃわなかった", "仰る", "v5aru", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_aru() {
    assert_golden("仰っちゃって", "仰る", "v5aru", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_aru() {
    assert_golden("仰っちゃえば", "仰る", "v5aru", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_aru() {
    assert_golden("仰っちゃわなければ", "仰る", "v5aru", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_aru() {
    assert_golden("仰っちゃわなかったら", "仰る", "v5aru", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_aru() {
    assert_golden("仰っちゃおう", "仰る", "v5aru", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_aru() {
    assert_golden("仰っちゃえる", "仰る", "v5aru", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_aru() {
    assert_golden("仰っておく", "仰る", "v5aru", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_aru() {
    assert_golden("仰っておかない", "仰る", "v5aru", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_aru() {
    assert_golden("仰っておいた", "仰る", "v5aru", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_aru() {
    assert_golden("仰っておかなかった", "仰る", "v5aru", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_aru() {
    assert_golden("仰っておいて", "仰る", "v5aru", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_aru() {
    assert_golden("仰っておけば", "仰る", "v5aru", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_aru() {
    assert_golden("仰っておいたら", "仰る", "v5aru", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_aru() {
    assert_golden("仰っておこう", "仰る", "v5aru", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_aru() {
    assert_golden("仰っておける", "仰る", "v5aru", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_aru() {
    assert_golden("仰っておかれる", "仰る", "v5aru", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_aru() {
    assert_golden("仰っとく", "仰る", "v5aru", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_aru() {
    assert_golden("仰っとかない", "仰る", "v5aru", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_aru() {
    assert_golden("仰っといた", "仰る", "v5aru", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_aru() {
    assert_golden("仰っとかなかった", "仰る", "v5aru", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_aru() {
    assert_golden("仰っといて", "仰る", "v5aru", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_aru() {
    assert_golden("仰っとけば", "仰る", "v5aru", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_aru() {
    assert_golden("仰っといたら", "仰る", "v5aru", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_aru() {
    assert_golden("仰っとこう", "仰る", "v5aru", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_aru() {
    assert_golden("仰っとける", "仰る", "v5aru", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_aru() {
    assert_golden("仰っとかれる", "仰る", "v5aru", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_aru() {
    assert_golden("仰ってある", "仰る", "v5aru", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_aru() {
    assert_golden("仰ってあった", "仰る", "v5aru", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_aru() {
    assert_golden("仰ってあって", "仰る", "v5aru", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_aru() {
    assert_golden("仰ってあったら", "仰る", "v5aru", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_aru() {
    assert_golden("仰ってあれば", "仰る", "v5aru", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_aru() {
    assert_golden("仰っていく", "仰る", "v5aru", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_aru() {
    assert_golden("仰っていかない", "仰る", "v5aru", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_aru() {
    assert_golden("仰っていった", "仰る", "v5aru", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_aru() {
    assert_golden("仰っていかなかった", "仰る", "v5aru", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_aru() {
    assert_golden("仰っていって", "仰る", "v5aru", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_aru() {
    assert_golden("仰っていこう", "仰る", "v5aru", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_aru() {
    assert_golden("仰っていける", "仰る", "v5aru", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_aru() {
    assert_golden("仰っていかれる", "仰る", "v5aru", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_aru() {
    assert_golden("仰っていかせる", "仰る", "v5aru", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_aru() {
    assert_golden("仰ってくる", "仰る", "v5aru", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_aru() {
    assert_golden("仰ってこない", "仰る", "v5aru", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_aru() {
    assert_golden("仰ってきた", "仰る", "v5aru", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_aru() {
    assert_golden("仰ってこなかった", "仰る", "v5aru", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_aru() {
    assert_golden("仰ってきて", "仰る", "v5aru", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_aru() {
    assert_golden("仰ってくれば", "仰る", "v5aru", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_aru() {
    assert_golden("仰ってきたら", "仰る", "v5aru", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_aru() {
    assert_golden("仰ってこられる", "仰る", "v5aru", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_aru() {
    assert_golden("仰ってこさせる", "仰る", "v5aru", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_aru() {
    assert_golden("仰いながら", "仰る", "v5aru", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_aru() {
    assert_golden("仰いすぎる", "仰る", "v5aru", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_aru() {
    assert_golden("仰いそう", "仰る", "v5aru", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_aru() {
    assert_golden("仰いぬ", "仰る", "v5aru", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_aru() {
    assert_golden("仰いず", "仰る", "v5aru", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_aru() {
    assert_golden("仰いずに", "仰る", "v5aru", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_aru() {
    assert_golden("仰ったり", "仰る", "v5aru", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_aru() {
    assert_golden("仰らなかったり", "仰る", "v5aru", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_negative_v5_aru() {
    assert_golden("仰らん", "仰る", "v5aru", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_aru() {
    assert_golden("仰らんかった", "仰る", "v5aru", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_aru() {
    assert_golden("仰いざる", "仰る", "v5aru", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_aru() {
    assert_golden("仰れよう", "仰る", "v5aru", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_aru() {
    assert_golden("仰れよ", "仰る", "v5aru", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_aru() {
    assert_golden("仰れろ", "仰る", "v5aru", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_aru() {
    assert_golden("仰れて", "仰る", "v5aru", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_aru() {
    assert_golden("仰れたら", "仰る", "v5aru", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_aru() {
    assert_golden("仰れれば", "仰る", "v5aru", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_honorific_v5_aru() {
    assert_golden("仰れられる", "仰る", "v5aru", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_aru() {
    assert_golden("仰れさせる", "仰る", "v5aru", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_aru() {
    assert_golden("仰ってあげる", "仰る", "v5aru", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_aru() {
    assert_golden("仰ってあげられる", "仰る", "v5aru", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_aru() {
    assert_golden("仰っておる", "仰る", "v5aru", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_aru() {
    assert_golden("仰っておらない", "仰る", "v5aru", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_aru() {
    assert_golden("仰っておらん", "仰る", "v5aru", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_aru() {
    assert_golden("仰っておった", "仰る", "v5aru", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_aru() {
    assert_golden("仰っておらなかった", "仰る", "v5aru", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_aru() {
    assert_golden("仰っております", "仰る", "v5aru", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_aru() {
    assert_golden("仰っておりません", "仰る", "v5aru", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_aru() {
    assert_golden("仰っておりました", "仰る", "v5aru", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_aru() {
    assert_golden("仰っておりませんでした", "仰る", "v5aru", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_aru() {
    assert_golden("仰っておって", "仰る", "v5aru", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_aru() {
    assert_golden("仰っておろう", "仰る", "v5aru", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_aru() {
    assert_golden("仰っておれる", "仰る", "v5aru", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_aru() {
    assert_golden("仰っておられる", "仰る", "v5aru", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_aru() {
    assert_golden("仰っとる", "仰る", "v5aru", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_aru() {
    assert_golden("仰っとらない", "仰る", "v5aru", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_aru() {
    assert_golden("仰っとらん", "仰る", "v5aru", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_aru() {
    assert_golden("仰っとった", "仰る", "v5aru", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_aru() {
    assert_golden("仰っとらなかった", "仰る", "v5aru", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_aru() {
    assert_golden("仰っとります", "仰る", "v5aru", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_aru() {
    assert_golden("仰っとりません", "仰る", "v5aru", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_aru() {
    assert_golden("仰っとりました", "仰る", "v5aru", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_aru() {
    assert_golden("仰っとりませんでした", "仰る", "v5aru", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_aru() {
    assert_golden("仰っとって", "仰る", "v5aru", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_aru() {
    assert_golden("仰っとろう", "仰る", "v5aru", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_aru() {
    assert_golden("仰っとれる", "仰る", "v5aru", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_aru() {
    assert_golden("仰っとられる", "仰る", "v5aru", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_aru() {
    assert_golden("仰らす", "仰る", "v5aru", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_aru() {
    assert_golden("仰っては", "仰る", "v5aru", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_aru() {
    assert_golden("仰っちゃ", "仰る", "v5aru", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_aru() {
    assert_golden("仰らなきゃ", "仰る", "v5aru", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_aru() {
    assert_golden("仰っちまう", "仰る", "v5aru", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_aru() {
    assert_golden("仰っちゃう", "仰る", "v5aru", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_aru() {
    assert_golden("仰っていらっしゃる", "仰る", "v5aru", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_aru() {
    assert_golden("仰っていらっしゃらない", "仰る", "v5aru", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_aru() {
    assert_golden("仰いつつ", "仰る", "v5aru", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_aru() {
    assert_golden("仰ってくれる", "仰る", "v5aru", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_aru() {
    assert_golden("仰ってくれない", "仰る", "v5aru", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_aru() {
    assert_golden("仰ってくれます", "仰る", "v5aru", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_aru() {
    assert_golden("仰ってくれません", "仰る", "v5aru", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_aru() {
    assert_golden("仰ってくれ", "仰る", "v5aru", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_aru() {
    assert_golden("仰らへん", "仰る", "v5aru", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_aru() {
    assert_golden("仰らへんかった", "仰る", "v5aru", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_aru() {
    assert_golden("仰らひん", "仰る", "v5aru", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_aru() {
    assert_golden("仰らひんかった", "仰る", "v5aru", "～negative→ksb→past");
}

#[test]
fn deconjugate_contracted_provisional_conditional_rya_v5_aru() {
    assert_golden("仰りゃ", "仰る", "v5aru", "～provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_aru() {
    assert_golden("仰らさない", "仰る", "v5aru", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_aru() {
    assert_golden("仰いましたら", "仰る", "v5aru", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_aru() {
    assert_golden("仰るになる", "仰る", "v5aru", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_aru() {
    assert_golden("仰いなさる", "仰る", "v5aru", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_aru() {
    assert_golden("仰ってはる", "仰る", "v5aru", "～teru→honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_aru() {
    assert_golden("仰いなさるな", "仰る", "v5aru", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_aru() {
    assert_golden("仰るまい", "仰る", "v5aru", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_aru() {
    assert_golden("仰りますまい", "仰る", "v5aru", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_aru() {
    assert_golden("仰らば", "仰る", "v5aru", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_aru() {
    assert_golden("仰らねば", "仰る", "v5aru", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_aru() {
    assert_golden("仰らにゃ", "仰る", "v5aru", "～colloquial negative conditional");
}
