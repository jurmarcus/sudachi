//! Golden tests ported from JL's `DeconjugatorTestsForV5S.cs`.
//! 226 test cases proving deconjugator output matches
//! JL's expectations for class V5S.

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
fn deconjugate_masu_stem_v5_s() {
    assert_golden("壊し", "壊す", "v5s", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v5_s() {
    assert_golden("壊さない", "壊す", "v5s", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v5_s() {
    assert_golden("壊します", "壊す", "v5s", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v5_s() {
    assert_golden("壊しましょう", "壊す", "v5s", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v5_s() {
    assert_golden("壊しません", "壊す", "v5s", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v5_s() {
    assert_golden("壊した", "壊す", "v5s", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v5_s() {
    assert_golden("壊さなかった", "壊す", "v5s", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v5_s() {
    assert_golden("壊しました", "壊す", "v5s", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v5_s() {
    assert_golden("壊しませんでした", "壊す", "v5s", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v5_s() {
    assert_golden("壊して", "壊す", "v5s", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v5_s() {
    assert_golden("壊さなくて", "壊す", "v5s", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v5_s() {
    assert_golden("壊さないで", "壊す", "v5s", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v5_s() {
    assert_golden("壊しまして", "壊す", "v5s", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_v5_s() {
    assert_golden("壊せる", "壊す", "v5s", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_v5_s() {
    assert_golden("壊される", "壊す", "v5s", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_v5_s() {
    assert_golden("壊せない", "壊す", "v5s", "～potential→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_v5_s() {
    assert_golden("壊されない", "壊す", "v5s", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_v5_s() {
    assert_golden("壊せた", "壊す", "v5s", "～potential→past");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_v5_s() {
    assert_golden("壊された", "壊す", "v5s", "～passive→past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative_v5_s() {
    assert_golden("壊せました", "壊す", "v5s", "～potential→polite past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_v5_s() {
    assert_golden("壊されました", "壊す", "v5s", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_potential_negative_v5_s() {
    assert_golden("壊せなかった", "壊す", "v5s", "～potential→negative→past");
}

#[test]
fn deconjugate_plain_past_passive_negative_v5_s() {
    assert_golden("壊されなかった", "壊す", "v5s", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_potential_negative_v5_s() {
    assert_golden("壊せませんでした", "壊す", "v5s", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_past_passive_negative_v5_s() {
    assert_golden("壊されませんでした", "壊す", "v5s", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_potential_affirmative_v5_s() {
    assert_golden("壊せます", "壊す", "v5s", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_affirmative_v5_s() {
    assert_golden("壊されます", "壊す", "v5s", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_negative_v5_s() {
    assert_golden("壊せません", "壊す", "v5s", "～potential→polite negative");
}

#[test]
fn deconjugate_polite_passive_negative_v5_s() {
    assert_golden("壊されません", "壊す", "v5s", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v5_s() {
    assert_golden("壊せ", "壊す", "v5s", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v5_s() {
    assert_golden("壊すな", "壊す", "v5s", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v5_s() {
    assert_golden("壊しなさい", "壊す", "v5s", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v5_s() {
    assert_golden("壊してください", "壊す", "v5s", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v5_s() {
    assert_golden("壊さないでください", "壊す", "v5s", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v5_s() {
    assert_golden("壊そう", "壊す", "v5s", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v5_s() {
    assert_golden("壊そ", "壊す", "v5s", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v5_s() {
    assert_golden("壊しましょう", "壊す", "v5s", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v5_s() {
    assert_golden("壊せば", "壊す", "v5s", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v5_s() {
    assert_golden("壊さなければ", "壊す", "v5s", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v5_s() {
    assert_golden("壊したら", "壊す", "v5s", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v5_s() {
    assert_golden("壊したらば", "壊す", "v5s", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v5_s() {
    assert_golden("壊さなかったら", "壊す", "v5s", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v5_s() {
    assert_golden("壊させる", "壊す", "v5s", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v5_s() {
    assert_golden("壊させない", "壊す", "v5s", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v5_s() {
    assert_golden("壊させん", "壊す", "v5s", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v5_s() {
    assert_golden("壊させます", "壊す", "v5s", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v5_s() {
    assert_golden("壊さします", "壊す", "v5s", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v5_s() {
    assert_golden("壊させません", "壊す", "v5s", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v5_s() {
    assert_golden("壊させた", "壊す", "v5s", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v5_s() {
    assert_golden("壊させなかった", "壊す", "v5s", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v5_s() {
    assert_golden("壊させました", "壊す", "v5s", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v5_s() {
    assert_golden("壊させませんでした", "壊す", "v5s", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v5_s() {
    assert_golden("壊させられる", "壊す", "v5s", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v5_s() {
    assert_golden("壊させられない", "壊す", "v5s", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v5_s() {
    assert_golden("壊させられます", "壊す", "v5s", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v5_s() {
    assert_golden("壊させられません", "壊す", "v5s", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v5_s() {
    assert_golden("壊したい", "壊す", "v5s", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v5_s() {
    assert_golden("壊したくありません", "壊す", "v5s", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v5_s() {
    assert_golden("壊したくありませんでした", "壊す", "v5s", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v5_s() {
    assert_golden("壊したくない", "壊す", "v5s", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v5_s() {
    assert_golden("壊したかった", "壊す", "v5s", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v5_s() {
    assert_golden("壊したくなかった", "壊す", "v5s", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v5_s() {
    assert_golden("壊している", "壊す", "v5s", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v5_s() {
    assert_golden("壊していない", "壊す", "v5s", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v5_s() {
    assert_golden("壊していた", "壊す", "v5s", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v5_s() {
    assert_golden("壊していなかった", "壊す", "v5s", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v5_s() {
    assert_golden("壊しています", "壊す", "v5s", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v5_s() {
    assert_golden("壊していません", "壊す", "v5s", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v5_s() {
    assert_golden("壊していました", "壊す", "v5s", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v5_s() {
    assert_golden("壊していませんでした", "壊す", "v5s", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v5_s() {
    assert_golden("壊してる", "壊す", "v5s", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v5_s() {
    assert_golden("壊してない", "壊す", "v5s", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v5_s() {
    assert_golden("壊してた", "壊す", "v5s", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v5_s() {
    assert_golden("壊してなかった", "壊す", "v5s", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v5_s() {
    assert_golden("壊してます", "壊す", "v5s", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v5_s() {
    assert_golden("壊してません", "壊す", "v5s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v5_s() {
    assert_golden("壊してました", "壊す", "v5s", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v5_s() {
    assert_golden("壊してません", "壊す", "v5s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v5_s() {
    assert_golden("壊してませんでした", "壊す", "v5s", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v5_s() {
    assert_golden("壊してしまう", "壊す", "v5s", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v5_s() {
    assert_golden("壊してもう", "壊す", "v5s", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v5_s() {
    assert_golden("壊してしまわない", "壊す", "v5s", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v5_s() {
    assert_golden("壊してしまった", "壊す", "v5s", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v5_s() {
    assert_golden("壊してしまわなかった", "壊す", "v5s", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v5_s() {
    assert_golden("壊してしまって", "壊す", "v5s", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v5_s() {
    assert_golden("壊してしまえば", "壊す", "v5s", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v5_s() {
    assert_golden("壊してしまわなければ", "壊す", "v5s", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v5_s() {
    assert_golden("壊してしまわなかったら", "壊す", "v5s", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v5_s() {
    assert_golden("壊してしまったら", "壊す", "v5s", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v5_s() {
    assert_golden("壊してしまおう", "壊す", "v5s", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v5_s() {
    assert_golden("壊してしまいます", "壊す", "v5s", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v5_s() {
    assert_golden("壊してしまいません", "壊す", "v5s", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v5_s() {
    assert_golden("壊してしまいました", "壊す", "v5s", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v5_s() {
    assert_golden("壊してしまいませんでした", "壊す", "v5s", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v5_s() {
    assert_golden("壊してしまえる", "壊す", "v5s", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v5_s() {
    assert_golden("壊してしまわれる", "壊す", "v5s", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v5_s() {
    assert_golden("壊してしまわせる", "壊す", "v5s", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v5_s() {
    assert_golden("壊しちゃう", "壊す", "v5s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v5_s() {
    assert_golden("壊しちゃわない", "壊す", "v5s", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v5_s() {
    assert_golden("壊しちゃった", "壊す", "v5s", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v5_s() {
    assert_golden("壊しちゃわなかった", "壊す", "v5s", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v5_s() {
    assert_golden("壊しちゃって", "壊す", "v5s", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v5_s() {
    assert_golden("壊しちゃえば", "壊す", "v5s", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v5_s() {
    assert_golden("壊しちゃわなければ", "壊す", "v5s", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v5_s() {
    assert_golden("壊しちゃわなかったら", "壊す", "v5s", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v5_s() {
    assert_golden("壊しちゃおう", "壊す", "v5s", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v5_s() {
    assert_golden("壊しちゃえる", "壊す", "v5s", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v5_s() {
    assert_golden("壊しておく", "壊す", "v5s", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v5_s() {
    assert_golden("壊しておかない", "壊す", "v5s", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v5_s() {
    assert_golden("壊しておいた", "壊す", "v5s", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v5_s() {
    assert_golden("壊しておかなかった", "壊す", "v5s", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v5_s() {
    assert_golden("壊しておいて", "壊す", "v5s", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v5_s() {
    assert_golden("壊しておけば", "壊す", "v5s", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v5_s() {
    assert_golden("壊しておいたら", "壊す", "v5s", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v5_s() {
    assert_golden("壊しておこう", "壊す", "v5s", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v5_s() {
    assert_golden("壊しておける", "壊す", "v5s", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v5_s() {
    assert_golden("壊しておかれる", "壊す", "v5s", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v5_s() {
    assert_golden("壊しとく", "壊す", "v5s", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v5_s() {
    assert_golden("壊しとかない", "壊す", "v5s", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v5_s() {
    assert_golden("壊しといた", "壊す", "v5s", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v5_s() {
    assert_golden("壊しとかなかった", "壊す", "v5s", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v5_s() {
    assert_golden("壊しといて", "壊す", "v5s", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v5_s() {
    assert_golden("壊しとけば", "壊す", "v5s", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v5_s() {
    assert_golden("壊しといたら", "壊す", "v5s", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v5_s() {
    assert_golden("壊しとこう", "壊す", "v5s", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v5_s() {
    assert_golden("壊しとける", "壊す", "v5s", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v5_s() {
    assert_golden("壊しとかれる", "壊す", "v5s", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v5_s() {
    assert_golden("壊してある", "壊す", "v5s", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v5_s() {
    assert_golden("壊してあった", "壊す", "v5s", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v5_s() {
    assert_golden("壊してあって", "壊す", "v5s", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v5_s() {
    assert_golden("壊してあったら", "壊す", "v5s", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v5_s() {
    assert_golden("壊してあれば", "壊す", "v5s", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v5_s() {
    assert_golden("壊していく", "壊す", "v5s", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v5_s() {
    assert_golden("壊していかない", "壊す", "v5s", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v5_s() {
    assert_golden("壊していった", "壊す", "v5s", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v5_s() {
    assert_golden("壊していかなかった", "壊す", "v5s", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v5_s() {
    assert_golden("壊していって", "壊す", "v5s", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v5_s() {
    assert_golden("壊していこう", "壊す", "v5s", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v5_s() {
    assert_golden("壊していける", "壊す", "v5s", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v5_s() {
    assert_golden("壊していかれる", "壊す", "v5s", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v5_s() {
    assert_golden("壊していかせる", "壊す", "v5s", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v5_s() {
    assert_golden("壊してくる", "壊す", "v5s", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v5_s() {
    assert_golden("壊してこない", "壊す", "v5s", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v5_s() {
    assert_golden("壊してきた", "壊す", "v5s", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v5_s() {
    assert_golden("壊してこなかった", "壊す", "v5s", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v5_s() {
    assert_golden("壊してきて", "壊す", "v5s", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v5_s() {
    assert_golden("壊してくれば", "壊す", "v5s", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v5_s() {
    assert_golden("壊してきたら", "壊す", "v5s", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v5_s() {
    assert_golden("壊してこられる", "壊す", "v5s", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v5_s() {
    assert_golden("壊してこさせる", "壊す", "v5s", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v5_s() {
    assert_golden("壊しながら", "壊す", "v5s", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v5_s() {
    assert_golden("壊しすぎる", "壊す", "v5s", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v5_s() {
    assert_golden("壊しそう", "壊す", "v5s", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v5_s() {
    assert_golden("壊さぬ", "壊す", "v5s", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v5_s() {
    assert_golden("壊さず", "壊す", "v5s", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v5_s() {
    assert_golden("壊さずに", "壊す", "v5s", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v5_s() {
    assert_golden("壊したり", "壊す", "v5s", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v5_s() {
    assert_golden("壊さなかったり", "壊す", "v5s", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v5_s() {
    assert_golden("壊さん", "壊す", "v5s", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v5_s() {
    assert_golden("壊さんかった", "壊す", "v5s", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v5_s() {
    assert_golden("壊さざる", "壊す", "v5s", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_v5_s() {
    assert_golden("壊せよう", "壊す", "v5s", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_v5_s() {
    assert_golden("壊せよ", "壊す", "v5s", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_v5_s() {
    assert_golden("壊せろ", "壊す", "v5s", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_v5_s() {
    assert_golden("壊せて", "壊す", "v5s", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_v5_s() {
    assert_golden("壊せたら", "壊す", "v5s", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_v5_s() {
    assert_golden("壊せれば", "壊す", "v5s", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_v5_s() {
    assert_golden("壊せられる", "壊す", "v5s", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_v5_s() {
    assert_golden("壊せさせる", "壊す", "v5s", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v5_s() {
    assert_golden("壊してあげる", "壊す", "v5s", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v5_s() {
    assert_golden("壊してあげられる", "壊す", "v5s", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v5_s() {
    assert_golden("壊しておる", "壊す", "v5s", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v5_s() {
    assert_golden("壊しておらない", "壊す", "v5s", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v5_s() {
    assert_golden("壊しておらん", "壊す", "v5s", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v5_s() {
    assert_golden("壊しておった", "壊す", "v5s", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v5_s() {
    assert_golden("壊しておらなかった", "壊す", "v5s", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v5_s() {
    assert_golden("壊しております", "壊す", "v5s", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v5_s() {
    assert_golden("壊しておりません", "壊す", "v5s", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v5_s() {
    assert_golden("壊しておりました", "壊す", "v5s", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v5_s() {
    assert_golden("壊しておりませんでした", "壊す", "v5s", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v5_s() {
    assert_golden("壊しておって", "壊す", "v5s", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v5_s() {
    assert_golden("壊しておろう", "壊す", "v5s", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v5_s() {
    assert_golden("壊しておれる", "壊す", "v5s", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v5_s() {
    assert_golden("壊しておられる", "壊す", "v5s", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v5_s() {
    assert_golden("壊しとる", "壊す", "v5s", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v5_s() {
    assert_golden("壊しとらない", "壊す", "v5s", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v5_s() {
    assert_golden("壊しとらん", "壊す", "v5s", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v5_s() {
    assert_golden("壊しとった", "壊す", "v5s", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v5_s() {
    assert_golden("壊しとらなかった", "壊す", "v5s", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v5_s() {
    assert_golden("壊しとります", "壊す", "v5s", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v5_s() {
    assert_golden("壊しとりません", "壊す", "v5s", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v5_s() {
    assert_golden("壊しとりました", "壊す", "v5s", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v5_s() {
    assert_golden("壊しとりませんでした", "壊す", "v5s", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v5_s() {
    assert_golden("壊しとって", "壊す", "v5s", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v5_s() {
    assert_golden("壊しとろう", "壊す", "v5s", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v5_s() {
    assert_golden("壊しとれる", "壊す", "v5s", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v5_s() {
    assert_golden("壊しとられる", "壊す", "v5s", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v5_s() {
    assert_golden("壊さす", "壊す", "v5s", "～short causative");
}

#[test]
fn deconjugate_topic_or_condition_v5_s() {
    assert_golden("壊しては", "壊す", "v5s", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v5_s() {
    assert_golden("壊しちゃ", "壊す", "v5s", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v5_s() {
    assert_golden("壊さなきゃ", "壊す", "v5s", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v5_s() {
    assert_golden("壊しちまう", "壊す", "v5s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v5_s() {
    assert_golden("壊しちゃう", "壊す", "v5s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v5_s() {
    assert_golden("壊していらっしゃる", "壊す", "v5s", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v5_s() {
    assert_golden("壊していらっしゃらない", "壊す", "v5s", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v5_s() {
    assert_golden("壊しつつ", "壊す", "v5s", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v5_s() {
    assert_golden("壊してくれる", "壊す", "v5s", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v5_s() {
    assert_golden("壊してくれない", "壊す", "v5s", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v5_s() {
    assert_golden("壊してくれます", "壊す", "v5s", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v5_s() {
    assert_golden("壊してくれません", "壊す", "v5s", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v5_s() {
    assert_golden("壊してくれ", "壊す", "v5s", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v5_s() {
    assert_golden("壊さへん", "壊す", "v5s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v5_s() {
    assert_golden("壊さへんかった", "壊す", "v5s", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v5_s() {
    assert_golden("壊さひん", "壊す", "v5s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v5_s() {
    assert_golden("壊さひんかった", "壊す", "v5s", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v5_s() {
    assert_golden("壊ささない", "壊す", "v5s", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v5_s() {
    assert_golden("壊しましたら", "壊す", "v5s", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v5_s() {
    assert_golden("壊しになる", "壊す", "v5s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v5_s() {
    assert_golden("壊しなさる", "壊す", "v5s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v5_s() {
    assert_golden("壊しはる", "壊す", "v5s", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v5_s() {
    assert_golden("壊しなさるな", "壊す", "v5s", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v5_s() {
    assert_golden("壊すまい", "壊す", "v5s", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v5_s() {
    assert_golden("壊しますまい", "壊す", "v5s", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_classical_hypothetical_conditional_v5_s() {
    assert_golden("壊さば", "壊す", "v5s", "～classical hypothetical conditional");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v5_s() {
    assert_golden("壊さねば", "壊す", "v5s", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v5_s() {
    assert_golden("壊さにゃ", "壊す", "v5s", "～colloquial negative conditional");
}
