//! Split stages — break apart over-merged Sudachi tokens.
//!
//! When Sudachi outputs a single token that should have been multiple
//! (e.g., compound verbs like `し終わる`), these rules split it.

pub mod compound_auxiliary_verbs;
pub mod proper_noun_with_particle;
pub mod tan_suffix;
pub mod tatte_particle;
pub mod tawake_noun;
