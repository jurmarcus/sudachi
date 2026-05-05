//! Golden tests ported from JL's `DeconjugatorTestsForV1.cs`.
//! 228 test cases proving deconjugator output matches
//! JL's expectations for class V1.

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
fn deconjugate_masu_stem_v1() {
    assert_golden("生き", "生きる", "v1", "～masu stem");
}

#[test]
fn deconjugate_plain_non_past_negative_v1() {
    assert_golden("生きない", "生きる", "v1", "～negative");
}

#[test]
fn deconjugate_polite_non_past_affirmative_v1() {
    assert_golden("生きます", "生きる", "v1", "～polite");
}

#[test]
fn deconjugate_polite_non_past_volitional_v1() {
    assert_golden("生きましょう", "生きる", "v1", "～polite volitional");
}

#[test]
fn deconjugate_polite_non_past_negative_v1() {
    assert_golden("生きません", "生きる", "v1", "～polite negative");
}

#[test]
fn deconjugate_plain_past_affirmative_v1() {
    assert_golden("生きた", "生きる", "v1", "～past");
}

#[test]
fn deconjugate_plain_past_negative_v1() {
    assert_golden("生きなかった", "生きる", "v1", "～negative→past");
}

#[test]
fn deconjugate_polite_past_affirmative_v1() {
    assert_golden("生きました", "生きる", "v1", "～polite past");
}

#[test]
fn deconjugate_polite_past_negative_v1() {
    assert_golden("生きませんでした", "生きる", "v1", "～polite past negative");
}

