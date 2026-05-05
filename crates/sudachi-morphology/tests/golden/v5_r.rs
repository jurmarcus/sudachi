//! Golden tests ported from JL's `DeconjugatorTestsForV5R.cs`.
//! 228 test cases proving deconjugator output matches
//! JL's expectations for class V5R.

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
fn deconjugate_masu_stem_v5_r() {
    assert_golden("終わり", "終わる", "v5r", "～masu stem");
}

#[test]
fn deconjugate_masu_stem2_v5_r() {
    assert_golden("御座い", "御座る", "v5r", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_r() {
    assert_golden("終わらない", "終わる", "v5r", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_r() {
    assert_golden("終わります", "終わる", "v5r", "～polite");
}

#[test]
fn deconjugate_polite_non_past_affirmative2_v5_r() {
    assert_golden("御座います", "御座る", "v5r", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_r() {
    assert_golden("終わりましょう", "終わる", "v5r", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_r() {
    assert_golden("終わりません", "終わる", "v5r", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_r() {
    assert_golden("終わった", "終わる", "v5r", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_r() {
    assert_golden("終わらなかった", "終わる", "v5r", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_r() {
    assert_golden("終わりました", "終わる", "v5r", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_r() {
    assert_golden("終わりませんでした", "終わる", "v5r", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_r() {
    assert_golden("終わって", "終わる", "v5r", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_r() {
    assert_golden("終わらなくて", "終わる", "v5r", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_r() {
    assert_golden("終わらないで", "終わる", "v5r", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_r() {
    assert_golden("終わりまして", "終わる", "v5r", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_r() {
    assert_golden("終われる", "終わる", "v5r", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_r() {
    assert_golden("終わられる", "終わる", "v5r", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_r() {
    assert_golden("終われない", "終わる", "v5r", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_r() {
    assert_golden("終わられない", "終わる", "v5r", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_r() {
    assert_golden("終われた", "終わる", "v5r", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_r() {
    assert_golden("終わられた", "終わる", "v5r", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_r() {
    assert_golden("終われました", "終わる", "v5r", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_r() {
    assert_golden("終わられました", "終わる", "v5r", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_r() {
    assert_golden("終われなかった", "終わる", "v5r", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_r() {
    assert_golden("終わられなかった", "終わる", "v5r", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_r() {
    assert_golden("終われませんでした", "終わる", "v5r", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_r() {
    assert_golden("終わられませんでした", "終わる", "v5r", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_r() {
    assert_golden("終われます", "終わる", "v5r", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_r() {
    assert_golden("終わられます", "終わる", "v5r", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_r() {
    assert_golden("終われません", "終わる", "v5r", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_r() {
    assert_golden("終わられません", "終わる", "v5r", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_r() {
    assert_golden("終われ", "終わる", "v5r", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_r() {
    assert_golden("終わるな", "終わる", "v5r", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_r() {
    assert_golden("終わりなさい", "終わる", "v5r", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_r() {
    assert_golden("終わってください", "終わる", "v5r", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_r() {
    assert_golden("終わらないでください", "終わる", "v5r", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_r() {
    assert_golden("終わろう", "終わる", "v5r", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_r() {
    assert_golden("終わろ", "終わる", "v5r", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_r() {
    assert_golden("終わりましょう", "終わる", "v5r", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_r() {
    assert_golden("終われば", "終わる", "v5r", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_r() {
    assert_golden("終わらなければ", "終わる", "v5r", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_r() {
    assert_golden("終わったら", "終わる", "v5r", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_r() {
    assert_golden("終わったらば", "終わる", "v5r", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_r() {
    assert_golden("終わらなかったら", "終わる", "v5r", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_r() {
    assert_golden("終わらせる", "終わる", "v5r", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_r() {
    assert_golden("終わらせない", "終わる", "v5r", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_r() {
    assert_golden("終わらせん", "終わる", "v5r", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_r() {
    assert_golden("終わらせます", "終わる", "v5r", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_r() {
    assert_golden("終わらします", "終わる", "v5r", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_r() {
    assert_golden("終わらせません", "終わる", "v5r", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_r() {
    assert_golden("終わらせた", "終わる", "v5r", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_r() {
    assert_golden("終わらせなかった", "終わる", "v5r", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_r() {
    assert_golden("終わらせました", "終わる", "v5r", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_r() {
    assert_golden("終わらせませんでした", "終わる", "v5r", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_r() {
    assert_golden("終わらせられる", "終わる", "v5r", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_r() {
    assert_golden("終わらせられない", "終わる", "v5r", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_r() {
    assert_golden("終わらせられます", "終わる", "v5r", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_r() {
    assert_golden("終わらせられません", "終わる", "v5r", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_r() {
    assert_golden("終わりたい", "終わる", "v5r", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_r() {
    assert_golden("終わりたくありません", "終わる", "v5r", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_r() {
    assert_golden("終わりたくありませんでした", "終わる", "v5r", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_r() {
    assert_golden("終わりたくない", "終わる", "v5r", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_r() {
    assert_golden("終わりたかった", "終わる", "v5r", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_r() {
    assert_golden("終わりたくなかった", "終わる", "v5r", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_r() {
    assert_golden("終わっている", "終わる", "v5r", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_r() {
    assert_golden("終わっていない", "終わる", "v5r", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_r() {
    assert_golden("終わっていた", "終わる", "v5r", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_r() {
    assert_golden("終わっていなかった", "終わる", "v5r", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_r() {
    assert_golden("終わっています", "終わる", "v5r", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_r() {
    assert_golden("終わっていません", "終わる", "v5r", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_r() {
    assert_golden("終わっていました", "終わる", "v5r", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_r() {
    assert_golden("終わっていませんでした", "終わる", "v5r", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_r() {
    assert_golden("終わってる", "終わる", "v5r", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_r() {
    assert_golden("終わってない", "終わる", "v5r", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_r() {
    assert_golden("終わってた", "終わる", "v5r", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_r() {
    assert_golden("終わってなかった", "終わる", "v5r", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_r() {
    assert_golden("終わってます", "終わる", "v5r", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_r() {
    assert_golden("終わってません", "終わる", "v5r", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_r() {
    assert_golden("終わってました", "終わる", "v5r", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_r() {
    assert_golden("終わってません", "終わる", "v5r", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_r() {
    assert_golden("終わってませんでした", "終わる", "v5r", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_r() {
    assert_golden("終わってしまう", "終わる", "v5r", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_r() {
    assert_golden("終わってもう", "終わる", "v5r", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_r() {
    assert_golden("終わってしまわない", "終わる", "v5r", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_r() {
    assert_golden("終わってしまった", "終わる", "v5r", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_r() {
    assert_golden("終わってしまわなかった", "終わる", "v5r", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_r() {
    assert_golden("終わってしまって", "終わる", "v5r", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_r() {
    assert_golden("終わってしまえば", "終わる", "v5r", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_r() {
    assert_golden("終わってしまわなければ", "終わる", "v5r", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_r() {
    assert_golden("終わってしまわなかったら", "終わる", "v5r", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_r() {
    assert_golden("終わってしまったら", "終わる", "v5r", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_r() {
    assert_golden("終わってしまおう", "終わる", "v5r", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_r() {
    assert_golden("終わってしまいます", "終わる", "v5r", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_r() {
    assert_golden("終わってしまいません", "終わる", "v5r", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_r() {
    assert_golden("終わってしまいました", "終わる", "v5r", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_r() {
    assert_golden("終わってしまいませんでした", "終わる", "v5r", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_r() {
    assert_golden("終わってしまえる", "終わる", "v5r", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_r() {
    assert_golden("終わってしまわれる", "終わる", "v5r", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_r() {
    assert_golden("終わってしまわせる", "終わる", "v5r", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_r() {
    assert_golden("終わっちゃう", "終わる", "v5r", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_r() {
    assert_golden("終わっちゃわない", "終わる", "v5r", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_r() {
    assert_golden("終わっちゃった", "終わる", "v5r", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_r() {
    assert_golden("終わっちゃわなかった", "終わる", "v5r", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_r() {
    assert_golden("終わっちゃって", "終わる", "v5r", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_r() {
    assert_golden("終わっちゃえば", "終わる", "v5r", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_r() {
    assert_golden("終わっちゃわなければ", "終わる", "v5r", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_r() {
    assert_golden("終わっちゃわなかったら", "終わる", "v5r", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_r() {
    assert_golden("終わっちゃおう", "終わる", "v5r", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_r() {
    assert_golden("終わっちゃえる", "終わる", "v5r", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_r() {
    assert_golden("終わっておく", "終わる", "v5r", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_r() {
    assert_golden("終わっておかない", "終わる", "v5r", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_r() {
    assert_golden("終わっておいた", "終わる", "v5r", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_r() {
    assert_golden("終わっておかなかった", "終わる", "v5r", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_r() {
    assert_golden("終わっておいて", "終わる", "v5r", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_r() {
    assert_golden("終わっておけば", "終わる", "v5r", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_r() {
    assert_golden("終わっておいたら", "終わる", "v5r", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_r() {
    assert_golden("終わっておこう", "終わる", "v5r", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_r() {
    assert_golden("終わっておける", "終わる", "v5r", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_r() {
    assert_golden("終わっておかれる", "終わる", "v5r", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_r() {
    assert_golden("終わっとく", "終わる", "v5r", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_r() {
    assert_golden("終わっとかない", "終わる", "v5r", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_r() {
    assert_golden("終わっといた", "終わる", "v5r", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_r() {
    assert_golden("終わっとかなかった", "終わる", "v5r", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_r() {
    assert_golden("終わっといて", "終わる", "v5r", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_r() {
    assert_golden("終わっとけば", "終わる", "v5r", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_r() {
    assert_golden("終わっといたら", "終わる", "v5r", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_r() {
    assert_golden("終わっとこう", "終わる", "v5r", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_r() {
    assert_golden("終わっとける", "終わる", "v5r", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_r() {
    assert_golden("終わっとかれる", "終わる", "v5r", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_r() {
    assert_golden("終わってある", "終わる", "v5r", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_r() {
    assert_golden("終わってあった", "終わる", "v5r", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_r() {
    assert_golden("終わってあって", "終わる", "v5r", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_r() {
    assert_golden("終わってあったら", "終わる", "v5r", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_r() {
    assert_golden("終わってあれば", "終わる", "v5r", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_r() {
    assert_golden("終わっていく", "終わる", "v5r", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_r() {
    assert_golden("終わっていかない", "終わる", "v5r", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_r() {
    assert_golden("終わっていった", "終わる", "v5r", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_r() {
    assert_golden("終わっていかなかった", "終わる", "v5r", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_r() {
    assert_golden("終わっていって", "終わる", "v5r", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_r() {
    assert_golden("終わっていこう", "終わる", "v5r", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_r() {
    assert_golden("終わっていける", "終わる", "v5r", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_r() {
    assert_golden("終わっていかれる", "終わる", "v5r", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_r() {
    assert_golden("終わっていかせる", "終わる", "v5r", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_r() {
    assert_golden("終わってくる", "終わる", "v5r", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_r() {
    assert_golden("終わってこない", "終わる", "v5r", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_r() {
    assert_golden("終わってきた", "終わる", "v5r", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_r() {
    assert_golden("終わってこなかった", "終わる", "v5r", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_r() {
    assert_golden("終わってきて", "終わる", "v5r", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_r() {
    assert_golden("終わってくれば", "終わる", "v5r", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_r() {
    assert_golden("終わってきたら", "終わる", "v5r", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_r() {
    assert_golden("終わってこられる", "終わる", "v5r", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_r() {
    assert_golden("終わってこさせる", "終わる", "v5r", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_r() {
    assert_golden("終わりながら", "終わる", "v5r", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_r() {
    assert_golden("終わりすぎる", "終わる", "v5r", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_r() {
    assert_golden("終わりそう", "終わる", "v5r", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_r() {
    assert_golden("終わらぬ", "終わる", "v5r", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_r() {
    assert_golden("終わらず", "終わる", "v5r", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_r() {
    assert_golden("終わらずに", "終わる", "v5r", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_r() {
    assert_golden("終わったり", "終わる", "v5r", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_r() {
    assert_golden("終わらなかったり", "終わる", "v5r", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_r() {
    assert_golden("終わらん", "終わる", "v5r", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_r() {
    assert_golden("終わらんかった", "終わる", "v5r", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_r() {
    assert_golden("終わらざる", "終わる", "v5r", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_r() {
    assert_golden("終われよう", "終わる", "v5r", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_r() {
    assert_golden("終われよ", "終わる", "v5r", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_r() {
    assert_golden("終われろ", "終わる", "v5r", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_r() {
    assert_golden("終われて", "終わる", "v5r", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_r() {
    assert_golden("終われたら", "終わる", "v5r", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_r() {
    assert_golden("終われれば", "終わる", "v5r", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_r() {
    assert_golden("終われられる", "終わる", "v5r", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_r() {
    assert_golden("終われさせる", "終わる", "v5r", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_r() {
    assert_golden("終わってあげる", "終わる", "v5r", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_r() {
    assert_golden("終わってあげられる", "終わる", "v5r", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_r() {
    assert_golden("終わっておる", "終わる", "v5r", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_r() {
    assert_golden("終わっておらない", "終わる", "v5r", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_r() {
    assert_golden("終わっておらん", "終わる", "v5r", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_r() {
    assert_golden("終わっておった", "終わる", "v5r", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_r() {
    assert_golden("終わっておらなかった", "終わる", "v5r", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_r() {
    assert_golden("終わっております", "終わる", "v5r", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_r() {
    assert_golden("終わっておりません", "終わる", "v5r", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_r() {
    assert_golden("終わっておりました", "終わる", "v5r", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_r() {
    assert_golden("終わっておりませんでした", "終わる", "v5r", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_r() {
    assert_golden("終わっておって", "終わる", "v5r", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_r() {
    assert_golden("終わっておろう", "終わる", "v5r", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_r() {
    assert_golden("終わっておれる", "終わる", "v5r", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_r() {
    assert_golden("終わっておられる", "終わる", "v5r", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_r() {
    assert_golden("終わっとる", "終わる", "v5r", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_r() {
    assert_golden("終わっとらない", "終わる", "v5r", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_r() {
    assert_golden("終わっとらん", "終わる", "v5r", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_r() {
    assert_golden("終わっとった", "終わる", "v5r", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_r() {
    assert_golden("終わっとらなかった", "終わる", "v5r", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_r() {
    assert_golden("終わっとります", "終わる", "v5r", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_r() {
    assert_golden("終わっとりません", "終わる", "v5r", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_r() {
    assert_golden("終わっとりました", "終わる", "v5r", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_r() {
    assert_golden("終わっとりませんでした", "終わる", "v5r", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_r() {
    assert_golden("終わっとって", "終わる", "v5r", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_r() {
    assert_golden("終わっとろう", "終わる", "v5r", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_r() {
    assert_golden("終わっとれる", "終わる", "v5r", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_r() {
    assert_golden("終わっとられる", "終わる", "v5r", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_r() {
    assert_golden("終わらす", "終わる", "v5r", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_r() {
    assert_golden("終わっては", "終わる", "v5r", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_r() {
    assert_golden("終わっちゃ", "終わる", "v5r", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_r() {
    assert_golden("終わらなきゃ", "終わる", "v5r", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_r() {
    assert_golden("終わっちまう", "終わる", "v5r", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_r() {
    assert_golden("終わっちゃう", "終わる", "v5r", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_r() {
    assert_golden("終わっていらっしゃる", "終わる", "v5r", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_r() {
    assert_golden("終わっていらっしゃらない", "終わる", "v5r", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_r() {
    assert_golden("終わりつつ", "終わる", "v5r", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_r() {
    assert_golden("終わってくれる", "終わる", "v5r", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_r() {
    assert_golden("終わってくれない", "終わる", "v5r", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_r() {
    assert_golden("終わってくれます", "終わる", "v5r", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_r() {
    assert_golden("終わってくれません", "終わる", "v5r", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_r() {
    assert_golden("終わってくれ", "終わる", "v5r", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_r() {
    assert_golden("終わらへん", "終わる", "v5r", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_r() {
    assert_golden("終わらへんかった", "終わる", "v5r", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_r() {
    assert_golden("終わらひん", "終わる", "v5r", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_r() {
    assert_golden("終わらひんかった", "終わる", "v5r", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_r() {
    assert_golden("終わらさない", "終わる", "v5r", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_r() {
    assert_golden("終わりましたら", "終わる", "v5r", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_r() {
    assert_golden("終わりになる", "終わる", "v5r", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_r() {
    assert_golden("終わりなさる", "終わる", "v5r", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_r() {
    assert_golden("終わりはる", "終わる", "v5r", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_r() {
    assert_golden("終わりなさるな", "終わる", "v5r", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_r() {
    assert_golden("終わるまい", "終わる", "v5r", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_r() {
    assert_golden("終わりますまい", "終わる", "v5r", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_r() {
    assert_golden("終わらば", "終わる", "v5r", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_r() {
    assert_golden("終わらねば", "終わる", "v5r", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_r() {
    assert_golden("終わらにゃ", "終わる", "v5r", "～colloquial negative conditional");
}

// ─── Verb-producing aux on renyou base (added 2026-05-06) ────────────

#[test]
fn deconjugate_aux_hajimeru_past_v5_r() {
    assert_golden("走り始めた", "走る", "v5r", "～start V-ing→past");
}

#[test]
fn deconjugate_aux_tsuzukeru_teiru_v5_r() {
    assert_golden("走り続けている", "走る", "v5r", "～continue V-ing→teiru");
}

#[test]
fn deconjugate_aux_te_morau_past_v5_r() {
    assert_golden("走ってもらった", "走る", "v5r", "～have someone do→past");
}
