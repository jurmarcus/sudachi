//! Combine stages — glue together tokens that should be one.
//!
//! Adjacent tokens (auxiliaries, prefixes, suffixes, particles, …)
//! that Sudachi over-split get merged here. Most of the rule body
//! count lives in this category.

pub mod adverbial_particle;
pub mod amounts;
pub mod auxiliary;
pub mod auxiliary_verb_stem;
pub mod conjunctive_particle;
pub mod final_;
pub mod inflections;
pub mod particles;
pub mod prefixes;
pub mod suffix;
pub mod to_naru;
pub mod tte;
pub mod verb_dependant;
