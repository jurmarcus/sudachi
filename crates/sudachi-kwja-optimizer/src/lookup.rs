//! [`Lexicon`] — vocab-knowledge callback exposed to optimizer stages.
//!
//! Mirrors `sudachi_optimizer::lookup::Lexicon` in shape. Most (2)
//! rules are purely mechanical (NE filtering by surface heuristics,
//! BIO sequence validation, label normalisation) and don't need
//! vocab data. But the trait is here for parity so that any rule
//! that DOES want corroboration from a consumer-supplied catalog can
//! reach for it via the same uniform interface.
//!
//! ## Boundary with layer (3)
//!
//! If a rule needs vocab corroboration that fundamentally changes
//! KWJA output based on jisho data (e.g. "use KWJA's reading only
//! when a vocab-table entry confirms it"), that rule belongs in (3)
//! as a hybrid rule, not in (2). See `COMPREHENSION_PIPELINE.md`.
//!
//! The Lexicon trait here is for cases where a (2) rule wants a
//! mild signal — e.g. "is this surface a known term in some
//! catalog?" — to filter NE candidates more conservatively. The
//! rule's behaviour is still deterministic and explainable; the
//! lexicon is an optional refinement, not the rule's reason for
//! existing.

/// Vocab-knowledge interface for optional refinement of (2) rules.
///
/// All methods have working defaults that return `None` so consumers
/// without a vocab catalog can use [`EmptyLexicon`] and rules will
/// fall through to their default behaviour.
pub trait Lexicon {
    /// Does this surface exist as a known term in the consumer's
    /// catalog?
    ///
    /// - `Some(true)` — definitely a known term
    /// - `Some(false)` — definitely NOT a known term
    /// - `None` — no information (default for [`EmptyLexicon`])
    fn knows(&self, _surface: &str) -> Option<bool> {
        None
    }
}

/// Null lexicon — every query returns `None`. Use this when the
/// consumer doesn't have a catalog or doesn't want to expose one.
pub struct EmptyLexicon;

impl Lexicon for EmptyLexicon {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lexicon_returns_none() {
        assert_eq!(EmptyLexicon.knows("山田"), None);
    }
}
