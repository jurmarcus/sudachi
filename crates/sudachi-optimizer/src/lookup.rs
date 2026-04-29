//! [`Lexicon`] — vocab-knowledge callback exposed to optimizer stages.
//!
//! Some stages need to know whether a given dictionary form exists as
//! a single entry: if it does, certain splits/repairs should be
//! skipped because the consumer's vocab catalog already knows about
//! the whole word.
//!
//! sudachi-optimizer doesn't own a vocab catalog (that would be a
//! domain leak). Consumers implement this trait and pass it to
//! [`Optimizer::tokenize_with`](crate::Optimizer::tokenize_with) or
//! [`optimize`](crate::optimize). Pure post-tokenisation stages
//! ignore the lexicon; vocab-aware stages query it.
//!
//! ## Three-state knowledge
//!
//! Methods return `Option<bool>` so the lexicon can express:
//! - `Some(true)` — definitely in the catalog
//! - `Some(false)` — definitely not in the catalog
//! - `None` — no information available (default for [`EmptyLexicon`])
//!
//! Stages typically split on these three cases:
//! - `Some(true)` → keep the morpheme intact (catalog knows it)
//! - `Some(false)` → apply the split / repair confidently
//! - `None` → apply the split eagerly (no veto from the lexicon)

/// Vocab-knowledge interface implemented by consumers.
///
/// All methods default to `None` ("I have no information"), so
/// implementing only the methods your consumer has data for is fine.
pub trait Lexicon {
    /// Does this dictionary form exist as a single entry in the
    /// consumer's vocab catalog?
    ///
    /// - `Some(true)` — known compound entry (e.g., 滲み出す in
    ///   JMDict). Stages should keep it intact.
    /// - `Some(false)` — confirmed not in the catalog. Stages may
    ///   apply repairs confidently.
    /// - `None` — no information. Stages typically apply repairs
    ///   eagerly (no veto).
    fn has_compound_entry(&self, _term: &str) -> Option<bool> {
        None
    }
}

/// Null implementation — every query returns `None` (no info). Use
/// this from consumers without a vocab catalog (search engines doing
/// pure tokenisation).
pub struct EmptyLexicon;

impl Lexicon for EmptyLexicon {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lexicon_returns_none() {
        assert_eq!(EmptyLexicon.has_compound_entry("食べ終わる"), None);
    }
}
