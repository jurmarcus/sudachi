//! Golden tests ported from JL's `DeconjugatorTestsForV5N.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V5N.

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
fn deconjugate_masu_stem_v5_n() {
    assert_golden("死に", "死ぬ", "v5n", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_n() {
    assert_golden("死なない", "死ぬ", "v5n", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_n() {
    assert_golden("死にます", "死ぬ", "v5n", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_n() {
    assert_golden("死にましょう", "死ぬ", "v5n", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_n() {
    assert_golden("死にません", "死ぬ", "v5n", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_n() {
    assert_golden("死んだ", "死ぬ", "v5n", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_n() {
    assert_golden("死ななかった", "死ぬ", "v5n", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_n() {
    assert_golden("死にました", "死ぬ", "v5n", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_n() {
    assert_golden("死にませんでした", "死ぬ", "v5n", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_n() {
    assert_golden("死んで", "死ぬ", "v5n", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_n() {
    assert_golden("死ななくて", "死ぬ", "v5n", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_n() {
    assert_golden("死なないで", "死ぬ", "v5n", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_n() {
    assert_golden("死にまして", "死ぬ", "v5n", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_n() {
    assert_golden("死ねる", "死ぬ", "v5n", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_n() {
    assert_golden("死なれる", "死ぬ", "v5n", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_n() {
    assert_golden("死ねない", "死ぬ", "v5n", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_n() {
    assert_golden("死なれない", "死ぬ", "v5n", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_n() {
    assert_golden("死ねた", "死ぬ", "v5n", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_n() {
    assert_golden("死なれた", "死ぬ", "v5n", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_n() {
    assert_golden("死ねました", "死ぬ", "v5n", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_n() {
    assert_golden("死なれました", "死ぬ", "v5n", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_n() {
    assert_golden("死ねなかった", "死ぬ", "v5n", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_n() {
    assert_golden("死なれなかった", "死ぬ", "v5n", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_n() {
    assert_golden("死ねませんでした", "死ぬ", "v5n", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_n() {
    assert_golden("死なれませんでした", "死ぬ", "v5n", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_n() {
    assert_golden("死ねます", "死ぬ", "v5n", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_n() {
    assert_golden("死なれます", "死ぬ", "v5n", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_n() {
    assert_golden("死ねません", "死ぬ", "v5n", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_n() {
    assert_golden("死なれません", "死ぬ", "v5n", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_n() {
    assert_golden("死ね", "死ぬ", "v5n", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_n() {
    assert_golden("死ぬな", "死ぬ", "v5n", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_n() {
    assert_golden("死になさい", "死ぬ", "v5n", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_n() {
    assert_golden("死んでください", "死ぬ", "v5n", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_n() {
    assert_golden("死なないでください", "死ぬ", "v5n", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_n() {
    assert_golden("死のう", "死ぬ", "v5n", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_n() {
    assert_golden("死の", "死ぬ", "v5n", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_n() {
    assert_golden("死にましょう", "死ぬ", "v5n", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_n() {
    assert_golden("死ねば", "死ぬ", "v5n", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_n() {
    assert_golden("死ななければ", "死ぬ", "v5n", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_n() {
    assert_golden("死んだら", "死ぬ", "v5n", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_n() {
    assert_golden("死んだらば", "死ぬ", "v5n", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_n() {
    assert_golden("死ななかったら", "死ぬ", "v5n", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_n() {
    assert_golden("死なせる", "死ぬ", "v5n", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_n() {
    assert_golden("死なせない", "死ぬ", "v5n", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_n() {
    assert_golden("死なせん", "死ぬ", "v5n", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_n() {
    assert_golden("死なせます", "死ぬ", "v5n", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_n() {
    assert_golden("死なします", "死ぬ", "v5n", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_n() {
    assert_golden("死なせません", "死ぬ", "v5n", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_n() {
    assert_golden("死なせた", "死ぬ", "v5n", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_n() {
    assert_golden("死なせなかった", "死ぬ", "v5n", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_n() {
    assert_golden("死なせました", "死ぬ", "v5n", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_n() {
    assert_golden("死なせませんでした", "死ぬ", "v5n", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_n() {
    assert_golden("死なせられる", "死ぬ", "v5n", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_n() {
    assert_golden("死なせられない", "死ぬ", "v5n", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_n() {
    assert_golden("死なせられます", "死ぬ", "v5n", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_n() {
    assert_golden("死なせられません", "死ぬ", "v5n", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_n() {
    assert_golden("死にたい", "死ぬ", "v5n", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_n() {
    assert_golden("死にたくありません", "死ぬ", "v5n", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_n() {
    assert_golden("死にたくありませんでした", "死ぬ", "v5n", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_n() {
    assert_golden("死にたくない", "死ぬ", "v5n", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_n() {
    assert_golden("死にたかった", "死ぬ", "v5n", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_n() {
    assert_golden("死にたくなかった", "死ぬ", "v5n", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_n() {
    assert_golden("死んでいる", "死ぬ", "v5n", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_n() {
    assert_golden("死んでいない", "死ぬ", "v5n", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_n() {
    assert_golden("死んでいた", "死ぬ", "v5n", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_n() {
    assert_golden("死んでいなかった", "死ぬ", "v5n", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_n() {
    assert_golden("死んでいます", "死ぬ", "v5n", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_n() {
    assert_golden("死んでいません", "死ぬ", "v5n", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_n() {
    assert_golden("死んでいました", "死ぬ", "v5n", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_n() {
    assert_golden("死んでいませんでした", "死ぬ", "v5n", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_n() {
    assert_golden("死んでる", "死ぬ", "v5n", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_n() {
    assert_golden("死んでない", "死ぬ", "v5n", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_n() {
    assert_golden("死んでた", "死ぬ", "v5n", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_n() {
    assert_golden("死んでなかった", "死ぬ", "v5n", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_n() {
    assert_golden("死んでます", "死ぬ", "v5n", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_n() {
    assert_golden("死んでません", "死ぬ", "v5n", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_n() {
    assert_golden("死んでました", "死ぬ", "v5n", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_n() {
    assert_golden("死んでません", "死ぬ", "v5n", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_n() {
    assert_golden("死んでませんでした", "死ぬ", "v5n", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_n() {
    assert_golden("死んでしまう", "死ぬ", "v5n", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_n() {
    assert_golden("死んでもう", "死ぬ", "v5n", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_n() {
    assert_golden("死んでしまわない", "死ぬ", "v5n", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_n() {
    assert_golden("死んでしまった", "死ぬ", "v5n", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_n() {
    assert_golden("死んでしまわなかった", "死ぬ", "v5n", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_n() {
    assert_golden("死んでしまって", "死ぬ", "v5n", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_n() {
    assert_golden("死んでしまえば", "死ぬ", "v5n", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_n() {
    assert_golden("死んでしまわなければ", "死ぬ", "v5n", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_n() {
    assert_golden("死んでしまわなかったら", "死ぬ", "v5n", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_n() {
    assert_golden("死んでしまったら", "死ぬ", "v5n", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_n() {
    assert_golden("死んでしまおう", "死ぬ", "v5n", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_n() {
    assert_golden("死んでしまいます", "死ぬ", "v5n", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_n() {
    assert_golden("死んでしまいません", "死ぬ", "v5n", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_n() {
    assert_golden("死んでしまいました", "死ぬ", "v5n", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_n() {
    assert_golden("死んでしまいませんでした", "死ぬ", "v5n", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_n() {
    assert_golden("死んでしまえる", "死ぬ", "v5n", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_n() {
    assert_golden("死んでしまわれる", "死ぬ", "v5n", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_n() {
    assert_golden("死んでしまわせる", "死ぬ", "v5n", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_n() {
    assert_golden("死んじゃう", "死ぬ", "v5n", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_n() {
    assert_golden("死んじゃわない", "死ぬ", "v5n", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_n() {
    assert_golden("死んじゃった", "死ぬ", "v5n", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_n() {
    assert_golden("死んじゃわなかった", "死ぬ", "v5n", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_n() {
    assert_golden("死んじゃって", "死ぬ", "v5n", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_n() {
    assert_golden("死んじゃえば", "死ぬ", "v5n", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_n() {
    assert_golden("死んじゃわなければ", "死ぬ", "v5n", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_n() {
    assert_golden("死んじゃわなかったら", "死ぬ", "v5n", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_n() {
    assert_golden("死んじゃおう", "死ぬ", "v5n", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_n() {
    assert_golden("死んじゃえる", "死ぬ", "v5n", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_n() {
    assert_golden("死んでおく", "死ぬ", "v5n", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_n() {
    assert_golden("死んでおかない", "死ぬ", "v5n", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_n() {
    assert_golden("死んでおいた", "死ぬ", "v5n", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_n() {
    assert_golden("死んでおかなかった", "死ぬ", "v5n", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_n() {
    assert_golden("死んでおいて", "死ぬ", "v5n", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_n() {
    assert_golden("死んでおけば", "死ぬ", "v5n", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_n() {
    assert_golden("死んでおいたら", "死ぬ", "v5n", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_n() {
    assert_golden("死んでおこう", "死ぬ", "v5n", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_n() {
    assert_golden("死んでおける", "死ぬ", "v5n", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_n() {
    assert_golden("死んでおかれる", "死ぬ", "v5n", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_n() {
    assert_golden("死んどく", "死ぬ", "v5n", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_n() {
    assert_golden("死んどかない", "死ぬ", "v5n", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_n() {
    assert_golden("死んどいた", "死ぬ", "v5n", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_n() {
    assert_golden("死んどかなかった", "死ぬ", "v5n", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_n() {
    assert_golden("死んどいて", "死ぬ", "v5n", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_n() {
    assert_golden("死んどけば", "死ぬ", "v5n", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_n() {
    assert_golden("死んどいたら", "死ぬ", "v5n", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_n() {
    assert_golden("死んどこう", "死ぬ", "v5n", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_n() {
    assert_golden("死んどける", "死ぬ", "v5n", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_n() {
    assert_golden("死んどかれる", "死ぬ", "v5n", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_n() {
    assert_golden("死んである", "死ぬ", "v5n", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_n() {
    assert_golden("死んであった", "死ぬ", "v5n", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_n() {
    assert_golden("死んであって", "死ぬ", "v5n", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_n() {
    assert_golden("死んであったら", "死ぬ", "v5n", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_n() {
    assert_golden("死んであれば", "死ぬ", "v5n", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_n() {
    assert_golden("死んでいく", "死ぬ", "v5n", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_n() {
    assert_golden("死んでいかない", "死ぬ", "v5n", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_n() {
    assert_golden("死んでいった", "死ぬ", "v5n", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_n() {
    assert_golden("死んでいかなかった", "死ぬ", "v5n", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_n() {
    assert_golden("死んでいって", "死ぬ", "v5n", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_n() {
    assert_golden("死んでいこう", "死ぬ", "v5n", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_n() {
    assert_golden("死んでいける", "死ぬ", "v5n", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_n() {
    assert_golden("死んでいかれる", "死ぬ", "v5n", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_n() {
    assert_golden("死んでいかせる", "死ぬ", "v5n", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_n() {
    assert_golden("死んでくる", "死ぬ", "v5n", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_n() {
    assert_golden("死んでこない", "死ぬ", "v5n", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_n() {
    assert_golden("死んできた", "死ぬ", "v5n", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_n() {
    assert_golden("死んでこなかった", "死ぬ", "v5n", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_n() {
    assert_golden("死んできて", "死ぬ", "v5n", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_n() {
    assert_golden("死んでくれば", "死ぬ", "v5n", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_n() {
    assert_golden("死んできたら", "死ぬ", "v5n", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_n() {
    assert_golden("死んでこられる", "死ぬ", "v5n", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_n() {
    assert_golden("死んでこさせる", "死ぬ", "v5n", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_n() {
    assert_golden("死にながら", "死ぬ", "v5n", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_n() {
    assert_golden("死にすぎる", "死ぬ", "v5n", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_n() {
    assert_golden("死にそう", "死ぬ", "v5n", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_n() {
    assert_golden("死なぬ", "死ぬ", "v5n", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_n() {
    assert_golden("死なず", "死ぬ", "v5n", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_n() {
    assert_golden("死なずに", "死ぬ", "v5n", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_n() {
    assert_golden("死んだり", "死ぬ", "v5n", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_n() {
    assert_golden("死ななかったり", "死ぬ", "v5n", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_n() {
    assert_golden("死なん", "死ぬ", "v5n", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_n() {
    assert_golden("死なんかった", "死ぬ", "v5n", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_n() {
    assert_golden("死なざる", "死ぬ", "v5n", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_n() {
    assert_golden("死ねよう", "死ぬ", "v5n", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_n() {
    assert_golden("死ねよ", "死ぬ", "v5n", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_n() {
    assert_golden("死ねろ", "死ぬ", "v5n", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_n() {
    assert_golden("死ねて", "死ぬ", "v5n", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_n() {
    assert_golden("死ねたら", "死ぬ", "v5n", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_n() {
    assert_golden("死ねれば", "死ぬ", "v5n", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_n() {
    assert_golden("死ねられる", "死ぬ", "v5n", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_n() {
    assert_golden("死ねさせる", "死ぬ", "v5n", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_n() {
    assert_golden("死んであげる", "死ぬ", "v5n", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_n() {
    assert_golden("死んであげられる", "死ぬ", "v5n", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_n() {
    assert_golden("死んでおる", "死ぬ", "v5n", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_n() {
    assert_golden("死んでおらない", "死ぬ", "v5n", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_n() {
    assert_golden("死んでおらん", "死ぬ", "v5n", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_n() {
    assert_golden("死んでおった", "死ぬ", "v5n", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_n() {
    assert_golden("死んでおらなかった", "死ぬ", "v5n", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_n() {
    assert_golden("死んでおります", "死ぬ", "v5n", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_n() {
    assert_golden("死んでおりません", "死ぬ", "v5n", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_n() {
    assert_golden("死んでおりました", "死ぬ", "v5n", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_n() {
    assert_golden("死んでおりませんでした", "死ぬ", "v5n", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_n() {
    assert_golden("死んでおって", "死ぬ", "v5n", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_n() {
    assert_golden("死んでおろう", "死ぬ", "v5n", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_n() {
    assert_golden("死んでおれる", "死ぬ", "v5n", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_n() {
    assert_golden("死んでおられる", "死ぬ", "v5n", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_n() {
    assert_golden("死んどる", "死ぬ", "v5n", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_n() {
    assert_golden("死んどらない", "死ぬ", "v5n", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_n() {
    assert_golden("死んどらん", "死ぬ", "v5n", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_n() {
    assert_golden("死んどった", "死ぬ", "v5n", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_n() {
    assert_golden("死んどらなかった", "死ぬ", "v5n", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_n() {
    assert_golden("死んどります", "死ぬ", "v5n", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_n() {
    assert_golden("死んどりません", "死ぬ", "v5n", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_n() {
    assert_golden("死んどりました", "死ぬ", "v5n", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_n() {
    assert_golden("死んどりませんでした", "死ぬ", "v5n", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_n() {
    assert_golden("死んどって", "死ぬ", "v5n", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_n() {
    assert_golden("死んどろう", "死ぬ", "v5n", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_n() {
    assert_golden("死んどれる", "死ぬ", "v5n", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_n() {
    assert_golden("死んどられる", "死ぬ", "v5n", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_n() {
    assert_golden("死なす", "死ぬ", "v5n", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_n() {
    assert_golden("死んでは", "死ぬ", "v5n", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_n() {
    assert_golden("死んじゃ", "死ぬ", "v5n", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_n() {
    assert_golden("死ななきゃ", "死ぬ", "v5n", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_n() {
    assert_golden("死んじまう", "死ぬ", "v5n", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_n() {
    assert_golden("死んじゃう", "死ぬ", "v5n", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_n() {
    assert_golden("死んでいらっしゃる", "死ぬ", "v5n", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_n() {
    assert_golden("死んでいらっしゃらない", "死ぬ", "v5n", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_n() {
    assert_golden("死につつ", "死ぬ", "v5n", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_n() {
    assert_golden("死んでくれる", "死ぬ", "v5n", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_n() {
    assert_golden("死んでくれない", "死ぬ", "v5n", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_n() {
    assert_golden("死んでくれます", "死ぬ", "v5n", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_n() {
    assert_golden("死んでくれません", "死ぬ", "v5n", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_n() {
    assert_golden("死んでくれ", "死ぬ", "v5n", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_n() {
    assert_golden("死なへん", "死ぬ", "v5n", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_n() {
    assert_golden("死なへんかった", "死ぬ", "v5n", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_n() {
    assert_golden("死なひん", "死ぬ", "v5n", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_n() {
    assert_golden("死なひんかった", "死ぬ", "v5n", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_n() {
    assert_golden("死なさない", "死ぬ", "v5n", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_n() {
    assert_golden("死にましたら", "死ぬ", "v5n", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_n() {
    assert_golden("死にになる", "死ぬ", "v5n", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_n() {
    assert_golden("死になさる", "死ぬ", "v5n", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_n() {
    assert_golden("死にはる", "死ぬ", "v5n", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_n() {
    assert_golden("死になさるな", "死ぬ", "v5n", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_n() {
    assert_golden("死ぬまい", "死ぬ", "v5n", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_n() {
    assert_golden("死にますまい", "死ぬ", "v5n", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_n() {
    assert_golden("死なば", "死ぬ", "v5n", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_n() {
    assert_golden("死なねば", "死ぬ", "v5n", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_n() {
    assert_golden("死なにゃ", "死ぬ", "v5n", "～colloquial negative conditional");
}
