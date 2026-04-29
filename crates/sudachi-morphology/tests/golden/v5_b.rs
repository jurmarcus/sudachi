//! Golden tests ported from JL's `DeconjugatorTestsForV5B.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V5B.

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
fn deconjugate_masu_stem_v5_b() {
    assert_golden("選び", "選ぶ", "v5b", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_b() {
    assert_golden("選ばない", "選ぶ", "v5b", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_b() {
    assert_golden("選びます", "選ぶ", "v5b", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_b() {
    assert_golden("選びましょう", "選ぶ", "v5b", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_b() {
    assert_golden("選びません", "選ぶ", "v5b", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_b() {
    assert_golden("選んだ", "選ぶ", "v5b", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_b() {
    assert_golden("選ばなかった", "選ぶ", "v5b", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_b() {
    assert_golden("選びました", "選ぶ", "v5b", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_b() {
    assert_golden("選びませんでした", "選ぶ", "v5b", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_b() {
    assert_golden("選んで", "選ぶ", "v5b", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_b() {
    assert_golden("選ばなくて", "選ぶ", "v5b", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_b() {
    assert_golden("選ばないで", "選ぶ", "v5b", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_b() {
    assert_golden("選びまして", "選ぶ", "v5b", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_b() {
    assert_golden("選べる", "選ぶ", "v5b", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_b() {
    assert_golden("選ばれる", "選ぶ", "v5b", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_b() {
    assert_golden("選べない", "選ぶ", "v5b", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_b() {
    assert_golden("選ばれない", "選ぶ", "v5b", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_b() {
    assert_golden("選べた", "選ぶ", "v5b", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_b() {
    assert_golden("選ばれた", "選ぶ", "v5b", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_b() {
    assert_golden("選べました", "選ぶ", "v5b", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_b() {
    assert_golden("選ばれました", "選ぶ", "v5b", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_b() {
    assert_golden("選べなかった", "選ぶ", "v5b", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_b() {
    assert_golden("選ばれなかった", "選ぶ", "v5b", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_b() {
    assert_golden("選べませんでした", "選ぶ", "v5b", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_b() {
    assert_golden("選ばれませんでした", "選ぶ", "v5b", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_b() {
    assert_golden("選べます", "選ぶ", "v5b", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_b() {
    assert_golden("選ばれます", "選ぶ", "v5b", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_b() {
    assert_golden("選べません", "選ぶ", "v5b", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_b() {
    assert_golden("選ばれません", "選ぶ", "v5b", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_b() {
    assert_golden("選べ", "選ぶ", "v5b", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_b() {
    assert_golden("選ぶな", "選ぶ", "v5b", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_b() {
    assert_golden("選びなさい", "選ぶ", "v5b", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_b() {
    assert_golden("選んでください", "選ぶ", "v5b", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_b() {
    assert_golden("選ばないでください", "選ぶ", "v5b", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_b() {
    assert_golden("選ぼう", "選ぶ", "v5b", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_b() {
    assert_golden("選ぼ", "選ぶ", "v5b", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_b() {
    assert_golden("選びましょう", "選ぶ", "v5b", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_b() {
    assert_golden("選べば", "選ぶ", "v5b", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_b() {
    assert_golden("選ばなければ", "選ぶ", "v5b", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_b() {
    assert_golden("選んだら", "選ぶ", "v5b", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_b() {
    assert_golden("選んだらば", "選ぶ", "v5b", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_b() {
    assert_golden("選ばなかったら", "選ぶ", "v5b", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_b() {
    assert_golden("選ばせる", "選ぶ", "v5b", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_b() {
    assert_golden("選ばせない", "選ぶ", "v5b", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_b() {
    assert_golden("選ばせん", "選ぶ", "v5b", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_b() {
    assert_golden("選ばせます", "選ぶ", "v5b", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_b() {
    assert_golden("選ばします", "選ぶ", "v5b", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_b() {
    assert_golden("選ばせません", "選ぶ", "v5b", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_b() {
    assert_golden("選ばせた", "選ぶ", "v5b", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_b() {
    assert_golden("選ばせなかった", "選ぶ", "v5b", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_b() {
    assert_golden("選ばせました", "選ぶ", "v5b", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_b() {
    assert_golden("選ばせませんでした", "選ぶ", "v5b", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_b() {
    assert_golden("選ばせられる", "選ぶ", "v5b", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_b() {
    assert_golden("選ばせられない", "選ぶ", "v5b", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_b() {
    assert_golden("選ばせられます", "選ぶ", "v5b", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_b() {
    assert_golden("選ばせられません", "選ぶ", "v5b", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_b() {
    assert_golden("選びたい", "選ぶ", "v5b", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_b() {
    assert_golden("選びたくありません", "選ぶ", "v5b", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_b() {
    assert_golden("選びたくありませんでした", "選ぶ", "v5b", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_b() {
    assert_golden("選びたくない", "選ぶ", "v5b", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_b() {
    assert_golden("選びたかった", "選ぶ", "v5b", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_b() {
    assert_golden("選びたくなかった", "選ぶ", "v5b", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_b() {
    assert_golden("選んでいる", "選ぶ", "v5b", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_b() {
    assert_golden("選んでいない", "選ぶ", "v5b", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_b() {
    assert_golden("選んでいた", "選ぶ", "v5b", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_b() {
    assert_golden("選んでいなかった", "選ぶ", "v5b", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_b() {
    assert_golden("選んでいます", "選ぶ", "v5b", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_b() {
    assert_golden("選んでいません", "選ぶ", "v5b", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_b() {
    assert_golden("選んでいました", "選ぶ", "v5b", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_b() {
    assert_golden("選んでいませんでした", "選ぶ", "v5b", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_b() {
    assert_golden("選んでる", "選ぶ", "v5b", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_b() {
    assert_golden("選んでない", "選ぶ", "v5b", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_b() {
    assert_golden("選んでた", "選ぶ", "v5b", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_b() {
    assert_golden("選んでなかった", "選ぶ", "v5b", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_b() {
    assert_golden("選んでます", "選ぶ", "v5b", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_b() {
    assert_golden("選んでません", "選ぶ", "v5b", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_b() {
    assert_golden("選んでました", "選ぶ", "v5b", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_b() {
    assert_golden("選んでません", "選ぶ", "v5b", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_b() {
    assert_golden("選んでませんでした", "選ぶ", "v5b", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_b() {
    assert_golden("選んでしまう", "選ぶ", "v5b", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_b() {
    assert_golden("選んでもう", "選ぶ", "v5b", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_b() {
    assert_golden("選んでしまわない", "選ぶ", "v5b", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_b() {
    assert_golden("選んでしまった", "選ぶ", "v5b", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_b() {
    assert_golden("選んでしまわなかった", "選ぶ", "v5b", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_b() {
    assert_golden("選んでしまって", "選ぶ", "v5b", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_b() {
    assert_golden("選んでしまえば", "選ぶ", "v5b", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_b() {
    assert_golden("選んでしまわなければ", "選ぶ", "v5b", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_b() {
    assert_golden("選んでしまわなかったら", "選ぶ", "v5b", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_b() {
    assert_golden("選んでしまったら", "選ぶ", "v5b", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_b() {
    assert_golden("選んでしまおう", "選ぶ", "v5b", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_b() {
    assert_golden("選んでしまいます", "選ぶ", "v5b", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_b() {
    assert_golden("選んでしまいません", "選ぶ", "v5b", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_b() {
    assert_golden("選んでしまいました", "選ぶ", "v5b", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_b() {
    assert_golden("選んでしまいませんでした", "選ぶ", "v5b", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_b() {
    assert_golden("選んでしまえる", "選ぶ", "v5b", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_b() {
    assert_golden("選んでしまわれる", "選ぶ", "v5b", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_b() {
    assert_golden("選んでしまわせる", "選ぶ", "v5b", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_b() {
    assert_golden("選んじゃう", "選ぶ", "v5b", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_b() {
    assert_golden("選んじゃわない", "選ぶ", "v5b", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_b() {
    assert_golden("選んじゃった", "選ぶ", "v5b", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_b() {
    assert_golden("選んじゃわなかった", "選ぶ", "v5b", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_b() {
    assert_golden("選んじゃって", "選ぶ", "v5b", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_b() {
    assert_golden("選んじゃえば", "選ぶ", "v5b", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_b() {
    assert_golden("選んじゃわなければ", "選ぶ", "v5b", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_b() {
    assert_golden("選んじゃわなかったら", "選ぶ", "v5b", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_b() {
    assert_golden("選んじゃおう", "選ぶ", "v5b", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_b() {
    assert_golden("選んじゃえる", "選ぶ", "v5b", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_b() {
    assert_golden("選んでおく", "選ぶ", "v5b", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_b() {
    assert_golden("選んでおかない", "選ぶ", "v5b", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_b() {
    assert_golden("選んでおいた", "選ぶ", "v5b", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_b() {
    assert_golden("選んでおかなかった", "選ぶ", "v5b", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_b() {
    assert_golden("選んでおいて", "選ぶ", "v5b", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_b() {
    assert_golden("選んでおけば", "選ぶ", "v5b", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_b() {
    assert_golden("選んでおいたら", "選ぶ", "v5b", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_b() {
    assert_golden("選んでおこう", "選ぶ", "v5b", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_b() {
    assert_golden("選んでおける", "選ぶ", "v5b", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_b() {
    assert_golden("選んでおかれる", "選ぶ", "v5b", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_b() {
    assert_golden("選んどく", "選ぶ", "v5b", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_b() {
    assert_golden("選んどかない", "選ぶ", "v5b", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_b() {
    assert_golden("選んどいた", "選ぶ", "v5b", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_b() {
    assert_golden("選んどかなかった", "選ぶ", "v5b", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_b() {
    assert_golden("選んどいて", "選ぶ", "v5b", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_b() {
    assert_golden("選んどけば", "選ぶ", "v5b", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_b() {
    assert_golden("選んどいたら", "選ぶ", "v5b", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_b() {
    assert_golden("選んどこう", "選ぶ", "v5b", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_b() {
    assert_golden("選んどける", "選ぶ", "v5b", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_b() {
    assert_golden("選んどかれる", "選ぶ", "v5b", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_b() {
    assert_golden("選んである", "選ぶ", "v5b", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_b() {
    assert_golden("選んであった", "選ぶ", "v5b", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_b() {
    assert_golden("選んであって", "選ぶ", "v5b", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_b() {
    assert_golden("選んであったら", "選ぶ", "v5b", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_b() {
    assert_golden("選んであれば", "選ぶ", "v5b", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_b() {
    assert_golden("選んでいく", "選ぶ", "v5b", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_b() {
    assert_golden("選んでいかない", "選ぶ", "v5b", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_b() {
    assert_golden("選んでいった", "選ぶ", "v5b", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_b() {
    assert_golden("選んでいかなかった", "選ぶ", "v5b", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_b() {
    assert_golden("選んでいって", "選ぶ", "v5b", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_b() {
    assert_golden("選んでいこう", "選ぶ", "v5b", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_b() {
    assert_golden("選んでいける", "選ぶ", "v5b", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_b() {
    assert_golden("選んでいかれる", "選ぶ", "v5b", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_b() {
    assert_golden("選んでいかせる", "選ぶ", "v5b", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_b() {
    assert_golden("選んでくる", "選ぶ", "v5b", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_b() {
    assert_golden("選んでこない", "選ぶ", "v5b", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_b() {
    assert_golden("選んできた", "選ぶ", "v5b", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_b() {
    assert_golden("選んでこなかった", "選ぶ", "v5b", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_b() {
    assert_golden("選んできて", "選ぶ", "v5b", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_b() {
    assert_golden("選んでくれば", "選ぶ", "v5b", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_b() {
    assert_golden("選んできたら", "選ぶ", "v5b", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_b() {
    assert_golden("選んでこられる", "選ぶ", "v5b", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_b() {
    assert_golden("選んでこさせる", "選ぶ", "v5b", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_b() {
    assert_golden("選びながら", "選ぶ", "v5b", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_b() {
    assert_golden("選びすぎる", "選ぶ", "v5b", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_b() {
    assert_golden("選びそう", "選ぶ", "v5b", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_b() {
    assert_golden("選ばぬ", "選ぶ", "v5b", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_b() {
    assert_golden("選ばず", "選ぶ", "v5b", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_b() {
    assert_golden("選ばずに", "選ぶ", "v5b", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_b() {
    assert_golden("選んだり", "選ぶ", "v5b", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_b() {
    assert_golden("選ばなかったり", "選ぶ", "v5b", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_b() {
    assert_golden("選ばん", "選ぶ", "v5b", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_b() {
    assert_golden("選ばんかった", "選ぶ", "v5b", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_b() {
    assert_golden("選ばざる", "選ぶ", "v5b", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_b() {
    assert_golden("選べよう", "選ぶ", "v5b", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_b() {
    assert_golden("選べよ", "選ぶ", "v5b", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_b() {
    assert_golden("選べろ", "選ぶ", "v5b", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_b() {
    assert_golden("選べて", "選ぶ", "v5b", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_b() {
    assert_golden("選べたら", "選ぶ", "v5b", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_b() {
    assert_golden("選べれば", "選ぶ", "v5b", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_b() {
    assert_golden("選べられる", "選ぶ", "v5b", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_b() {
    assert_golden("選べさせる", "選ぶ", "v5b", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_b() {
    assert_golden("選んであげる", "選ぶ", "v5b", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_b() {
    assert_golden("選んであげられる", "選ぶ", "v5b", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_b() {
    assert_golden("選んでおる", "選ぶ", "v5b", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_b() {
    assert_golden("選んでおらない", "選ぶ", "v5b", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_b() {
    assert_golden("選んでおらん", "選ぶ", "v5b", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_b() {
    assert_golden("選んでおった", "選ぶ", "v5b", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_b() {
    assert_golden("選んでおらなかった", "選ぶ", "v5b", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_b() {
    assert_golden("選んでおります", "選ぶ", "v5b", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_b() {
    assert_golden("選んでおりません", "選ぶ", "v5b", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_b() {
    assert_golden("選んでおりました", "選ぶ", "v5b", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_b() {
    assert_golden("選んでおりませんでした", "選ぶ", "v5b", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_b() {
    assert_golden("選んでおって", "選ぶ", "v5b", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_b() {
    assert_golden("選んでおろう", "選ぶ", "v5b", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_b() {
    assert_golden("選んでおれる", "選ぶ", "v5b", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_b() {
    assert_golden("選んでおられる", "選ぶ", "v5b", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_b() {
    assert_golden("選んどる", "選ぶ", "v5b", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_b() {
    assert_golden("選んどらない", "選ぶ", "v5b", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_b() {
    assert_golden("選んどらん", "選ぶ", "v5b", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_b() {
    assert_golden("選んどった", "選ぶ", "v5b", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_b() {
    assert_golden("選んどらなかった", "選ぶ", "v5b", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_b() {
    assert_golden("選んどります", "選ぶ", "v5b", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_b() {
    assert_golden("選んどりません", "選ぶ", "v5b", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_b() {
    assert_golden("選んどりました", "選ぶ", "v5b", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_b() {
    assert_golden("選んどりませんでした", "選ぶ", "v5b", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_b() {
    assert_golden("選んどって", "選ぶ", "v5b", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_b() {
    assert_golden("選んどろう", "選ぶ", "v5b", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_b() {
    assert_golden("選んどれる", "選ぶ", "v5b", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_b() {
    assert_golden("選んどられる", "選ぶ", "v5b", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_b() {
    assert_golden("選ばす", "選ぶ", "v5b", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_b() {
    assert_golden("選んでは", "選ぶ", "v5b", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_b() {
    assert_golden("選んじゃ", "選ぶ", "v5b", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_b() {
    assert_golden("選ばなきゃ", "選ぶ", "v5b", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_b() {
    assert_golden("選んじまう", "選ぶ", "v5b", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_b() {
    assert_golden("選んじゃう", "選ぶ", "v5b", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_b() {
    assert_golden("選んでいらっしゃる", "選ぶ", "v5b", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_b() {
    assert_golden("選んでいらっしゃらない", "選ぶ", "v5b", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_b() {
    assert_golden("選びつつ", "選ぶ", "v5b", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_b() {
    assert_golden("選んでくれる", "選ぶ", "v5b", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_b() {
    assert_golden("選んでくれない", "選ぶ", "v5b", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_b() {
    assert_golden("選んでくれます", "選ぶ", "v5b", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_b() {
    assert_golden("選んでくれません", "選ぶ", "v5b", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_b() {
    assert_golden("選んでくれ", "選ぶ", "v5b", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_b() {
    assert_golden("選ばへん", "選ぶ", "v5b", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_b() {
    assert_golden("選ばへんかった", "選ぶ", "v5b", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_b() {
    assert_golden("選ばひん", "選ぶ", "v5b", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_b() {
    assert_golden("選ばひんかった", "選ぶ", "v5b", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_b() {
    assert_golden("選ばさない", "選ぶ", "v5b", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_b() {
    assert_golden("選びましたら", "選ぶ", "v5b", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_b() {
    assert_golden("選びになる", "選ぶ", "v5b", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_b() {
    assert_golden("選びなさる", "選ぶ", "v5b", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_b() {
    assert_golden("選びはる", "選ぶ", "v5b", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_b() {
    assert_golden("選びなさるな", "選ぶ", "v5b", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_b() {
    assert_golden("選ぶまい", "選ぶ", "v5b", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_b() {
    assert_golden("選びますまい", "選ぶ", "v5b", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_b() {
    assert_golden("選ばば", "選ぶ", "v5b", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_b() {
    assert_golden("選ばねば", "選ぶ", "v5b", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_b() {
    assert_golden("選ばにゃ", "選ぶ", "v5b", "～colloquial negative conditional");
}
