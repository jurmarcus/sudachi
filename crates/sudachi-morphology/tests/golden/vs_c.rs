//! Golden tests ported from JL's `DeconjugatorTestsForVSC.cs`.
//! 210 test cases proving deconjugator output matches
//! JL's expectations for class VSC.

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
fn deconjugate_masu_stem_vsc() {
    assert_golden("御座し", "御座す", "vs-c", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_vsc() {
    assert_golden("御座せない", "御座す", "vs-c", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_vsc() {
    assert_golden("御座します", "御座す", "vs-c", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_vsc() {
    assert_golden("御座しましょう", "御座す", "vs-c", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_vsc() {
    assert_golden("御座しません", "御座す", "vs-c", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_vsc() {
    assert_golden("御座した", "御座す", "vs-c", "～past");
}

#[test]
fn deconjugate_plain_past_negative_vsc() {
    assert_golden("御座せなかった", "御座す", "vs-c", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_vsc() {
    assert_golden("御座しました", "御座す", "vs-c", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_vsc() {
    assert_golden("御座しませんでした", "御座す", "vs-c", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_vsc() {
    assert_golden("御座して", "御座す", "vs-c", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_vsc() {
    assert_golden("御座せなくて", "御座す", "vs-c", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_vsc() {
    assert_golden("御座しまして", "御座す", "vs-c", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_affirmative_vsc() {
    assert_golden("御座せられる", "御座す", "vs-c", "～passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_passive_negative_vsc() {
    assert_golden("御座せられない", "御座す", "vs-c", "～passive/potential/honorific→negative");
}

#[test]
fn deconjugate_plain_past_passive_affirmative_vsc() {
    assert_golden("御座せられた", "御座す", "vs-c", "～passive/potential/honorific→past");
}

#[test]
fn deconjugate_polite_past_passive_affirmative_vsc() {
    assert_golden("御座せられました", "御座す", "vs-c", "～passive/potential/honorific→polite past");
}

#[test]
fn deconjugate_plain_past_passive_negative_vsc() {
    assert_golden("御座せられなかった", "御座す", "vs-c", "～passive/potential/honorific→negative→past");
}

#[test]
fn deconjugate_polite_past_passive_negative_vsc() {
    assert_golden("御座せられませんでした", "御座す", "vs-c", "～passive/potential/honorific→polite past negative");
}

#[test]
fn deconjugate_polite_passive_affirmative_vsc() {
    assert_golden("御座せられます", "御座す", "vs-c", "～passive/potential/honorific→polite");
}

#[test]
fn deconjugate_polite_passive_negative_vsc() {
    assert_golden("御座せられません", "御座す", "vs-c", "～passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_vsc() {
    assert_golden("御座せよ", "御座す", "vs-c", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_affirmative2_vsc() {
    assert_golden("御座せよ", "御座す", "vs-c", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_vsc() {
    assert_golden("御座すな", "御座す", "vs-c", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_vsc() {
    assert_golden("御座しなさい", "御座す", "vs-c", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_vsc() {
    assert_golden("御座してください", "御座す", "vs-c", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_vsc() {
    assert_golden("御座せないでください", "御座す", "vs-c", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_vsc() {
    assert_golden("御座せむ", "御座す", "vs-c", "～volitional");
}

#[test]
fn deconjugate_polite_volitional_affirmative_vsc() {
    assert_golden("御座しましょう", "御座す", "vs-c", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_vsc() {
    assert_golden("御座すれば", "御座す", "vs-c", "～provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_vsc() {
    assert_golden("御座したら", "御座す", "vs-c", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_vsc() {
    assert_golden("御座したらば", "御座す", "vs-c", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_vsc() {
    assert_golden("御座せなかったら", "御座す", "vs-c", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_vsc() {
    assert_golden("御座せさす", "御座す", "vs-c", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_vsc() {
    assert_golden("御座せさせない", "御座す", "vs-c", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_vsc() {
    assert_golden("御座せさせん", "御座す", "vs-c", "～causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_vsc() {
    assert_golden("御座せさします", "御座す", "vs-c", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_vsc() {
    assert_golden("御座さします", "御座す", "vs-c", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_vsc() {
    assert_golden("御座せさしません", "御座す", "vs-c", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_vsc() {
    assert_golden("御座せさした", "御座す", "vs-c", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_vsc() {
    assert_golden("御座せさせなかった", "御座す", "vs-c", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_vsc() {
    assert_golden("御座せさしました", "御座す", "vs-c", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_vsc() {
    assert_golden("御座せさしませんでした", "御座す", "vs-c", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_vsc() {
    assert_golden("御座せさせられる", "御座す", "vs-c", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_vsc() {
    assert_golden("御座せさせられない", "御座す", "vs-c", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_vsc() {
    assert_golden("御座せさせられます", "御座す", "vs-c", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_vsc() {
    assert_golden("御座せさせられません", "御座す", "vs-c", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_vsc() {
    assert_golden("御座したい", "御座す", "vs-c", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_vsc() {
    assert_golden("御座したくありません", "御座す", "vs-c", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_vsc() {
    assert_golden("御座したくありませんでした", "御座す", "vs-c", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_vsc() {
    assert_golden("御座したくない", "御座す", "vs-c", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_vsc() {
    assert_golden("御座したかった", "御座す", "vs-c", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_vsc() {
    assert_golden("御座したくなかった", "御座す", "vs-c", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_vsc() {
    assert_golden("御座している", "御座す", "vs-c", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_vsc() {
    assert_golden("御座していない", "御座す", "vs-c", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_vsc() {
    assert_golden("御座していた", "御座す", "vs-c", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_vsc() {
    assert_golden("御座していなかった", "御座す", "vs-c", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_vsc() {
    assert_golden("御座しています", "御座す", "vs-c", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_vsc() {
    assert_golden("御座していません", "御座す", "vs-c", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_vsc() {
    assert_golden("御座していました", "御座す", "vs-c", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_vsc() {
    assert_golden("御座していませんでした", "御座す", "vs-c", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_vsc() {
    assert_golden("御座してる", "御座す", "vs-c", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_vsc() {
    assert_golden("御座してない", "御座す", "vs-c", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_vsc() {
    assert_golden("御座してた", "御座す", "vs-c", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_vsc() {
    assert_golden("御座してなかった", "御座す", "vs-c", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_vsc() {
    assert_golden("御座してます", "御座す", "vs-c", "～teru→polite");
}

#[test]
fn deconjugate_polite_past_teru_vsc() {
    assert_golden("御座してました", "御座す", "vs-c", "～teru→polite past");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_vsc() {
    assert_golden("御座してません", "御座す", "vs-c", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative_vsc() {
    assert_golden("御座してませんでした", "御座す", "vs-c", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_vsc() {
    assert_golden("御座してしまう", "御座す", "vs-c", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_vsc() {
    assert_golden("御座してもう", "御座す", "vs-c", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_vsc() {
    assert_golden("御座してしまわない", "御座す", "vs-c", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_vsc() {
    assert_golden("御座してしまった", "御座す", "vs-c", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_vsc() {
    assert_golden("御座してしまわなかった", "御座す", "vs-c", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_vsc() {
    assert_golden("御座してしまって", "御座す", "vs-c", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_vsc() {
    assert_golden("御座してしまえば", "御座す", "vs-c", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_vsc() {
    assert_golden("御座してしまわなければ", "御座す", "vs-c", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_vsc() {
    assert_golden("御座してしまわなかったら", "御座す", "vs-c", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_vsc() {
    assert_golden("御座してしまったら", "御座す", "vs-c", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_vsc() {
    assert_golden("御座してしまおう", "御座す", "vs-c", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_vsc() {
    assert_golden("御座してしまいます", "御座す", "vs-c", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_vsc() {
    assert_golden("御座してしまいません", "御座す", "vs-c", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_vsc() {
    assert_golden("御座してしまいました", "御座す", "vs-c", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_vsc() {
    assert_golden("御座してしまいませんでした", "御座す", "vs-c", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_vsc() {
    assert_golden("御座してしまえる", "御座す", "vs-c", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_vsc() {
    assert_golden("御座してしまわれる", "御座す", "vs-c", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_vsc() {
    assert_golden("御座してしまわせる", "御座す", "vs-c", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_vsc() {
    assert_golden("御座しちゃう", "御座す", "vs-c", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_vsc() {
    assert_golden("御座しちゃわない", "御座す", "vs-c", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_vsc() {
    assert_golden("御座しちゃった", "御座す", "vs-c", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_vsc() {
    assert_golden("御座しちゃわなかった", "御座す", "vs-c", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_vsc() {
    assert_golden("御座しちゃって", "御座す", "vs-c", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_vsc() {
    assert_golden("御座しちゃえば", "御座す", "vs-c", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_vsc() {
    assert_golden("御座しちゃわなければ", "御座す", "vs-c", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_vsc() {
    assert_golden("御座しちゃわなかったら", "御座す", "vs-c", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_vsc() {
    assert_golden("御座しちゃおう", "御座す", "vs-c", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_vsc() {
    assert_golden("御座しちゃえる", "御座す", "vs-c", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_vsc() {
    assert_golden("御座しておく", "御座す", "vs-c", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_vsc() {
    assert_golden("御座しておかない", "御座す", "vs-c", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_vsc() {
    assert_golden("御座しておいた", "御座す", "vs-c", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_vsc() {
    assert_golden("御座しておかなかった", "御座す", "vs-c", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_vsc() {
    assert_golden("御座しておいて", "御座す", "vs-c", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_vsc() {
    assert_golden("御座しておけば", "御座す", "vs-c", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_vsc() {
    assert_golden("御座しておいたら", "御座す", "vs-c", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_vsc() {
    assert_golden("御座しておこう", "御座す", "vs-c", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_vsc() {
    assert_golden("御座しておける", "御座す", "vs-c", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_vsc() {
    assert_golden("御座しておかれる", "御座す", "vs-c", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_vsc() {
    assert_golden("御座しとく", "御座す", "vs-c", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_vsc() {
    assert_golden("御座しとかない", "御座す", "vs-c", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_vsc() {
    assert_golden("御座しといた", "御座す", "vs-c", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_vsc() {
    assert_golden("御座しとかなかった", "御座す", "vs-c", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_vsc() {
    assert_golden("御座しといて", "御座す", "vs-c", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_vsc() {
    assert_golden("御座しとけば", "御座す", "vs-c", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_vsc() {
    assert_golden("御座しといたら", "御座す", "vs-c", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_vsc() {
    assert_golden("御座しとこう", "御座す", "vs-c", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_vsc() {
    assert_golden("御座しとける", "御座す", "vs-c", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_vsc() {
    assert_golden("御座しとかれる", "御座す", "vs-c", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_vsc() {
    assert_golden("御座してある", "御座す", "vs-c", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_vsc() {
    assert_golden("御座してあった", "御座す", "vs-c", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_vsc() {
    assert_golden("御座してあって", "御座す", "vs-c", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_vsc() {
    assert_golden("御座してあったら", "御座す", "vs-c", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_vsc() {
    assert_golden("御座してあれば", "御座す", "vs-c", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_vsc() {
    assert_golden("御座していく", "御座す", "vs-c", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_vsc() {
    assert_golden("御座していかない", "御座す", "vs-c", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_vsc() {
    assert_golden("御座していった", "御座す", "vs-c", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_vsc() {
    assert_golden("御座していかなかった", "御座す", "vs-c", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_vsc() {
    assert_golden("御座していって", "御座す", "vs-c", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_vsc() {
    assert_golden("御座していこう", "御座す", "vs-c", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_vsc() {
    assert_golden("御座していける", "御座す", "vs-c", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_vsc() {
    assert_golden("御座していかれる", "御座す", "vs-c", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_vsc() {
    assert_golden("御座していかせる", "御座す", "vs-c", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_vsc() {
    assert_golden("御座してくる", "御座す", "vs-c", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_vsc() {
    assert_golden("御座してこない", "御座す", "vs-c", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_vsc() {
    assert_golden("御座してきた", "御座す", "vs-c", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_vsc() {
    assert_golden("御座してこなかった", "御座す", "vs-c", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_vsc() {
    assert_golden("御座してきて", "御座す", "vs-c", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_vsc() {
    assert_golden("御座してくれば", "御座す", "vs-c", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_vsc() {
    assert_golden("御座してきたら", "御座す", "vs-c", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_vsc() {
    assert_golden("御座してこられる", "御座す", "vs-c", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_vsc() {
    assert_golden("御座してこさせる", "御座す", "vs-c", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_vsc() {
    assert_golden("御座しながら", "御座す", "vs-c", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_vsc() {
    assert_golden("御座しすぎる", "御座す", "vs-c", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_vsc() {
    assert_golden("御座しそう", "御座す", "vs-c", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_vsc() {
    assert_golden("御座せぬ", "御座す", "vs-c", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_vsc() {
    assert_golden("御座せず", "御座す", "vs-c", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_vsc() {
    assert_golden("御座せずに", "御座す", "vs-c", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_vsc() {
    assert_golden("御座したり", "御座す", "vs-c", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_vsc() {
    assert_golden("御座せなかったり", "御座す", "vs-c", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_vsc() {
    assert_golden("御座せん", "御座す", "vs-c", "～slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_vsc() {
    assert_golden("御座せんかった", "御座す", "vs-c", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_vsc() {
    assert_golden("御座せざる", "御座す", "vs-c", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_potential_volitional_vsc() {
    assert_golden("御座せられよう", "御座す", "vs-c", "～passive/potential/honorific→volitional");
}

#[test]
fn deconjugate_plain_non_past_potential_te_form_vsc() {
    assert_golden("御座せられて", "御座す", "vs-c", "～passive/potential/honorific→te");
}

#[test]
fn deconjugate_plain_non_past_potential_temporal_conditional_vsc() {
    assert_golden("御座せられたら", "御座す", "vs-c", "～passive/potential/honorific→conditional");
}

#[test]
fn deconjugate_plain_non_past_potential_provisional_conditional_vsc() {
    assert_golden("御座せられれば", "御座す", "vs-c", "～passive/potential/honorific→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_vsc() {
    assert_golden("御座してあげる", "御座す", "vs-c", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_vsc() {
    assert_golden("御座してあげられる", "御座す", "vs-c", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_vsc() {
    assert_golden("御座しておる", "御座す", "vs-c", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_vsc() {
    assert_golden("御座しておらない", "御座す", "vs-c", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_vsc() {
    assert_golden("御座しておらん", "御座す", "vs-c", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_vsc() {
    assert_golden("御座しておった", "御座す", "vs-c", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_vsc() {
    assert_golden("御座しておらなかった", "御座す", "vs-c", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_vsc() {
    assert_golden("御座しております", "御座す", "vs-c", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_vsc() {
    assert_golden("御座しておりません", "御座す", "vs-c", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_vsc() {
    assert_golden("御座しておりました", "御座す", "vs-c", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_vsc() {
    assert_golden("御座しておりませんでした", "御座す", "vs-c", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_vsc() {
    assert_golden("御座しておって", "御座す", "vs-c", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_vsc() {
    assert_golden("御座しておろう", "御座す", "vs-c", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_vsc() {
    assert_golden("御座しておれる", "御座す", "vs-c", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_vsc() {
    assert_golden("御座しておられる", "御座す", "vs-c", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_vsc() {
    assert_golden("御座しとる", "御座す", "vs-c", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_vsc() {
    assert_golden("御座しとらない", "御座す", "vs-c", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_vsc() {
    assert_golden("御座しとらん", "御座す", "vs-c", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_vsc() {
    assert_golden("御座しとった", "御座す", "vs-c", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_vsc() {
    assert_golden("御座しとらなかった", "御座す", "vs-c", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_vsc() {
    assert_golden("御座しとります", "御座す", "vs-c", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_vsc() {
    assert_golden("御座しとりません", "御座す", "vs-c", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_vsc() {
    assert_golden("御座しとりました", "御座す", "vs-c", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_vsc() {
    assert_golden("御座しとりませんでした", "御座す", "vs-c", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_vsc() {
    assert_golden("御座しとって", "御座す", "vs-c", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_vsc() {
    assert_golden("御座しとろう", "御座す", "vs-c", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_vsc() {
    assert_golden("御座しとれる", "御座す", "vs-c", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_vsc() {
    assert_golden("御座しとられる", "御座す", "vs-c", "～toru→passive");
}

#[test]
fn deconjugate_plain_non_past_na_vsc() {
    assert_golden("御座しな", "御座す", "vs-c", "～casual polite imperative");
}

#[test]
fn deconjugate_topic_or_condition_vsc() {
    assert_golden("御座しては", "御座す", "vs-c", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_vsc() {
    assert_golden("御座しちゃ", "御座す", "vs-c", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_vsc() {
    assert_golden("御座せなきゃ", "御座す", "vs-c", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_vsc() {
    assert_golden("御座しちまう", "御座す", "vs-c", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_vsc() {
    assert_golden("御座しちゃう", "御座す", "vs-c", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_vsc() {
    assert_golden("御座していらっしゃる", "御座す", "vs-c", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_vsc() {
    assert_golden("御座していらっしゃらない", "御座す", "vs-c", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_vsc() {
    assert_golden("御座しつつ", "御座す", "vs-c", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_vsc() {
    assert_golden("御座してくれる", "御座す", "vs-c", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_vsc() {
    assert_golden("御座してくれない", "御座す", "vs-c", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_vsc() {
    assert_golden("御座してくれます", "御座す", "vs-c", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_vsc() {
    assert_golden("御座してくれません", "御座す", "vs-c", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_vsc() {
    assert_golden("御座してくれ", "御座す", "vs-c", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_vsc() {
    assert_golden("御座せへん", "御座す", "vs-c", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_vsc() {
    assert_golden("御座せへんかった", "御座す", "vs-c", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_vsc() {
    assert_golden("御座せひん", "御座す", "vs-c", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_vsc() {
    assert_golden("御座せひんかった", "御座す", "vs-c", "～negative→ksb→past");
}

#[test]
fn deconjugate_contracted_provisional_conditional_rya_vsc() {
    assert_golden("御座すりゃ", "御座す", "vs-c", "～provisional conditional→contracted");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_vsc() {
    assert_golden("御座しましたら", "御座す", "vs-c", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_vsc() {
    assert_golden("御座しになる", "御座す", "vs-c", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_vsc() {
    assert_golden("御座しなさる", "御座す", "vs-c", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_vsc() {
    assert_golden("御座しはる", "御座す", "vs-c", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_vsc() {
    assert_golden("御座しなさるな", "御座す", "vs-c", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_vsc() {
    assert_golden("御座すまい", "御座す", "vs-c", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_vsc() {
    assert_golden("御座しますまい", "御座す", "vs-c", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_vsc() {
    assert_golden("御座せねば", "御座す", "vs-c", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_vsc() {
    assert_golden("御座せにゃ", "御座す", "vs-c", "～colloquial negative conditional");
}
