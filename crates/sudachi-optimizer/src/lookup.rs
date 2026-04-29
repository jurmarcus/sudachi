//! [`OptimizerLookup`] — vocab-knowledge callback exposed to rules.
//!
//! Some rules (notably `SplitCompoundAuxiliaryVerbs` from Jiten) need
//! to know whether a given surface form exists as a single dictionary
//! entry: if it does, the compound shouldn't be split because the
//! whole-form lookup is more accurate than its decomposition.
//!
//! We don't want sudachi-optimizer to own a vocab catalog (that would
//! be a domain leak). Instead, consumers implement this trait and pass
//! it into the pipeline. Pure post-tokenisation rules ignore the
//! lookup; vocab-aware rules use it.
//!
//! For consumers that have no vocab data ([`sudachi-search`] doing
//! pure FTS, for example), use [`NoLookup`].

/// Vocab-knowledge interface implemented by consumers.
///
/// All methods default to "no", so implementing only the methods
/// you have data for is fine.
pub trait OptimizerLookup {
    /// Does this surface form exist as a single dictionary entry?
    /// `term` is the dictionary form (lemma), not the conjugated
    /// surface. Used by `SplitCompoundAuxiliaryVerbs` to keep
    /// dictionary-known compounds intact.
    fn has_compound_entry(&self, _term: &str) -> bool {
        false
    }
}

/// Null implementation — every query returns false. Use this from
/// consumers that don't have a vocab catalog (search engines doing
/// pure tokenisation).
pub struct NoLookup;

impl OptimizerLookup for NoLookup {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_lookup_returns_false() {
        assert!(!NoLookup.has_compound_entry("食べ終わる"));
    }
}
