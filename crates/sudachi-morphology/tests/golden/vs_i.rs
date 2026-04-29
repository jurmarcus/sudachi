//! Golden tests ported from JL's `DeconjugatorTestsForVSI.cs`.
//! 237 test cases proving deconjugator output matches
//! JL's expectations for class VSI.

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
fn deconjugate_masu_stem_vsi() {
    assert_golden("し", "する", "vs-i", "～masu stem");
}

#[test]
fn deconjugate_masu_stem2_vsi() {
    assert_golden("さ", "する", "vs-i", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_vsi() {
    assert_golden("しない", "する", "vs-i", "～negative");
}

#[test]
fn deconjugate_plain_non_past_negative2_vsi() {
    assert_golden("さない", "する", "vs-i", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_vsi() {
    assert_golden("します", "する", "vs-i", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_vsi() {
    assert_golden("しましょう", "する", "vs-i", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_vsi() {
    assert_golden("しません", "する", "vs-i", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_vsi() {
    assert_golden("した", "する", "vs-i", "～past");
}

#[test]
fn deconjugate_plain_past_negative_vsi() {
    assert_golden("しなかった", "する", "vs-i", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_vsi() {
    assert_golden("しました", "する", "vs-i", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_vsi() {
    assert_golden("しませんでした", "する", "vs-i", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_vsi() {
    assert_golden("して", "する", "vs-i", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_vsi() {
    assert_golden("しなくて", "する", "vs-i", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_vsi() {
    assert_golden("しないで", "する", "vs-i", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_vsi() {
    assert_golden("しまして", "する", "vs-i", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_passive_affirmative_vsi() {
    assert_golden("される", "する", "vs-i", "～passive");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative_vsi() {
    assert_golden("できる", "する", "vs-i", "～potential");
}

#[test]
fn deconjugate_plain_non_past_potential_affirmative2_vsi() {
    assert_golden("せる", "する", "vs-i", "～potential");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_vsi() {
    assert_golden("されない", "する", "vs-i", "～passive→negative");
}

#[test]
fn deconjugate_plain_non_past_potential_negative_vsi() {
    assert_golden("できない", "する", "vs-i", "～potential→negative");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_vsi() {
    assert_golden("された", "する", "vs-i", "～passive→past");
}

#[test]
fn deconjugate_plain_past_potential_affirmative_vsi() {
    assert_golden("できた", "する", "vs-i", "～potential→past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_vsi() {
    assert_golden("されました", "する", "vs-i", "～passive→polite past");
}

#[test]
fn deconjugate_polite_past_potential_affirmative2_vsi() {
    assert_golden("できました", "する", "vs-i", "～potential→polite past");
}

#[test]
fn deconjugate_plain_past_passive_negative_vsi() {
    assert_golden("されなかった", "する", "vs-i", "～passive→negative→past");
}

#[test]
fn deconjugate_plain_past_potential_negative_vsi() {
    assert_golden("できなかった", "する", "vs-i", "～potential→negative→past");
}

#[test]
fn deconjugate_polite_past_passive_negative_vsi() {
    assert_golden("されませんでした", "する", "vs-i", "～passive→polite past negative");
}

#[test]
fn deconjugate_polite_past_potential_negative2_vsi() {
    assert_golden("できませんでした", "する", "vs-i", "～potential→polite past negative");
}

#[test]
fn deconjugate_polite_passive_affirmative_vsi() {
    assert_golden("されます", "する", "vs-i", "～passive→polite");
}

#[test]
fn deconjugate_polite_potential_affirmative_vsi() {
    assert_golden("できます", "する", "vs-i", "～potential→polite");
}

#[test]
fn deconjugate_polite_passive_negative_vsi() {
    assert_golden("されません", "する", "vs-i", "～passive→polite negative");
}

#[test]
fn deconjugate_polite_potential_negative_vsi() {
    assert_golden("できません", "する", "vs-i", "～potential→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_vsi() {
    assert_golden("しろ", "する", "vs-i", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_affirmative2_vsi() {
    assert_golden("せよ", "する", "vs-i", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_affirmative3_vsi() {
    assert_golden("せ", "する", "vs-i", "～imperative; masu stem");
}

#[test]
fn deconjugate_plain_imperative_negative_vsi() {
    assert_golden("するな", "する", "vs-i", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_vsi() {
    assert_golden("しなさい", "する", "vs-i", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_vsi() {
    assert_golden("してください", "する", "vs-i", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_vsi() {
    assert_golden("しないでください", "する", "vs-i", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_vsi() {
    assert_golden("しよう", "する", "vs-i", "～volitional");
}

#[test]
fn deconjugate_plain_volitional_affirmative2_vsi() {
    assert_golden("そう", "する", "vs-i", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_vsi() {
    assert_golden("しよ", "する", "vs-i", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_vsi() {
    assert_golden("しましょう", "する", "vs-i", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_vsi() {
    assert_golden("すれば", "する", "vs-i", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative2_vsi() {
    assert_golden("せば", "する", "vs-i", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_vsi() {
    assert_golden("しなければ", "する", "vs-i", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_vsi() {
    assert_golden("したら", "する", "vs-i", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_vsi() {
    assert_golden("したらば", "する", "vs-i", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_vsi() {
    assert_golden("しなかったら", "する", "vs-i", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_vsi() {
    assert_golden("させる", "する", "vs-i", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_vsi() {
    assert_golden("させない", "する", "vs-i", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_vsi() {
    assert_golden("させん", "する", "vs-i", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_vsi() {
    assert_golden("させます", "する", "vs-i", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_vsi() {
    assert_golden("さします", "する", "vs-i", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_vsi() {
    assert_golden("させません", "する", "vs-i", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_vsi() {
    assert_golden("させた", "する", "vs-i", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_vsi() {
    assert_golden("させなかった", "する", "vs-i", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_vsi() {
    assert_golden("させました", "する", "vs-i", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_vsi() {
    assert_golden("させませんでした", "する", "vs-i", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_vsi() {
    assert_golden("させられる", "する", "vs-i", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_vsi() {
    assert_golden("させられない", "する", "vs-i", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_vsi() {
    assert_golden("させられます", "する", "vs-i", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_vsi() {
    assert_golden("させられません", "する", "vs-i", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_vsi() {
    assert_golden("したい", "する", "vs-i", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_vsi() {
    assert_golden("したくありません", "する", "vs-i", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_vsi() {
    assert_golden("したくありませんでした", "する", "vs-i", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_vsi() {
    assert_golden("したくない", "する", "vs-i", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_vsi() {
    assert_golden("したかった", "する", "vs-i", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_vsi() {
    assert_golden("したくなかった", "する", "vs-i", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_vsi() {
    assert_golden("している", "する", "vs-i", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_vsi() {
    assert_golden("していない", "する", "vs-i", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_vsi() {
    assert_golden("していた", "する", "vs-i", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_vsi() {
    assert_golden("していなかった", "する", "vs-i", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_vsi() {
    assert_golden("しています", "する", "vs-i", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_vsi() {
    assert_golden("していません", "する", "vs-i", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_vsi() {
    assert_golden("していました", "する", "vs-i", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_vsi() {
    assert_golden("していませんでした", "する", "vs-i", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_vsi() {
    assert_golden("してる", "する", "vs-i", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_vsi() {
    assert_golden("してない", "する", "vs-i", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_vsi() {
    assert_golden("してた", "する", "vs-i", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_vsi() {
    assert_golden("してなかった", "する", "vs-i", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_vsi() {
    assert_golden("してます", "する", "vs-i", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_vsi() {
    assert_golden("してません", "する", "vs-i", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_vsi() {
    assert_golden("してました", "する", "vs-i", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_vsi() {
    assert_golden("してません", "する", "vs-i", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_vsi() {
    assert_golden("してませんでした", "する", "vs-i", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_vsi() {
    assert_golden("してしまう", "する", "vs-i", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_vsi() {
    assert_golden("してもう", "する", "vs-i", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_vsi() {
    assert_golden("してしまわない", "する", "vs-i", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_vsi() {
    assert_golden("してしまった", "する", "vs-i", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_vsi() {
    assert_golden("してしまわなかった", "する", "vs-i", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_vsi() {
    assert_golden("してしまって", "する", "vs-i", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_vsi() {
    assert_golden("してしまえば", "する", "vs-i", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_vsi() {
    assert_golden("してしまわなければ", "する", "vs-i", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_vsi() {
    assert_golden("してしまわなかったら", "する", "vs-i", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_vsi() {
    assert_golden("してしまったら", "する", "vs-i", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_vsi() {
    assert_golden("してしまおう", "する", "vs-i", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_vsi() {
    assert_golden("してしまいます", "する", "vs-i", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_vsi() {
    assert_golden("してしまいません", "する", "vs-i", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_vsi() {
    assert_golden("してしまいました", "する", "vs-i", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_vsi() {
    assert_golden("してしまいませんでした", "する", "vs-i", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_vsi() {
    assert_golden("してしまえる", "する", "vs-i", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_vsi() {
    assert_golden("してしまわれる", "する", "vs-i", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_vsi() {
    assert_golden("してしまわせる", "する", "vs-i", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_vsi() {
    assert_golden("しちゃう", "する", "vs-i", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_vsi() {
    assert_golden("しちゃわない", "する", "vs-i", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_vsi() {
    assert_golden("しちゃった", "する", "vs-i", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_vsi() {
    assert_golden("しちゃわなかった", "する", "vs-i", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_vsi() {
    assert_golden("しちゃって", "する", "vs-i", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_vsi() {
    assert_golden("しちゃえば", "する", "vs-i", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_vsi() {
    assert_golden("しちゃわなければ", "する", "vs-i", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_vsi() {
    assert_golden("しちゃわなかったら", "する", "vs-i", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_vsi() {
    assert_golden("しちゃおう", "する", "vs-i", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_vsi() {
    assert_golden("しちゃえる", "する", "vs-i", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_vsi() {
    assert_golden("しておく", "する", "vs-i", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_vsi() {
    assert_golden("しておかない", "する", "vs-i", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_vsi() {
    assert_golden("しておいた", "する", "vs-i", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_vsi() {
    assert_golden("しておかなかった", "する", "vs-i", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_vsi() {
    assert_golden("しておいて", "する", "vs-i", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_vsi() {
    assert_golden("しておけば", "する", "vs-i", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_vsi() {
    assert_golden("しておいたら", "する", "vs-i", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_vsi() {
    assert_golden("しておこう", "する", "vs-i", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_vsi() {
    assert_golden("しておける", "する", "vs-i", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_vsi() {
    assert_golden("しておかれる", "する", "vs-i", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_vsi() {
    assert_golden("しとく", "する", "vs-i", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_vsi() {
    assert_golden("しとかない", "する", "vs-i", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_vsi() {
    assert_golden("しといた", "する", "vs-i", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_vsi() {
    assert_golden("しとかなかった", "する", "vs-i", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_vsi() {
    assert_golden("しといて", "する", "vs-i", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_vsi() {
    assert_golden("しとけば", "する", "vs-i", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_vsi() {
    assert_golden("しといたら", "する", "vs-i", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_vsi() {
    assert_golden("しとこう", "する", "vs-i", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_vsi() {
    assert_golden("しとける", "する", "vs-i", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_vsi() {
    assert_golden("しとかれる", "する", "vs-i", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_vsi() {
    assert_golden("してある", "する", "vs-i", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_vsi() {
    assert_golden("してあった", "する", "vs-i", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_vsi() {
    assert_golden("してあって", "する", "vs-i", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_vsi() {
    assert_golden("してあったら", "する", "vs-i", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_vsi() {
    assert_golden("してあれば", "する", "vs-i", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_vsi() {
    assert_golden("していく", "する", "vs-i", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_vsi() {
    assert_golden("していかない", "する", "vs-i", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_vsi() {
    assert_golden("していった", "する", "vs-i", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_vsi() {
    assert_golden("していかなかった", "する", "vs-i", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_vsi() {
    assert_golden("していって", "する", "vs-i", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_vsi() {
    assert_golden("していこう", "する", "vs-i", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_vsi() {
    assert_golden("していける", "する", "vs-i", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_vsi() {
    assert_golden("していかれる", "する", "vs-i", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_vsi() {
    assert_golden("していかせる", "する", "vs-i", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_vsi() {
    assert_golden("してくる", "する", "vs-i", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_vsi() {
    assert_golden("してこない", "する", "vs-i", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_vsi() {
    assert_golden("してきた", "する", "vs-i", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_vsi() {
    assert_golden("してこなかった", "する", "vs-i", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_vsi() {
    assert_golden("してきて", "する", "vs-i", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_vsi() {
    assert_golden("してくれば", "する", "vs-i", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_vsi() {
    assert_golden("してきたら", "する", "vs-i", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_vsi() {
    assert_golden("してこられる", "する", "vs-i", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_vsi() {
    assert_golden("してこさせる", "する", "vs-i", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_vsi() {
    assert_golden("しながら", "する", "vs-i", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_vsi() {
    assert_golden("しすぎる", "する", "vs-i", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_vsi() {
    assert_golden("しそう", "する", "vs-i", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_vsi() {
    assert_golden("せぬ", "する", "vs-i", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_vsi() {
    assert_golden("せず", "する", "vs-i", "～adverbial negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu2_vsi() {
    assert_golden("さず", "する", "vs-i", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_vsi() {
    assert_golden("せずに", "する", "vs-i", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_vsi() {
    assert_golden("したり", "する", "vs-i", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_vsi() {
    assert_golden("しなかったり", "する", "vs-i", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_vsi() {
    assert_golden("せん", "する", "vs-i", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_vsi() {
    assert_golden("せんかった", "する", "vs-i", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_vsi() {
    assert_golden("せざる", "する", "vs-i", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_vsi() {
    assert_golden("できよう", "する", "vs-i", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_potential_volitional_vsi() {
    assert_golden("できよ", "する", "vs-i", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_potential_imperative_vsi() {
    assert_golden("できろ", "する", "vs-i", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_vsi() {
    assert_golden("できて", "する", "vs-i", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_vsi() {
    assert_golden("できたら", "する", "vs-i", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_vsi() {
    assert_golden("できれば", "する", "vs-i", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_passive_potential_vsi() {
    assert_golden("できられる", "する", "vs-i", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_potential_causative_vsi() {
    assert_golden("できさせる", "する", "vs-i", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_vsi() {
    assert_golden("してあげる", "する", "vs-i", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_vsi() {
    assert_golden("してあげられる", "する", "vs-i", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_vsi() {
    assert_golden("しておる", "する", "vs-i", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_vsi() {
    assert_golden("しておらない", "する", "vs-i", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_vsi() {
    assert_golden("しておらん", "する", "vs-i", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_vsi() {
    assert_golden("しておった", "する", "vs-i", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_vsi() {
    assert_golden("しておらなかった", "する", "vs-i", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_vsi() {
    assert_golden("しております", "する", "vs-i", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_vsi() {
    assert_golden("しておりません", "する", "vs-i", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_vsi() {
    assert_golden("しておりました", "する", "vs-i", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_vsi() {
    assert_golden("しておりませんでした", "する", "vs-i", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_vsi() {
    assert_golden("しておって", "する", "vs-i", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_vsi() {
    assert_golden("しておろう", "する", "vs-i", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_vsi() {
    assert_golden("しておれる", "する", "vs-i", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_vsi() {
    assert_golden("しておられる", "する", "vs-i", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_vsi() {
    assert_golden("しとる", "する", "vs-i", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_vsi() {
    assert_golden("しとらない", "する", "vs-i", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_vsi() {
    assert_golden("しとらん", "する", "vs-i", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_vsi() {
    assert_golden("しとった", "する", "vs-i", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_vsi() {
    assert_golden("しとらなかった", "する", "vs-i", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_vsi() {
    assert_golden("しとります", "する", "vs-i", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_vsi() {
    assert_golden("しとりません", "する", "vs-i", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_vsi() {
    assert_golden("しとりました", "する", "vs-i", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_vsi() {
    assert_golden("しとりませんでした", "する", "vs-i", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_vsi() {
    assert_golden("しとって", "する", "vs-i", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_vsi() {
    assert_golden("しとろう", "する", "vs-i", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_vsi() {
    assert_golden("しとれる", "する", "vs-i", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_vsi() {
    assert_golden("しとられる", "する", "vs-i", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_vsi() {
    assert_golden("さす", "する", "vs-i", "～short causative");
}

#[test]
fn deconjugate_plain_non_past_na_vsi() {
    assert_golden("しな", "する", "vs-i", "～casual polite imperative");
}

#[test]
fn deconjugate_topic_or_condition_vsi() {
    assert_golden("しては", "する", "vs-i", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_vsi() {
    assert_golden("しちゃ", "する", "vs-i", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_vsi() {
    assert_golden("しなきゃ", "する", "vs-i", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_vsi() {
    assert_golden("しちまう", "する", "vs-i", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_vsi() {
    assert_golden("しちゃう", "する", "vs-i", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_vsi() {
    assert_golden("していらっしゃる", "する", "vs-i", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_vsi() {
    assert_golden("していらっしゃらない", "する", "vs-i", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_vsi() {
    assert_golden("しつつ", "する", "vs-i", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_vsi() {
    assert_golden("してくれる", "する", "vs-i", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_vsi() {
    assert_golden("してくれない", "する", "vs-i", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_vsi() {
    assert_golden("してくれます", "する", "vs-i", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_vsi() {
    assert_golden("してくれません", "する", "vs-i", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_vsi() {
    assert_golden("してくれ", "する", "vs-i", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_vsi() {
    assert_golden("せえへん", "する", "vs-i", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_vsi() {
    assert_golden("せえへんかった", "する", "vs-i", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_vsi() {
    assert_golden("しひん", "する", "vs-i", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_vsi() {
    assert_golden("しひんかった", "する", "vs-i", "～negative→ksb→past");
}

#[test]
fn deconjugate_contracted_provisional_conditional_rya_vsi() {
    assert_golden("すりゃ", "する", "vs-i", "～provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_short_causative_negative_vsi() {
    assert_golden("ささない", "する", "vs-i", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_vsi() {
    assert_golden("しましたら", "する", "vs-i", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_vsi() {
    assert_golden("しになる", "する", "vs-i", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_vsi() {
    assert_golden("なさる", "する", "vs-i", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_vsi() {
    assert_golden("しはる", "する", "vs-i", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_vsi() {
    assert_golden("なさるな", "する", "vs-i", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_vsi() {
    assert_golden("するまい", "する", "vs-i", "～negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural2_vsi() {
    assert_golden("すまい", "する", "vs-i", "～negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural3_vsi() {
    assert_golden("しまい", "する", "vs-i", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_vsi() {
    assert_golden("しますまい", "する", "vs-i", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_vsi() {
    assert_golden("せねば", "する", "vs-i", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_vsi() {
    assert_golden("せにゃ", "する", "vs-i", "～colloquial negative conditional");
}
