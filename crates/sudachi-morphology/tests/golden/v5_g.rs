//! Golden tests ported from JL's `DeconjugatorTestsForV5G.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V5G.

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
fn deconjugate_masu_stem_v5_g() {
    assert_golden("繋ぎ", "繋ぐ", "v5g", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_g() {
    assert_golden("繋がない", "繋ぐ", "v5g", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_g() {
    assert_golden("繋ぎます", "繋ぐ", "v5g", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_g() {
    assert_golden("繋ぎましょう", "繋ぐ", "v5g", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_g() {
    assert_golden("繋ぎません", "繋ぐ", "v5g", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_g() {
    assert_golden("繋いだ", "繋ぐ", "v5g", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_g() {
    assert_golden("繋がなかった", "繋ぐ", "v5g", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_g() {
    assert_golden("繋ぎました", "繋ぐ", "v5g", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_g() {
    assert_golden("繋ぎませんでした", "繋ぐ", "v5g", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_g() {
    assert_golden("繋いで", "繋ぐ", "v5g", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_g() {
    assert_golden("繋がなくて", "繋ぐ", "v5g", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_g() {
    assert_golden("繋がないで", "繋ぐ", "v5g", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_g() {
    assert_golden("繋ぎまして", "繋ぐ", "v5g", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_g() {
    assert_golden("繋げる", "繋ぐ", "v5g", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_g() {
    assert_golden("繋がれる", "繋ぐ", "v5g", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_g() {
    assert_golden("繋げない", "繋ぐ", "v5g", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_g() {
    assert_golden("繋がれない", "繋ぐ", "v5g", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_g() {
    assert_golden("繋げた", "繋ぐ", "v5g", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_g() {
    assert_golden("繋がれた", "繋ぐ", "v5g", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_g() {
    assert_golden("繋げました", "繋ぐ", "v5g", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_g() {
    assert_golden("繋がれました", "繋ぐ", "v5g", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_g() {
    assert_golden("繋げなかった", "繋ぐ", "v5g", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_g() {
    assert_golden("繋がれなかった", "繋ぐ", "v5g", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_g() {
    assert_golden("繋げませんでした", "繋ぐ", "v5g", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_g() {
    assert_golden("繋がれませんでした", "繋ぐ", "v5g", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_g() {
    assert_golden("繋げます", "繋ぐ", "v5g", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_g() {
    assert_golden("繋がれます", "繋ぐ", "v5g", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_g() {
    assert_golden("繋げません", "繋ぐ", "v5g", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_g() {
    assert_golden("繋がれません", "繋ぐ", "v5g", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_g() {
    assert_golden("繋げ", "繋ぐ", "v5g", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_g() {
    assert_golden("繋ぐな", "繋ぐ", "v5g", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_g() {
    assert_golden("繋ぎなさい", "繋ぐ", "v5g", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_g() {
    assert_golden("繋いでください", "繋ぐ", "v5g", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_g() {
    assert_golden("繋がないでください", "繋ぐ", "v5g", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_g() {
    assert_golden("繋ごう", "繋ぐ", "v5g", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_g() {
    assert_golden("繋ご", "繋ぐ", "v5g", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_g() {
    assert_golden("繋ぎましょう", "繋ぐ", "v5g", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_g() {
    assert_golden("繋げば", "繋ぐ", "v5g", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_g() {
    assert_golden("繋がなければ", "繋ぐ", "v5g", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_g() {
    assert_golden("繋いだら", "繋ぐ", "v5g", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_g() {
    assert_golden("繋いだらば", "繋ぐ", "v5g", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_g() {
    assert_golden("繋がなかったら", "繋ぐ", "v5g", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_g() {
    assert_golden("繋がせる", "繋ぐ", "v5g", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_g() {
    assert_golden("繋がせない", "繋ぐ", "v5g", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_g() {
    assert_golden("繋がせん", "繋ぐ", "v5g", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_g() {
    assert_golden("繋がせます", "繋ぐ", "v5g", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_g() {
    assert_golden("繋がします", "繋ぐ", "v5g", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_g() {
    assert_golden("繋がせません", "繋ぐ", "v5g", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_g() {
    assert_golden("繋がせた", "繋ぐ", "v5g", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_g() {
    assert_golden("繋がせなかった", "繋ぐ", "v5g", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_g() {
    assert_golden("繋がせました", "繋ぐ", "v5g", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_g() {
    assert_golden("繋がせませんでした", "繋ぐ", "v5g", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_g() {
    assert_golden("繋がせられる", "繋ぐ", "v5g", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_g() {
    assert_golden("繋がせられない", "繋ぐ", "v5g", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_g() {
    assert_golden("繋がせられます", "繋ぐ", "v5g", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_g() {
    assert_golden("繋がせられません", "繋ぐ", "v5g", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_g() {
    assert_golden("繋ぎたい", "繋ぐ", "v5g", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_g() {
    assert_golden("繋ぎたくありません", "繋ぐ", "v5g", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_g() {
    assert_golden("繋ぎたくありませんでした", "繋ぐ", "v5g", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_g() {
    assert_golden("繋ぎたくない", "繋ぐ", "v5g", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_g() {
    assert_golden("繋ぎたかった", "繋ぐ", "v5g", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_g() {
    assert_golden("繋ぎたくなかった", "繋ぐ", "v5g", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_g() {
    assert_golden("繋いでいる", "繋ぐ", "v5g", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_g() {
    assert_golden("繋いでいない", "繋ぐ", "v5g", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_g() {
    assert_golden("繋いでいた", "繋ぐ", "v5g", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_g() {
    assert_golden("繋いでいなかった", "繋ぐ", "v5g", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_g() {
    assert_golden("繋いでいます", "繋ぐ", "v5g", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_g() {
    assert_golden("繋いでいません", "繋ぐ", "v5g", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_g() {
    assert_golden("繋いでいました", "繋ぐ", "v5g", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_g() {
    assert_golden("繋いでいませんでした", "繋ぐ", "v5g", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_g() {
    assert_golden("繋いでる", "繋ぐ", "v5g", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_g() {
    assert_golden("繋いでない", "繋ぐ", "v5g", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_g() {
    assert_golden("繋いでた", "繋ぐ", "v5g", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_g() {
    assert_golden("繋いでなかった", "繋ぐ", "v5g", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_g() {
    assert_golden("繋いでます", "繋ぐ", "v5g", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_g() {
    assert_golden("繋いでません", "繋ぐ", "v5g", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_g() {
    assert_golden("繋いでました", "繋ぐ", "v5g", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_g() {
    assert_golden("繋いでません", "繋ぐ", "v5g", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_g() {
    assert_golden("繋いでませんでした", "繋ぐ", "v5g", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_g() {
    assert_golden("繋いでしまう", "繋ぐ", "v5g", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_g() {
    assert_golden("繋いでもう", "繋ぐ", "v5g", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_g() {
    assert_golden("繋いでしまわない", "繋ぐ", "v5g", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_g() {
    assert_golden("繋いでしまった", "繋ぐ", "v5g", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_g() {
    assert_golden("繋いでしまわなかった", "繋ぐ", "v5g", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_g() {
    assert_golden("繋いでしまって", "繋ぐ", "v5g", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_g() {
    assert_golden("繋いでしまえば", "繋ぐ", "v5g", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_g() {
    assert_golden("繋いでしまわなければ", "繋ぐ", "v5g", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_g() {
    assert_golden("繋いでしまわなかったら", "繋ぐ", "v5g", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_g() {
    assert_golden("繋いでしまったら", "繋ぐ", "v5g", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_g() {
    assert_golden("繋いでしまおう", "繋ぐ", "v5g", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_g() {
    assert_golden("繋いでしまいます", "繋ぐ", "v5g", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_g() {
    assert_golden("繋いでしまいません", "繋ぐ", "v5g", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_g() {
    assert_golden("繋いでしまいました", "繋ぐ", "v5g", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_g() {
    assert_golden("繋いでしまいませんでした", "繋ぐ", "v5g", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_g() {
    assert_golden("繋いでしまえる", "繋ぐ", "v5g", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_g() {
    assert_golden("繋いでしまわれる", "繋ぐ", "v5g", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_g() {
    assert_golden("繋いでしまわせる", "繋ぐ", "v5g", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_g() {
    assert_golden("繋いじゃう", "繋ぐ", "v5g", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_g() {
    assert_golden("繋いじゃわない", "繋ぐ", "v5g", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_g() {
    assert_golden("繋いじゃった", "繋ぐ", "v5g", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_g() {
    assert_golden("繋いじゃわなかった", "繋ぐ", "v5g", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_g() {
    assert_golden("繋いじゃって", "繋ぐ", "v5g", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_g() {
    assert_golden("繋いじゃえば", "繋ぐ", "v5g", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_g() {
    assert_golden("繋いじゃわなければ", "繋ぐ", "v5g", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_g() {
    assert_golden("繋いじゃわなかったら", "繋ぐ", "v5g", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_g() {
    assert_golden("繋いじゃおう", "繋ぐ", "v5g", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_g() {
    assert_golden("繋いじゃえる", "繋ぐ", "v5g", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_g() {
    assert_golden("繋いでおく", "繋ぐ", "v5g", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_g() {
    assert_golden("繋いでおかない", "繋ぐ", "v5g", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_g() {
    assert_golden("繋いでおいた", "繋ぐ", "v5g", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_g() {
    assert_golden("繋いでおかなかった", "繋ぐ", "v5g", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_g() {
    assert_golden("繋いでおいて", "繋ぐ", "v5g", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_g() {
    assert_golden("繋いでおけば", "繋ぐ", "v5g", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_g() {
    assert_golden("繋いでおいたら", "繋ぐ", "v5g", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_g() {
    assert_golden("繋いでおこう", "繋ぐ", "v5g", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_g() {
    assert_golden("繋いでおける", "繋ぐ", "v5g", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_g() {
    assert_golden("繋いでおかれる", "繋ぐ", "v5g", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_g() {
    assert_golden("繋いどく", "繋ぐ", "v5g", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_g() {
    assert_golden("繋いどかない", "繋ぐ", "v5g", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_g() {
    assert_golden("繋いどいた", "繋ぐ", "v5g", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_g() {
    assert_golden("繋いどかなかった", "繋ぐ", "v5g", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_g() {
    assert_golden("繋いどいて", "繋ぐ", "v5g", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_g() {
    assert_golden("繋いどけば", "繋ぐ", "v5g", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_g() {
    assert_golden("繋いどいたら", "繋ぐ", "v5g", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_g() {
    assert_golden("繋いどこう", "繋ぐ", "v5g", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_g() {
    assert_golden("繋いどける", "繋ぐ", "v5g", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_g() {
    assert_golden("繋いどかれる", "繋ぐ", "v5g", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_g() {
    assert_golden("繋いである", "繋ぐ", "v5g", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_g() {
    assert_golden("繋いであった", "繋ぐ", "v5g", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_g() {
    assert_golden("繋いであって", "繋ぐ", "v5g", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_g() {
    assert_golden("繋いであったら", "繋ぐ", "v5g", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_g() {
    assert_golden("繋いであれば", "繋ぐ", "v5g", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_g() {
    assert_golden("繋いでいく", "繋ぐ", "v5g", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_g() {
    assert_golden("繋いでいかない", "繋ぐ", "v5g", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_g() {
    assert_golden("繋いでいった", "繋ぐ", "v5g", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_g() {
    assert_golden("繋いでいかなかった", "繋ぐ", "v5g", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_g() {
    assert_golden("繋いでいって", "繋ぐ", "v5g", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_g() {
    assert_golden("繋いでいこう", "繋ぐ", "v5g", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_g() {
    assert_golden("繋いでいける", "繋ぐ", "v5g", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_g() {
    assert_golden("繋いでいかれる", "繋ぐ", "v5g", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_g() {
    assert_golden("繋いでいかせる", "繋ぐ", "v5g", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_g() {
    assert_golden("繋いでくる", "繋ぐ", "v5g", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_g() {
    assert_golden("繋いでこない", "繋ぐ", "v5g", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_g() {
    assert_golden("繋いできた", "繋ぐ", "v5g", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_g() {
    assert_golden("繋いでこなかった", "繋ぐ", "v5g", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_g() {
    assert_golden("繋いできて", "繋ぐ", "v5g", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_g() {
    assert_golden("繋いでくれば", "繋ぐ", "v5g", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_g() {
    assert_golden("繋いできたら", "繋ぐ", "v5g", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_g() {
    assert_golden("繋いでこられる", "繋ぐ", "v5g", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_g() {
    assert_golden("繋いでこさせる", "繋ぐ", "v5g", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_g() {
    assert_golden("繋ぎながら", "繋ぐ", "v5g", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_g() {
    assert_golden("繋ぎすぎる", "繋ぐ", "v5g", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_g() {
    assert_golden("繋ぎそう", "繋ぐ", "v5g", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_g() {
    assert_golden("繋がぬ", "繋ぐ", "v5g", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_g() {
    assert_golden("繋がず", "繋ぐ", "v5g", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_g() {
    assert_golden("繋がずに", "繋ぐ", "v5g", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_g() {
    assert_golden("繋いだり", "繋ぐ", "v5g", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_g() {
    assert_golden("繋がなかったり", "繋ぐ", "v5g", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_g() {
    assert_golden("繋がん", "繋ぐ", "v5g", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_g() {
    assert_golden("繋がんかった", "繋ぐ", "v5g", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_g() {
    assert_golden("繋がざる", "繋ぐ", "v5g", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_g() {
    assert_golden("繋げよう", "繋ぐ", "v5g", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_g() {
    assert_golden("繋げよ", "繋ぐ", "v5g", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_g() {
    assert_golden("繋げろ", "繋ぐ", "v5g", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_g() {
    assert_golden("繋げて", "繋ぐ", "v5g", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_g() {
    assert_golden("繋げたら", "繋ぐ", "v5g", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_g() {
    assert_golden("繋げれば", "繋ぐ", "v5g", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_g() {
    assert_golden("繋げられる", "繋ぐ", "v5g", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_g() {
    assert_golden("繋げさせる", "繋ぐ", "v5g", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_g() {
    assert_golden("繋いであげる", "繋ぐ", "v5g", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_g() {
    assert_golden("繋いであげられる", "繋ぐ", "v5g", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_g() {
    assert_golden("繋いでおる", "繋ぐ", "v5g", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_g() {
    assert_golden("繋いでおらない", "繋ぐ", "v5g", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_g() {
    assert_golden("繋いでおらん", "繋ぐ", "v5g", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_g() {
    assert_golden("繋いでおった", "繋ぐ", "v5g", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_g() {
    assert_golden("繋いでおらなかった", "繋ぐ", "v5g", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_g() {
    assert_golden("繋いでおります", "繋ぐ", "v5g", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_g() {
    assert_golden("繋いでおりません", "繋ぐ", "v5g", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_g() {
    assert_golden("繋いでおりました", "繋ぐ", "v5g", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_g() {
    assert_golden("繋いでおりませんでした", "繋ぐ", "v5g", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_g() {
    assert_golden("繋いでおって", "繋ぐ", "v5g", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_g() {
    assert_golden("繋いでおろう", "繋ぐ", "v5g", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_g() {
    assert_golden("繋いでおれる", "繋ぐ", "v5g", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_g() {
    assert_golden("繋いでおられる", "繋ぐ", "v5g", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_g() {
    assert_golden("繋いどる", "繋ぐ", "v5g", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_g() {
    assert_golden("繋いどらない", "繋ぐ", "v5g", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_g() {
    assert_golden("繋いどらん", "繋ぐ", "v5g", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_g() {
    assert_golden("繋いどった", "繋ぐ", "v5g", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_g() {
    assert_golden("繋いどらなかった", "繋ぐ", "v5g", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_g() {
    assert_golden("繋いどります", "繋ぐ", "v5g", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_g() {
    assert_golden("繋いどりません", "繋ぐ", "v5g", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_g() {
    assert_golden("繋いどりました", "繋ぐ", "v5g", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_g() {
    assert_golden("繋いどりませんでした", "繋ぐ", "v5g", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_g() {
    assert_golden("繋いどって", "繋ぐ", "v5g", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_g() {
    assert_golden("繋いどろう", "繋ぐ", "v5g", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_g() {
    assert_golden("繋いどれる", "繋ぐ", "v5g", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_g() {
    assert_golden("繋いどられる", "繋ぐ", "v5g", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_g() {
    assert_golden("繋がす", "繋ぐ", "v5g", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_g() {
    assert_golden("繋いでは", "繋ぐ", "v5g", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_g() {
    assert_golden("繋いじゃ", "繋ぐ", "v5g", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_g() {
    assert_golden("繋がなきゃ", "繋ぐ", "v5g", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_g() {
    assert_golden("繋いじまう", "繋ぐ", "v5g", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_g() {
    assert_golden("繋いじゃう", "繋ぐ", "v5g", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_g() {
    assert_golden("繋いでいらっしゃる", "繋ぐ", "v5g", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_g() {
    assert_golden("繋いでいらっしゃらない", "繋ぐ", "v5g", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_g() {
    assert_golden("繋ぎつつ", "繋ぐ", "v5g", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_g() {
    assert_golden("繋いでくれる", "繋ぐ", "v5g", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_g() {
    assert_golden("繋いでくれない", "繋ぐ", "v5g", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_g() {
    assert_golden("繋いでくれます", "繋ぐ", "v5g", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_g() {
    assert_golden("繋いでくれません", "繋ぐ", "v5g", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_g() {
    assert_golden("繋いでくれ", "繋ぐ", "v5g", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_g() {
    assert_golden("繋がへん", "繋ぐ", "v5g", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_g() {
    assert_golden("繋がへんかった", "繋ぐ", "v5g", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_g() {
    assert_golden("繋がひん", "繋ぐ", "v5g", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_g() {
    assert_golden("繋がひんかった", "繋ぐ", "v5g", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_g() {
    assert_golden("繋がさない", "繋ぐ", "v5g", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_g() {
    assert_golden("繋ぎましたら", "繋ぐ", "v5g", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_g() {
    assert_golden("繋ぎになる", "繋ぐ", "v5g", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_g() {
    assert_golden("繋ぎなさる", "繋ぐ", "v5g", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_g() {
    assert_golden("繋ぎはる", "繋ぐ", "v5g", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_g() {
    assert_golden("繋ぎなさるな", "繋ぐ", "v5g", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_g() {
    assert_golden("繋ぐまい", "繋ぐ", "v5g", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_g() {
    assert_golden("繋ぎますまい", "繋ぐ", "v5g", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_g() {
    assert_golden("繋がば", "繋ぐ", "v5g", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_g() {
    assert_golden("繋がねば", "繋ぐ", "v5g", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_g() {
    assert_golden("繋がにゃ", "繋ぐ", "v5g", "～colloquial negative conditional");
}
