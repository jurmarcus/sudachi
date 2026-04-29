//! Pipeline orchestrator: ordered stage list + feature-gated runner.
//!
//! The runner mirrors Jiten's `RunPipeline` (Stages/MorphologicalAnalyser.Pipeline.cs):
//! scan token stream → for each stage, skip if its feature gate
//! doesn't match → apply → re-scan if the token list changed.

use crate::lookup::{NoLookup, OptimizerLookup};
use crate::stage::Stage;
#[cfg(test)]
use crate::stage::StageGroup;
use crate::token::OptimizerToken;
use crate::token_features::TokenFeatures;

/// A bundle of stages plus the order they run in.
///
/// Different consumers want different rule subsets. Build a custom
/// set with [`RuleSet::new`], or use the convenience constructors:
///
/// - [`RuleSet::all`] — every rule, full Jiten-equivalent pipeline.
///   Default for jisho-core.
/// - [`RuleSet::analysis`] — alias for `all`. Reads better in
///   analysis contexts.
/// - [`RuleSet::search`] — minimal set for FTS consumers. Today this
///   is empty (search wants raw Sudachi); kept as a hook so we can
///   add search-specific rules without breaking callers.
/// - [`RuleSet::empty`] — no rules. Test fixture.
pub struct RuleSet {
    stages: Vec<Stage>,
}

impl RuleSet {
    pub fn new(stages: Vec<Stage>) -> Self {
        Self { stages }
    }

    pub fn empty() -> Self {
        Self { stages: Vec::new() }
    }

    /// Every rule, in the canonical Jiten ordering.
    ///
    /// Stage order from Sirush/Jiten Stages/MorphologicalAnalyser.Pipeline.cs:
    ///
    /// ```text
    /// Split:  CompoundAuxiliaryVerbs, TatteParticle, TanSuffix, TawakeNoun
    /// Repair: HasaNoun, NTokenisation, VowelElongation, ProcessSpecialCases,
    ///         ColloquialNegativeNee, ColloquialRanNai
    /// Combine:Prefixes, Inflections, Amounts, Tte, AuxiliaryVerbStem, Suffix
    /// Cleanup:ReclassifyOrphanedSuffixes
    /// Combine:ConjunctiveParticle, Auxiliary, ToNaru
    /// Repair: FusedInterjectionParticle, OrphanedAuxiliary
    /// Combine:AdverbialParticle, VerbDependant, Particles, Final
    /// Repair: TankaToTaNKa
    /// Cleanup:FilterMisparse
    /// Disambiguation: FixReadingAmbiguity
    /// ```
    pub fn all() -> Self {
        Self::new(crate::pipeline::canonical_stages())
    }

    /// Alias for [`Self::all`] — reads better in analysis contexts.
    pub fn analysis() -> Self {
        Self::all()
    }

    /// Search-engine ruleset. Currently empty (search consumers want
    /// raw Sudachi; rules would interfere with FTS index alignment).
    /// Kept as a hook so we can add search-friendly rules later
    /// (e.g., normalise long-vowel marks for fuzzy matching) without
    /// breaking callers.
    pub fn search() -> Self {
        Self::empty()
    }

    /// Returns the underlying stage list. Mostly useful for
    /// diagnostics and tests.
    pub fn stages(&self) -> &[Stage] {
        &self.stages
    }
}

/// Run the optimizer pipeline against `tokens` using `lookup` for
/// vocab queries. `tokens` is consumed; the returned vector is the
/// post-optimisation stream.
///
/// Generic over any [`OptimizerLookup`]; the dyn-cast happens at the
/// stage boundary so each stage's closure sees a uniform
/// `&dyn OptimizerLookup`.
pub fn optimize_tokens<L: OptimizerLookup>(
    mut tokens: Vec<OptimizerToken>,
    rules: &RuleSet,
    lookup: &L,
) -> Vec<OptimizerToken> {
    let lookup_dyn: &dyn OptimizerLookup = lookup;
    let mut features = TokenFeatures::scan(&tokens);

    for stage in &rules.stages {
        if !stage.required_features.is_empty()
            && (features & stage.required_features).is_empty()
        {
            continue;
        }
        let prev_len = tokens.len();
        let next = stage.apply(tokens, lookup_dyn);
        // Re-scan only when the stage actually changed the stream.
        let changed = next.len() != prev_len;
        tokens = next;
        if changed {
            features = TokenFeatures::scan(&tokens);
        }
    }
    tokens
}

