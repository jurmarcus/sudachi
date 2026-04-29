//! Golden tests ported from JL's `DeconjugatorTestsForVSS.cs`.
//! 228 test cases proving deconjugator output matches
//! JL's expectations for class VSS.

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
fn deconjugate_masu_stem_vss() {
    assert_golden("愛し", "愛する", "vs-s", "～masu stem");
}

#[test]
fn deconjugate_masu_stem2_vss() {
    assert_golden("愛さ", "愛する", "vs-s", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_vss() {
    assert_golden("愛しない", "愛する", "vs-s", "～negative");
}

#[test]
fn deconjugate_plain_non_past_negative2_vss() {
    assert_golden("愛さない", "愛する", "vs-s", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_vss() {
    assert_golden("愛します", "愛する", "vs-s", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_vss() {
    assert_golden("愛しましょう", "愛する", "vs-s", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_vss() {
    assert_golden("愛しません", "愛する", "vs-s", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_vss() {
    assert_golden("愛した", "愛する", "vs-s", "～past");
}

#[test]
fn deconjugate_plain_past_negative_vss() {
    assert_golden("愛しなかった", "愛する", "vs-s", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_vss() {
    assert_golden("愛しました", "愛する", "vs-s", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_vss() {
    assert_golden("愛しませんでした", "愛する", "vs-s", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_vss() {
    assert_golden("愛して", "愛する", "vs-s", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_vss() {
    assert_golden("愛しなくて", "愛する", "vs-s", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_vss() {
    assert_golden("愛しないで", "愛する", "vs-s", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_vss() {
    assert_golden("愛しまして", "愛する", "vs-s", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_vss() {
    assert_golden("愛される", "愛する", "vs-s", "～passive");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_affirmative_vss() {
    assert_golden("罰せられる", "罰する", "vs-s", "～passive/potential");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_affirmative2_vss() {
    assert_golden("罰しられる", "罰する", "vs-s", "～passive/potential");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_vss() {
    assert_golden("愛せる", "愛する", "vs-s", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_vss() {
    assert_golden("愛されない", "愛する", "vs-s", "～passive→negative");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_vss() {
    assert_golden("愛された", "愛する", "vs-s", "～passive→past");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_vss() {
    assert_golden("愛せた", "愛する", "vs-s", "～want→stem; potential→past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_vss() {
    assert_golden("愛されました", "愛する", "vs-s", "～passive→polite past");
}

#[test]
fn deconjugate_plain_past_passive_negative_vss() {
    assert_golden("愛されなかった", "愛する", "vs-s", "～passive→negative→past");
}

#[test]
fn deconjugate_polite_past_passive_negative_vss() {
    assert_golden("愛されませんでした", "愛する", "vs-s", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_passive_affirmative_vss() {
    assert_golden("愛されます", "愛する", "vs-s", "～passive→polite");
}

#[test]
fn deconjugate_polite_passive_negative_vss() {
    assert_golden("愛されません", "愛する", "vs-s", "～passive→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_vss() {
    assert_golden("愛しろ", "愛する", "vs-s", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_affirmative2_vss() {
    assert_golden("愛せよ", "愛する", "vs-s", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_affirmative3_vss() {
    assert_golden("愛せ", "愛する", "vs-s", "～imperative; masu stem");
}

#[test]
fn deconjugate_plain_imperative_negative_vss() {
    assert_golden("愛するな", "愛する", "vs-s", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_vss() {
    assert_golden("愛しなさい", "愛する", "vs-s", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_vss() {
    assert_golden("愛してください", "愛する", "vs-s", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_vss() {
    assert_golden("愛しないでください", "愛する", "vs-s", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_vss() {
    assert_golden("愛しよう", "愛する", "vs-s", "～volitional");
}

#[test]
fn deconjugate_plain_volitional_affirmative2_vss() {
    assert_golden("愛そう", "愛する", "vs-s", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_vss() {
    assert_golden("愛しよ", "愛する", "vs-s", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_vss() {
    assert_golden("愛しましょう", "愛する", "vs-s", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_vss() {
    assert_golden("愛すれば", "愛する", "vs-s", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative2_vss() {
    assert_golden("愛せば", "愛する", "vs-s", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_vss() {
    assert_golden("愛しなければ", "愛する", "vs-s", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_vss() {
    assert_golden("愛したら", "愛する", "vs-s", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_vss() {
    assert_golden("愛したらば", "愛する", "vs-s", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_vss() {
    assert_golden("愛しなかったら", "愛する", "vs-s", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_vss() {
    assert_golden("愛させる", "愛する", "vs-s", "～causative");
}

#[test]
fn deconjugate_plain_causative_affirmative2_vss() {
    assert_golden("罰しさせる", "罰する", "vs-s", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_vss() {
    assert_golden("愛させない", "愛する", "vs-s", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_vss() {
    assert_golden("愛させん", "愛する", "vs-s", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_vss() {
    assert_golden("愛させます", "愛する", "vs-s", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_vss() {
    assert_golden("愛さします", "愛する", "vs-s", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_vss() {
    assert_golden("愛させません", "愛する", "vs-s", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_vss() {
    assert_golden("愛させた", "愛する", "vs-s", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_vss() {
    assert_golden("愛させなかった", "愛する", "vs-s", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_vss() {
    assert_golden("愛させました", "愛する", "vs-s", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_vss() {
    assert_golden("愛させませんでした", "愛する", "vs-s", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_vss() {
    assert_golden("愛させられる", "愛する", "vs-s", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_vss() {
    assert_golden("愛させられない", "愛する", "vs-s", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_vss() {
    assert_golden("愛させられます", "愛する", "vs-s", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_vss() {
    assert_golden("愛させられません", "愛する", "vs-s", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_vss() {
    assert_golden("愛したい", "愛する", "vs-s", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_vss() {
    assert_golden("愛したくありません", "愛する", "vs-s", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_vss() {
    assert_golden("愛したくありませんでした", "愛する", "vs-s", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_vss() {
    assert_golden("愛したくない", "愛する", "vs-s", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_vss() {
    assert_golden("愛したかった", "愛する", "vs-s", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_vss() {
    assert_golden("愛したくなかった", "愛する", "vs-s", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_vss() {
    assert_golden("愛している", "愛する", "vs-s", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_vss() {
    assert_golden("愛していない", "愛する", "vs-s", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_vss() {
    assert_golden("愛していた", "愛する", "vs-s", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_vss() {
    assert_golden("愛していなかった", "愛する", "vs-s", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_vss() {
    assert_golden("愛しています", "愛する", "vs-s", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_vss() {
    assert_golden("愛していません", "愛する", "vs-s", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_vss() {
    assert_golden("愛していました", "愛する", "vs-s", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_vss() {
    assert_golden("愛していませんでした", "愛する", "vs-s", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_vss() {
    assert_golden("愛してる", "愛する", "vs-s", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_vss() {
    assert_golden("愛してない", "愛する", "vs-s", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_vss() {
    assert_golden("愛してた", "愛する", "vs-s", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_vss() {
    assert_golden("愛してなかった", "愛する", "vs-s", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_vss() {
    assert_golden("愛してます", "愛する", "vs-s", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_vss() {
    assert_golden("愛してません", "愛する", "vs-s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_vss() {
    assert_golden("愛してました", "愛する", "vs-s", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_vss() {
    assert_golden("愛してません", "愛する", "vs-s", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_vss() {
    assert_golden("愛してませんでした", "愛する", "vs-s", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_vss() {
    assert_golden("愛してしまう", "愛する", "vs-s", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_vss() {
    assert_golden("愛してもう", "愛する", "vs-s", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_vss() {
    assert_golden("愛してしまわない", "愛する", "vs-s", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_vss() {
    assert_golden("愛してしまった", "愛する", "vs-s", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_vss() {
    assert_golden("愛してしまわなかった", "愛する", "vs-s", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_vss() {
    assert_golden("愛してしまって", "愛する", "vs-s", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_vss() {
    assert_golden("愛してしまえば", "愛する", "vs-s", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_vss() {
    assert_golden("愛してしまわなければ", "愛する", "vs-s", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_vss() {
    assert_golden("愛してしまわなかったら", "愛する", "vs-s", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_vss() {
    assert_golden("愛してしまったら", "愛する", "vs-s", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_vss() {
    assert_golden("愛してしまおう", "愛する", "vs-s", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_vss() {
    assert_golden("愛してしまいます", "愛する", "vs-s", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_vss() {
    assert_golden("愛してしまいません", "愛する", "vs-s", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_vss() {
    assert_golden("愛してしまいました", "愛する", "vs-s", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_vss() {
    assert_golden("愛してしまいませんでした", "愛する", "vs-s", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_vss() {
    assert_golden("愛してしまえる", "愛する", "vs-s", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_vss() {
    assert_golden("愛してしまわれる", "愛する", "vs-s", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_vss() {
    assert_golden("愛してしまわせる", "愛する", "vs-s", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_vss() {
    assert_golden("愛しちゃう", "愛する", "vs-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_vss() {
    assert_golden("愛しちゃわない", "愛する", "vs-s", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_vss() {
    assert_golden("愛しちゃった", "愛する", "vs-s", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_vss() {
    assert_golden("愛しちゃわなかった", "愛する", "vs-s", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_vss() {
    assert_golden("愛しちゃって", "愛する", "vs-s", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_vss() {
    assert_golden("愛しちゃえば", "愛する", "vs-s", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_vss() {
    assert_golden("愛しちゃわなければ", "愛する", "vs-s", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_vss() {
    assert_golden("愛しちゃわなかったら", "愛する", "vs-s", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_vss() {
    assert_golden("愛しちゃおう", "愛する", "vs-s", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_vss() {
    assert_golden("愛しちゃえる", "愛する", "vs-s", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_vss() {
    assert_golden("愛しておく", "愛する", "vs-s", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_vss() {
    assert_golden("愛しておかない", "愛する", "vs-s", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_vss() {
    assert_golden("愛しておいた", "愛する", "vs-s", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_vss() {
    assert_golden("愛しておかなかった", "愛する", "vs-s", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_vss() {
    assert_golden("愛しておいて", "愛する", "vs-s", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_vss() {
    assert_golden("愛しておけば", "愛する", "vs-s", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_vss() {
    assert_golden("愛しておいたら", "愛する", "vs-s", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_vss() {
    assert_golden("愛しておこう", "愛する", "vs-s", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_vss() {
    assert_golden("愛しておける", "愛する", "vs-s", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_vss() {
    assert_golden("愛しておかれる", "愛する", "vs-s", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_vss() {
    assert_golden("愛しとく", "愛する", "vs-s", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_vss() {
    assert_golden("愛しとかない", "愛する", "vs-s", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_vss() {
    assert_golden("愛しといた", "愛する", "vs-s", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_vss() {
    assert_golden("愛しとかなかった", "愛する", "vs-s", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_vss() {
    assert_golden("愛しといて", "愛する", "vs-s", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_vss() {
    assert_golden("愛しとけば", "愛する", "vs-s", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_vss() {
    assert_golden("愛しといたら", "愛する", "vs-s", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_vss() {
    assert_golden("愛しとこう", "愛する", "vs-s", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_vss() {
    assert_golden("愛しとける", "愛する", "vs-s", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_vss() {
    assert_golden("愛しとかれる", "愛する", "vs-s", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_vss() {
    assert_golden("愛してある", "愛する", "vs-s", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_vss() {
    assert_golden("愛してあった", "愛する", "vs-s", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_vss() {
    assert_golden("愛してあって", "愛する", "vs-s", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_vss() {
    assert_golden("愛してあったら", "愛する", "vs-s", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_vss() {
    assert_golden("愛してあれば", "愛する", "vs-s", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_vss() {
    assert_golden("愛していく", "愛する", "vs-s", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_vss() {
    assert_golden("愛していかない", "愛する", "vs-s", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_vss() {
    assert_golden("愛していった", "愛する", "vs-s", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_vss() {
    assert_golden("愛していかなかった", "愛する", "vs-s", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_vss() {
    assert_golden("愛していって", "愛する", "vs-s", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_vss() {
    assert_golden("愛していこう", "愛する", "vs-s", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_vss() {
    assert_golden("愛していける", "愛する", "vs-s", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_vss() {
    assert_golden("愛していかれる", "愛する", "vs-s", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_vss() {
    assert_golden("愛していかせる", "愛する", "vs-s", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_vss() {
    assert_golden("愛してくる", "愛する", "vs-s", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_vss() {
    assert_golden("愛してこない", "愛する", "vs-s", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_vss() {
    assert_golden("愛してきた", "愛する", "vs-s", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_vss() {
    assert_golden("愛してこなかった", "愛する", "vs-s", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_vss() {
    assert_golden("愛してきて", "愛する", "vs-s", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_vss() {
    assert_golden("愛してくれば", "愛する", "vs-s", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_vss() {
    assert_golden("愛してきたら", "愛する", "vs-s", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_vss() {
    assert_golden("愛してこられる", "愛する", "vs-s", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_vss() {
    assert_golden("愛してこさせる", "愛する", "vs-s", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_vss() {
    assert_golden("愛しながら", "愛する", "vs-s", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_vss() {
    assert_golden("愛しすぎる", "愛する", "vs-s", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_vss() {
    assert_golden("愛しそう", "愛する", "vs-s", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_vss() {
    assert_golden("愛せぬ", "愛する", "vs-s", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_vss() {
    assert_golden("罰せず", "罰する", "vs-s", "～adverbial negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu2_vss() {
    assert_golden("愛さず", "愛する", "vs-s", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_vss() {
    assert_golden("愛せずに", "愛する", "vs-s", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_vss() {
    assert_golden("愛したり", "愛する", "vs-s", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_vss() {
    assert_golden("愛しなかったり", "愛する", "vs-s", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_vss() {
    assert_golden("愛せん", "愛する", "vs-s", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_vss() {
    assert_golden("愛せんかった", "愛する", "vs-s", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_vss() {
    assert_golden("愛せざる", "愛する", "vs-s", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_vss() {
    assert_golden("愛せよう", "愛する", "vs-s", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_vss() {
    assert_golden("愛せて", "愛する", "vs-s", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_vss() {
    assert_golden("愛せたら", "愛する", "vs-s", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_vss() {
    assert_golden("愛せれば", "愛する", "vs-s", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_vss() {
    assert_golden("愛せさせる", "愛する", "vs-s", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_vss() {
    assert_golden("愛してあげる", "愛する", "vs-s", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_vss() {
    assert_golden("愛してあげられる", "愛する", "vs-s", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_vss() {
    assert_golden("愛しておる", "愛する", "vs-s", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_vss() {
    assert_golden("愛しておらない", "愛する", "vs-s", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_vss() {
    assert_golden("愛しておらん", "愛する", "vs-s", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_vss() {
    assert_golden("愛しておった", "愛する", "vs-s", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_vss() {
    assert_golden("愛しておらなかった", "愛する", "vs-s", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_vss() {
    assert_golden("愛しております", "愛する", "vs-s", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_vss() {
    assert_golden("愛しておりません", "愛する", "vs-s", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_vss() {
    assert_golden("愛しておりました", "愛する", "vs-s", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_vss() {
    assert_golden("愛しておりませんでした", "愛する", "vs-s", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_vss() {
    assert_golden("愛しておって", "愛する", "vs-s", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_vss() {
    assert_golden("愛しておろう", "愛する", "vs-s", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_vss() {
    assert_golden("愛しておれる", "愛する", "vs-s", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_vss() {
    assert_golden("愛しておられる", "愛する", "vs-s", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_vss() {
    assert_golden("愛しとる", "愛する", "vs-s", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_vss() {
    assert_golden("愛しとらない", "愛する", "vs-s", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_vss() {
    assert_golden("愛しとらん", "愛する", "vs-s", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_vss() {
    assert_golden("愛しとった", "愛する", "vs-s", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_vss() {
    assert_golden("愛しとらなかった", "愛する", "vs-s", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_vss() {
    assert_golden("愛しとります", "愛する", "vs-s", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_vss() {
    assert_golden("愛しとりません", "愛する", "vs-s", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_vss() {
    assert_golden("愛しとりました", "愛する", "vs-s", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_vss() {
    assert_golden("愛しとりませんでした", "愛する", "vs-s", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_vss() {
    assert_golden("愛しとって", "愛する", "vs-s", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_vss() {
    assert_golden("愛しとろう", "愛する", "vs-s", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_vss() {
    assert_golden("愛しとれる", "愛する", "vs-s", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_vss() {
    assert_golden("愛しとられる", "愛する", "vs-s", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_vss() {
    assert_golden("愛さす", "愛する", "vs-s", "～short causative");
}

#[test]
fn deconjugate_plain_short_causative_affirmative2_vss() {
    assert_golden("罰しさす", "罰する", "vs-s", "～short causative");
}

#[test]
fn deconjugate_plain_non_past_na_vss() {
    assert_golden("愛しな", "愛する", "vs-s", "～casual polite imperative");
}

#[test]
fn deconjugate_topic_or_condition_vss() {
    assert_golden("愛しては", "愛する", "vs-s", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_vss() {
    assert_golden("愛しちゃ", "愛する", "vs-s", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_vss() {
    assert_golden("愛しなきゃ", "愛する", "vs-s", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_vss() {
    assert_golden("愛しちまう", "愛する", "vs-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_vss() {
    assert_golden("愛しちゃう", "愛する", "vs-s", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_vss() {
    assert_golden("愛していらっしゃる", "愛する", "vs-s", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_vss() {
    assert_golden("愛していらっしゃらない", "愛する", "vs-s", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_vss() {
    assert_golden("愛しつつ", "愛する", "vs-s", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_vss() {
    assert_golden("愛してくれる", "愛する", "vs-s", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_vss() {
    assert_golden("愛してくれない", "愛する", "vs-s", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_vss() {
    assert_golden("愛してくれます", "愛する", "vs-s", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_vss() {
    assert_golden("愛してくれません", "愛する", "vs-s", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_vss() {
    assert_golden("愛してくれ", "愛する", "vs-s", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_vss() {
    assert_golden("愛せえへん", "愛する", "vs-s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_vss() {
    assert_golden("愛せえへんかった", "愛する", "vs-s", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_vss() {
    assert_golden("愛しひん", "愛する", "vs-s", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_vss() {
    assert_golden("愛しひんかった", "愛する", "vs-s", "～negative→ksb→past");
}

#[test]
fn deconjugate_contracted_provisional_conditional_rya_vss() {
    assert_golden("愛すりゃ", "愛する", "vs-s", "～provisional conditional→contracted");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_vss() {
    assert_golden("愛しましたら", "愛する", "vs-s", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_vss() {
    assert_golden("愛しになる", "愛する", "vs-s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_vss() {
    assert_golden("愛しなさる", "愛する", "vs-s", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_vss() {
    assert_golden("愛しはる", "愛する", "vs-s", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_vss() {
    assert_golden("愛しなさるな", "愛する", "vs-s", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_vss() {
    assert_golden("愛すまい", "愛する", "vs-s", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_vss() {
    assert_golden("愛しますまい", "愛する", "vs-s", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_vss() {
    assert_golden("愛せねば", "愛する", "vs-s", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_vss() {
    assert_golden("愛せにゃ", "愛する", "vs-s", "～colloquial negative conditional");
}
