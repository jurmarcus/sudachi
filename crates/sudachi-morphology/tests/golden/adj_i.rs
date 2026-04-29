//! Golden tests ported from JL's `DeconjugatorTestsForAdjI.cs`.
//! 27 test cases proving deconjugator output matches
//! JL's expectations for class AdjI.

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
fn deconjugate_stem_adj_i() {
    assert_golden("小さ", "小さい", "adj-i", "～stem");
}

#[test]
fn deconjugate_plain_non_past_negative_adj_i() {
    assert_golden("小さくない", "小さい", "adj-i", "～negative");
}

#[test]
fn deconjugate_plain_past_affirmative_adj_i() {
    assert_golden("小さかった", "小さい", "adj-i", "～past");
}

#[test]
fn deconjugate_plain_past_negative_adj_i() {
    assert_golden("小さくなかった", "小さい", "adj-i", "～negative→past");
}

#[test]
fn deconjugate_plain_te_form_affirmative_adj_i() {
    assert_golden("小さくて", "小さい", "adj-i", "～te");
}

#[test]
fn deconjugate_plain_te_form_negative_adj_i() {
    assert_golden("小さくなくて", "小さい", "adj-i", "～negative→te");
}

#[test]
fn deconjugate_provisional_conditional_affirmative_adj_i() {
    assert_golden("小さければ", "小さい", "adj-i", "～provisional conditional");
}

#[test]
fn deconjugate_provisional_conditional_negative_adj_i() {
    assert_golden("小さくなければ", "小さい", "adj-i", "～negative→provisional conditional");
}

#[test]
fn deconjugate_temporal_conditional_affirmative_adj_i() {
    assert_golden("小さかったら", "小さい", "adj-i", "～conditional");
}

#[test]
fn deconjugate_formal_conditional_affirmative_adj_i() {
    assert_golden("小さかったらば", "小さい", "adj-i", "～formal conditional");
}

#[test]
fn deconjugate_temporal_conditional_negative_adj_i() {
    assert_golden("小さくなかったら", "小さい", "adj-i", "～negative→conditional");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_adj_i() {
    assert_golden("小さすぎる", "小さい", "adj-i", "～excess");
}

#[test]
fn deconjugate_plain_non_past_sugiru_affirmative_2_adj_i() {
    assert_golden("小さ過ぎる", "小さい", "adj-i", "～excess");
}

#[test]
fn deconjugate_plain_non_past_sou_affirmative_adj_i() {
    assert_golden("小さそう", "小さい", "adj-i", "～seemingness");
}

#[test]
fn deconjugate_topic_or_condition_adj_i() {
    assert_golden("小さくては", "小さい", "adj-i", "～topic/condition");
}

#[test]
fn deconjugate_contracted_topic_or_condition_cha_adj_i() {
    assert_golden("小さくちゃ", "小さい", "adj-i", "～topic/condition→contracted");
}

#[test]
fn deconjugate_plain_non_past_contracted_provisional_conditional_negative_kya_adj_i() {
    assert_golden("小さくなきゃ", "小さい", "adj-i", "～negative→provisional conditional→contracted");
}

#[test]
fn deconjugate_plain_non_past_kansaiben_negative_adj_i() {
    assert_golden("小さくへん", "小さい", "adj-i", "～negative→ksb");
}

#[test]
fn deconjugate_plain_past_kansaiben_negative_adj_i() {
    assert_golden("小さくへんかった", "小さい", "adj-i", "～negative→ksb→past");
}

#[test]
fn deconjugate_contracted_provisional_conditional_rya_adj_i() {
    assert_golden("小さきゃ", "小さい", "adj-i", "～provisional conditional→contracted");
}

#[test]
fn deconjugate_adverbial_stem_adj_i() {
    assert_golden("小さく", "小さい", "adj-i", "～adverbial stem");
}

#[test]
fn deconjugate_noun_form_adj_i() {
    assert_golden("小ささ", "小さい", "adj-i", "～noun form");
}

#[test]
fn deconjugate_classical_attributive_adj_i() {
    assert_golden("小さき", "小さい", "adj-i", "～classical attributive");
}

#[test]
fn deconjugate_ge_adj_i() {
    assert_golden("怪しげ", "怪しい", "adj-i", "～seeming");
}

#[test]
fn deconjugate_ge_2_adj_i() {
    assert_golden("怪し気", "怪しい", "adj-i", "～seeming");
}

#[test]
fn deconjugate_noun_form_seemingness_adj_i() {
    assert_golden("良さそう", "良い", "adj-i", "～noun form→seemingness");
}

#[test]
fn deconjugate_volitional_adj_i() {
    assert_golden("良かろう", "良い", "adj-i", "～volitional");
}
