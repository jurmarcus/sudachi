//! Backward deconjugation — given a surface, return all candidate
//! base forms with their derivation chains.
//!
//! Algorithm (based on the nazeka/Yomichan deinflector lineage):
//!
//! 1. Seed a queue with the input as a [`Form`] with empty tags.
//! 2. Pop forms; for each, try every rule. If the rule matches, push
//!    the resulting form back onto the queue.
//! 3. A form is "valid as endpoint" if it has at least one rule
//!    applied AND its last rule wasn't a `NeverFinal`.
//! 4. Cycle detection: track every text the chain has produced; reject
//!    rules that would re-produce a seen text.
//! 5. Length / depth / tag-density limits to keep the search bounded.

use crate::rule::{ContextKind, Rule, RuleKind, load_default_rules};

/// One candidate deinflection of an input surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Form {
    /// The deinflected text — what the original surface could be a
    /// conjugation of.
    pub text: String,
    /// The original surface that started this chain.
    pub original: String,
    /// Accumulated grammatical tags. The first tag is the
    /// originating verb/adjective class (`v1`, `v5r`, etc.); later
    /// tags are intermediate stem markers.
    pub tags: Vec<String>,
    /// Names of rules applied, in chain order. E.g.
    /// `["past", "negative"]` for `食べなかった → 食べる`.
    pub process: Vec<String>,
    /// Every text encountered in this chain (for cycle detection).
    seen: Vec<String>,
    /// Reserved (legacy field — see commit history). Always false
    /// in the corrected algorithm.
    pub is_intermediate: bool,
    /// Reserved (legacy field — see commit history). Always false.
    pub locked: bool,
}

impl Form {
    fn seed(input: &str) -> Self {
        Self {
            text: input.to_string(),
            original: input.to_string(),
            tags: Vec::new(),
            process: Vec::new(),
            seen: vec![input.to_string()],
            is_intermediate: false,
            locked: false,
        }
    }
}

/// Run the deconjugator against `input` using the bundled rules.
/// Returns every valid endpoint form found.
pub fn deconjugate(input: &str) -> Vec<Form> {
    deconjugate_with(input, &DEFAULT_RULES)
}

/// Same as [`deconjugate`] but using a custom rule set.
///
/// Algorithm: BFS over the form-graph. Every form (including
/// intermediate stems) gets emitted; the caller filters by terminal
/// tag (`v1`, `v5*`, `adj-i`, `vk`, `vs-i`, etc.) to find candidate
/// dictionary forms. This matches nazeka's "process all forms,
/// caller chooses what's a valid result" model.
///
/// Rule kind semantics (reverse of what the names suggest):
/// - `OnlyFinal`: only applies as the FIRST rule in a chain (when
///   `form.tags` is empty). E.g., imperative.
/// - `NeverFinal`: only applies as a NON-FIRST rule (when
///   `form.tags` is non-empty). E.g., stem expansions like
///   stem-mizenkei → v1.
/// - `Standard`: applies anywhere.
pub fn deconjugate_with(input: &str, rules: &[Rule]) -> Vec<Form> {
    if input.is_empty() {
        return Vec::new();
    }
    use std::collections::HashSet;

    let mut processed: Vec<Form> = vec![Form::seed(input)];
    let mut novel: Vec<Form> = vec![Form::seed(input)];
    let mut seen: HashSet<(String, Vec<String>, Vec<String>)> = HashSet::new();
    seen.insert((input.to_string(), Vec::new(), Vec::new()));

    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 50_000;

    while !novel.is_empty() && iterations < MAX_ITERATIONS {
        let mut new_novel: Vec<Form> = Vec::new();
        for form in &novel {
            for rule in rules {
                iterations += 1;
                if iterations >= MAX_ITERATIONS {
                    break;
                }
                if let Some(new_form) = apply_rule(form, rule) {
                    let key = (
                        new_form.text.clone(),
                        new_form.tags.clone(),
                        new_form.process.clone(),
                    );
                    if seen.insert(key) {
                        new_novel.push(new_form);
                    }
                }
            }
        }
        processed.extend(novel.iter().cloned());
        novel = new_novel;
    }
    processed.extend(novel);

    // Filter to forms that have actually had a rule applied.
    processed
        .into_iter()
        .filter(|f| !f.process.is_empty())
        .collect()
}

