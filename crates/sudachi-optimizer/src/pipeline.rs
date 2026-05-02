//! [`Pipeline`] — bundle of stages + the optimizer orchestrator.
//!
//! The runner mirrors Jiten's `RunPipeline`
//! (Stages/MorphologicalAnalyser.Pipeline.cs):
//! scan morpheme stream → for each stage, skip if its feature gate
//! doesn't match → apply → re-scan if the morpheme list changed.

use crate::lookup::{EmptyLexicon, Lexicon};
use crate::stage::Stage;
#[cfg(test)]
use crate::stage::Phase;
use crate::token::Morpheme;
use crate::token_features::MorphemeFeatures;

/// An ordered bundle of optimizer [`Stage`]s.
///
/// Different consumers want different stage subsets. Build a custom
/// pipeline with [`Pipeline::new`], or use the convenience
/// constructors:
///
/// - [`Pipeline::analysis`] — every stage, full Jiten-equivalent
///   pipeline. The default for dictionary-lookup consumers.
/// - [`Pipeline::search`] — minimal set for FTS consumers. Today this
///   is empty (search wants raw Sudachi); kept as a hook so we can
///   add search-specific rules without breaking callers.
/// - [`Pipeline::empty`] — no stages. Test fixture.
pub struct Pipeline {
    stages: Vec<Stage>,
}

impl Pipeline {
    pub fn new(stages: Vec<Stage>) -> Self {
        Self { stages }
    }

    pub fn empty() -> Self {
        Self { stages: Vec::new() }
    }

    /// Every stage, in the canonical Jiten ordering.
    ///
    /// Sequence (from Sirush/Jiten Stages/MorphologicalAnalyser.Pipeline.cs):
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
    pub fn analysis() -> Self {
        Self::new(canonical_stages())
    }

    /// Search-engine pipeline. Currently empty (search consumers
    /// want raw Sudachi; rules would interfere with FTS index
    /// alignment). Kept as a hook so we can add search-friendly
    /// stages later (e.g., normalise long-vowel marks for fuzzy
    /// matching) without breaking callers.
    pub fn search() -> Self {
        Self::empty()
    }

    /// The underlying stage list. Mostly useful for diagnostics and
    /// tests.
    pub fn stages(&self) -> &[Stage] {
        &self.stages
    }
}

/// Run the optimizer `pipeline` against `morphemes` using `lexicon`
/// for vocab queries. `morphemes` is consumed; the returned vector
/// is the post-optimisation stream.
///
/// Generic over any [`Lexicon`]; the dyn-cast happens at the stage
/// boundary so each stage's closure sees a uniform `&dyn Lexicon`.
pub fn optimize<L: Lexicon>(
    mut morphemes: Vec<Morpheme>,
    pipeline: &Pipeline,
    lexicon: &L,
) -> Vec<Morpheme> {
    let lexicon_dyn: &dyn Lexicon = lexicon;
    let mut features = MorphemeFeatures::scan(&morphemes);

    for stage in &pipeline.stages {
        if !stage.required_features.is_empty()
            && (features & stage.required_features).is_empty()
        {
            continue;
        }
        let prev_len = morphemes.len();
        let next = stage.apply(morphemes, lexicon_dyn);
        // Re-scan only when the stage actually changed the stream.
        let changed = next.len() != prev_len;
        morphemes = next;
        if changed {
            features = MorphemeFeatures::scan(&morphemes);
        }
    }
    morphemes
}

/// Convenience: run with [`EmptyLexicon`].
pub fn optimize_no_lexicon(morphemes: Vec<Morpheme>, pipeline: &Pipeline) -> Vec<Morpheme> {
    optimize(morphemes, pipeline, &EmptyLexicon)
}

// ────────────────────────────────────────────────────────────────────
// Canonical stage list construction
// ────────────────────────────────────────────────────────────────────

/// Build the full Jiten-equivalent stage list. Ordering matches
/// Sirush/Jiten Stages/MorphologicalAnalyser.Pipeline.cs.
pub fn canonical_stages() -> Vec<Stage> {
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
        repair::honorific_lemma::stage(),
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
    fn empty_pipeline_is_identity() {
        let ms = vec![Morpheme::synthesize(
            "猫",
            "ねこ",
            "猫",
            vec!["名詞".into()],
            0..1,
        )];
        let out = optimize_no_lexicon(ms, &Pipeline::empty());
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].surface, "猫");
    }

    #[test]
    fn canonical_pipeline_compiles_and_runs() {
        // Smoke test: building the full canonical Pipeline must not
        // panic, and running it on an empty morpheme stream returns
        // an empty stream.
        let pipeline = Pipeline::analysis();
        assert!(!pipeline.stages().is_empty());
        let out = optimize_no_lexicon(Vec::new(), &pipeline);
        assert!(out.is_empty());
    }

    #[test]
    fn pipeline_phases_are_well_ordered() {
        let pipeline = Pipeline::analysis();
        let phases: Vec<_> = pipeline.stages().iter().map(|s| s.phase).collect();
        // Sanity: pipeline starts with a Split and ends with a
        // Disambiguation, matching Jiten's macro-shape.
        assert_eq!(phases.first(), Some(&Phase::Split));
        assert_eq!(phases.last(), Some(&Phase::Disambiguation));
    }
}
