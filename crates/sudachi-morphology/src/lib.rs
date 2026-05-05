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

pub mod adjective;
pub mod conjugation;
pub mod copula;
pub mod deconjugate;
pub mod irregular;
pub mod kana;
pub mod rule;
pub mod rule_index;
pub mod tag;
pub mod verb;
pub mod verb_class;

pub use adjective::{IAdjective, NaAdjective};
pub use conjugation::{
    Axis, ChainStep, ChainedConjugation, Conjugation, Mood, Polarity, Politeness, Tense, Voice,
};
pub use copula::{conjugate_copula, conjugate_explanatory, CopulaForm};
pub use deconjugate::{deconjugate, deconjugate_to_lemma, deconjugate_with, Form};

pub use tag::ConjForm;
pub use verb::{Conjugated, Verb};
pub use verb_class::VerbClass;

/// Politeness register for forms that come in plain + polite pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polite {
    Plain,
    Polite,
}

/// Honorific prefix selection for keigo constructions
/// ([`Verb::honorific_oninaru`] + [`Verb::humble_osuru`]).
///
/// Japanese keigo prefixes the verb with `お` for native-vocabulary
/// (wago) verbs and `ご` for Sino-Japanese (kango) verbs:
/// `お読みになる`, `お書きする` (wago) vs. `ご説明になる`,
/// `ご報告する` (kango).
///
/// The crate's plain `honorific_oninaru()` and `humble_osuru()`
/// default to `O` since wago verbs are the more common case.
/// Vocab-aware callers should use the `_with_prefix()` variants
/// and pass the right prefix per their dictionary knowledge
/// (jisho-core does this via its vocab catalog).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HonorificPrefix {
    /// `お` — for native-vocabulary (wago) verbs.
    O,
    /// `ご` — for Sino-Japanese (kango) verbs.
    Go,
}

impl HonorificPrefix {
    /// The kana surface of this prefix (`"お"` or `"ご"`).
    pub fn surface(self) -> &'static str {
        match self {
            Self::O => "お",
            Self::Go => "ご",
        }
    }
}
