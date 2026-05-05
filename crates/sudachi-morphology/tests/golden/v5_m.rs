//! Golden tests ported from JL's `DeconjugatorTestsForV5M.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V5M.

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
fn deconjugate_masu_stem_v5_m() {
    assert_golden("読み", "読む", "v5m", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_m() {
    assert_golden("読まない", "読む", "v5m", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_m() {
    assert_golden("読みます", "読む", "v5m", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_m() {
    assert_golden("読みましょう", "読む", "v5m", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_m() {
    assert_golden("読みません", "読む", "v5m", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_m() {
    assert_golden("読んだ", "読む", "v5m", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_m() {
    assert_golden("読まなかった", "読む", "v5m", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_m() {
    assert_golden("読みました", "読む", "v5m", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_m() {
    assert_golden("読みませんでした", "読む", "v5m", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_m() {
    assert_golden("読んで", "読む", "v5m", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_m() {
    assert_golden("読まなくて", "読む", "v5m", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_m() {
    assert_golden("読まないで", "読む", "v5m", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_m() {
    assert_golden("読みまして", "読む", "v5m", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_m() {
    assert_golden("読める", "読む", "v5m", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_m() {
    assert_golden("読まれる", "読む", "v5m", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_m() {
    assert_golden("読めない", "読む", "v5m", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_m() {
    assert_golden("読まれない", "読む", "v5m", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_m() {
    assert_golden("読めた", "読む", "v5m", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_m() {
    assert_golden("読まれた", "読む", "v5m", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_m() {
    assert_golden("読めました", "読む", "v5m", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_m() {
    assert_golden("読まれました", "読む", "v5m", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_m() {
    assert_golden("読めなかった", "読む", "v5m", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_m() {
    assert_golden("読まれなかった", "読む", "v5m", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_m() {
    assert_golden("読めませんでした", "読む", "v5m", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_m() {
    assert_golden("読まれませんでした", "読む", "v5m", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_m() {
    assert_golden("読めます", "読む", "v5m", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_m() {
    assert_golden("読まれます", "読む", "v5m", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_m() {
    assert_golden("読めません", "読む", "v5m", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_m() {
    assert_golden("読まれません", "読む", "v5m", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_m() {
    assert_golden("読め", "読む", "v5m", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_m() {
    assert_golden("読むな", "読む", "v5m", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_m() {
    assert_golden("読みなさい", "読む", "v5m", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_m() {
    assert_golden("読んでください", "読む", "v5m", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_m() {
    assert_golden("読まないでください", "読む", "v5m", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_m() {
    assert_golden("読もう", "読む", "v5m", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_m() {
    assert_golden("読も", "読む", "v5m", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_m() {
    assert_golden("読みましょう", "読む", "v5m", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_m() {
    assert_golden("読めば", "読む", "v5m", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_m() {
    assert_golden("読まなければ", "読む", "v5m", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_m() {
    assert_golden("読んだら", "読む", "v5m", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_m() {
    assert_golden("読んだらば", "読む", "v5m", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_m() {
    assert_golden("読まなかったら", "読む", "v5m", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_m() {
    assert_golden("読ませる", "読む", "v5m", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_m() {
    assert_golden("読ませない", "読む", "v5m", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_m() {
    assert_golden("読ません", "読む", "v5m", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_m() {
    assert_golden("読ませます", "読む", "v5m", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_m() {
    assert_golden("読まします", "読む", "v5m", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_m() {
    assert_golden("読ませません", "読む", "v5m", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_m() {
    assert_golden("読ませた", "読む", "v5m", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_m() {
    assert_golden("読ませなかった", "読む", "v5m", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_m() {
    assert_golden("読ませました", "読む", "v5m", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_m() {
    assert_golden("読ませませんでした", "読む", "v5m", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_m() {
    assert_golden("読ませられる", "読む", "v5m", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_m() {
    assert_golden("読ませられない", "読む", "v5m", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_m() {
    assert_golden("読ませられます", "読む", "v5m", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_m() {
    assert_golden("読ませられません", "読む", "v5m", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_m() {
    assert_golden("読みたい", "読む", "v5m", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_m() {
    assert_golden("読みたくありません", "読む", "v5m", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_m() {
    assert_golden("読みたくありませんでした", "読む", "v5m", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_m() {
    assert_golden("読みたくない", "読む", "v5m", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_m() {
    assert_golden("読みたかった", "読む", "v5m", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_m() {
    assert_golden("読みたくなかった", "読む", "v5m", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_m() {
    assert_golden("読んでいる", "読む", "v5m", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_m() {
    assert_golden("読んでいない", "読む", "v5m", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_m() {
    assert_golden("読んでいた", "読む", "v5m", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_m() {
    assert_golden("読んでいなかった", "読む", "v5m", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_m() {
    assert_golden("読んでいます", "読む", "v5m", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_m() {
    assert_golden("読んでいません", "読む", "v5m", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_m() {
    assert_golden("読んでいました", "読む", "v5m", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_m() {
    assert_golden("読んでいませんでした", "読む", "v5m", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_m() {
    assert_golden("読んでる", "読む", "v5m", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_m() {
    assert_golden("読んでない", "読む", "v5m", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_m() {
    assert_golden("読んでた", "読む", "v5m", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_m() {
    assert_golden("読んでなかった", "読む", "v5m", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_m() {
    assert_golden("読んでます", "読む", "v5m", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_m() {
    assert_golden("読んでません", "読む", "v5m", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_m() {
    assert_golden("読んでました", "読む", "v5m", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_m() {
    assert_golden("読んでません", "読む", "v5m", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_m() {
    assert_golden("読んでませんでした", "読む", "v5m", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_m() {
    assert_golden("読んでしまう", "読む", "v5m", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_m() {
    assert_golden("読んでもう", "読む", "v5m", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_m() {
    assert_golden("読んでしまわない", "読む", "v5m", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_m() {
    assert_golden("読んでしまった", "読む", "v5m", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_m() {
    assert_golden("読んでしまわなかった", "読む", "v5m", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_m() {
    assert_golden("読んでしまって", "読む", "v5m", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_m() {
    assert_golden("読んでしまえば", "読む", "v5m", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_m() {
    assert_golden("読んでしまわなければ", "読む", "v5m", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_m() {
    assert_golden("読んでしまわなかったら", "読む", "v5m", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_m() {
    assert_golden("読んでしまったら", "読む", "v5m", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_m() {
    assert_golden("読んでしまおう", "読む", "v5m", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_m() {
    assert_golden("読んでしまいます", "読む", "v5m", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_m() {
    assert_golden("読んでしまいません", "読む", "v5m", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_m() {
    assert_golden("読んでしまいました", "読む", "v5m", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_m() {
    assert_golden("読んでしまいませんでした", "読む", "v5m", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_m() {
    assert_golden("読んでしまえる", "読む", "v5m", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_m() {
    assert_golden("読んでしまわれる", "読む", "v5m", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_m() {
    assert_golden("読んでしまわせる", "読む", "v5m", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_m() {
    assert_golden("読んじゃう", "読む", "v5m", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_m() {
    assert_golden("読んじゃわない", "読む", "v5m", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_m() {
    assert_golden("読んじゃった", "読む", "v5m", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_m() {
    assert_golden("読んじゃわなかった", "読む", "v5m", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_m() {
    assert_golden("読んじゃって", "読む", "v5m", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_m() {
    assert_golden("読んじゃえば", "読む", "v5m", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_m() {
    assert_golden("読んじゃわなければ", "読む", "v5m", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_m() {
    assert_golden("読んじゃわなかったら", "読む", "v5m", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_m() {
    assert_golden("読んじゃおう", "読む", "v5m", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_m() {
    assert_golden("読んじゃえる", "読む", "v5m", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_m() {
    assert_golden("読んでおく", "読む", "v5m", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_m() {
    assert_golden("読んでおかない", "読む", "v5m", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_m() {
    assert_golden("読んでおいた", "読む", "v5m", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_m() {
    assert_golden("読んでおかなかった", "読む", "v5m", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_m() {
    assert_golden("読んでおいて", "読む", "v5m", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_m() {
    assert_golden("読んでおけば", "読む", "v5m", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_m() {
    assert_golden("読んでおいたら", "読む", "v5m", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_m() {
    assert_golden("読んでおこう", "読む", "v5m", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_m() {
    assert_golden("読んでおける", "読む", "v5m", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_m() {
    assert_golden("読んでおかれる", "読む", "v5m", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_m() {
    assert_golden("読んどく", "読む", "v5m", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_m() {
    assert_golden("読んどかない", "読む", "v5m", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_m() {
    assert_golden("読んどいた", "読む", "v5m", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_m() {
    assert_golden("読んどかなかった", "読む", "v5m", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_m() {
    assert_golden("読んどいて", "読む", "v5m", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_m() {
    assert_golden("読んどけば", "読む", "v5m", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_m() {
    assert_golden("読んどいたら", "読む", "v5m", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_m() {
    assert_golden("読んどこう", "読む", "v5m", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_m() {
    assert_golden("読んどける", "読む", "v5m", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_m() {
    assert_golden("読んどかれる", "読む", "v5m", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_m() {
    assert_golden("読んである", "読む", "v5m", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_m() {
    assert_golden("読んであった", "読む", "v5m", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_m() {
    assert_golden("読んであって", "読む", "v5m", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_m() {
    assert_golden("読んであったら", "読む", "v5m", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_m() {
    assert_golden("読んであれば", "読む", "v5m", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_m() {
    assert_golden("読んでいく", "読む", "v5m", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_m() {
    assert_golden("読んでいかない", "読む", "v5m", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_m() {
    assert_golden("読んでいった", "読む", "v5m", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_m() {
    assert_golden("読んでいかなかった", "読む", "v5m", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_m() {
    assert_golden("読んでいって", "読む", "v5m", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_m() {
    assert_golden("読んでいこう", "読む", "v5m", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_m() {
    assert_golden("読んでいける", "読む", "v5m", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_m() {
    assert_golden("読んでいかれる", "読む", "v5m", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_m() {
    assert_golden("読んでいかせる", "読む", "v5m", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_m() {
    assert_golden("読んでくる", "読む", "v5m", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_m() {
    assert_golden("読んでこない", "読む", "v5m", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_m() {
    assert_golden("読んできた", "読む", "v5m", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_m() {
    assert_golden("読んでこなかった", "読む", "v5m", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_m() {
    assert_golden("読んできて", "読む", "v5m", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_m() {
    assert_golden("読んでくれば", "読む", "v5m", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_m() {
    assert_golden("読んできたら", "読む", "v5m", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_m() {
    assert_golden("読んでこられる", "読む", "v5m", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_m() {
    assert_golden("読んでこさせる", "読む", "v5m", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_m() {
    assert_golden("読みながら", "読む", "v5m", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_m() {
    assert_golden("読みすぎる", "読む", "v5m", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_m() {
    assert_golden("読みそう", "読む", "v5m", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_m() {
    assert_golden("読まぬ", "読む", "v5m", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_m() {
    assert_golden("読まず", "読む", "v5m", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_m() {
    assert_golden("読まずに", "読む", "v5m", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_m() {
    assert_golden("読んだり", "読む", "v5m", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_m() {
    assert_golden("読まなかったり", "読む", "v5m", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_m() {
    assert_golden("読まん", "読む", "v5m", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_m() {
    assert_golden("読まんかった", "読む", "v5m", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_m() {
    assert_golden("読まざる", "読む", "v5m", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_m() {
    assert_golden("読めよう", "読む", "v5m", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_m() {
    assert_golden("読めよ", "読む", "v5m", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_m() {
    assert_golden("読めろ", "読む", "v5m", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_m() {
    assert_golden("読めて", "読む", "v5m", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_m() {
    assert_golden("読めたら", "読む", "v5m", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_m() {
    assert_golden("読めれば", "読む", "v5m", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_m() {
    assert_golden("読められる", "読む", "v5m", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_m() {
    assert_golden("読めさせる", "読む", "v5m", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_m() {
    assert_golden("読んであげる", "読む", "v5m", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_m() {
    assert_golden("読んであげられる", "読む", "v5m", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_m() {
    assert_golden("読んでおる", "読む", "v5m", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_m() {
    assert_golden("読んでおらない", "読む", "v5m", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_m() {
    assert_golden("読んでおらん", "読む", "v5m", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_m() {
    assert_golden("読んでおった", "読む", "v5m", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_m() {
    assert_golden("読んでおらなかった", "読む", "v5m", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_m() {
    assert_golden("読んでおります", "読む", "v5m", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_m() {
    assert_golden("読んでおりません", "読む", "v5m", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_m() {
    assert_golden("読んでおりました", "読む", "v5m", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_m() {
    assert_golden("読んでおりませんでした", "読む", "v5m", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_m() {
    assert_golden("読んでおって", "読む", "v5m", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_m() {
    assert_golden("読んでおろう", "読む", "v5m", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_m() {
    assert_golden("読んでおれる", "読む", "v5m", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_m() {
    assert_golden("読んでおられる", "読む", "v5m", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_m() {
    assert_golden("読んどる", "読む", "v5m", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_m() {
    assert_golden("読んどらない", "読む", "v5m", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_m() {
    assert_golden("読んどらん", "読む", "v5m", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_m() {
    assert_golden("読んどった", "読む", "v5m", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_m() {
    assert_golden("読んどらなかった", "読む", "v5m", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_m() {
    assert_golden("読んどります", "読む", "v5m", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_m() {
    assert_golden("読んどりません", "読む", "v5m", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_m() {
    assert_golden("読んどりました", "読む", "v5m", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_m() {
    assert_golden("読んどりませんでした", "読む", "v5m", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_m() {
    assert_golden("読んどって", "読む", "v5m", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_m() {
    assert_golden("読んどろう", "読む", "v5m", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_m() {
    assert_golden("読んどれる", "読む", "v5m", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_m() {
    assert_golden("読んどられる", "読む", "v5m", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_m() {
    assert_golden("読ます", "読む", "v5m", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_m() {
    assert_golden("読んでは", "読む", "v5m", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_m() {
    assert_golden("読んじゃ", "読む", "v5m", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_m() {
    assert_golden("読まなきゃ", "読む", "v5m", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_m() {
    assert_golden("読んじまう", "読む", "v5m", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_m() {
    assert_golden("読んじゃう", "読む", "v5m", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_m() {
    assert_golden("読んでいらっしゃる", "読む", "v5m", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_m() {
    assert_golden("読んでいらっしゃらない", "読む", "v5m", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_m() {
    assert_golden("読みつつ", "読む", "v5m", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_m() {
    assert_golden("読んでくれる", "読む", "v5m", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_m() {
    assert_golden("読んでくれない", "読む", "v5m", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_m() {
    assert_golden("読んでくれます", "読む", "v5m", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_m() {
    assert_golden("読んでくれません", "読む", "v5m", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_m() {
    assert_golden("読んでくれ", "読む", "v5m", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_m() {
    assert_golden("読まへん", "読む", "v5m", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_m() {
    assert_golden("読まへんかった", "読む", "v5m", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_m() {
    assert_golden("読まひん", "読む", "v5m", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_m() {
    assert_golden("読まひんかった", "読む", "v5m", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_m() {
    assert_golden("読まさない", "読む", "v5m", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_m() {
    assert_golden("読みましたら", "読む", "v5m", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_m() {
    assert_golden("読みになる", "読む", "v5m", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_m() {
    assert_golden("読みなさる", "読む", "v5m", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_m() {
    assert_golden("読みはる", "読む", "v5m", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_m() {
    assert_golden("読みなさるな", "読む", "v5m", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_m() {
    assert_golden("読むまい", "読む", "v5m", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_m() {
    assert_golden("読みますまい", "読む", "v5m", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_m() {
    assert_golden("読まば", "読む", "v5m", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_m() {
    assert_golden("読まねば", "読む", "v5m", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_m() {
    assert_golden("読まにゃ", "読む", "v5m", "～colloquial negative conditional");
}

// ─── Verb-producing aux on renyou base (added 2026-05-06) ────────────

#[test]
fn deconjugate_aux_hajimeru_past_v5_m() {
    assert_golden("読み始めた", "読む", "v5m", "～start V-ing→past");
}

#[test]
fn deconjugate_aux_tsuzukeru_teiru_v5_m() {
    assert_golden("読み続けている", "読む", "v5m", "～continue V-ing→teiru");
}

#[test]
fn deconjugate_aux_te_morau_past_v5_m() {
    assert_golden("読んでもらった", "読む", "v5m", "～have someone do→past");
}
