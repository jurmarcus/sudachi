//! Golden tests ported from JL's `DeconjugatorTestsForCop.cs`.
//! 10 test cases proving deconjugator output matches
//! JL's expectations for class Cop.

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
fn deconjugate_past_です() {
    assert_golden("でした", "です", "cop", "～past");
}

#[test]
fn deconjugate_conditional_です() {
    assert_golden("でしたら", "です", "cop", "～conditional");
}

#[test]
fn deconjugate_conjectural_です() {
    assert_golden("でしょう", "です", "cop", "～conjectural");
}

#[test]
fn deconjugate_te_です() {
    assert_golden("でして", "です", "cop", "～te");
}

#[test]
fn deconjugate_past_だ() {
    assert_golden("だった", "だ", "cop", "～past");
}

#[test]
fn deconjugate_conditional_だ() {
    assert_golden("だったら", "だ", "cop", "～conditional");
}

#[test]
fn deconjugate_conjectural_だ() {
    assert_golden("だろう", "だ", "cop", "～conjectural");
}

#[test]
fn deconjugate_past_じゃ() {
    assert_golden("じゃった", "じゃ", "cop", "～past");
}

#[test]
fn deconjugate_conditional_じゃ() {
    assert_golden("じゃったら", "じゃ", "cop", "～conditional");
}

#[test]
fn deconjugate_conjectural_じゃ() {
    assert_golden("じゃろう", "じゃ", "cop", "～conjectural");
}
