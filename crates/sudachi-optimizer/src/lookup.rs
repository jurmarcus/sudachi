//! [`Lexicon`] — vocab-knowledge callback exposed to optimizer stages.
//!
//! Some stages (notably `split::compound_auxiliary_verbs` from Jiten)
//! need to know whether a given dictionary form exists as a single
//! entry: if it does, the compound shouldn't be split because the
//! whole-form lookup is more accurate than its decomposition.
//!
//! sudachi-optimizer doesn't own a vocab catalog (that would be a
//! domain leak). Consumers implement this trait and pass it to
//! [`Optimizer::tokenize_with`](crate::Optimizer::tokenize_with) or
//! [`optimize`](crate::optimize). Pure post-tokenisation stages
//! ignore the lexicon; vocab-aware stages query it.
//!
//! For consumers with no vocab data (sudachi-search doing pure FTS,
//! for example), use [`EmptyLexicon`].

/// Vocab-knowledge interface implemented by consumers.
///
/// All methods default to "no", so implementing only the methods
/// your consumer has data for is fine.
pub trait Lexicon {
    /// Does this dictionary form exist as a single entry in the
    /// consumer's vocab catalog? Used by
    /// `split::compound_auxiliary_verbs` to keep dictionary-known
    /// compounds intact (e.g., 滲み出す stays one morpheme rather
    /// than splitting into 滲み + 出す).
    fn has_compound_entry(&self, _term: &str) -> bool {
        false
    }
}

/// Null implementation — every query returns false. Use this from
/// consumers without a vocab catalog (search engines doing pure
/// tokenisation).
pub struct EmptyLexicon;

impl Lexicon for EmptyLexicon {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lexicon_returns_false() {
        assert!(!EmptyLexicon.has_compound_entry("食べ終わる"));
    }
}
