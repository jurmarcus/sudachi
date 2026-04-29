//! Golden tests ported from JL's `DeconjugatorTestsForVK.cs`.
//! 225 test cases proving deconjugator output matches
//! JL's expectations for class VK.

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
fn deconjugate_masu_stem_vk() {
    assert_golden("来", "来る", "vk", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_vk() {
    assert_golden("来ない", "来る", "vk", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_vk() {
    assert_golden("来ます", "来る", "vk", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_vk() {
    assert_golden("来ましょう", "来る", "vk", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_vk() {
    assert_golden("来ません", "来る", "vk", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_vk() {
    assert_golden("来た", "来る", "vk", "～past");
}

#[test]
fn deconjugate_plain_past_negative_vk() {
    assert_golden("来なかった", "来る", "vk", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_vk() {
    assert_golden("来ました", "来る", "vk", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_vk() {
    assert_golden("来ませんでした", "来る", "vk", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_vk() {
    assert_golden("来て", "来る", "vk", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_vk() {
    assert_golden("来なくて", "来る", "vk", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_vk() {
    assert_golden("来ないで", "来る", "vk", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_vk() {
    assert_golden("来まして", "来る", "vk", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_honorific_affirmative_vk() {
    assert_golden("来られる", "来る", "vk", "～passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_honorific_negative_vk() {
    assert_golden("来られない", "来る", "vk", "～passive/potential/honorific→negative");
}

#[test]
fn deconjugate_plain_past_passive_potential_honorific_affirmative_vk() {
    assert_golden("来られた", "来る", "vk", "～passive/potential/honorific→past");
}

#[test]
fn deconjugate_polite_past_passive_potential_honorific_affirmative_vk() {
    assert_golden("来られました", "来る", "vk", "～passive/potential/honorific→polite past");
}

#[test]
fn deconjugate_plain_past_passive_potential_honorific_negative_vk() {
    assert_golden("来られなかった", "来る", "vk", "～passive/potential/honorific→negative→past");
}

#[test]
fn deconjugate_polite_past_passive_potential_honorific_negative_vk() {
    assert_golden("来られませんでした", "来る", "vk", "～passive/potential/honorific→polite past negative");
}

#[test]
fn deconjugate_polite_passive_potential_honorific_affirmative_vk() {
    assert_golden("来られます", "来る", "vk", "～passive/potential/honorific→polite");
}

#[test]
fn deconjugate_polite_passive_potential_honorific_negative_vk() {
    assert_golden("来られません", "来る", "vk", "～passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_vk() {
    assert_golden("来い", "来る", "vk", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_vk() {
    assert_golden("来るな", "来る", "vk", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_vk() {
    assert_golden("来なさい", "来る", "vk", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_vk() {
    assert_golden("来てください", "来る", "vk", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_vk() {
    assert_golden("来ないでください", "来る", "vk", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_vk() {
    assert_golden("来よう", "来る", "vk", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_vk() {
    assert_golden("来よ", "来る", "vk", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_vk() {
    assert_golden("来ましょう", "来る", "vk", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_vk() {
    assert_golden("来れば", "来る", "vk", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_vk() {
    assert_golden("来なければ", "来る", "vk", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_vk() {
    assert_golden("来たら", "来る", "vk", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_vk() {
    assert_golden("来たらば", "来る", "vk", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_vk() {
    assert_golden("来なかったら", "来る", "vk", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_vk() {
    assert_golden("来させる", "来る", "vk", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_vk() {
    assert_golden("来させない", "来る", "vk", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_vk() {
    assert_golden("来させん", "来る", "vk", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_vk() {
    assert_golden("来させます", "来る", "vk", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_vk() {
    assert_golden("来さします", "来る", "vk", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_vk() {
    assert_golden("来させません", "来る", "vk", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_vk() {
    assert_golden("来させた", "来る", "vk", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_vk() {
    assert_golden("来させなかった", "来る", "vk", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_vk() {
    assert_golden("来させました", "来る", "vk", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_vk() {
    assert_golden("来させませんでした", "来る", "vk", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_vk() {
    assert_golden("来させられる", "来る", "vk", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_vk() {
    assert_golden("来させられない", "来る", "vk", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_vk() {
    assert_golden("来させられます", "来る", "vk", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_vk() {
    assert_golden("来させられません", "来る", "vk", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_vk() {
    assert_golden("来たい", "来る", "vk", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_vk() {
    assert_golden("来たくありません", "来る", "vk", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_vk() {
    assert_golden("来たくありませんでした", "来る", "vk", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_vk() {
    assert_golden("来たくない", "来る", "vk", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_vk() {
    assert_golden("来たかった", "来る", "vk", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_vk() {
    assert_golden("来たくなかった", "来る", "vk", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_vk() {
    assert_golden("来ている", "来る", "vk", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_vk() {
    assert_golden("来ていない", "来る", "vk", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_vk() {
    assert_golden("来ていた", "来る", "vk", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_vk() {
    assert_golden("来ていなかった", "来る", "vk", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_vk() {
    assert_golden("来ています", "来る", "vk", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_vk() {
    assert_golden("来ていません", "来る", "vk", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_vk() {
    assert_golden("来ていました", "来る", "vk", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_vk() {
    assert_golden("来ていませんでした", "来る", "vk", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_vk() {
    assert_golden("来てる", "来る", "vk", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_vk() {
    assert_golden("来てない", "来る", "vk", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_vk() {
    assert_golden("来てた", "来る", "vk", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_vk() {
    assert_golden("来てなかった", "来る", "vk", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_vk() {
    assert_golden("来てます", "来る", "vk", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_vk() {
    assert_golden("来てません", "来る", "vk", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_vk() {
    assert_golden("来てました", "来る", "vk", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_vk() {
    assert_golden("来てません", "来る", "vk", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_vk() {
    assert_golden("来てませんでした", "来る", "vk", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_vk() {
    assert_golden("来てしまう", "来る", "vk", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_vk() {
    assert_golden("来てもう", "来る", "vk", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_vk() {
    assert_golden("来てしまわない", "来る", "vk", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_vk() {
    assert_golden("来てしまった", "来る", "vk", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_vk() {
    assert_golden("来てしまわなかった", "来る", "vk", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_vk() {
    assert_golden("来てしまって", "来る", "vk", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_vk() {
    assert_golden("来てしまえば", "来る", "vk", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_vk() {
    assert_golden("来てしまわなければ", "来る", "vk", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_vk() {
    assert_golden("来てしまわなかったら", "来る", "vk", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_vk() {
    assert_golden("来てしまったら", "来る", "vk", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_vk() {
    assert_golden("来てしまおう", "来る", "vk", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_vk() {
    assert_golden("来てしまいます", "来る", "vk", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_vk() {
    assert_golden("来てしまいません", "来る", "vk", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_vk() {
    assert_golden("来てしまいました", "来る", "vk", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_vk() {
    assert_golden("来てしまいませんでした", "来る", "vk", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_vk() {
    assert_golden("来てしまえる", "来る", "vk", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_vk() {
    assert_golden("来てしまわれる", "来る", "vk", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_vk() {
    assert_golden("来てしまわせる", "来る", "vk", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_vk() {
    assert_golden("来ちゃう", "来る", "vk", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_vk() {
    assert_golden("来ちゃわない", "来る", "vk", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_vk() {
    assert_golden("来ちゃった", "来る", "vk", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_vk() {
    assert_golden("来ちゃわなかった", "来る", "vk", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_vk() {
    assert_golden("来ちゃって", "来る", "vk", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_vk() {
    assert_golden("来ちゃえば", "来る", "vk", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_vk() {
    assert_golden("来ちゃわなければ", "来る", "vk", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_vk() {
    assert_golden("来ちゃわなかったら", "来る", "vk", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_vk() {
    assert_golden("来ちゃおう", "来る", "vk", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_vk() {
    assert_golden("来ちゃえる", "来る", "vk", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_vk() {
    assert_golden("来ておく", "来る", "vk", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_vk() {
    assert_golden("来ておかない", "来る", "vk", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_vk() {
    assert_golden("来ておいた", "来る", "vk", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_vk() {
    assert_golden("来ておかなかった", "来る", "vk", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_vk() {
    assert_golden("来ておいて", "来る", "vk", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_vk() {
    assert_golden("来ておけば", "来る", "vk", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_vk() {
    assert_golden("来ておいたら", "来る", "vk", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_vk() {
    assert_golden("来ておこう", "来る", "vk", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_vk() {
    assert_golden("来ておける", "来る", "vk", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_vk() {
    assert_golden("来ておかれる", "来る", "vk", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_vk() {
    assert_golden("来とく", "来る", "vk", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_vk() {
    assert_golden("来とかない", "来る", "vk", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_vk() {
    assert_golden("来といた", "来る", "vk", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_vk() {
    assert_golden("来とかなかった", "来る", "vk", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_vk() {
    assert_golden("来といて", "来る", "vk", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_vk() {
    assert_golden("来とけば", "来る", "vk", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_vk() {
    assert_golden("来といたら", "来る", "vk", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_vk() {
    assert_golden("来とこう", "来る", "vk", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_vk() {
    assert_golden("来とける", "来る", "vk", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_vk() {
    assert_golden("来とかれる", "来る", "vk", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_vk() {
    assert_golden("来てある", "来る", "vk", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_vk() {
    assert_golden("来てあった", "来る", "vk", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_vk() {
    assert_golden("来てあって", "来る", "vk", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_vk() {
    assert_golden("来てあったら", "来る", "vk", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_vk() {
    assert_golden("来てあれば", "来る", "vk", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_vk() {
    assert_golden("来ていく", "来る", "vk", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_vk() {
    assert_golden("来ていかない", "来る", "vk", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_vk() {
    assert_golden("来ていった", "来る", "vk", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_vk() {
    assert_golden("来ていかなかった", "来る", "vk", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_vk() {
    assert_golden("来ていって", "来る", "vk", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_vk() {
    assert_golden("来ていこう", "来る", "vk", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_vk() {
    assert_golden("来ていける", "来る", "vk", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_vk() {
    assert_golden("来ていかれる", "来る", "vk", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_vk() {
    assert_golden("来ていかせる", "来る", "vk", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_vk() {
    assert_golden("来てくる", "来る", "vk", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_vk() {
    assert_golden("来てこない", "来る", "vk", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_vk() {
    assert_golden("来てきた", "来る", "vk", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_vk() {
    assert_golden("来てこなかった", "来る", "vk", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_vk() {
    assert_golden("来てきて", "来る", "vk", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_vk() {
    assert_golden("来てくれば", "来る", "vk", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_vk() {
    assert_golden("来てきたら", "来る", "vk", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_vk() {
    assert_golden("来てこられる", "来る", "vk", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_vk() {
    assert_golden("来てこさせる", "来る", "vk", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_vk() {
    assert_golden("来ながら", "来る", "vk", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_vk() {
    assert_golden("来すぎる", "来る", "vk", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_vk() {
    assert_golden("来そう", "来る", "vk", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_vk() {
    assert_golden("来ぬ", "来る", "vk", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_vk() {
    assert_golden("来ず", "来る", "vk", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_vk() {
    assert_golden("来ずに", "来る", "vk", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_vk() {
    assert_golden("来たり", "来る", "vk", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_vk() {
    assert_golden("来なかったり", "来る", "vk", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_vk() {
    assert_golden("来ん", "来る", "vk", "～slurred; slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_vk() {
    assert_golden("来んかった", "来る", "vk", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_vk() {
    assert_golden("来ざる", "来る", "vk", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_affirmative_vk() {
    assert_golden("来れる", "来る", "vk", "～potential");
}

#[test]
fn deconjugate_polite_non_past_colloquial_potential_affirmative_vk() {
    assert_golden("来れます", "来る", "vk", "～potential→polite");
}

#[test]
fn deconjugate_plain_past_colloquial_potential_affirmative_vk() {
    assert_golden("来れた", "来る", "vk", "～potential→past");
}

#[test]
fn deconjugate_polite_past_colloquial_potential_affirmative_vk() {
    assert_golden("来れました", "来る", "vk", "～potential→polite past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_negative_vk() {
    assert_golden("来れない", "来る", "vk", "～potential→negative");
}

#[test]
fn deconjugate_polite_non_past_colloquial_potential_negative_vk() {
    assert_golden("来れません", "来る", "vk", "～potential→polite negative");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_volitional_vk() {
    assert_golden("来れよう", "来る", "vk", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_colloquial_potential_volitional_vk() {
    assert_golden("来れよ", "来る", "vk", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_imperative_vk() {
    assert_golden("来れろ", "来る", "vk", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_te_form_vk() {
    assert_golden("来れて", "来る", "vk", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_temporal_conditional_vk() {
    assert_golden("来れたら", "来る", "vk", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_provisional_conditional_vk() {
    assert_golden("来れれば", "来る", "vk", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_passive_potential_vk() {
    assert_golden("来れられる", "来る", "vk", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_causative_vk() {
    assert_golden("来れさせる", "来る", "vk", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_vk() {
    assert_golden("来てあげる", "来る", "vk", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_vk() {
    assert_golden("来てあげられる", "来る", "vk", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_vk() {
    assert_golden("来ておる", "来る", "vk", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_vk() {
    assert_golden("来ておらない", "来る", "vk", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_vk() {
    assert_golden("来ておらん", "来る", "vk", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_vk() {
    assert_golden("来ておった", "来る", "vk", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_vk() {
    assert_golden("来ておらなかった", "来る", "vk", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_vk() {
    assert_golden("来ております", "来る", "vk", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_vk() {
    assert_golden("来ておりません", "来る", "vk", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_vk() {
    assert_golden("来ておりました", "来る", "vk", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_vk() {
    assert_golden("来ておりませんでした", "来る", "vk", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_vk() {
    assert_golden("来ておって", "来る", "vk", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_vk() {
    assert_golden("来ておろう", "来る", "vk", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_vk() {
    assert_golden("来ておれる", "来る", "vk", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_vk() {
    assert_golden("来ておられる", "来る", "vk", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_vk() {
    assert_golden("来とる", "来る", "vk", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_vk() {
    assert_golden("来とらない", "来る", "vk", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_vk() {
    assert_golden("来とらん", "来る", "vk", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_vk() {
    assert_golden("来とった", "来る", "vk", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_vk() {
    assert_golden("来とらなかった", "来る", "vk", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_vk() {
    assert_golden("来とります", "来る", "vk", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_vk() {
    assert_golden("来とりません", "来る", "vk", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_vk() {
    assert_golden("来とりました", "来る", "vk", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_vk() {
    assert_golden("来とりませんでした", "来る", "vk", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_vk() {
    assert_golden("来とって", "来る", "vk", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_vk() {
    assert_golden("来とろう", "来る", "vk", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_vk() {
    assert_golden("来とれる", "来る", "vk", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_vk() {
    assert_golden("来とられる", "来る", "vk", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_vk() {
    assert_golden("来さす", "来る", "vk", "～short causative");
}

#[test]
fn deconjugate_plain_non_past_na_vk() {
    assert_golden("来な", "来る", "vk", "～casual polite imperative");
}

#[test]
fn deconjugate_topic_or_condition_vk() {
    assert_golden("来ては", "来る", "vk", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_vk() {
    assert_golden("来ちゃ", "来る", "vk", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_vk() {
    assert_golden("来なきゃ", "来る", "vk", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_vk() {
    assert_golden("来ちまう", "来る", "vk", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_vk() {
    assert_golden("来ちゃう", "来る", "vk", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_vk() {
    assert_golden("来ていらっしゃる", "来る", "vk", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_vk() {
    assert_golden("来ていらっしゃらない", "来る", "vk", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_vk() {
    assert_golden("来つつ", "来る", "vk", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_vk() {
    assert_golden("来てくれる", "来る", "vk", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_vk() {
    assert_golden("来てくれない", "来る", "vk", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_vk() {
    assert_golden("来てくれます", "来る", "vk", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_vk() {
    assert_golden("来てくれません", "来る", "vk", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_vk() {
    assert_golden("来てくれ", "来る", "vk", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_vk() {
    assert_golden("来へん", "来る", "vk", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_vk() {
    assert_golden("来へんかった", "来る", "vk", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_vk() {
    assert_golden("来ひん", "来る", "vk", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_vk() {
    assert_golden("来ひんかった", "来る", "vk", "～negative→ksb→past");
}

#[test]
fn deconjugate_contracted_provisional_conditional_rya_vk() {
    assert_golden("来りゃ", "来る", "vk", "～provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_vk() {
    assert_golden("来ささない", "来る", "vk", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_vk() {
    assert_golden("来ましたら", "来る", "vk", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_vk() {
    assert_golden("来になる", "来る", "vk", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_vk() {
    assert_golden("来なさる", "来る", "vk", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_vk() {
    assert_golden("来はる", "来る", "vk", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_vk() {
    assert_golden("来なさるな", "来る", "vk", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_vk() {
    assert_golden("来まい", "来る", "vk", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_vk() {
    assert_golden("来ますまい", "来る", "vk", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_vk() {
    assert_golden("来ねば", "来る", "vk", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_vk() {
    assert_golden("来にゃ", "来る", "vk", "～colloquial negative conditional");
}