/// Try to apply one rule to one form. Returns the new form if it
/// matches, None otherwise.
fn apply_rule(form: &Form, rule: &Rule) -> Option<Form> {
    // Substitution: pure surface rewrite, no tag matching.
    if matches!(rule.kind, RuleKind::Substitution) {
        if !form.text.ends_with(&rule.con_end) || rule.con_end.is_empty() {
            return None;
        }
        let prefix = &form.text[..form.text.len() - rule.con_end.len()];
        let new_text = format!("{}{}", prefix, rule.dec_end);
        if form.seen.contains(&new_text) {
            return None;
        }
        let mut next = form.clone();
        next.text = new_text.clone();
        next.seen.push(new_text);
        if !rule.detail.is_empty() {
            next.process.push(rule.detail.clone());
        }
        return Some(next);
    }

    // Rule-kind chain-position guards (nazeka semantics):
    // OnlyFinal only fires as the first rule (when form.tags is empty);
    // NeverFinal only fires as a non-first rule (when form.tags is not).
    // Context rules carry per-kind predicates (see JL Deconjugator.cs).
    match rule.kind {
        RuleKind::OnlyFinal if !form.tags.is_empty() => return None,
        RuleKind::NeverFinal if form.tags.is_empty() => return None,
        RuleKind::Context(ContextKind::V1InfTrap) => {
            // Block when form has exactly one tag and it's "stem-ren".
            // Prevents the teru-stripping rule misfiring on what's
            // actually a v1 verb's masu-stem form.
            if form.tags.len() == 1 && form.tags[0] == "stem-ren" {
                return None;
            }
        }
        RuleKind::Context(ContextKind::SaSpecial) => {
            // Block when the text portion before con_end ends in さ
            // (would create double-さ in short causative).
            if form.text.is_empty() {
                return None;
            }
            let con_end_chars = rule.con_end.chars().count();
            let text_chars = form.text.chars().count();
            if text_chars < con_end_chars {
                return None;
            }
            let prefix: String = form
                .text
                .chars()
                .take(text_chars - con_end_chars)
                .collect();
            if prefix.ends_with('さ') {
                return None;
            }
        }
        RuleKind::Context(ContextKind::Other) => return None,
        _ => {}
    }

    // All other rules: con_end must match suffix.
    if !form.text.ends_with(&rule.con_end) {
        return None;
    }
    // Sanity bounds — same as nazeka.
    if form.text.chars().count() > form.original.chars().count() + 10 {
        return None;
    }
    if form.tags.len() > form.original.chars().count() + 6 {
        return None;
    }
    // Empty-detail rules can't be the FIRST rule applied (they're
    // intermediate-only roots). Matches nazeka's `if my_rule.detail
    // == "" && my_form.tags.length == 0 return`.
    if rule.detail.is_empty() && form.tags.is_empty() {
        return None;
    }
    // Tag-history check: if the form has accumulated tags, the most
    // recent tag must match the rule's con_tag. The "uninflectable"
    // wildcard always matches.
    if !form.tags.is_empty() {
        let last_tag = form.tags.last().unwrap();
        if rule.con_tag != "uninflectable" && rule.con_tag != *last_tag {
            return None;
        }
    }

    // JL's anti-empty check: block rules that would consume the
    // entire text without producing anything (text.len == con_end.len
    // AND dec_end empty).
    if form.text.len() == rule.con_end.len() && rule.dec_end.is_empty() {
        return None;
    }

    let prefix = &form.text[..form.text.len() - rule.con_end.len()];
    let new_text = format!("{}{}", prefix, rule.dec_end);
    let mut next = form.clone();
    next.text = new_text;
    // Drop the per-form seen-text cycle check — JL doesn't have one,
    // relies on the bounds + new-novel duplicate check at the BFS
    // layer + MAX_ITERATIONS cap. Forms with identical (text, tags,
    // process) get deduplicated below.
    if !rule.detail.is_empty() {
        next.process.push(rule.detail.clone());
    }
    if next.tags.is_empty() {
        // First rule applied: tags become [con_tag, dec_tag].
        next.tags.push(rule.con_tag.clone());
    }
    next.tags.push(rule.dec_tag.clone());

    Some(next)
}