#[test]
fn deconjugate_plain_te_form_affirmative_v1() {
    assert_golden("生きて", "生きる", "v1", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_v1() {
    assert_golden("生きなくて", "生きる", "v1", "～negative→te");
}

#[test]
fn deconjugate_plain_te_form_negative2_v1() {
    assert_golden("生きないで", "生きる", "v1", "～negative→te");
}

#[test]
fn deconjugate_polite_te_form_affirmative_v1() {
    assert_golden("生きまして", "生きる", "v1", "～polite te");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_honorific_affirmative_v1() {
    assert_golden("生きられる", "生きる", "v1", "～passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_honorific_negative_v1() {
    assert_golden("生きられない", "生きる", "v1", "～passive/potential/honorific→negative");
}

#[test]
fn deconjugate_plain_non_past_passive_potential_honorific_slurred_negative_v1() {
    assert_golden("生きらんない", "生きる", "v1", "～passive/potential/honorific→negative→slurred");
}

#[test]
fn deconjugate_plain_past_passive_potential_honorific_affirmative_v1() {
    assert_golden("生きられた", "生きる", "v1", "～passive/potential/honorific→past");
}

#[test]
fn deconjugate_polite_past_passive_potential_honorific_affirmative_v1() {
    assert_golden("生きられました", "生きる", "v1", "～passive/potential/honorific→polite past");
}

#[test]
fn deconjugate_plain_past_passive_potential_honorific_negative_v1() {
    assert_golden("生きられなかった", "生きる", "v1", "～passive/potential/honorific→negative→past");
}

#[test]
fn deconjugate_polite_past_passive_potential_honorific_negative_v1() {
    assert_golden("生きられませんでした", "生きる", "v1", "～passive/potential/honorific→polite past negative");
}

#[test]
fn deconjugate_polite_passive_potential_honorific_affirmative_v1() {
    assert_golden("生きられます", "生きる", "v1", "～passive/potential/honorific→polite");
}

#[test]
fn deconjugate_polite_passive_potential_honorific_negative_v1() {
    assert_golden("生きられません", "生きる", "v1", "～passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_imperative_affirmative_v1() {
    assert_golden("生きろ", "生きる", "v1", "～imperative");
}

#[test]
fn deconjugate_plain_imperative_negative_v1() {
    assert_golden("生きるな", "生きる", "v1", "～imperative negative");
}

#[test]
fn deconjugate_polite_imperative_affirmative_v1() {
    assert_golden("生きなさい", "生きる", "v1", "～polite imperative");
}

#[test]
fn deconjugate_polite_request_affirmative_v1() {
    assert_golden("生きてください", "生きる", "v1", "～polite request");
}

#[test]
fn deconjugate_polite_request_negative_v1() {
    assert_golden("生きないでください", "生きる", "v1", "～negative→polite request");
}

#[test]
fn deconjugate_plain_volitional_affirmative_v1() {
    assert_golden("生きよう", "生きる", "v1", "～volitional");
}

#[test]
fn deconjugate_plain_kansaiben_volitional_affirmative_v1() {
    assert_golden("生きよ", "生きる", "v1", "～volitional→ksb");
}

#[test]
fn deconjugate_polite_volitional_affirmative_v1() {
    assert_golden("生きましょう", "生きる", "v1", "～polite volitional");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_v1() {
    assert_golden("生きれば", "生きる", "v1", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_v1() {
    assert_golden("生きなければ", "生きる", "v1", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_v1() {
    assert_golden("生きたら", "生きる", "v1", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_v1() {
    assert_golden("生きたらば", "生きる", "v1", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_v1() {
    assert_golden("生きなかったら", "生きる", "v1", "～negative→conditional");
}

#[test]
fn deconjugate_plain_causative_affirmative_v1() {
    assert_golden("生きさせる", "生きる", "v1", "～causative");
}

#[test]
fn deconjugate_plain_causative_negative_v1() {
    assert_golden("生きさせない", "生きる", "v1", "～causative→negative");
}

#[test]
fn deconjugate_plain_causative_slurred_v1() {
    assert_golden("生きさせん", "生きる", "v1", "～causative→slurred; causative→slurred negative");
}

#[test]
fn deconjugate_polite_causative_affirmative_v1() {
    assert_golden("生きさせます", "生きる", "v1", "～causative→polite");
}

#[test]
fn deconjugate_polite_short_causative_affirmative_v1() {
    assert_golden("生きさします", "生きる", "v1", "～short causative→polite");
}

#[test]
fn deconjugate_polite_causative_negative_v1() {
    assert_golden("生きさせません", "生きる", "v1", "～causative→polite negative");
}

#[test]
fn deconjugate_plain_causative_past_v1() {
    assert_golden("生きさせた", "生きる", "v1", "～causative→past");
}

#[test]
fn deconjugate_plain_causative_past_negative_v1() {
    assert_golden("生きさせなかった", "生きる", "v1", "～causative→negative→past");
}

#[test]
fn deconjugate_polite_causative_past_v1() {
    assert_golden("生きさせました", "生きる", "v1", "～causative→polite past");
}

#[test]
fn deconjugate_polite_causative_past_negative_v1() {
    assert_golden("生きさせませんでした", "生きる", "v1", "～causative→polite past negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_affirmative_v1() {
    assert_golden("生きさせられる", "生きる", "v1", "～causative→passive/potential/honorific");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_plain_negative_v1() {
    assert_golden("生きさせられない", "生きる", "v1", "～causative→passive/potential/honorific→negative");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_affirmative_v1() {
    assert_golden("生きさせられます", "生きる", "v1", "～causative→passive/potential/honorific→polite");
}

#[test]
fn deconjugate_causative_passive_potential_honorific_polite_negative_v1() {
    assert_golden("生きさせられません", "生きる", "v1", "～causative→passive/potential/honorific→polite negative");
}

#[test]
fn deconjugate_plain_non_past_desiderative_v1() {
    assert_golden("生きたい", "生きる", "v1", "～want");
}

#[test]
fn deconjugate_plain_non_past_desiderative_formal_negative_v1() {
    assert_golden("生きたくありません", "生きる", "v1", "～want→formal negative");
}

#[test]
fn deconjugate_plain_past_desiderative_formal_negative_v1() {
    assert_golden("生きたくありませんでした", "生きる", "v1", "～want→formal negative past");
}

#[test]
fn deconjugate_plain_non_past_desiderative_negative_v1() {
    assert_golden("生きたくない", "生きる", "v1", "～want→negative");
}

#[test]
fn deconjugate_plain_past_desiderative_v1() {
    assert_golden("生きたかった", "生きる", "v1", "～want→past");
}

#[test]
fn deconjugate_plain_past_desiderative_negative_v1() {
    assert_golden("生きたくなかった", "生きる", "v1", "～want→negative→past");
}

#[test]
fn deconjugate_plain_non_past_teiru_v1() {
    assert_golden("生きている", "生きる", "v1", "～teiru");
}

#[test]
fn deconjugate_plain_non_past_teiru_negative_v1() {
    assert_golden("生きていない", "生きる", "v1", "～teiru→negative");
}

#[test]
fn deconjugate_plain_past_teiru_affirmative_v1() {
    assert_golden("生きていた", "生きる", "v1", "～teiru→past");
}

#[test]
fn deconjugate_plain_past_teiru_negative_v1() {
    assert_golden("生きていなかった", "生きる", "v1", "～teiru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teiru_v1() {
    assert_golden("生きています", "生きる", "v1", "～teiru→polite");
}

#[test]
fn deconjugate_polite_non_past_teiru_negative_v1() {
    assert_golden("生きていません", "生きる", "v1", "～teiru→polite negative");
}

#[test]
fn deconjugate_polite_past_teiru_v1() {
    assert_golden("生きていました", "生きる", "v1", "～teiru→polite past");
}

#[test]
fn deconjugate_polite_past_teiru_negative_v1() {
    assert_golden("生きていませんでした", "生きる", "v1", "～teiru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_teru_v1() {
    assert_golden("生きてる", "生きる", "v1", "～teru");
}

#[test]
fn deconjugate_plain_non_past_teru_negative_v1() {
    assert_golden("生きてない", "生きる", "v1", "～teru→negative");
}

#[test]
fn deconjugate_plain_past_teru_v1() {
    assert_golden("生きてた", "生きる", "v1", "～teru→past");
}

#[test]
fn deconjugate_plain_past_teru_negative_v1() {
    assert_golden("生きてなかった", "生きる", "v1", "～teru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teru_v1() {
    assert_golden("生きてます", "生きる", "v1", "～teru→polite");
}

#[test]
fn deconjugate_polite_non_past_teru_negative_v1() {
    assert_golden("生きてません", "生きる", "v1", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_v1() {
    assert_golden("生きてました", "生きる", "v1", "～teru→polite past");
}

#[test]
fn deconjugate_polite_past_teru_negative_v1() {
    assert_golden("生きてません", "生きる", "v1", "～teru→polite negative");
}

#[test]
fn deconjugate_polite_past_teru_negative2_v1() {
    assert_golden("生きてませんでした", "生きる", "v1", "～teru→polite past negative");
}

#[test]
fn deconjugate_plain_non_past_shimau_affirmative_v1() {
    assert_golden("生きてしまう", "生きる", "v1", "～finish/completely/end up");
}

#[test]
fn deconjugate_plain_non_past_shimau_kansaiben_affirmative_v1() {
    assert_golden("生きてもう", "生きる", "v1", "～finish/completely/end up→ksb");
}

#[test]
fn deconjugate_plain_non_past_shimau_negative_v1() {
    assert_golden("生きてしまわない", "生きる", "v1", "～finish/completely/end up→negative");
}

#[test]
fn deconjugate_plain_past_shimau_affirmative_v1() {
    assert_golden("生きてしまった", "生きる", "v1", "～finish/completely/end up→past");
}

#[test]
fn deconjugate_plain_past_shimau_negative_v1() {
    assert_golden("生きてしまわなかった", "生きる", "v1", "～finish/completely/end up→negative→past");
}

#[test]
fn deconjugate_plain_shimau_te_form_v1() {
    assert_golden("生きてしまって", "生きる", "v1", "～finish/completely/end up→te");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_v1() {
    assert_golden("生きてしまえば", "生きる", "v1", "～finish/completely/end up→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_provisional_conditional_negative_v1() {
    assert_golden("生きてしまわなければ", "生きる", "v1", "～finish/completely/end up→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_negative_v1() {
    assert_golden("生きてしまわなかったら", "生きる", "v1", "～finish/completely/end up→negative→conditional");
}

#[test]
fn deconjugate_plain_shimau_temporal_conditional_v1() {
    assert_golden("生きてしまったら", "生きる", "v1", "～finish/completely/end up→conditional");
}

#[test]
fn deconjugate_plain_shimau_volitional_v1() {
    assert_golden("生きてしまおう", "生きる", "v1", "～finish/completely/end up→volitional");
}

#[test]
fn deconjugate_polite_non_past_shimau_affirmative_v1() {
    assert_golden("生きてしまいます", "生きる", "v1", "～finish/completely/end up→polite");
}

#[test]
fn deconjugate_polite_non_past_shimau_negative_v1() {
    assert_golden("生きてしまいません", "生きる", "v1", "～finish/completely/end up→polite negative");
}

#[test]
fn deconjugate_polite_past_shimau_affirmative_v1() {
    assert_golden("生きてしまいました", "生きる", "v1", "～finish/completely/end up→polite past");
}

#[test]
fn deconjugate_polite_past_shimau_negative_v1() {
    assert_golden("生きてしまいませんでした", "生きる", "v1", "～finish/completely/end up→polite past negative");
}

#[test]
fn deconjugate_plain_shimau_potential_v1() {
    assert_golden("生きてしまえる", "生きる", "v1", "～finish/completely/end up→potential");
}

#[test]
fn deconjugate_plain_shimau_passive_v1() {
    assert_golden("生きてしまわれる", "生きる", "v1", "～finish/completely/end up→passive");
}

#[test]
fn deconjugate_plain_shimau_causative_v1() {
    assert_golden("生きてしまわせる", "生きる", "v1", "～finish/completely/end up→causative");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_affirmative_v1() {
    assert_golden("生きちゃう", "生きる", "v1", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_negative_v1() {
    assert_golden("生きちゃわない", "生きる", "v1", "～finish/completely/end up→contracted→negative");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_affirmative_v1() {
    assert_golden("生きちゃった", "生きる", "v1", "～finish/completely/end up→contracted→past");
}

#[test]
fn deconjugate_plain_past_contracted_shimau_negative_v1() {
    assert_golden("生きちゃわなかった", "生きる", "v1", "～finish/completely/end up→contracted→negative→past");
}

#[test]
fn deconjugate_plain_contracted_shimau_te_form_v1() {
    assert_golden("生きちゃって", "生きる", "v1", "～finish/completely/end up→contracted→te");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_v1() {
    assert_golden("生きちゃえば", "生きる", "v1", "～finish/completely/end up→contracted→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_provisional_conditional_negative_v1() {
    assert_golden("生きちゃわなければ", "生きる", "v1", "～finish/completely/end up→contracted→negative→provisional conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_temporal_conditional_negative_v1() {
    assert_golden("生きちゃわなかったら", "生きる", "v1", "～finish/completely/end up→contracted→negative→conditional");
}

#[test]
fn deconjugate_plain_contracted_shimau_volitional_v1() {
    assert_golden("生きちゃおう", "生きる", "v1", "～finish/completely/end up→contracted→volitional");
}

#[test]
fn deconjugate_plain_contracted_shimau_potential_v1() {
    assert_golden("生きちゃえる", "生きる", "v1", "～finish/completely/end up→contracted→potential");
}

#[test]
fn deconjugate_plain_non_past_oku_affirmative_v1() {
    assert_golden("生きておく", "生きる", "v1", "～for now");
}

#[test]
fn deconjugate_plain_non_past_oku_negative_v1() {
    assert_golden("生きておかない", "生きる", "v1", "～for now→negative");
}

#[test]
fn deconjugate_plain_past_oku_affirmative_v1() {
    assert_golden("生きておいた", "生きる", "v1", "～for now→past");
}

#[test]
fn deconjugate_plain_past_oku_negative_v1() {
    assert_golden("生きておかなかった", "生きる", "v1", "～for now→negative→past");
}

#[test]
fn deconjugate_plain_oku_te_form_v1() {
    assert_golden("生きておいて", "生きる", "v1", "～for now→te");
}

#[test]
fn deconjugate_plain_oku_provisional_conditional_v1() {
    assert_golden("生きておけば", "生きる", "v1", "～for now→provisional conditional");
}

#[test]
fn deconjugate_plain_oku_temporal_conditional_v1() {
    assert_golden("生きておいたら", "生きる", "v1", "～for now→conditional");
}

#[test]
fn deconjugate_plain_oku_volitional_v1() {
    assert_golden("生きておこう", "生きる", "v1", "～for now→volitional");
}

#[test]
fn deconjugate_plain_oku_potential_v1() {
    assert_golden("生きておける", "生きる", "v1", "～for now→potential");
}

#[test]
fn deconjugate_plain_oku_passive_v1() {
    assert_golden("生きておかれる", "生きる", "v1", "～for now→passive");
}

#[test]
fn deconjugate_plain_non_past_toku_affirmative_v1() {
    assert_golden("生きとく", "生きる", "v1", "～toku (for now)");
}

#[test]
fn deconjugate_plain_non_past_toku_negative_v1() {
    assert_golden("生きとかない", "生きる", "v1", "～toku (for now)→negative");
}

#[test]
fn deconjugate_plain_past_toku_affirmative_v1() {
    assert_golden("生きといた", "生きる", "v1", "～toku (for now)→past");
}

#[test]
fn deconjugate_plain_past_toku_negative_v1() {
    assert_golden("生きとかなかった", "生きる", "v1", "～toku (for now)→negative→past");
}

#[test]
fn deconjugate_plain_toku_te_form_v1() {
    assert_golden("生きといて", "生きる", "v1", "～toku (for now)→te");
}

#[test]
fn deconjugate_plain_toku_provisional_conditional_v1() {
    assert_golden("生きとけば", "生きる", "v1", "～toku (for now)→provisional conditional");
}

#[test]
fn deconjugate_plain_toku_temporal_conditional_v1() {
    assert_golden("生きといたら", "生きる", "v1", "～toku (for now)→conditional");
}

#[test]
fn deconjugate_plain_toku_volitional_v1() {
    assert_golden("生きとこう", "生きる", "v1", "～toku (for now)→volitional");
}

#[test]
fn deconjugate_plain_toku_potential_v1() {
    assert_golden("生きとける", "生きる", "v1", "～toku (for now)→potential");
}

#[test]
fn deconjugate_plain_toku_passive_v1() {
    assert_golden("生きとかれる", "生きる", "v1", "～toku (for now)→passive");
}

#[test]
fn deconjugate_plain_non_past_tearu_affirmative_v1() {
    assert_golden("生きてある", "生きる", "v1", "～tearu");
}

#[test]
fn deconjugate_plain_past_tearu_affirmative_v1() {
    assert_golden("生きてあった", "生きる", "v1", "～tearu→past");
}

#[test]
fn deconjugate_plain_tearu_te_form_v1() {
    assert_golden("生きてあって", "生きる", "v1", "～tearu→te");
}

#[test]
fn deconjugate_plain_tearu_temporal_conditional_v1() {
    assert_golden("生きてあったら", "生きる", "v1", "～tearu→conditional");
}

#[test]
fn deconjugate_plain_tearu_provisional_conditional_v1() {
    assert_golden("生きてあれば", "生きる", "v1", "～tearu→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_teiku_affirmative_v1() {
    assert_golden("生きていく", "生きる", "v1", "～teiku");
}

#[test]
fn deconjugate_plain_non_past_teiku_negative_v1() {
    assert_golden("生きていかない", "生きる", "v1", "～teiku→negative");
}

#[test]
fn deconjugate_plain_past_teiku_affirmative_v1() {
    assert_golden("生きていった", "生きる", "v1", "～teiku→past");
}

#[test]
fn deconjugate_plain_past_teiku_negative_v1() {
    assert_golden("生きていかなかった", "生きる", "v1", "～teiku→negative→past");
}

#[test]
fn deconjugate_teiku_te_form_v1() {
    assert_golden("生きていって", "生きる", "v1", "～teiku→te");
}

#[test]
fn deconjugate_teiku_volitional_v1() {
    assert_golden("生きていこう", "生きる", "v1", "～teiku→volitional");
}

#[test]
fn deconjugate_teiku_potential_v1() {
    assert_golden("生きていける", "生きる", "v1", "～teiku→potential");
}

#[test]
fn deconjugate_teiku_passive_v1() {
    assert_golden("生きていかれる", "生きる", "v1", "～teiku→passive");
}

#[test]
fn deconjugate_teiku_causative_v1() {
    assert_golden("生きていかせる", "生きる", "v1", "～teiku→causative");
}

#[test]
fn deconjugate_plain_non_past_tekuru_affirmative_v1() {
    assert_golden("生きてくる", "生きる", "v1", "～tekuru");
}

#[test]
fn deconjugate_plain_non_past_tekuru_negative_v1() {
    assert_golden("生きてこない", "生きる", "v1", "～tekuru→negative");
}

#[test]
fn deconjugate_plain_past_tekuru_affirmative_v1() {
    assert_golden("生きてきた", "生きる", "v1", "～tekuru→past");
}

#[test]
fn deconjugate_plain_past_tekuru_negative_v1() {
    assert_golden("生きてこなかった", "生きる", "v1", "～tekuru→negative→past");
}

#[test]
fn deconjugate_tekuru_te_form_v1() {
    assert_golden("生きてきて", "生きる", "v1", "～tekuru→te");
}

#[test]
fn deconjugate_tekuru_provisional_conditional_v1() {
    assert_golden("生きてくれば", "生きる", "v1", "～tekuru→provisional conditional");
}

#[test]
fn deconjugate_tekuru_temporal_conditional_v1() {
    assert_golden("生きてきたら", "生きる", "v1", "～tekuru→conditional");
}

#[test]
fn deconjugate_plain_tekuru_passive_potential_affirmative_v1() {
    assert_golden("生きてこられる", "生きる", "v1", "～tekuru→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_tekuru_causative_affirmative_v1() {
    assert_golden("生きてこさせる", "生きる", "v1", "～tekuru→causative");
}

#[test]
fn deconjugate_nagara_v1() {
    assert_golden("生きながら", "生きる", "v1", "～while");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_v1() {
    assert_golden("生きすぎる", "生きる", "v1", "～too much");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_v1() {
    assert_golden("生きそう", "生きる", "v1", "～seemingness");
}

#[test]
fn deconjugate_classical_negative_form_nu_v1() {
    assert_golden("生きぬ", "生きる", "v1", "～archaic negative");
}

#[test]
fn deconjugate_classical_negative_continuative_form_zu_v1() {
    assert_golden("生きず", "生きる", "v1", "～adverbial negative");
}

#[test]
fn deconjugate_classical_adverbial_form_zu_ni_v1() {
    assert_golden("生きずに", "生きる", "v1", "～without doing so");
}

#[test]
fn deconjugate_plain_non_past_tari_affirmative_v1() {
    assert_golden("生きたり", "生きる", "v1", "～tari");
}

#[test]
fn deconjugate_plain_non_past_tari_negative_v1() {
    assert_golden("生きなかったり", "生きる", "v1", "～negative→tari");
}

#[test]
fn deconjugate_plain_non_past_slurred_affirmative_v1() {
    assert_golden("生きん", "生きる", "v1", "～slurred; slurred negative");
}

#[test]
fn deconjugate_plain_past_slurred_negative_v1() {
    assert_golden("生きんかった", "生きる", "v1", "～slurred negative→past");
}

#[test]
fn deconjugate_zaru_v1() {
    assert_golden("生きざる", "生きる", "v1", "～archaic attributive negative");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_affirmative_v1() {
    assert_golden("生きれる", "生きる", "v1", "～potential");
}

#[test]
fn deconjugate_polite_non_past_colloquial_potential_affirmative_v1() {
    assert_golden("生きれます", "生きる", "v1", "～potential→polite");
}

#[test]
fn deconjugate_plain_past_colloquial_potential_affirmative_v1() {
    assert_golden("生きれた", "生きる", "v1", "～potential→past");
}

#[test]
fn deconjugate_polite_past_colloquial_potential_affirmative_v1() {
    assert_golden("生きれました", "生きる", "v1", "～potential→polite past");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_negative_v1() {
    assert_golden("生きれない", "生きる", "v1", "～potential→negative");
}

#[test]
fn deconjugate_polite_non_past_colloquial_potential_negative_v1() {
    assert_golden("生きれません", "生きる", "v1", "～potential→polite negative");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_volitional_v1() {
    assert_golden("生きれよう", "生きる", "v1", "～potential→volitional");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_colloquial_potential_volitional_v1() {
    assert_golden("生きれよ", "生きる", "v1", "～potential→volitional→ksb");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_imperative_v1() {
    assert_golden("生きれろ", "生きる", "v1", "～potential→imperative");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_te_form_v1() {
    assert_golden("生きれて", "生きる", "v1", "～potential→te");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_temporal_conditional_v1() {
    assert_golden("生きれたら", "生きる", "v1", "～potential→conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_provisional_conditional_v1() {
    assert_golden("生きれれば", "生きる", "v1", "～potential→provisional conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_passive_potential_v1() {
    assert_golden("生きれられる", "生きる", "v1", "～potential→passive/potential/honorific");
}

#[test]
fn deconjugate_plain_non_past_colloquial_potential_causative_v1() {
    assert_golden("生きれさせる", "生きる", "v1", "～potential→causative");
}

#[test]
fn deconjugate_plain_non_past_ageru_affirmative_v1() {
    assert_golden("生きてあげる", "生きる", "v1", "～do for someone");
}

#[test]
fn deconjugate_plain_non_past_ageru_passive_v1() {
    assert_golden("生きてあげられる", "生きる", "v1", "～do for someone→passive");
}

#[test]
fn deconjugate_plain_non_past_teoru_v1() {
    assert_golden("生きておる", "生きる", "v1", "～teoru");
}

#[test]
fn deconjugate_plain_non_past_teoru_negative_v1() {
    assert_golden("生きておらない", "生きる", "v1", "～teoru→negative");
}

#[test]
fn deconjugate_plain_non_past_teoru_slurred_negative_v1() {
    assert_golden("生きておらん", "生きる", "v1", "～teoru→slurred negative");
}

#[test]
fn deconjugate_plain_past_teoru_affirmative_v1() {
    assert_golden("生きておった", "生きる", "v1", "～teoru→past");
}

#[test]
fn deconjugate_plain_past_teoru_negative_v1() {
    assert_golden("生きておらなかった", "生きる", "v1", "～teoru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_teoru_v1() {
    assert_golden("生きております", "生きる", "v1", "～teoru→polite");
}

#[test]
fn deconjugate_polite_non_past_teoru_negative_v1() {
    assert_golden("生きておりません", "生きる", "v1", "～teoru→polite negative");
}

#[test]
fn deconjugate_polite_past_teoru_v1() {
    assert_golden("生きておりました", "生きる", "v1", "～teoru→polite past");
}

#[test]
fn deconjugate_polite_past_teoru_negative_v1() {
    assert_golden("生きておりませんでした", "生きる", "v1", "～teoru→polite past negative");
}

#[test]
fn deconjugate_polite_past_teoru_te_form_v1() {
    assert_golden("生きておって", "生きる", "v1", "～teoru→te");
}

#[test]
fn deconjugate_polite_past_teoru_volitional_v1() {
    assert_golden("生きておろう", "生きる", "v1", "～teoru→volitional");
}

#[test]
fn deconjugate_polite_past_teoru_potential_v1() {
    assert_golden("生きておれる", "生きる", "v1", "～teoru→potential");
}

#[test]
fn deconjugate_polite_past_teoru_passive_v1() {
    assert_golden("生きておられる", "生きる", "v1", "～teoru→passive");
}

#[test]
fn deconjugate_plain_non_past_toru_v1() {
    assert_golden("生きとる", "生きる", "v1", "～toru");
}

#[test]
fn deconjugate_plain_non_past_toru_negative_v1() {
    assert_golden("生きとらない", "生きる", "v1", "～toru→negative");
}

#[test]
fn deconjugate_plain_non_past_toru_slurred_negative_v1() {
    assert_golden("生きとらん", "生きる", "v1", "～toru→slurred negative");
}

#[test]
fn deconjugate_plain_past_toru_affirmative_v1() {
    assert_golden("生きとった", "生きる", "v1", "～toru→past");
}

#[test]
fn deconjugate_plain_past_toru_negative_v1() {
    assert_golden("生きとらなかった", "生きる", "v1", "～toru→negative→past");
}

#[test]
fn deconjugate_polite_non_past_toru_v1() {
    assert_golden("生きとります", "生きる", "v1", "～toru→polite");
}

#[test]
fn deconjugate_polite_non_past_toru_negative_v1() {
    assert_golden("生きとりません", "生きる", "v1", "～toru→polite negative");
}

#[test]
fn deconjugate_polite_past_toru_v1() {
    assert_golden("生きとりました", "生きる", "v1", "～toru→polite past");
}

#[test]
fn deconjugate_polite_past_toru_negative_v1() {
    assert_golden("生きとりませんでした", "生きる", "v1", "～toru→polite past negative");
}

#[test]
fn deconjugate_polite_past_toru_te_form_v1() {
    assert_golden("生きとって", "生きる", "v1", "～toru→te");
}

#[test]
fn deconjugate_polite_past_toru_volitional_v1() {
    assert_golden("生きとろう", "生きる", "v1", "～toru→volitional");
}

#[test]
fn deconjugate_polite_past_toru_potential_v1() {
    assert_golden("生きとれる", "生きる", "v1", "～toru→potential");
}

#[test]
fn deconjugate_polite_past_toru_passive_v1() {
    assert_golden("生きとられる", "生きる", "v1", "～toru→passive");
}

#[test]
fn deconjugate_plain_short_causative_affirmative_v1() {
    assert_golden("生きさす", "生きる", "v1", "～short causative");
}

#[test]
fn deconjugate_plain_non_past_na_v1() {
    assert_golden("生きな", "生きる", "v1", "～casual polite imperative");
}

#[test]
fn deconjugate_topic_or_condition_v1() {
    assert_golden("生きては", "生きる", "v1", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_v1() {
    assert_golden("生きちゃ", "生きる", "v1", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_v1() {
    assert_golden("生きなきゃ", "生きる", "v1", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chimau_v1() {
    assert_golden("生きちまう", "生きる", "v1", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_shimau_chau_v1() {
    assert_golden("生きちゃう", "生きる", "v1", "～finish/completely/end up→contracted");
}

#[test]
fn deconjugate_plain_non_past_irassharu_affirmative_v1() {
    assert_golden("生きていらっしゃる", "生きる", "v1", "～honorific teiru");
}

#[test]
fn deconjugate_plain_non_past_irassharu_negative_v1() {
    assert_golden("生きていらっしゃらない", "生きる", "v1", "～honorific teiru→negative");
}

#[test]
fn deconjugate_tsutsu_v1() {
    assert_golden("生きつつ", "生きる", "v1", "～while/although");
}

#[test]
fn deconjugate_plain_non_past_statement_request_affirmative_v1() {
    assert_golden("生きてくれる", "生きる", "v1", "～statement/request");
}

#[test]
fn deconjugate_plain_non_past_statement_request_negative_v1() {
    assert_golden("生きてくれない", "生きる", "v1", "～statement/request→negative");
}

#[test]
fn deconjugate_polite_non_past_statement_request_affirmative_v1() {
    assert_golden("生きてくれます", "生きる", "v1", "～statement/request→polite");
}

#[test]
fn deconjugate_polite_non_past_statement_request_negative_v1() {
    assert_golden("生きてくれません", "生きる", "v1", "～statement/request→polite negative");
}

#[test]
fn deconjugate_polite_non_past_statement_imperative_v1() {
    assert_golden("生きてくれ", "生きる", "v1", "～statement/request→imperative; statement/request→masu stem");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_v1() {
    assert_golden("生きへん", "生きる", "v1", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_v1() {
    assert_golden("生きへんかった", "生きる", "v1", "～negative→ksb→past");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_sub_dialect_negative_v1() {
    assert_golden("生きひん", "生きる", "v1", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_sub_dialect_negative_v1() {
    assert_golden("生きひんかった", "生きる", "v1", "～negative→ksb→past");
}

#[test]
fn deconjugate_kansaiben_imperative_v1() {
    assert_golden("生きい", "生きる", "v1", "～imperative (ksb)");
}

#[test]
fn deconjugate_contracted_provisional_conditional_rya_v1() {
    assert_golden("生きりゃ", "生きる", "v1", "～provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_colloquial_causative_negative_v1() {
    assert_golden("生きささない", "生きる", "v1", "～short causative→negative");
}

#[test]
fn deconjugate_polite_non_past_temporal_conditional_v1() {
    assert_golden("生きましたら", "生きる", "v1", "～polite conditional");
}

#[test]
fn deconjugate_polite_non_past_honorific_ninaru_v1() {
    assert_golden("生きになる", "生きる", "v1", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_nasaru_v1() {
    assert_golden("生きなさる", "生きる", "v1", "～honorific");
}

#[test]
fn deconjugate_polite_non_past_honorific_haru_ksb_affirmative_v1() {
    assert_golden("生きはる", "生きる", "v1", "～honorific (ksb)");
}

#[test]
fn deconjugate_plain_non_past_honorific_negative_nasaruna_v1() {
    assert_golden("生きなさるな", "生きる", "v1", "～honorific→imperative negative");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_v1() {
    assert_golden("生きまい", "生きる", "v1", "～negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conjectural_2_v1() {
    assert_golden("生きるまい", "生きる", "v1", "～negative conjectural");
}

#[test]
fn deconjugate_polite_non_past_negative_conjectural_v1() {
    assert_golden("生きますまい", "生きる", "v1", "～polite negative conjectural");
}

#[test]
fn deconjugate_plain_non_past_negative_conditional_v1() {
    assert_golden("生きねば", "生きる", "v1", "～negative conditional");
}

#[test]
fn deconjugate_plain_non_past_colloquial_negative_conditional_v1() {
    assert_golden("生きにゃ", "生きる", "v1", "～colloquial negative conditional");
}

// ─── Verb-producing aux on renyou base (added 2026-05-06) ────────────
//
// Six new rules for compound aux verbs that attach to a verb's masu-
// stem (renyou) and produce another verb of the aux's own class.
// Original goal: cover spans like 思い続けている, 食べ始めた, 走り出した
// that the analyzer's enrich_morph couldn't resolve through the
// per-surface morpheme_conjugations cache. With these rules the
// deconjugator can walk surface → aux-stripped renyou → base verb in
// one BFS pass.

#[test]
fn deconjugate_aux_hajimeru_past_v1() {
    assert_golden("食べ始めた", "食べる", "v1", "～start V-ing→past");
}

#[test]
fn deconjugate_aux_hajimeru_kana_past_v1() {
    assert_golden("食べはじめた", "食べる", "v1", "～start V-ing→past");
}

#[test]
fn deconjugate_aux_tsuzukeru_teiru_v1() {
    assert_golden("食べ続けている", "食べる", "v1", "～continue V-ing→teiru");
}

#[test]
fn deconjugate_aux_tsuzukeru_polite_v1() {
    assert_golden("食べ続けます", "食べる", "v1", "～continue V-ing→polite");
}

#[test]
fn deconjugate_aux_owaru_past_v1() {
    assert_golden("食べ終わった", "食べる", "v1", "～finish V-ing→past");
}

#[test]
fn deconjugate_aux_dasu_past_v1() {
    assert_golden("食べ出した", "食べる", "v1", "～burst into V-ing→past");
}

#[test]
fn deconjugate_aux_sugiru_past_v1() {
    assert_golden("食べすぎた", "食べる", "v1", "～excess V-ing→past");
}

// ─── Te-aux do-for-me variants (separate from existing くれる "statement/request") ──

#[test]
fn deconjugate_aux_te_kureru_past_v1() {
    // Two valid candidates: the existing くれる "statement/request"
    // rule (uses stem-te-verbal, for V-てくれる as a discourse marker)
    // and the new "do for me" rule (uses stem-te, for V-てくれる as
    // the benefactive aux). matches_expected only requires the
    // expected chain to be present in the output set; we lock in the
    // benefactive one.
    assert_golden("食べてくれた", "食べる", "v1", "～do for me→past");
}

#[test]
fn deconjugate_aux_te_morau_past_v1() {
    assert_golden("食べてもらった", "食べる", "v1", "～have someone do→past");
}
