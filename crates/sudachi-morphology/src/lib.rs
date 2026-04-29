//! # sudachi-morphology
//!
//! Bidirectional Japanese morphology — verb / adjective / copula
//! conjugation forward, rule-table deconjugation backward, irregular
//! handling, slang and dialect coverage.
//!
//! ## Two paradigms, one library
//!
//! | Direction | Use case | API |
//! |---|---|---|
//! | **Forward** (verb-class methods) | I have a known verb, give me a specific form | [`Verb::negative`], [`Verb::past`], [`Verb::te_form`], … |
//! | **Backward** (rule-table) | I see an arbitrary surface, what could it derive from? | [`deconjugate`] |
//!
//! Both share the same [`ConjForm`] tag taxonomy and operate on the
//! same rule corpus, so a verb's forward output round-trips through
//! `deconjugate()` back to the original verb.
//!
//! ## Quick start — forward
//!
//! ```ignore
//! use sudachi_morphology::{Verb, VerbClass, Polite};
//!
//! let taberu = Verb::new("食べる", VerbClass::Ichidan);
//! assert_eq!(taberu.negative().surface, "食べない");
//! assert_eq!(taberu.past().surface, "食べた");
//! assert_eq!(taberu.te_form().surface, "食べて");
//! assert_eq!(taberu.causative_passive().surface, "食べさせられる");
//! ```
//!
//! ## Quick start — backward
//!
//! ```ignore
//! use sudachi_morphology::deconjugate;
//!
//! let forms = deconjugate("食べさせられた");
//! // forms includes { text: "食べる", class: Ichidan, chain: [Causative, Passive, Past] }
//! ```
//!
//! ## Data organisation
//!
//! Conjugation rules live in `data/`, classified by what they
//! linguistically encode:
//!
//! - `data/stems/` — izenkei / mizenkei / renyoukei / shuushikei
//! - `data/verb/` — negation / past / te / polite / causative /
//!   passive / volitional / imperative / conditional / desiderative
//! - `data/auxiliary/` — てしまう / ておく / ている / etc.
//! - `data/adjective/` — i-adj + na-adj forms
//! - `data/copula/` — だ / です / である / のだ
//! - `data/colloquial/` — ちゃう / じゃう / ねえ / らん / etc.
//! - `data/dialect/` — Kansai (へん, やん, とる, …)
//! - `data/keigo/` — 尊敬語 / 謙譲語 constructions
//! - `data/irregular/` — full paradigms for する / 来る / ある / 行く
//! - `data/negative_chain/` — なくて / なければ / ずに

pub mod kana;
pub mod tag;
pub mod verb;
pub mod verb_class;

pub use tag::ConjForm;
pub use verb::{Conjugated, Verb};
pub use verb_class::VerbClass;

/// Politeness register for forms that come in plain + polite pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polite {
    Plain,
    Polite,
}