// Lazy-static — load rules once.
use std::sync::LazyLock;
static DEFAULT_RULES: LazyLock<Vec<Rule>> = LazyLock::new(load_default_rules);

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: assert that `deconjugate(input)` returns at least one
    /// candidate matching `(text, last_tag)`.
    fn assert_candidate(input: &str, expected_text: &str, expected_last_tag: &str) {
        let forms = deconjugate(input);
        let matches: Vec<_> = forms
            .iter()
            .filter(|f| {
                f.text == expected_text
                    && f.tags.last().map(String::as_str) == Some(expected_last_tag)
            })
            .collect();
        assert!(
            !matches.is_empty(),
            "deconjugate({:?}) did not produce {:?} with last tag {:?}.\nGot: {:?}",
            input,
            expected_text,
            expected_last_tag,
            forms
                .iter()
                .map(|f| (f.text.as_str(), f.tags.last().map(String::as_str)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn deconjugates_taberu_past() {
        // 食べた → 食べる (v1, past)
        assert_candidate("食べた", "食べる", "v1");
    }

    #[test]
    fn deconjugates_kaita_past() {
        // 書いた → 書く (v5k, past)
        assert_candidate("書いた", "書く", "v5k");
    }

    #[test]
    fn deconjugates_yonda_past() {
        // 読んだ → 読む (v5m, past)
        assert_candidate("読んだ", "読む", "v5m");
    }

    #[test]
    fn deconjugates_negative() {
        // 食べない → 食べる (v1, negative)
        assert_candidate("食べない", "食べる", "v1");
    }

    #[test]
    fn deconjugates_polite_form() {
        // 食べます → 食べる (v1, polite)
        assert_candidate("食べます", "食べる", "v1");
    }

    #[test]
    fn deconjugates_te_form() {
        // 食べて → 食べる (v1, te)
        assert_candidate("食べて", "食べる", "v1");
    }

    #[test]
    fn deconjugates_chained_polite_past() {
        // 食べました → 食べる (v1) via polite + past chain.
        assert_candidate("食べました", "食べる", "v1");
    }

    #[test]
    fn deconjugates_negative_past() {
        // 食べなかった → 食べる (v1)
        assert_candidate("食べなかった", "食べる", "v1");
    }

    #[test]
    fn deconjugates_iku_irregular_past() {
        // 行った → 行く (v5k-s, past).
        assert_candidate("行った", "行く", "v5k-s");
    }

    #[test]
    fn deconjugates_suru_irregular() {
        // した → する (vs-i, past)
        assert_candidate("した", "する", "vs-i");
    }

    #[test]
    fn deconjugates_kuru_irregular() {
        // 来た → 来る (vk, past)
        assert_candidate("来た", "来る", "vk");
    }

    #[test]
    fn empty_input_returns_empty() {
        assert!(deconjugate("").is_empty());
    }

    #[test]
    fn unknown_input_returns_empty_or_self() {
        // Pure katakana noun shouldn't deconjugate.
        let forms = deconjugate("コンピューター");
        // Either empty or just the input back as-is — but no real
        // verb deinflection candidates.
        let has_verb_candidate = forms.iter().any(|f| {
            f.tags.first().is_some_and(|t| t.starts_with("v") || t.starts_with("adj"))
        });
        assert!(!has_verb_candidate, "katakana noun produced verb candidates: {:?}", forms);
    }
}
