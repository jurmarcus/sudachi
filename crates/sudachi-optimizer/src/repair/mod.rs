//! Repair stages — fix specific known mis-tokenisations.
//!
//! Surface-form fixes (vowel elongation, colloquial inflections,
//! special token sequences) that don't change the token count but
//! correct individual tokens' surface / reading / dictionary form.

pub mod colloquial_negative_nee;
pub mod colloquial_ran_nai;
pub mod fused_interjection_particle;
pub mod hasa_noun;
pub mod honorific_lemma;
pub mod n_tokenisation;
pub mod orphaned_auxiliary;
pub mod process_special_cases;
pub mod tanka_to_ta_n_ka;
pub mod vowel_elongation;
