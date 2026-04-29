//! [`Lexicon`] — vocab-knowledge callback exposed to optimizer stages.
//!
//! Some stages need information beyond the bare token stream — is
//! this a known vocab compound? is this a valid verb conjugation?
//! The `Lexicon` trait is the consumer-supplied oracle for those
//! questions. sudachi-optimizer doesn't own a vocab catalog (that
//! would be a domain leak); consumers implement this trait and pass
//! it to [`Optimizer::tokenize_with`](crate::Optimizer::tokenize_with)
//! or [`optimize`](crate::optimize).
//!
//! ## Methods
//!
//! - [`Lexicon::has_compound_entry`] — vocab catalog membership for
//!   a dictionary-form term. Three-state (`Some(true)` / `Some(false)`
//!   / `None`) so consumers can express "no info" distinctly from
//!   "definitely not present".
//!
//! - [`Lexicon::lookup_conjugated_form`] — deconjugate a surface to
//!   candidate base forms. The default implementation uses the
//!   bundled [`sudachi_morphology`] rule corpus (4,781 JL goldens'
//!   worth of coverage); consumers with their own catalog (e.g.,
//!   jisho-core's `vocab_forms` table) can override to merge their
//!   own knowledge with the morphology library's.
//!
//! ## Three-state knowledge
//!
//! `has_compound_entry` returns `Option<bool>`:
//! - `Some(true)` — definitely in the catalog
//! - `Some(false)` — definitely not in the catalog
//! - `None` — no information available (default for [`EmptyLexicon`])
//!
//! Stages typically split on these three cases:
//! - `Some(true)` → keep the morpheme intact (catalog knows it)
//! - `Some(false)` → apply the split / repair confidently
//! - `None` → apply the split eagerly (no veto from the lexicon)

use sudachi_morphology::Form;

/// Vocab + conjugation knowledge interface implemented by consumers.
///
/// All methods have working defaults so implementing only the
/// methods your consumer has data for is fine. The default
/// `lookup_conjugated_form` uses sudachi-morphology's bundled rule
/// corpus, which is suitable for any consumer that doesn't override
/// it.
pub trait Lexicon {
    /// Does this dictionary form exist as a single entry in the
    /// consumer's vocab catalog?
    ///
    /// - `Some(true)` — known compound entry (e.g., 滲み出す in
    ///   JMDict). Stages should keep it intact.
    /// - `Some(false)` — confirmed not in the catalog. Stages may
    ///   apply repairs confidently.
    /// - `None` — no information. Stages typically apply repairs
    ///   eagerly (no veto). Default behaviour.
    fn has_compound_entry(&self, _term: &str) -> Option<bool> {
        None
    }

    /// Deconjugate `surface` to candidate base forms with their
    /// derivation chains. Used by stages that need to validate
    /// "is this token a valid verb form?" before merging or
    /// splitting.
    ///
    /// The default implementation calls
    /// [`sudachi_morphology::deconjugate`], which is bit-for-bit
    /// equivalent to JL/nazeka's deinflector across 4,781 golden
    /// test cases.
    ///
    /// Override when your consumer has a richer notion of "valid
    /// form" — e.g., jisho-core can additionally check its
    /// `vocab_forms` table for hits the rule-based deconjugator
    /// misses (irregular forms, slang specific to a dictionary
    /// corpus).
    fn lookup_conjugated_form(&self, surface: &str) -> Vec<Form> {
        sudachi_morphology::deconjugate(surface)
    }

    /// Convenience: any deconjugation candidate that ends in a
    /// verb-class tag (`v1`, `v5*`, `vk`, `vs-i`, `vs-s`, `v5aru`,
    /// `v5r-i`, etc.).
    ///
    /// Use this in stages that just need "is this surface a verb
    /// form?" without inspecting the chain.
    fn is_valid_verb(&self, surface: &str) -> bool {
        self.lookup_conjugated_form(surface)
            .iter()
            .any(|f| f.tags.last().is_some_and(|t| t.starts_with('v')))
    }

