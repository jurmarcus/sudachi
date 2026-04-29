//! Golden tests ported from JL's `DeconjugatorTestsForV5KS.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V5KS.

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
fn deconjugate_masu_stem_v5_ks() {
    assert_golden("行き", "行く", "v5k-s", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_ks() {
    assert_golden("行かない", "行く", "v5k-s", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_ks() {
    assert_golden("行きます", "行く", "v5k-s", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_ks() {
    assert_golden("行きましょう", "行く", "v5k-s", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_ks() {
    assert_golden("行きません", "行く", "v5k-s", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_ks() {
    assert_golden("行った", "行く", "v5k-s", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_ks() {
    assert_golden("行かなかった", "行く", "v5k-s", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_ks() {
    assert_golden("行きました", "行く", "v5k-s", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_ks() {
    assert_golden("行きませんでした", "行く", "v5k-s", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_ks() {
    assert_golden("行って", "行く", "v5k-s", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_ks() {
    assert_golden("行かなくて", "行く", "v5k-s", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_ks() {
    assert_golden("行かないで", "行く", "v5k-s", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_ks() {
    assert_golden("行きまして", "行く", "v5k-s", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_ks() {
    assert_golden("行ける", "行く", "v5k-s", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_ks() {
    assert_golden("行かれる", "行く", "v5k-s", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_ks() {
    assert_golden("行けない", "行く", "v5k-s", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_ks() {
    assert_golden("行かれない", "行く", "v5k-s", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_ks() {
    assert_golden("行けた", "行く", "v5k-s", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_ks() {
    assert_golden("行かれた", "行く", "v5k-s", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_ks() {
    assert_golden("行けました", "行く", "v5k-s", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_ks() {
    assert_golden("行かれました", "行く", "v5k-s", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_ks() {
    assert_golden("行けなかった", "行く", "v5k-s", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_ks() {
    assert_golden("行かれなかった", "行く", "v5k-s", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_ks() {
    assert_golden("行けませんでした", "行く", "v5k-s", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_ks() {
    assert_golden("行かれませんでした", "行く", "v5k-s", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_ks() {
    assert_golden("行けます", "行く", "v5k-s", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_ks() {
    assert_golden("行かれます", "行く", "v5k-s", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_ks() {
    assert_golden("行けません", "行く", "v5k-s", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_ks() {
    assert_golden("行かれません", "行く", "v5k-s", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_ks() {
    assert_golden("行け", "行く", "v5k-s", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_ks() {
    assert_golden("行くな", "行く", "v5k-s", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_ks() {
    assert_golden("行きなさい", "行く", "v5k-s", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_ks() {
    assert_golden("行ってください", "行く", "v5k-s", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_ks() {
    assert_golden("行かないでください", "行く", "v5k-s", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_ks() {
    assert_golden("行こう", "行く", "v5k-s", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_ks() {
    assert_golden("行こ", "行く", "v5k-s", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_ks() {
    assert_golden("行きましょう", "行く", "v5k-s", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_ks() {
    assert_golden("行けば", "行く", "v5k-s", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_ks() {
    assert_golden("行かなければ", "行く", "v5k-s", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_ks() {
    assert_golden("行ったら", "行く", "v5k-s", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_ks() {
    assert_golden("行ったらば", "行く", "v5k-s", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_ks() {
    assert_golden("行かなかったら", "行く", "v5k-s", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_ks() {
    assert_golden("行かせる", "行く", "v5k-s", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_ks() {
    assert_golden("行かせない", "行く", "v5k-s", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_ks() {
    assert_golden("行かせん", "行く", "v5k-s", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_ks() {
    assert_golden("行かせます", "行く", "v5k-s", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_ks() {
    assert_golden("行かします", "行く", "v5k-s", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_ks() {
    assert_golden("行かせません", "行く", "v5k-s", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_ks() {
    assert_golden("行かせた", "行く", "v5k-s", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_ks() {
    assert_golden("行かせなかった", "行く", "v5k-s", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_ks() {
    assert_golden("行かせました", "行く", "v5k-s", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_ks() {
    assert_golden("行かせませんでした", "行く", "v5k-s", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_ks() {
    assert_golden("行かせられる", "行く", "v5k-s", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_ks() {
    assert_golden("行かせられない", "行く", "v5k-s", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_ks() {
    assert_golden("行かせられます", "行く", "v5k-s", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_ks() {
    assert_golden("行かせられません", "行く", "v5k-s", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_ks() {
    assert_golden("行きたい", "行く", "v5k-s", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_ks() {
    assert_golden("行きたくありません", "行く", "v5k-s", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_ks() {
    assert_golden("行きたくありませんでした", "行く", "v5k-s", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_ks() {
    assert_golden("行きたくない", "行く", "v5k-s", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_ks() {
    assert_golden("行きたかった", "行く", "v5k-s", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_ks() {
    assert_golden("行きたくなかった", "行く", "v5k-s", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_ks() {
    assert_golden("行っている", "行く", "v5k-s", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_ks() {
    assert_golden("行っていない", "行く", "v5k-s", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_ks() {
    assert_golden("行っていた", "行く", "v5k-s", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_ks() {
    assert_golden("行っていなかった", "行く", "v5k-s", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_ks() {
    assert_golden("行っています", "行く", "v5k-s", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_ks() {
    assert_golden("行っていません", "行く", "v5k-s", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_ks() {
    assert_golden("行っていました", "行く", "v5k-s", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_ks() {
    assert_golden("行っていませんでした", "行く", "v5k-s", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_ks() {
    assert_golden("行ってる", "行く", "v5k-s", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_ks() {
    assert_golden("行ってない", "行く", "v5k-s", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_ks() {
    assert_golden("行ってた", "行く", "v5k-s", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_ks() {
    assert_golden("行ってなかった", "行く", "v5k-s", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_ks() {
    assert_golden("行ってます", "行く", "v5k-s", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_ks() {
    assert_golden("行ってません", "行く", "v5k-s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_ks() {
    assert_golden("行ってました", "行く", "v5k-s", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_ks() {
    assert_golden("行ってません", "行く", "v5k-s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_ks() {
    assert_golden("行ってませんでした", "行く", "v5k-s", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_ks() {
    assert_golden("行ってしまう", "行く", "v5k-s", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_ks() {
    assert_golden("行ってもう", "行く", "v5k-s", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_ks() {
    assert_golden("行ってしまわない", "行く", "v5k-s", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_ks() {
    assert_golden("行ってしまった", "行く", "v5k-s", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_ks() {
    assert_golden("行ってしまわなかった", "行く", "v5k-s", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_ks() {
    assert_golden("行ってしまって", "行く", "v5k-s", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_ks() {
    assert_golden("行ってしまえば", "行く", "v5k-s", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_ks() {
    assert_golden("行ってしまわなければ", "行く", "v5k-s", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_ks() {
    assert_golden("行ってしまわなかったら", "行く", "v5k-s", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_ks() {
    assert_golden("行ってしまったら", "行く", "v5k-s", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_ks() {
    assert_golden("行ってしまおう", "行く", "v5k-s", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_ks() {
    assert_golden("行ってしまいます", "行く", "v5k-s", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_ks() {
    assert_golden("行ってしまいません", "行く", "v5k-s", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_ks() {
    assert_golden("行ってしまいました", "行く", "v5k-s", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_ks() {
    assert_golden("行ってしまいませんでした", "行く", "v5k-s", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_ks() {
    assert_golden("行ってしまえる", "行く", "v5k-s", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_ks() {
    assert_golden("行ってしまわれる", "行く", "v5k-s", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_ks() {
    assert_golden("行ってしまわせる", "行く", "v5k-s", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_ks() {
    assert_golden("行っちゃう", "行く", "v5k-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_ks() {
    assert_golden("行っちゃわない", "行く", "v5k-s", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_ks() {
    assert_golden("行っちゃった", "行く", "v5k-s", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_ks() {
    assert_golden("行っちゃわなかった", "行く", "v5k-s", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_ks() {
    assert_golden("行っちゃって", "行く", "v5k-s", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_ks() {
    assert_golden("行っちゃえば", "行く", "v5k-s", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_ks() {
    assert_golden("行っちゃわなければ", "行く", "v5k-s", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_ks() {
    assert_golden("行っちゃわなかったら", "行く", "v5k-s", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_ks() {
    assert_golden("行っちゃおう", "行く", "v5k-s", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_ks() {
    assert_golden("行っちゃえる", "行く", "v5k-s", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_ks() {
    assert_golden("行っておく", "行く", "v5k-s", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_ks() {
    assert_golden("行っておかない", "行く", "v5k-s", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_ks() {
    assert_golden("行っておいた", "行く", "v5k-s", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_ks() {
    assert_golden("行っておかなかった", "行く", "v5k-s", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_ks() {
    assert_golden("行っておいて", "行く", "v5k-s", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_ks() {
    assert_golden("行っておけば", "行く", "v5k-s", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_ks() {
    assert_golden("行っておいたら", "行く", "v5k-s", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_ks() {
    assert_golden("行っておこう", "行く", "v5k-s", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_ks() {
    assert_golden("行っておける", "行く", "v5k-s", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_ks() {
    assert_golden("行っておかれる", "行く", "v5k-s", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_ks() {
    assert_golden("行っとく", "行く", "v5k-s", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_ks() {
    assert_golden("行っとかない", "行く", "v5k-s", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_ks() {
    assert_golden("行っといた", "行く", "v5k-s", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_ks() {
    assert_golden("行っとかなかった", "行く", "v5k-s", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_ks() {
    assert_golden("行っといて", "行く", "v5k-s", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_ks() {
    assert_golden("行っとけば", "行く", "v5k-s", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_ks() {
    assert_golden("行っといたら", "行く", "v5k-s", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_ks() {
    assert_golden("行っとこう", "行く", "v5k-s", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_ks() {
    assert_golden("行っとける", "行く", "v5k-s", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_ks() {
    assert_golden("行っとかれる", "行く", "v5k-s", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_ks() {
    assert_golden("行ってある", "行く", "v5k-s", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_ks() {
    assert_golden("行ってあった", "行く", "v5k-s", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_ks() {
    assert_golden("行ってあって", "行く", "v5k-s", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_ks() {
    assert_golden("行ってあったら", "行く", "v5k-s", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_ks() {
    assert_golden("行ってあれば", "行く", "v5k-s", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_ks() {
    assert_golden("行っていく", "行く", "v5k-s", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_ks() {
    assert_golden("行っていかない", "行く", "v5k-s", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_ks() {
    assert_golden("行っていった", "行く", "v5k-s", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_ks() {
    assert_golden("行っていかなかった", "行く", "v5k-s", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_ks() {
    assert_golden("行っていって", "行く", "v5k-s", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_ks() {
    assert_golden("行っていこう", "行く", "v5k-s", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_ks() {
    assert_golden("行っていける", "行く", "v5k-s", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_ks() {
    assert_golden("行っていかれる", "行く", "v5k-s", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_ks() {
    assert_golden("行っていかせる", "行く", "v5k-s", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_ks() {
    assert_golden("行ってくる", "行く", "v5k-s", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_ks() {
    assert_golden("行ってこない", "行く", "v5k-s", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_ks() {
    assert_golden("行ってきた", "行く", "v5k-s", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_ks() {
    assert_golden("行ってこなかった", "行く", "v5k-s", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_ks() {
    assert_golden("行ってきて", "行く", "v5k-s", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_ks() {
    assert_golden("行ってくれば", "行く", "v5k-s", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_ks() {
    assert_golden("行ってきたら", "行く", "v5k-s", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_ks() {
    assert_golden("行ってこられる", "行く", "v5k-s", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_ks() {
    assert_golden("行ってこさせる", "行く", "v5k-s", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_ks() {
    assert_golden("行きながら", "行く", "v5k-s", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_ks() {
    assert_golden("行きすぎる", "行く", "v5k-s", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_ks() {
    assert_golden("行きそう", "行く", "v5k-s", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_ks() {
    assert_golden("行かぬ", "行く", "v5k-s", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_ks() {
    assert_golden("行かず", "行く", "v5k-s", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_ks() {
    assert_golden("行かずに", "行く", "v5k-s", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_ks() {
    assert_golden("行ったり", "行く", "v5k-s", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_ks() {
    assert_golden("行かなかったり", "行く", "v5k-s", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_ks() {
    assert_golden("行かん", "行く", "v5k-s", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_ks() {
    assert_golden("行かんかった", "行く", "v5k-s", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_ks() {
    assert_golden("行かざる", "行く", "v5k-s", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_ks() {
    assert_golden("行けよう", "行く", "v5k-s", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_ks() {
    assert_golden("行けよ", "行く", "v5k-s", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_ks() {
    assert_golden("行けろ", "行く", "v5k-s", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_ks() {
    assert_golden("行けて", "行く", "v5k-s", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_ks() {
    assert_golden("行けたら", "行く", "v5k-s", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_ks() {
    assert_golden("行ければ", "行く", "v5k-s", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_ks() {
    assert_golden("行けられる", "行く", "v5k-s", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_ks() {
    assert_golden("行けさせる", "行く", "v5k-s", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_ks() {
    assert_golden("行ってあげる", "行く", "v5k-s", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_ks() {
    assert_golden("行ってあげられる", "行く", "v5k-s", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_ks() {
    assert_golden("行っておる", "行く", "v5k-s", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_ks() {
    assert_golden("行っておらない", "行く", "v5k-s", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_ks() {
    assert_golden("行っておらん", "行く", "v5k-s", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_ks() {
    assert_golden("行っておった", "行く", "v5k-s", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_ks() {
    assert_golden("行っておらなかった", "行く", "v5k-s", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_ks() {
    assert_golden("行っております", "行く", "v5k-s", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_ks() {
    assert_golden("行っておりません", "行く", "v5k-s", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_ks() {
    assert_golden("行っておりました", "行く", "v5k-s", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_ks() {
    assert_golden("行っておりませんでした", "行く", "v5k-s", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_ks() {
    assert_golden("行っておって", "行く", "v5k-s", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_ks() {
    assert_golden("行っておろう", "行く", "v5k-s", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_ks() {
    assert_golden("行っておれる", "行く", "v5k-s", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_ks() {
    assert_golden("行っておられる", "行く", "v5k-s", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_ks() {
    assert_golden("行っとる", "行く", "v5k-s", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_ks() {
    assert_golden("行っとらない", "行く", "v5k-s", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_ks() {
    assert_golden("行っとらん", "行く", "v5k-s", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_ks() {
    assert_golden("行っとった", "行く", "v5k-s", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_ks() {
    assert_golden("行っとらなかった", "行く", "v5k-s", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_ks() {
    assert_golden("行っとります", "行く", "v5k-s", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_ks() {
    assert_golden("行っとりません", "行く", "v5k-s", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_ks() {
    assert_golden("行っとりました", "行く", "v5k-s", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_ks() {
    assert_golden("行っとりませんでした", "行く", "v5k-s", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_ks() {
    assert_golden("行っとって", "行く", "v5k-s", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_ks() {
    assert_golden("行っとろう", "行く", "v5k-s", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_ks() {
    assert_golden("行っとれる", "行く", "v5k-s", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_ks() {
    assert_golden("行っとられる", "行く", "v5k-s", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_ks() {
    assert_golden("行かす", "行く", "v5k-s", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_ks() {
    assert_golden("行っては", "行く", "v5k-s", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_ks() {
    assert_golden("行っちゃ", "行く", "v5k-s", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_ks() {
    assert_golden("行かなきゃ", "行く", "v5k-s", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_ks() {
    assert_golden("行っちまう", "行く", "v5k-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_ks() {
    assert_golden("行っちゃう", "行く", "v5k-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_ks() {
    assert_golden("行っていらっしゃる", "行く", "v5k-s", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_ks() {
    assert_golden("行っていらっしゃらない", "行く", "v5k-s", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_ks() {
    assert_golden("行きつつ", "行く", "v5k-s", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_ks() {
    assert_golden("行ってくれる", "行く", "v5k-s", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_ks() {
    assert_golden("行ってくれない", "行く", "v5k-s", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_ks() {
    assert_golden("行ってくれます", "行く", "v5k-s", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_ks() {
    assert_golden("行ってくれません", "行く", "v5k-s", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_ks() {
    assert_golden("行ってくれ", "行く", "v5k-s", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_ks() {
    assert_golden("行かへん", "行く", "v5k-s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_ks() {
    assert_golden("行かへんかった", "行く", "v5k-s", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_ks() {
    assert_golden("行かひん", "行く", "v5k-s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_ks() {
    assert_golden("行かひんかった", "行く", "v5k-s", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_ks() {
    assert_golden("行かさない", "行く", "v5k-s", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_ks() {
    assert_golden("行きましたら", "行く", "v5k-s", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_ks() {
    assert_golden("行きになる", "行く", "v5k-s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_ks() {
    assert_golden("行きなさる", "行く", "v5k-s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_ks() {
    assert_golden("行きはる", "行く", "v5k-s", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_ks() {
    assert_golden("行きなさるな", "行く", "v5k-s", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_ks() {
    assert_golden("行くまい", "行く", "v5k-s", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_ks() {
    assert_golden("行きますまい", "行く", "v5k-s", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_ks() {
    assert_golden("行かば", "行く", "v5k-s", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_ks() {
    assert_golden("行かねば", "行く", "v5k-s", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_ks() {
    assert_golden("行かにゃ", "行く", "v5k-s", "～colloquial negative conditional");
}
