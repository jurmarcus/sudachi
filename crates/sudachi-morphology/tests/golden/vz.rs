//! Golden tests ported from JL's `DeconjugatorTestsForVZ.cs`.
//! 221 test cases proving deconjugator output matches
//! JL's expectations for class VZ.

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
fn deconjugate_masu_stem_vz() {
    assert_golden("命じ", "命ずる", "vz", "～masu stem");
}

#[test]
fn deconjugate_masu_stem2_vz() {
    assert_golden("命ぜ", "命ずる", "vz", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_vz() {
    assert_golden("命ぜない", "命ずる", "vz", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_vz() {
    assert_golden("命じます", "命ずる", "vz", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_vz() {
    assert_golden("命じましょう", "命ずる", "vz", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_vz() {
    assert_golden("命じません", "命ずる", "vz", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_vz() {
    assert_golden("命ぜた", "命ずる", "vz", "～past");
}

#[test]
fn deconjugate_plain_past_negative_vz() {
    assert_golden("命じなかった", "命ずる", "vz", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_vz() {
    assert_golden("命じました", "命ずる", "vz", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_vz() {
    assert_golden("命じませんでした", "命ずる", "vz", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_vz() {
    assert_golden("命じて", "命ずる", "vz", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_vz() {
    assert_golden("命じなくて", "命ずる", "vz", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_vz() {
    assert_golden("命じないで", "命ずる", "vz", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_vz() {
    assert_golden("命じまして", "命ずる", "vz", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_honorific_affirmative_vz() {
    assert_golden("命ぜられる", "命ずる", "vz", "～passive/potential");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_affirmative2_vz() {
    assert_golden("命じられる", "命ずる", "vz", "～passive/potential");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_negative_vz() {
    assert_golden("命ぜられない", "命ずる", "vz", "～passive/potential→negative");
}

#[test]
fn deconjugate_plain_past_passive_potential_affirmative_vz() {
    assert_golden("命ぜられた", "命ずる", "vz", "～passive/potential→past");
}

#[test]
fn deconjugate_polite_past_passive_potential_affirmative_vz() {
    assert_golden("命ぜられました", "命ずる", "vz", "～passive/potential→polite past");
}

#[test]
fn deconjugate_plain_past_passive_potential_negative_vz() {
    assert_golden("命ぜられなかった", "命ずる", "vz", "～passive/potential→negative→past");
}

#[test]
fn deconjugate_polite_past_passive_potential_negative_vz() {
    assert_golden("命ぜられませんでした", "命ずる", "vz", "～passive/potential→polite past negative");
}

#[test]
fn deconjugate_polite_passive_potential_affirmative_vz() {
    assert_golden("命ぜられます", "命ずる", "vz", "～passive/potential→polite");
}

#[test]
fn deconjugate_polite_passive_potential_negative_vz() {
    assert_golden("命ぜられません", "命ずる", "vz", "～passive/potential→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_vz() {
    assert_golden("命じろ", "命ずる", "vz", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_affirmative2_vz() {
    assert_golden("命ぜよ", "命ずる", "vz", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_vz() {
    assert_golden("命ずるな", "命ずる", "vz", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_vz() {
    assert_golden("命じなさい", "命ずる", "vz", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_vz() {
    assert_golden("命じてください", "命ずる", "vz", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_vz() {
    assert_golden("命じないでください", "命ずる", "vz", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_vz() {
    assert_golden("命じよう", "命ずる", "vz", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_vz() {
    assert_golden("命じよ", "命ずる", "vz", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_vz() {
    assert_golden("命じましょう", "命ずる", "vz", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_vz() {
    assert_golden("命ずれば", "命ずる", "vz", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_vz() {
    assert_golden("命じなければ", "命ずる", "vz", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_vz() {
    assert_golden("命じたら", "命ずる", "vz", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_vz() {
    assert_golden("命じたらば", "命ずる", "vz", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_vz() {
    assert_golden("命じなかったら", "命ずる", "vz", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_vz() {
    assert_golden("命じさせる", "命ずる", "vz", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_vz() {
    assert_golden("命じさせない", "命ずる", "vz", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_vz() {
    assert_golden("命じさせん", "命ずる", "vz", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_vz() {
    assert_golden("命じさせます", "命ずる", "vz", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_vz() {
    assert_golden("命じさします", "命ずる", "vz", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_vz() {
    assert_golden("命じさせません", "命ずる", "vz", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_vz() {
    assert_golden("命じさせた", "命ずる", "vz", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_vz() {
    assert_golden("命じさせなかった", "命ずる", "vz", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_vz() {
    assert_golden("命じさせました", "命ずる", "vz", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_vz() {
    assert_golden("命じさせませんでした", "命ずる", "vz", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_vz() {
    assert_golden("命じさせられる", "命ずる", "vz", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_vz() {
    assert_golden("命じさせられない", "命ずる", "vz", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_vz() {
    assert_golden("命じさせられます", "命ずる", "vz", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_vz() {
    assert_golden("命じさせられません", "命ずる", "vz", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_vz() {
    assert_golden("命じたい", "命ずる", "vz", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_vz() {
    assert_golden("命じたくありません", "命ずる", "vz", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_vz() {
    assert_golden("命じたくありませんでした", "命ずる", "vz", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_vz() {
    assert_golden("命じたくない", "命ずる", "vz", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_vz() {
    assert_golden("命じたかった", "命ずる", "vz", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_vz() {
    assert_golden("命じたくなかった", "命ずる", "vz", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_vz() {
    assert_golden("命じている", "命ずる", "vz", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_vz() {
    assert_golden("命じていない", "命ずる", "vz", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_vz() {
    assert_golden("命じていた", "命ずる", "vz", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_vz() {
    assert_golden("命じていなかった", "命ずる", "vz", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_vz() {
    assert_golden("命じています", "命ずる", "vz", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_vz() {
    assert_golden("命じていません", "命ずる", "vz", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_vz() {
    assert_golden("命じていました", "命ずる", "vz", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_vz() {
    assert_golden("命じていませんでした", "命ずる", "vz", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_vz() {
    assert_golden("命じてる", "命ずる", "vz", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_vz() {
    assert_golden("命じてない", "命ずる", "vz", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_vz() {
    assert_golden("命じてた", "命ずる", "vz", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_vz() {
    assert_golden("命じてなかった", "命ずる", "vz", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_vz() {
    assert_golden("命じてます", "命ずる", "vz", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_vz() {
    assert_golden("命じてません", "命ずる", "vz", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_vz() {
    assert_golden("命じてました", "命ずる", "vz", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_vz() {
    assert_golden("命じてません", "命ずる", "vz", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_vz() {
    assert_golden("命じてませんでした", "命ずる", "vz", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_vz() {
    assert_golden("命じてしまう", "命ずる", "vz", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_vz() {
    assert_golden("命じてもう", "命ずる", "vz", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_vz() {
    assert_golden("命じてしまわない", "命ずる", "vz", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_vz() {
    assert_golden("命じてしまった", "命ずる", "vz", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_vz() {
    assert_golden("命じてしまわなかった", "命ずる", "vz", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_vz() {
    assert_golden("命じてしまって", "命ずる", "vz", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_vz() {
    assert_golden("命じてしまえば", "命ずる", "vz", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_vz() {
    assert_golden("命じてしまわなければ", "命ずる", "vz", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_vz() {
    assert_golden("命じてしまわなかったら", "命ずる", "vz", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_vz() {
    assert_golden("命じてしまったら", "命ずる", "vz", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_vz() {
    assert_golden("命じてしまおう", "命ずる", "vz", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_vz() {
    assert_golden("命じてしまいます", "命ずる", "vz", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_vz() {
    assert_golden("命じてしまいません", "命ずる", "vz", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_vz() {
    assert_golden("命じてしまいました", "命ずる", "vz", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_vz() {
    assert_golden("命じてしまいませんでした", "命ずる", "vz", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_vz() {
    assert_golden("命じてしまえる", "命ずる", "vz", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_vz() {
    assert_golden("命じてしまわれる", "命ずる", "vz", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_vz() {
    assert_golden("命じてしまわせる", "命ずる", "vz", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_vz() {
    assert_golden("命じちゃう", "命ずる", "vz", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_vz() {
    assert_golden("命じちゃわない", "命ずる", "vz", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_vz() {
    assert_golden("命じちゃった", "命ずる", "vz", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_vz() {
    assert_golden("命じちゃわなかった", "命ずる", "vz", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_vz() {
    assert_golden("命じちゃって", "命ずる", "vz", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_vz() {
    assert_golden("命じちゃえば", "命ずる", "vz", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_vz() {
    assert_golden("命じちゃわなければ", "命ずる", "vz", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_vz() {
    assert_golden("命じちゃわなかったら", "命ずる", "vz", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_vz() {
    assert_golden("命じちゃおう", "命ずる", "vz", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_vz() {
    assert_golden("命じちゃえる", "命ずる", "vz", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_vz() {
    assert_golden("命じておく", "命ずる", "vz", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_vz() {
    assert_golden("命じておかない", "命ずる", "vz", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_vz() {
    assert_golden("命じておいた", "命ずる", "vz", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_vz() {
    assert_golden("命じておかなかった", "命ずる", "vz", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_vz() {
    assert_golden("命じておいて", "命ずる", "vz", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_vz() {
    assert_golden("命じておけば", "命ずる", "vz", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_vz() {
    assert_golden("命じておいたら", "命ずる", "vz", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_vz() {
    assert_golden("命じておこう", "命ずる", "vz", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_vz() {
    assert_golden("命じておける", "命ずる", "vz", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_vz() {
    assert_golden("命じておかれる", "命ずる", "vz", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_vz() {
    assert_golden("命じとく", "命ずる", "vz", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_vz() {
    assert_golden("命じとかない", "命ずる", "vz", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_vz() {
    assert_golden("命じといた", "命ずる", "vz", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_vz() {
    assert_golden("命じとかなかった", "命ずる", "vz", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_vz() {
    assert_golden("命じといて", "命ずる", "vz", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_vz() {
    assert_golden("命じとけば", "命ずる", "vz", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_vz() {
    assert_golden("命じといたら", "命ずる", "vz", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_vz() {
    assert_golden("命じとこう", "命ずる", "vz", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_vz() {
    assert_golden("命じとける", "命ずる", "vz", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_vz() {
    assert_golden("命じとかれる", "命ずる", "vz", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_vz() {
    assert_golden("命じてある", "命ずる", "vz", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_vz() {
    assert_golden("命じてあった", "命ずる", "vz", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_vz() {
    assert_golden("命じてあって", "命ずる", "vz", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_vz() {
    assert_golden("命じてあったら", "命ずる", "vz", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_vz() {
    assert_golden("命じてあれば", "命ずる", "vz", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_vz() {
    assert_golden("命じていく", "命ずる", "vz", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_vz() {
    assert_golden("命じていかない", "命ずる", "vz", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_vz() {
    assert_golden("命じていった", "命ずる", "vz", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_vz() {
    assert_golden("命じていかなかった", "命ずる", "vz", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_vz() {
    assert_golden("命じていって", "命ずる", "vz", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_vz() {
    assert_golden("命じていこう", "命ずる", "vz", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_vz() {
    assert_golden("命じていける", "命ずる", "vz", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_vz() {
    assert_golden("命じていかれる", "命ずる", "vz", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_vz() {
    assert_golden("命じていかせる", "命ずる", "vz", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_vz() {
    assert_golden("命じてくる", "命ずる", "vz", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_vz() {
    assert_golden("命じてこない", "命ずる", "vz", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_vz() {
    assert_golden("命じてきた", "命ずる", "vz", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_vz() {
    assert_golden("命じてこなかった", "命ずる", "vz", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_vz() {
    assert_golden("命じてきて", "命ずる", "vz", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_vz() {
    assert_golden("命じてくれば", "命ずる", "vz", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_vz() {
    assert_golden("命じてきたら", "命ずる", "vz", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_vz() {
    assert_golden("命じてこられる", "命ずる", "vz", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_vz() {
    assert_golden("命じてこさせる", "命ずる", "vz", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_vz() {
    assert_golden("命じながら", "命ずる", "vz", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_vz() {
    assert_golden("命じすぎる", "命ずる", "vz", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_vz() {
    assert_golden("命じそう", "命ずる", "vz", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_vz() {
    assert_golden("命ぜぬ", "命ずる", "vz", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_vz() {
    assert_golden("命ぜず", "命ずる", "vz", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_vz() {
    assert_golden("命じずに", "命ずる", "vz", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_vz() {
    assert_golden("命じたり", "命ずる", "vz", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_vz() {
    assert_golden("命じなかったり", "命ずる", "vz", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_vz() {
    assert_golden("命じん", "命ずる", "vz", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_vz() {
    assert_golden("命じんかった", "命ずる", "vz", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_vz() {
    assert_golden("命じざる", "命ずる", "vz", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_volitional_vz() {
    assert_golden("命ぜられよう", "命ずる", "vz", "～passive/potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_passive_potential_volitional_vz() {
    assert_golden("命ぜられよ", "命ずる", "vz", "～passive/potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_imperative_vz() {
    assert_golden("命ぜられろ", "命ずる", "vz", "～passive/potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_te_form_vz() {
    assert_golden("命ぜられて", "命ずる", "vz", "～passive/potential→te");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_temporal_conditional_vz() {
    assert_golden("命ぜられたら", "命ずる", "vz", "～passive/potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_provisional_conditional_vz() {
    assert_golden("命ぜられれば", "命ずる", "vz", "～passive/potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_passive_potential_vz() {
    assert_golden("命ぜられられる", "命ずる", "vz", "～passive/potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_causative_vz() {
    assert_golden("命ぜられさせる", "命ずる", "vz", "～passive/potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_vz() {
    assert_golden("命じてあげる", "命ずる", "vz", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_vz() {
    assert_golden("命じてあげられる", "命ずる", "vz", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_vz() {
    assert_golden("命じておる", "命ずる", "vz", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_vz() {
    assert_golden("命じておらない", "命ずる", "vz", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_vz() {
    assert_golden("命じておらん", "命ずる", "vz", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_vz() {
    assert_golden("命じておった", "命ずる", "vz", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_vz() {
    assert_golden("命じておらなかった", "命ずる", "vz", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_vz() {
    assert_golden("命じております", "命ずる", "vz", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_vz() {
    assert_golden("命じておりません", "命ずる", "vz", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_vz() {
    assert_golden("命じておりました", "命ずる", "vz", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_vz() {
    assert_golden("命じておりませんでした", "命ずる", "vz", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_vz() {
    assert_golden("命じておって", "命ずる", "vz", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_vz() {
    assert_golden("命じておろう", "命ずる", "vz", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_vz() {
    assert_golden("命じておれる", "命ずる", "vz", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_vz() {
    assert_golden("命じておられる", "命ずる", "vz", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_vz() {
    assert_golden("命じとる", "命ずる", "vz", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_vz() {
    assert_golden("命じとらない", "命ずる", "vz", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_vz() {
    assert_golden("命じとらん", "命ずる", "vz", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_vz() {
    assert_golden("命じとった", "命ずる", "vz", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_vz() {
    assert_golden("命じとらなかった", "命ずる", "vz", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_vz() {
    assert_golden("命じとります", "命ずる", "vz", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_vz() {
    assert_golden("命じとりません", "命ずる", "vz", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_vz() {
    assert_golden("命じとりました", "命ずる", "vz", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_vz() {
    assert_golden("命じとりませんでした", "命ずる", "vz", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_vz() {
    assert_golden("命じとって", "命ずる", "vz", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_vz() {
    assert_golden("命じとろう", "命ずる", "vz", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_vz() {
    assert_golden("命じとれる", "命ずる", "vz", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_vz() {
    assert_golden("命じとられる", "命ずる", "vz", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_vz() {
    assert_golden("命じさす", "命ずる", "vz", "～short causative");
}

#[test]
fn deconjugate_plain_non_past_na_vz() {
    assert_golden("命じな", "命ずる", "vz", "～casual polite imperative");
}

#[test]
fn deconjugate_topic_or_condition_vz() {
    assert_golden("命じては", "命ずる", "vz", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_vz() {
    assert_golden("命じちゃ", "命ずる", "vz", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_vz() {
    assert_golden("命じなきゃ", "命ずる", "vz", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_vz() {
    assert_golden("命じちまう", "命ずる", "vz", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_vz() {
    assert_golden("命じちゃう", "命ずる", "vz", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_vz() {
    assert_golden("命じていらっしゃる", "命ずる", "vz", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_vz() {
    assert_golden("命じていらっしゃらない", "命ずる", "vz", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_vz() {
    assert_golden("命じつつ", "命ずる", "vz", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_vz() {
    assert_golden("命じてくれる", "命ずる", "vz", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_vz() {
    assert_golden("命じてくれない", "命ずる", "vz", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_vz() {
    assert_golden("命じてくれます", "命ずる", "vz", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_vz() {
    assert_golden("命じてくれません", "命ずる", "vz", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_vz() {
    assert_golden("命じてくれ", "命ずる", "vz", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_vz() {
    assert_golden("命じへん", "命ずる", "vz", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_vz() {
    assert_golden("命じへんかった", "命ずる", "vz", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_vz() {
    assert_golden("命じひん", "命ずる", "vz", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_vz() {
    assert_golden("命じひんかった", "命ずる", "vz", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_vz() {
    assert_golden("命じささない", "命ずる", "vz", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_vz() {
    assert_golden("命じましたら", "命ずる", "vz", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_vz() {
    assert_golden("命じになる", "命ずる", "vz", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_vz() {
    assert_golden("命じなさる", "命ずる", "vz", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_vz() {
    assert_golden("命じはる", "命ずる", "vz", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_vz() {
    assert_golden("命じなさるな", "命ずる", "vz", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_vz() {
    assert_golden("命ずるまい", "命ずる", "vz", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_vz() {
    assert_golden("命じますまい", "命ずる", "vz", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_vz() {
    assert_golden("命ぜねば", "命ずる", "vz", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_vsz() {
    assert_golden("命ぜにゃ", "命ずる", "vz", "～colloquial negative conditional");
}
