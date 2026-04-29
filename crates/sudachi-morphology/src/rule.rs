//! Rule representation for the backward deconjugator.
//!
//! Rules are loaded from `data/deconjugation_rules.json`. The file
//! format uses parallel-array compression: one rule entry like
//! `{con_end: ["く", "す", ...], con_tag: "...", dec_end: [...], dec_tag: [...]}`
//! expands into N concrete instances at load time, where N is the
//! length of the longest array. This module performs that expansion
//! up front so the deconjugator iterates a flat list.
//!
//! ## Six rule kinds
//!
//! - **Standard** — applies whenever surface matches; may appear as
//!   any step in a deinflection chain.
//! - **OnlyFinal** — same matching as Standard, but the resulting
//!   form is locked: no further rules apply on top.
//! - **NeverFinal** — same matching as Standard, but the resulting
//!   form is invalid as an endpoint (must be extended by another
//!   rule). Used for stems (e.g., izenkei) that are intermediate
//!   roots.
//! - **Rewrite** — single-pair surface rewrite (です → でした for
//!   past). No array expansion.
//! - **Context** — requires a named contextual condition beyond
//!   simple suffix matching (e.g., する's irregular short
//!   causative).
//! - **Substitution** — pure surface rewrite (０-９ ↔ 0-9). Ignores
//!   tags entirely.

use serde::Deserialize;

/// One concrete deconjugation rule after parallel-array expansion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    /// What kind of rule this is. Determines matching semantics and
    /// how it interacts with chain expansion.
    pub kind: RuleKind,
    /// Suffix the input surface must end with for this rule to fire.
    pub con_end: String,
    /// Suffix to produce on the deinflected output. The output is
    /// `input[..-con_end.len()] + dec_end`.
    pub dec_end: String,
    /// Tag that the input form must carry as its most recent tag, OR
    /// "uninflectable" sentinel meaning "applies regardless of input
    /// tag history". When the input form has no prior tags, any rule
    /// matches (the rule's con_tag is then taken as the originating
    /// class for the form).
    pub con_tag: String,
    /// Tag(s) added to the output form. For most rules this is one
    /// tag; for rules with parallel `dec_tag` arrays it's the
    /// per-instance tag at the same index as `con_end`.
    pub dec_tag: String,
    /// Human-readable name of what this rule does ("past",
    /// "negative", "imperative", etc.). Empty string means
    /// "intermediate" — the rule advances the chain but doesn't
    /// represent a user-facing grammatical operation.
    pub detail: String,
}

/// Kind of rule. Determines how the deconjugator treats matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    /// Applies anywhere in a chain. Output is a valid endpoint.
    Standard,
    /// Applies anywhere; output is LOCKED (no further rules can
    /// apply on top). E.g., imperative.
    OnlyFinal,
    /// Applies only as INTERMEDIATE; output must be extended by
    /// another rule (cannot be returned as a final candidate). E.g.,
    /// stems.
    NeverFinal,
    /// Single-pair rewrite, no array expansion. E.g., です → でした.
    Rewrite,
    /// Requires a named contextual condition (saspecial = する's
    /// irregular short causative).
    Context(ContextKind),
    /// Pure surface rewrite, ignores tags. E.g., ０-９ ↔ 0-9.
    Substitution,
}

/// Named context conditions that gate certain rules. Per JL's
/// `ContextRuleDeconjugate` switch — see
/// [`crate::deconjugate::apply_rule`] for the actual predicate
/// implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextKind {
    /// する's irregular short causative — blocks when the text
    /// portion before `con_end` ends in さ (prevents double-さ in
    /// compound short causatives).
    SaSpecial,
    /// V1 infinitive trap — blocks the te-iru → te-ru contraction
    /// rule when the form has exactly one tag and it's `stem-ren`
    /// (otherwise misfires on what's actually a v1 verb's masu-stem).
    V1InfTrap,
    /// Other / unrecognised context kind. Loaded but never fires.
    Other,
}

/// Raw JSON shape of one rule entry, before parallel-array expansion.
#[derive(Debug, Clone, Deserialize)]
struct RawRule {
    #[serde(default = "default_type")]
    r#type: String,
    #[serde(default)]
    con_end: serde_json::Value, // String or Vec<String>
    #[serde(default)]
    dec_end: serde_json::Value,
    #[serde(default)]
    con_tag: serde_json::Value,
    #[serde(default)]
    dec_tag: serde_json::Value,
    #[serde(default)]
    detail: String,
    #[serde(default)]
    contextrule: Option<String>,
}

fn default_type() -> String {
    "stdrule".to_string()
}

/// Parse the bundled `deconjugation_rules.json` and expand parallel
/// arrays into a flat `Vec<Rule>`.
pub fn load_default_rules() -> Vec<Rule> {
    let raw = include_str!("../data/deconjugation_rules.json");
    let entries: Vec<serde_json::Value> = serde_json::from_str(raw)
        .expect("bundled deconjugation_rules.json must be valid JSON");
    let mut out = Vec::with_capacity(entries.len() * 8);
    for e in entries {
        // Skip string entries (comments / section headers).
        if !e.is_object() {
            continue;
        }
        let raw: RawRule = serde_json::from_value(e).expect("rule entry must match RawRule");
        out.extend(expand_rule(&raw));
    }
    out
}

