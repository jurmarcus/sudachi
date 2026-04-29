//! Static data tables shared across stages.
//!
//! These constants live alongside the upstream Jiten tables in
//! `Jiten.Parser/Helpers/MorphologicalAnalyser.RuleData.cs`. Keeping
//! them in a single Rust module (rather than scattering each per-rule
//! file) lets multiple stages share the same vocabulary set without
//! duplication, and lets the user-supplied [`Lexicon`](crate::Lexicon)
//! stay focused on consumer-supplied lookups (the static tables here
//! never change between consumers).
//!
//! Add new tables here as their consuming rules are ported.

/// Auxiliary verbs that attach to a main verb stem to form a compound
/// verb (し終わる, 食べ続ける, 書き始める, …). Used by
/// [`split::compound_auxiliary_verbs`](crate::split::compound_auxiliary_verbs).
///
/// Mirror of `AuxiliaryVerbs` in
/// [Sirush/Jiten Helpers/MorphologicalAnalyser.RuleData.cs](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Helpers/MorphologicalAnalyser.RuleData.cs).
pub const AUXILIARY_VERBS: &[&str] = &[
    "続ける",
    "始める",
    "終わる",
    "終える",
    "出す",
    "かける",
    "いたす",
    "いただく",
    "頂く",
    "する",
];

/// Maps an auxiliary verb (dictionary form) to the surface stem it
/// inflects from. Used by `split::compound_auxiliary_verbs` to
/// validate that the surface form actually contains the stem before
/// splitting (guards against false matches like 出 inside 出会う).
///
/// Mirror of `AuxiliaryVerbStems` in
/// [Sirush/Jiten Helpers/MorphologicalAnalyser.RuleData.cs](https://github.com/Sirush/Jiten/blob/master/Jiten.Parser/Helpers/MorphologicalAnalyser.RuleData.cs).
pub fn auxiliary_verb_stem(dict_form: &str) -> Option<&'static str> {
    match dict_form {
        "続ける" => Some("続け"),
        "始める" => Some("始め"),
        "終わる" => Some("終わ"),
        "終える" => Some("終え"),
        "出す" => Some("出"),
        "かける" => Some("かけ"),
        _ => None,
    }
}
