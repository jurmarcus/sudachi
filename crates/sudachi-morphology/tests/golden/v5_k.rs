//! Golden tests ported from JL's `DeconjugatorTestsForV5K.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V5K.

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
fn deconjugate_masu_stem_v5_k() {
    assert_golden("泣き", "泣く", "v5k", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_k() {
    assert_golden("泣かない", "泣く", "v5k", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_k() {
    assert_golden("泣きます", "泣く", "v5k", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_k() {
    assert_golden("泣きましょう", "泣く", "v5k", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_k() {
    assert_golden("泣きません", "泣く", "v5k", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_k() {
    assert_golden("泣いた", "泣く", "v5k", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_k() {
    assert_golden("泣かなかった", "泣く", "v5k", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_k() {
    assert_golden("泣きました", "泣く", "v5k", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_k() {
    assert_golden("泣きませんでした", "泣く", "v5k", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_k() {
    assert_golden("泣いて", "泣く", "v5k", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_k() {
    assert_golden("泣かなくて", "泣く", "v5k", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_k() {
    assert_golden("泣かないで", "泣く", "v5k", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_k() {
    assert_golden("泣きまして", "泣く", "v5k", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_k() {
    assert_golden("泣ける", "泣く", "v5k", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_k() {
    assert_golden("泣かれる", "泣く", "v5k", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_k() {
    assert_golden("泣けない", "泣く", "v5k", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_k() {
    assert_golden("泣かれない", "泣く", "v5k", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_k() {
    assert_golden("泣けた", "泣く", "v5k", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_k() {
    assert_golden("泣かれた", "泣く", "v5k", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_k() {
    assert_golden("泣けました", "泣く", "v5k", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_k() {
    assert_golden("泣かれました", "泣く", "v5k", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_k() {
    assert_golden("泣けなかった", "泣く", "v5k", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_k() {
    assert_golden("泣かれなかった", "泣く", "v5k", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_k() {
    assert_golden("泣けませんでした", "泣く", "v5k", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_k() {
    assert_golden("泣かれませんでした", "泣く", "v5k", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_k() {
    assert_golden("泣けます", "泣く", "v5k", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_k() {
    assert_golden("泣かれます", "泣く", "v5k", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_k() {
    assert_golden("泣けません", "泣く", "v5k", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_k() {
    assert_golden("泣かれません", "泣く", "v5k", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_k() {
    assert_golden("泣け", "泣く", "v5k", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_k() {
    assert_golden("泣くな", "泣く", "v5k", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_k() {
    assert_golden("泣きなさい", "泣く", "v5k", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_k() {
    assert_golden("泣いてください", "泣く", "v5k", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_k() {
    assert_golden("泣かないでください", "泣く", "v5k", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_k() {
    assert_golden("泣こう", "泣く", "v5k", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_k() {
    assert_golden("泣こ", "泣く", "v5k", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_k() {
    assert_golden("泣きましょう", "泣く", "v5k", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_k() {
    assert_golden("泣けば", "泣く", "v5k", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_k() {
    assert_golden("泣かなければ", "泣く", "v5k", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_k() {
    assert_golden("泣いたら", "泣く", "v5k", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_k() {
    assert_golden("泣いたらば", "泣く", "v5k", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_k() {
    assert_golden("泣かなかったら", "泣く", "v5k", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_k() {
    assert_golden("泣かせる", "泣く", "v5k", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_k() {
    assert_golden("泣かせない", "泣く", "v5k", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_k() {
    assert_golden("泣かせん", "泣く", "v5k", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_k() {
    assert_golden("泣かせます", "泣く", "v5k", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_k() {
    assert_golden("泣かします", "泣く", "v5k", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_k() {
    assert_golden("泣かせません", "泣く", "v5k", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_k() {
    assert_golden("泣かせた", "泣く", "v5k", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_k() {
    assert_golden("泣かせなかった", "泣く", "v5k", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_k() {
    assert_golden("泣かせました", "泣く", "v5k", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_k() {
    assert_golden("泣かせませんでした", "泣く", "v5k", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_k() {
    assert_golden("泣かせられる", "泣く", "v5k", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_k() {
    assert_golden("泣かせられない", "泣く", "v5k", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_k() {
    assert_golden("泣かせられます", "泣く", "v5k", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_k() {
    assert_golden("泣かせられません", "泣く", "v5k", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_k() {
    assert_golden("泣きたい", "泣く", "v5k", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_k() {
    assert_golden("泣きたくありません", "泣く", "v5k", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_k() {
    assert_golden("泣きたくありませんでした", "泣く", "v5k", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_k() {
    assert_golden("泣きたくない", "泣く", "v5k", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_k() {
    assert_golden("泣きたかった", "泣く", "v5k", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_k() {
    assert_golden("泣きたくなかった", "泣く", "v5k", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_k() {
    assert_golden("泣いている", "泣く", "v5k", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_k() {
    assert_golden("泣いていない", "泣く", "v5k", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_k() {
    assert_golden("泣いていた", "泣く", "v5k", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_k() {
    assert_golden("泣いていなかった", "泣く", "v5k", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_k() {
    assert_golden("泣いています", "泣く", "v5k", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_k() {
    assert_golden("泣いていません", "泣く", "v5k", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_k() {
    assert_golden("泣いていました", "泣く", "v5k", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_k() {
    assert_golden("泣いていませんでした", "泣く", "v5k", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_k() {
    assert_golden("泣いてる", "泣く", "v5k", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_k() {
    assert_golden("泣いてない", "泣く", "v5k", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_k() {
    assert_golden("泣いてた", "泣く", "v5k", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_k() {
    assert_golden("泣いてなかった", "泣く", "v5k", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_k() {
    assert_golden("泣いてます", "泣く", "v5k", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_k() {
    assert_golden("泣いてません", "泣く", "v5k", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_k() {
    assert_golden("泣いてました", "泣く", "v5k", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_k() {
    assert_golden("泣いてません", "泣く", "v5k", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_k() {
    assert_golden("泣いてませんでした", "泣く", "v5k", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_k() {
    assert_golden("泣いてしまう", "泣く", "v5k", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_k() {
    assert_golden("泣いてもう", "泣く", "v5k", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_k() {
    assert_golden("泣いてしまわない", "泣く", "v5k", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_k() {
    assert_golden("泣いてしまった", "泣く", "v5k", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_k() {
    assert_golden("泣いてしまわなかった", "泣く", "v5k", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_k() {
    assert_golden("泣いてしまって", "泣く", "v5k", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_k() {
    assert_golden("泣いてしまえば", "泣く", "v5k", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_k() {
    assert_golden("泣いてしまわなければ", "泣く", "v5k", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_k() {
    assert_golden("泣いてしまわなかったら", "泣く", "v5k", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_k() {
    assert_golden("泣いてしまったら", "泣く", "v5k", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_k() {
    assert_golden("泣いてしまおう", "泣く", "v5k", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_k() {
    assert_golden("泣いてしまいます", "泣く", "v5k", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_k() {
    assert_golden("泣いてしまいません", "泣く", "v5k", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_k() {
    assert_golden("泣いてしまいました", "泣く", "v5k", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_k() {
    assert_golden("泣いてしまいませんでした", "泣く", "v5k", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_k() {
    assert_golden("泣いてしまえる", "泣く", "v5k", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_k() {
    assert_golden("泣いてしまわれる", "泣く", "v5k", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_k() {
    assert_golden("泣いてしまわせる", "泣く", "v5k", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_k() {
    assert_golden("泣いちゃう", "泣く", "v5k", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_k() {
    assert_golden("泣いちゃわない", "泣く", "v5k", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_k() {
    assert_golden("泣いちゃった", "泣く", "v5k", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_k() {
    assert_golden("泣いちゃわなかった", "泣く", "v5k", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_k() {
    assert_golden("泣いちゃって", "泣く", "v5k", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_k() {
    assert_golden("泣いちゃえば", "泣く", "v5k", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_k() {
    assert_golden("泣いちゃわなければ", "泣く", "v5k", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_k() {
    assert_golden("泣いちゃわなかったら", "泣く", "v5k", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_k() {
    assert_golden("泣いちゃおう", "泣く", "v5k", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_k() {
    assert_golden("泣いちゃえる", "泣く", "v5k", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_k() {
    assert_golden("泣いておく", "泣く", "v5k", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_k() {
    assert_golden("泣いておかない", "泣く", "v5k", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_k() {
    assert_golden("泣いておいた", "泣く", "v5k", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_k() {
    assert_golden("泣いておかなかった", "泣く", "v5k", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_k() {
    assert_golden("泣いておいて", "泣く", "v5k", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_k() {
    assert_golden("泣いておけば", "泣く", "v5k", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_k() {
    assert_golden("泣いておいたら", "泣く", "v5k", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_k() {
    assert_golden("泣いておこう", "泣く", "v5k", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_k() {
    assert_golden("泣いておける", "泣く", "v5k", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_k() {
    assert_golden("泣いておかれる", "泣く", "v5k", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_k() {
    assert_golden("泣いとく", "泣く", "v5k", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_k() {
    assert_golden("泣いとかない", "泣く", "v5k", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_k() {
    assert_golden("泣いといた", "泣く", "v5k", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_k() {
    assert_golden("泣いとかなかった", "泣く", "v5k", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_k() {
    assert_golden("泣いといて", "泣く", "v5k", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_k() {
    assert_golden("泣いとけば", "泣く", "v5k", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_k() {
    assert_golden("泣いといたら", "泣く", "v5k", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_k() {
    assert_golden("泣いとこう", "泣く", "v5k", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_k() {
    assert_golden("泣いとける", "泣く", "v5k", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_k() {
    assert_golden("泣いとかれる", "泣く", "v5k", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_k() {
    assert_golden("泣いてある", "泣く", "v5k", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_k() {
    assert_golden("泣いてあった", "泣く", "v5k", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_k() {
    assert_golden("泣いてあって", "泣く", "v5k", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_k() {
    assert_golden("泣いてあったら", "泣く", "v5k", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_k() {
    assert_golden("泣いてあれば", "泣く", "v5k", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_k() {
    assert_golden("泣いていく", "泣く", "v5k", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_k() {
    assert_golden("泣いていかない", "泣く", "v5k", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_k() {
    assert_golden("泣いていった", "泣く", "v5k", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_k() {
    assert_golden("泣いていかなかった", "泣く", "v5k", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_k() {
    assert_golden("泣いていって", "泣く", "v5k", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_k() {
    assert_golden("泣いていこう", "泣く", "v5k", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_k() {
    assert_golden("泣いていける", "泣く", "v5k", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_k() {
    assert_golden("泣いていかれる", "泣く", "v5k", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_k() {
    assert_golden("泣いていかせる", "泣く", "v5k", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_k() {
    assert_golden("泣いてくる", "泣く", "v5k", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_k() {
    assert_golden("泣いてこない", "泣く", "v5k", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_k() {
    assert_golden("泣いてきた", "泣く", "v5k", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_k() {
    assert_golden("泣いてこなかった", "泣く", "v5k", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_k() {
    assert_golden("泣いてきて", "泣く", "v5k", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_k() {
    assert_golden("泣いてくれば", "泣く", "v5k", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_k() {
    assert_golden("泣いてきたら", "泣く", "v5k", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_k() {
    assert_golden("泣いてこられる", "泣く", "v5k", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_k() {
    assert_golden("泣いてこさせる", "泣く", "v5k", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_k() {
    assert_golden("泣きながら", "泣く", "v5k", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_k() {
    assert_golden("泣きすぎる", "泣く", "v5k", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_k() {
    assert_golden("泣きそう", "泣く", "v5k", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_k() {
    assert_golden("泣かぬ", "泣く", "v5k", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_k() {
    assert_golden("泣かず", "泣く", "v5k", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_k() {
    assert_golden("泣かずに", "泣く", "v5k", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_k() {
    assert_golden("泣いたり", "泣く", "v5k", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_k() {
    assert_golden("泣かなかったり", "泣く", "v5k", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_k() {
    assert_golden("泣かん", "泣く", "v5k", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_k() {
    assert_golden("泣かんかった", "泣く", "v5k", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_k() {
    assert_golden("泣かざる", "泣く", "v5k", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_k() {
    assert_golden("泣けよう", "泣く", "v5k", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_k() {
    assert_golden("泣けよ", "泣く", "v5k", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_k() {
    assert_golden("泣けろ", "泣く", "v5k", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_k() {
    assert_golden("泣けて", "泣く", "v5k", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_k() {
    assert_golden("泣けたら", "泣く", "v5k", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_k() {
    assert_golden("泣ければ", "泣く", "v5k", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_k() {
    assert_golden("泣けられる", "泣く", "v5k", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_k() {
    assert_golden("泣けさせる", "泣く", "v5k", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_k() {
    assert_golden("泣いてあげる", "泣く", "v5k", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_k() {
    assert_golden("泣いてあげられる", "泣く", "v5k", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_k() {
    assert_golden("泣いておる", "泣く", "v5k", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_k() {
    assert_golden("泣いておらない", "泣く", "v5k", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_k() {
    assert_golden("泣いておらん", "泣く", "v5k", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_k() {
    assert_golden("泣いておった", "泣く", "v5k", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_k() {
    assert_golden("泣いておらなかった", "泣く", "v5k", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_k() {
    assert_golden("泣いております", "泣く", "v5k", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_k() {
    assert_golden("泣いておりません", "泣く", "v5k", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_k() {
    assert_golden("泣いておりました", "泣く", "v5k", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_k() {
    assert_golden("泣いておりませんでした", "泣く", "v5k", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_k() {
    assert_golden("泣いておって", "泣く", "v5k", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_k() {
    assert_golden("泣いておろう", "泣く", "v5k", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_k() {
    assert_golden("泣いておれる", "泣く", "v5k", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_k() {
    assert_golden("泣いておられる", "泣く", "v5k", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_k() {
    assert_golden("泣いとる", "泣く", "v5k", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_k() {
    assert_golden("泣いとらない", "泣く", "v5k", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_k() {
    assert_golden("泣いとらん", "泣く", "v5k", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_k() {
    assert_golden("泣いとった", "泣く", "v5k", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_k() {
    assert_golden("泣いとらなかった", "泣く", "v5k", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_k() {
    assert_golden("泣いとります", "泣く", "v5k", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_k() {
    assert_golden("泣いとりません", "泣く", "v5k", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_k() {
    assert_golden("泣いとりました", "泣く", "v5k", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_k() {
    assert_golden("泣いとりませんでした", "泣く", "v5k", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_k() {
    assert_golden("泣いとって", "泣く", "v5k", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_k() {
    assert_golden("泣いとろう", "泣く", "v5k", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_k() {
    assert_golden("泣いとれる", "泣く", "v5k", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_k() {
    assert_golden("泣いとられる", "泣く", "v5k", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_k() {
    assert_golden("泣かす", "泣く", "v5k", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_k() {
    assert_golden("泣いては", "泣く", "v5k", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_k() {
    assert_golden("泣いちゃ", "泣く", "v5k", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_k() {
    assert_golden("泣かなきゃ", "泣く", "v5k", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_k() {
    assert_golden("泣いちまう", "泣く", "v5k", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_k() {
    assert_golden("泣いちゃう", "泣く", "v5k", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_k() {
    assert_golden("泣いていらっしゃる", "泣く", "v5k", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_k() {
    assert_golden("泣いていらっしゃらない", "泣く", "v5k", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_k() {
    assert_golden("泣きつつ", "泣く", "v5k", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_k() {
    assert_golden("泣いてくれる", "泣く", "v5k", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_k() {
    assert_golden("泣いてくれない", "泣く", "v5k", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_k() {
    assert_golden("泣いてくれます", "泣く", "v5k", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_k() {
    assert_golden("泣いてくれません", "泣く", "v5k", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_k() {
    assert_golden("泣いてくれ", "泣く", "v5k", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_k() {
    assert_golden("泣かへん", "泣く", "v5k", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_k() {
    assert_golden("泣かへんかった", "泣く", "v5k", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_k() {
    assert_golden("泣かひん", "泣く", "v5k", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_k() {
    assert_golden("泣かひんかった", "泣く", "v5k", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_k() {
    assert_golden("泣かさない", "泣く", "v5k", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_k() {
    assert_golden("泣きましたら", "泣く", "v5k", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_k() {
    assert_golden("泣きになる", "泣く", "v5k", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_k() {
    assert_golden("泣きなさる", "泣く", "v5k", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_k() {
    assert_golden("泣きはる", "泣く", "v5k", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_k() {
    assert_golden("泣きなさるな", "泣く", "v5k", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_k() {
    assert_golden("泣くまい", "泣く", "v5k", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_k() {
    assert_golden("泣きますまい", "泣く", "v5k", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_k() {
    assert_golden("泣かば", "泣く", "v5k", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_k() {
    assert_golden("泣かねば", "泣く", "v5k", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_k() {
    assert_golden("泣かにゃ", "泣く", "v5k", "～colloquial negative conditional");
}

// ─── Verb-producing aux on renyou base (added 2026-05-06) ────────────

#[test]
fn deconjugate_aux_hajimeru_past_v5_k() {
    assert_golden("書き始めた", "書く", "v5k", "～start V-ing→past");
}

#[test]
fn deconjugate_aux_tsuzukeru_teiru_v5_k() {
    assert_golden("書き続けている", "書く", "v5k", "～continue V-ing→teiru");
}

#[test]
fn deconjugate_aux_te_morau_past_v5_k() {
    assert_golden("書いてもらった", "書く", "v5k", "～have someone do→past");
}