/// Convenience: run with [`NoLookup`].
pub fn optimize_tokens_no_lookup(
    tokens: Vec<OptimizerToken>,
    rules: &RuleSet,
) -> Vec<OptimizerToken> {
    optimize_tokens(tokens, rules, &NoLookup)
}

// ────────────────────────────────────────────────────────────────────
// Canonical stage list construction
// ────────────────────────────────────────────────────────────────────

/// Build the full Jiten-equivalent stage list. Ordering matches
/// Sirush/Jiten Stages/MorphologicalAnalyser.Pipeline.cs.
fn canonical_stages() -> Vec<Stage> {
    use crate::{cleanup, combine, disambiguation, repair, split};

    vec![
        // ── Split ────────────────────────────────────────────────────
        split::compound_auxiliary_verbs::stage(),
        split::tatte_particle::stage(),
        split::tan_suffix::stage(),
        split::tawake_noun::stage(),
        // ── Repair ───────────────────────────────────────────────────
        repair::hasa_noun::stage(),
        repair::n_tokenisation::stage(),
        repair::vowel_elongation::stage(),
        repair::process_special_cases::stage(),
        repair::colloquial_negative_nee::stage(),
        repair::colloquial_ran_nai::stage(),
        // ── Combine ──────────────────────────────────────────────────
        combine::prefixes::stage(),
        combine::inflections::stage(),
        combine::amounts::stage(),
        combine::tte::stage(),
        combine::auxiliary_verb_stem::stage(),
        combine::suffix::stage(),
        // ── Cleanup ──────────────────────────────────────────────────
        cleanup::reclassify_orphaned_suffixes::stage(),
        // ── Combine continued ────────────────────────────────────────
        combine::conjunctive_particle::stage(),
        combine::auxiliary::stage(),
        combine::to_naru::stage(),
        // ── Repair continued ─────────────────────────────────────────
        repair::fused_interjection_particle::stage(),
        repair::orphaned_auxiliary::stage(),
        // ── Combine continued ────────────────────────────────────────
        combine::adverbial_particle::stage(),
        combine::verb_dependant::stage(),
        combine::particles::stage(),
        combine::final_::stage(),
        // ── Repair continued ─────────────────────────────────────────
        repair::tanka_to_ta_n_ka::stage(),
        // ── Cleanup ──────────────────────────────────────────────────
        cleanup::filter_misparse::stage(),
        // ── Disambiguation ───────────────────────────────────────────
        disambiguation::fix_reading_ambiguity::stage(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_ruleset_is_identity() {
        let toks = vec![OptimizerToken::synthesize(
            "猫",
            "ねこ",
            "猫",
            vec!["名詞".into()],
            0..1,
        )];
        let out = optimize_tokens_no_lookup(toks.clone(), &RuleSet::empty());
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "猫");
    }

    #[test]
    fn canonical_stages_compile_and_run() {
        // Smoke test: building the full canonical RuleSet must not
        // panic, and running it on an empty token stream returns
        // an empty stream.
        let rules = RuleSet::all();
        assert!(!rules.stages().is_empty());
        let out = optimize_tokens_no_lookup(Vec::new(), &rules);
        assert!(out.is_empty());
    }

    #[test]
    fn pipeline_groups_are_well_ordered() {
        let rules = RuleSet::all();
        let groups: Vec<_> = rules.stages().iter().map(|s| s.group).collect();
        // Sanity: pipeline starts with a Split and ends with a
        // Disambiguation, matching Jiten's macro-shape.
        assert_eq!(groups.first(), Some(&StageGroup::Split));
        assert_eq!(groups.last(), Some(&StageGroup::Disambiguation));
    }
}