    /// Convenience: any candidate that ends in a verb-class tag
    /// AND has `"past"` somewhere in its derivation chain. Mirrors
    /// JL's `IsNdaVerbForm` and similar past-tense validators.
    fn is_valid_verb_past(&self, surface: &str) -> bool {
        self.lookup_conjugated_form(surface).iter().any(|f| {
            f.tags.last().is_some_and(|t| t.starts_with('v'))
                && f.process.iter().any(|p| p == "past")
        })
    }

    /// Convenience: does the surface deconjugate to a candidate of
    /// any of the listed JMdict verb-class tags (e.g., `["v5b", "v5m",
    /// "v5n", "v5g"]` for んだ-class verbs)?
    fn is_verb_of_class(&self, surface: &str, tags: &[&str]) -> bool {
        self.lookup_conjugated_form(surface)
            .iter()
            .any(|f| f.tags.last().is_some_and(|t| tags.contains(&t.as_str())))
    }
}

/// Null vocab implementation — `has_compound_entry` returns `None`
/// for every query, but `lookup_conjugated_form` still works (uses
/// the default morphology-backed deconjugator).
///
/// Use this from consumers without a vocab catalog (search engines
/// doing pure tokenisation).
pub struct EmptyLexicon;

impl Lexicon for EmptyLexicon {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lexicon_returns_none_for_compound_check() {
        assert_eq!(EmptyLexicon.has_compound_entry("食べ終わる"), None);
    }

    #[test]
    fn empty_lexicon_still_provides_morphology() {
        // EmptyLexicon doesn't override lookup_conjugated_form, so
        // the default morphology-backed impl is available.
        let forms = EmptyLexicon.lookup_conjugated_form("食べた");
        assert!(!forms.is_empty(), "default impl should find candidates");
        let has_taberu = forms
            .iter()
            .any(|f| f.text == "食べる" && f.tags.last().map(String::as_str) == Some("v1"));
        assert!(has_taberu, "should find 食べる (v1) candidate for 食べた");
    }

    #[test]
    fn is_valid_verb_works_via_default_impl() {
        let lex = EmptyLexicon;
        assert!(lex.is_valid_verb("食べる"));
        assert!(lex.is_valid_verb("書いた"));
        assert!(lex.is_valid_verb("行きました"));
        // Note: the deconjugator is generous — it returns every
        // candidate including spurious ones (a katakana noun might
        // suffix-match an "る" rule and produce a v5r candidate
        // wrongly). Real callers filter further by surface vs.
        // candidate (does the candidate text actually match a known
        // verb?). Here we only assert positive cases.
    }

    #[test]
    fn is_valid_verb_past_distinguishes_tense() {
        let lex = EmptyLexicon;
        assert!(lex.is_valid_verb_past("食べた"));
        assert!(lex.is_valid_verb_past("書いた"));
        assert!(lex.is_valid_verb_past("読んだ"));
        // Plain dict form isn't past.
        assert!(!lex.is_valid_verb_past("食べる"));
        // Negative non-past isn't past.
        assert!(!lex.is_valid_verb_past("食べない"));
    }

    #[test]
    fn is_verb_of_class_filters_by_jmdict_tag() {
        let lex = EmptyLexicon;
        // 食べた → v1 (ichidan)
        assert!(lex.is_verb_of_class("食べた", &["v1"]));
        assert!(!lex.is_verb_of_class("食べた", &["v5k"]));
        // 書いた → v5k
        assert!(lex.is_verb_of_class("書いた", &["v5k"]));
        // 読んだ → v5m, v5n, v5b, v5g (the deconjugator can't
        // distinguish without further context)
        assert!(lex.is_verb_of_class("読んだ", &["v5m", "v5n", "v5b", "v5g"]));
    }
}
