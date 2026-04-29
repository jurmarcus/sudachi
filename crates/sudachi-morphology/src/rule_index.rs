//! Suffix-trie index over deconjugation rules — narrows the
//! per-form rule iteration from O(n_rules) to O(input_length) in
//! practice.
//!
//! The deconjugator's hot loop matches each form's surface against
//! every rule's `con_end` suffix. With ~250 expanded rules and a BFS
//! that visits 10–30 forms per call, the naive O(n_rules) scan does
//! 2,500–7,500 suffix-match probes per `deconjugate()`. Building a
//! daachorse Aho-Corasick automaton over the `con_end` patterns at
//! crate-init time turns each per-form match into a single linear
//! walk: the typical input is 5–10 chars, so the work per form drops
//! by ~50× even before considering automaton constant factors.
//!
//! ## Strategy
//!
//! Daachorse does forward (left-to-right) substring search. We want
//! suffix matches — patterns that end at the input's last position.
//! Easy adaptation: run forward search, filter results to those
//! whose `match.end()` equals the input's byte length.
//!
//! ## Empty-`con_end` rules
//!
//! Rules with `con_end == ""` (pure tag-transition rules like
//! `(stem-mizenkei) → stem-a`) match every form trivially. These
//! can't go through the automaton (zero-length patterns are
//! pathological); they live in a separate `always_match` list and
//! are appended to every match enumeration.
//!
//! ## Rule grouping
//!
//! Different rules can share the same `con_end` (e.g., both the
//! plain past rule and the polite past rule consume `た` under
//! different `con_tag` constraints). The index groups rules by
//! `con_end` so the automaton has one pattern per unique suffix;
//! each match expands to the full set of rules with that suffix.

use crate::rule::Rule;
use daachorse::DoubleArrayAhoCorasick;
use std::collections::HashMap;

/// Index over a rule corpus accelerating "which rules match this
/// input as a suffix?".
pub struct RuleIndex {
    /// Aho-Corasick automaton over the unique non-empty `con_end`
    /// patterns. `match.value()` is an index into [`pattern_to_rules`].
    automaton: DoubleArrayAhoCorasick<u32>,
    /// Per pattern (in automaton insertion order), the list of rule
    /// indices in the original `Vec<Rule>` that share that `con_end`.
    pattern_to_rules: Vec<Vec<usize>>,
    /// Rules with `con_end == ""` — match every form trivially.
    always_match: Vec<usize>,
}

impl RuleIndex {
    /// Build the index from a flat rule list. Construction cost is
    /// paid once at crate init; per-call lookups are linear in
    /// input length.
    pub fn build(rules: &[Rule]) -> Self {
        let mut always_match = Vec::new();
        let mut suffix_to_pattern_idx: HashMap<String, usize> = HashMap::new();
        let mut patterns: Vec<String> = Vec::new();
        let mut pattern_to_rules: Vec<Vec<usize>> = Vec::new();

        for (rule_idx, rule) in rules.iter().enumerate() {
            if rule.con_end.is_empty() {
                always_match.push(rule_idx);
                continue;
            }
            let pattern_idx = match suffix_to_pattern_idx.get(&rule.con_end) {
                Some(&idx) => idx,
                None => {
                    let new_idx = patterns.len();
                    patterns.push(rule.con_end.clone());
                    pattern_to_rules.push(Vec::new());
                    suffix_to_pattern_idx.insert(rule.con_end.clone(), new_idx);
                    new_idx
                }
            };
            pattern_to_rules[pattern_idx].push(rule_idx);
        }

        let automaton = DoubleArrayAhoCorasick::new(patterns)
            .expect("daachorse build over deconjugation rule patterns");

        Self {
            automaton,
            pattern_to_rules,
            always_match,
        }
    }

    /// Yield the indices of every rule whose `con_end` is a suffix
    /// of `text`, plus every `con_end == ""` rule. Order: suffix
    /// matches first (in automaton order), then always-match.
    pub fn matching_rules<'a>(&'a self, text: &'a str) -> impl Iterator<Item = usize> + 'a {
        let input_byte_len = text.len();
        let suffix_matches = self
            .automaton
            .find_overlapping_iter(text)
            .filter(move |m| m.end() == input_byte_len)
            .flat_map(move |m| self.pattern_to_rules[m.value() as usize].iter().copied());
        suffix_matches.chain(self.always_match.iter().copied())
    }

    /// How many distinct `con_end` patterns are in the automaton.
    /// Useful for diagnostics and tests.
    pub fn pattern_count(&self) -> usize {
        self.pattern_to_rules.len()
    }

    /// How many empty-`con_end` rules are in the always-match list.
    pub fn always_match_count(&self) -> usize {
        self.always_match.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::load_default_rules;

    #[test]
    fn build_index_over_default_rules() {
        let rules = load_default_rules();
        let index = RuleIndex::build(&rules);
        assert!(index.pattern_count() > 50, "expected many distinct con_end patterns");
        // There should be at least a few empty-con_end rules
        // (intermediate stem transitions).
        assert!(index.always_match_count() > 0);
    }

    #[test]
    fn matching_rules_finds_past_rule_for_ta_input() {
        let rules = load_default_rules();
        let index = RuleIndex::build(&rules);
        // For input "食べた", we should match at least the past rule
        // (con_end="た"). Plus all always-match rules.
        let matches: Vec<usize> = index.matching_rules("食べた").collect();
        let has_past = matches.iter().any(|&i| {
            let r = &rules[i];
            r.con_end == "た" && r.detail == "past"
        });
        assert!(has_past, "no past rule matched for 食べた");
    }

    #[test]
    fn matching_rules_includes_always_match() {
        let rules = load_default_rules();
        let index = RuleIndex::build(&rules);
        let matches: Vec<usize> = index.matching_rules("anything").collect();
        // Every always_match rule should appear.
        for &am_idx in &index.always_match {
            assert!(matches.contains(&am_idx));
        }
    }

    #[test]
    fn matching_rules_skips_rules_without_suffix_match() {
        let rules = load_default_rules();
        let index = RuleIndex::build(&rules);
        // For input "猫" (a noun, ends in 猫), no rule with con_end
        // ending in て or ない etc. should fire.
        let matches: Vec<usize> = index.matching_rules("猫").collect();
        for &i in &matches {
            let r = &rules[i];
            assert!(
                r.con_end.is_empty() || "猫".ends_with(&r.con_end),
                "rule with con_end={:?} matched on 猫",
                r.con_end,
            );
        }
        // Sanity: "ない" rule should NOT be in matches.
        for &i in &matches {
            let r = &rules[i];
            assert_ne!(r.con_end, "ない", "negative rule shouldn't match on 猫");
        }
    }

    #[test]
    fn rules_with_kind_correctly_indexed() {
        let rules = load_default_rules();
        let index = RuleIndex::build(&rules);
        let matches: Vec<usize> = index.matching_rules("ない").collect();
        // Some matches should be Standard / NeverFinal / OnlyFinal —
        // we don't filter by kind here (the deconjugator does that).
        let kinds: std::collections::HashSet<_> =
            matches.iter().map(|&i| std::mem::discriminant(&rules[i].kind)).collect();
        assert!(!kinds.is_empty(), "no rules matched ない");
    }
}