/// Expand one raw rule entry into one or more flat `Rule`s.
pub fn expand_rule(raw: &RawRule) -> Vec<Rule> {
    let kind = match raw.r#type.as_str() {
        "stdrule" => RuleKind::Standard,
        "onlyfinalrule" => RuleKind::OnlyFinal,
        "neverfinalrule" => RuleKind::NeverFinal,
        "rewriterule" => RuleKind::Rewrite,
        "substitution" => RuleKind::Substitution,
        "contextrule" => match raw.contextrule.as_deref() {
            Some("saspecial") => RuleKind::Context(ContextKind::SaSpecial),
            Some("v1inftrap") => RuleKind::Context(ContextKind::V1InfTrap),
            _ => RuleKind::Context(ContextKind::Other),
        },
        other => panic!("unknown rule type: {}", other),
    };

    // Helper: coerce a Value to Vec<String>. String becomes [s];
    // Array<String> stays as-is; null becomes [""].
    fn as_strings(v: &serde_json::Value) -> Vec<String> {
        match v {
            serde_json::Value::String(s) => vec![s.clone()],
            serde_json::Value::Array(a) => a
                .iter()
                .filter_map(|e| e.as_str().map(String::from))
                .collect(),
            serde_json::Value::Null => vec![String::new()],
            _ => vec![String::new()],
        }
    }

    let con_ends = as_strings(&raw.con_end);
    let dec_ends = as_strings(&raw.dec_end);
    let con_tags = as_strings(&raw.con_tag);
    let dec_tags = as_strings(&raw.dec_tag);

    // Determine expansion length: longest array.
    let n = [con_ends.len(), dec_ends.len(), con_tags.len(), dec_tags.len()]
        .into_iter()
        .max()
        .unwrap_or(1);

    // Helper: take element at index, or the sole element if the array
    // has length 1 (broadcasting).
    fn pick(arr: &[String], i: usize) -> String {
        if arr.len() == 1 {
            arr[0].clone()
        } else if i < arr.len() {
            arr[i].clone()
        } else {
            String::new()
        }
    }

    (0..n)
        .map(|i| Rule {
            kind,
            con_end: pick(&con_ends, i),
            dec_end: pick(&dec_ends, i),
            con_tag: pick(&con_tags, i),
            dec_tag: pick(&dec_tags, i),
            detail: raw.detail.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_bundled_rules() {
        let rules = load_default_rules();
        // Ought to be a few hundred after expansion.
        assert!(rules.len() > 100, "expected > 100 rules, got {}", rules.len());
        // Spot-check: there should be a rule producing past form.
        let past_rules: Vec<_> = rules
            .iter()
            .filter(|r| r.detail == "past" || r.con_end.contains('た'))
            .collect();
        assert!(!past_rules.is_empty(), "no past-tense rules found");
    }

    #[test]
    fn expands_parallel_arrays_correctly() {
        let raw = RawRule {
            r#type: "stdrule".to_string(),
            con_end: serde_json::json!(["か", "き", "く"]),
            dec_end: serde_json::json!(["く", "く", "く"]),
            con_tag: serde_json::json!("stem"),
            dec_tag: serde_json::json!(["v5k", "v5k", "v5k"]),
            detail: "test".to_string(),
            contextrule: None,
        };
        let expanded = expand_rule(&raw);
        assert_eq!(expanded.len(), 3);
        assert_eq!(expanded[0].con_end, "か");
        assert_eq!(expanded[1].con_end, "き");
        assert_eq!(expanded[2].con_end, "く");
        for r in &expanded {
            assert_eq!(r.dec_end, "く");
            assert_eq!(r.con_tag, "stem");
            assert_eq!(r.dec_tag, "v5k");
        }
    }

    #[test]
    fn rewrite_rule_doesnt_expand() {
        let raw = RawRule {
            r#type: "rewriterule".to_string(),
            con_end: serde_json::json!("でした"),
            dec_end: serde_json::json!("です"),
            con_tag: serde_json::json!("stem-past"),
            dec_tag: serde_json::json!("exp"),
            detail: "past".to_string(),
            contextrule: None,
        };
        let expanded = expand_rule(&raw);
        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0].kind, RuleKind::Rewrite);
        assert_eq!(expanded[0].con_end, "でした");
        assert_eq!(expanded[0].dec_end, "です");
    }

    #[test]
    fn context_rule_carries_kind() {
        let raw = RawRule {
            r#type: "contextrule".to_string(),
            con_end: serde_json::json!("す"),
            dec_end: serde_json::json!(""),
            con_tag: serde_json::json!("v5s"),
            dec_tag: serde_json::json!("stem-a"),
            detail: "short causative".to_string(),
            contextrule: Some("saspecial".to_string()),
        };
        let expanded = expand_rule(&raw);
        assert_eq!(expanded.len(), 1);
        assert!(matches!(
            expanded[0].kind,
            RuleKind::Context(ContextKind::SaSpecial)
        ));
    }
}
