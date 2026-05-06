//! Feature-record conjugation model.
//!
//! ## Why a feature record, not an enum
//!
//! A Japanese conjugated verb is a *combination of independent feature
//! values* along a handful of axes — voice, polarity, politeness, mood,
//! tense — not one of N pre-named atoms. The legacy [`crate::tag::ConjForm`]
//! enum enumerated the cartesian product of these axes:
//!
//! ```text
//! ConjForm::PoliteNegativePast = {politeness: Polite, polarity: Negative, tense: Past}
//! ```
//!
//! That's 75+ enum variants for what is fundamentally `2 × 2 × 2 × N × 2`
//! axis combinations. It also forced the deconjugator (which has always
//! returned `Vec<String>` chains like `["polite", "negative", "past"]`)
//! and the forward conjugator to use *different* representations for the
//! same information.
//!
//! This module aligns both directions on a single record:
//!
//! ```text
//! 食べませんでした  ↔  Conjugation { politeness: Polite, polarity: Negative, tense: Past, .. }
//! ```
//!
//! ## Composition pipeline
//!
//! Each axis is applied in canonical order. Each application takes the
//! current `ConjugationState` (surface + verb class + formality marker)
//! and produces the next state. The resulting [`ChainedConjugation`]
//! includes every intermediate surface, so a UI rendering "食べる →
//! 食べます → 食べません → 食べませんでした" can iterate the chain.
//!
//! Order:
//!
//! 1. **Voice** — Causative / Passive / Causative-Passive / Potential.
//!    Each Voice transform produces a new ichidan-class verb that the
//!    later axes operate on. Voice can compose (Causative then Passive
//!    = Causative-Passive).
//! 2. **Mood** — selects which stem-form of the verb is used. Some
//!    moods (Imperative, Volitional, Te) terminate the chain — they
//!    don't take Politeness / Polarity / Tense.
//! 3. **Politeness** — inserts ます. Switches the working class to a
//!    "masu-verb" that has its own negative (ません) and past (ました)
//!    transformations.
//! 4. **Polarity** — appends ない (or transforms ます → ません).
//!    Switches the working class to "i-adjective-like" (ない conjugates
//!    as adj-i) for the tense step.
//! 5. **Tense** — applies past transformation. Sound changes depend on
//!    the current working class (godan, ichidan, masu-verb, adj-i).
//!
//! ## Validity constraints
//!
//! Some axis combinations are not valid Japanese. The pipeline returns
//! `None` for these:
//!
//! - `Mood::Imperative` + `Tense::Past` — "ate yesterday-imperatively" makes no sense.
//! - `Mood::Volitional` + `Tense::Past` — same.
//! - `Mood::VolitionalNegative` + `Polarity::Negative` — まい is already negative.
//! - `Mood::Imperative` + `Polarity::Negative` — use `Mood::ImperativeNegative` instead.

use crate::tag::ConjForm;
use crate::verb::Verb;
use crate::verb_class::VerbClass;

/// A target conjugation state — one value per axis. A consumer constructs
/// this and passes it to [`Verb::conjugate_axes`].
///
/// Use the `with_*` builder methods for ergonomic construction:
///
/// ```ignore
/// use sudachi_morphology::conjugation::Conjugation;
/// let c = Conjugation::dictionary()
///     .with_polite()
///     .with_negative()
///     .with_past();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Conjugation {
    pub voice: Voice,
    pub polarity: Polarity,
    pub politeness: Politeness,
    pub mood: Mood,
    pub tense: Tense,
}

impl Conjugation {
    /// The bare dictionary form — every axis at its default value.
    /// `verb.conjugate_axes(Conjugation::dictionary())` produces
    /// `(surface = dict_form, chain = [])`.
    pub const fn dictionary() -> Self {
        Self {
            voice: Voice::None,
            polarity: Polarity::Affirmative,
            politeness: Politeness::Plain,
            mood: Mood::Indicative,
            tense: Tense::Nonpast,
        }
    }

    pub const fn with_voice(mut self, v: Voice) -> Self { self.voice = v; self }
    pub const fn with_negative(mut self) -> Self { self.polarity = Polarity::Negative; self }
    pub const fn with_polite(mut self) -> Self { self.politeness = Politeness::Polite; self }
    pub const fn with_past(mut self) -> Self { self.tense = Tense::Past; self }
    pub const fn with_mood(mut self, m: Mood) -> Self { self.mood = m; self }

    /// Returns true if every axis is at its default value.
    pub fn is_dictionary(self) -> bool {
        self == Self::dictionary()
    }

    /// Validate the axis combination. Returns `Err(reason)` for forms
    /// that aren't well-formed Japanese. Called automatically by
    /// [`Verb::conjugate_axes`]; consumers can call it explicitly when
    /// generating the cartesian product to filter invalid combos.
    pub fn validate(self) -> Result<(), &'static str> {
        if matches!(self.mood, Mood::Imperative | Mood::ImperativeNegative)
            && self.tense == Tense::Past
        {
            return Err("imperative cannot be past tense");
        }
        if matches!(self.mood, Mood::Imperative) && self.polarity == Polarity::Negative {
            return Err("use Mood::ImperativeNegative instead of Imperative + Negative");
        }
        if matches!(self.mood, Mood::ImperativeNegative) && self.polarity == Polarity::Negative {
            return Err("ImperativeNegative is already negative");
        }
        if matches!(self.mood, Mood::Volitional | Mood::VolitionalNegative)
            && self.tense == Tense::Past
        {
            return Err("volitional cannot be past tense");
        }
        if matches!(self.mood, Mood::VolitionalNegative) && self.polarity == Polarity::Negative {
            return Err("VolitionalNegative (まい) is already negative");
        }
        // Te-form, ConditionalBa, ConditionalTara, ProvisionalNara are
        // *terminating* moods — they consume the past/polarity slots
        // into their own surface generation. Validate that consumers
        // aren't double-applying.
        if matches!(
            self.mood,
            Mood::ConditionalBa | Mood::ConditionalTara | Mood::ProvisionalNara | Mood::Te
        ) && self.tense == Tense::Past
        {
            // ConditionalTara *is* "past + ら" so this is implicit.
            // ConditionalBa, Nara, Te take their own forms.
            // For now, reject — we'll loosen if real corpora demand it.
            if !matches!(self.mood, Mood::ConditionalTara) {
                return Err("ba / nara / te-form cannot also be marked Past");
            }
        }
        Ok(())
    }
}

impl Default for Conjugation {
    fn default() -> Self {
        Self::dictionary()
    }
}

/// Voice axis — what the subject's relationship to the verb is.
/// `Causative` and `Passive` compose into `CausativePassive`; that's the
/// only meaningful composition. Other voices are mutually exclusive.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Voice {
    /// Plain — no voice transformation. 食べる.
    #[default]
    None,
    /// Causative — させる (ichidan) / ase-row + る (godan). 食べさせる.
    Causative,
    /// Causative short — さす / す (colloquial). 食べさす, 書かす.
    /// Same axis position as Causative; consumers pick stylistic variant.
    CausativeShort,
    /// Passive — られる (ichidan) / ar-row + reru (godan). 食べられる.
    Passive,
    /// Honorific — same surface as Passive, used as 尊敬語 register.
    /// Distinguished only by sentence context.
    Honorific,
    /// Causative-Passive — させられる. The composed form.
    CausativePassive,
    /// Potential — られる (ichidan) / e-row + る (godan). 食べられる, 書ける.
    /// In modern usage often distinguished from Passive (食べられる) by
    /// dropping the ら for ichidan: 食べれる (ら抜き言葉).
    Potential,
}

/// Polarity axis — affirmative or negated.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity { #[default] Affirmative, Negative }

/// Politeness axis — plain (です/だ-less) or polite (ます).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Politeness { #[default] Plain, Polite }

/// Tense axis — non-past (default) or past.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tense { #[default] Nonpast, Past }

/// Mood axis — what kind of clause this verb form heads. Most moods
/// are *terminating* (they don't take further Polarity/Tense modifiers
/// because they encode their own).
///
/// `Indicative` is the default and combines freely with all other axes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mood {
    /// Indicative / declarative — the default. Combines with every
    /// other axis: Polite × Negative × Past × ...
    #[default]
    Indicative,
    /// Imperative — ろ/え/positive command. 食べろ, 走れ. Doesn't take
    /// Past or Negative (use ImperativeNegative for the latter).
    Imperative,
    /// Imperative negative — V-ru + な suffix. 食べるな, 行くな.
    ImperativeNegative,
    /// Volitional — う/よう ("let's"). 食べよう, 行こう. Combines with
    /// Politeness (食べましょう) but not Tense or Negative.
    Volitional,
    /// Volitional negative — まい ("won't"). 行くまい. Doesn't combine
    /// with Tense or further Polarity.
    VolitionalNegative,
    /// Conditional ば — え-stem + ば. 食べれば, 書けば. Doesn't combine
    /// with Tense (the conditional encodes its own time).
    ConditionalBa,
    /// Conditional tara — past + ら. 食べたら. *Already* uses past
    /// internally, so `tense: Past` is implicit.
    ConditionalTara,
    /// Provisional nara — V-ru + なら. 食べるなら. Standalone.
    ProvisionalNara,
    /// Te-form — continuative / connective. 食べて, 書いて. Terminating.
    Te,
    /// Desiderative — たい ("want to"). 食べたい. The result behaves
    /// like an i-adjective (食べたかった = past, 食べたくない = negative).
    /// We model the inner たい as Mood::Desiderative, leaving Polarity
    /// and Tense to apply on top via the i-adj sound changes.
    Desiderative,
    /// Desiderative-other — たがる ("third-person wants to"). 食べたがる.
    /// The result is a godan-ru-class verb (食べたがった, 食べたがらない).
    DesiderativeOther,
}

/// One step in the composition chain. Generated by walking the axis
/// pipeline; each non-default axis emits one step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainStep {
    /// Which axis this step represents.
    pub axis: Axis,
    /// The cumulative surface AFTER this step is applied. So for a
    /// chain `[Polite → 食べます, Negative → 食べません, Past → 食べませんでした]`,
    /// the third step's surface is the final 食べませんでした.
    pub surface: String,
    /// Whether this step inherits the polite register. Set by the
    /// Politeness step itself, and propagated to subsequent steps so
    /// the UI can render the whole tail as polite-formal.
    pub formal: bool,
}

/// Axis label for a [`ChainStep`]. Mirrors the `Conjugation` field
/// names plus a value-discriminator for the multi-valued axes (Voice,
/// Mood) so a step can be rendered uniquely (e.g., "Causative" vs
/// "Passive" both live in the Voice axis).
///
/// `Aux(AuxKind)` is the bridge step a [`ChainSpec`] emits when
/// it splits a chain at a verb-producing aux: "everything before this
/// step conjugated the BASE verb; everything after conjugates the
/// AUX verb". Renderers display it as the aux's label (`Progressive`,
/// `Finish-completely`, `Start V-ing`, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    Voice(Voice),
    Mood(Mood),
    Politeness,
    Polarity,
    Tense,
    Aux(AuxKind),
}

/// Verb-producing auxiliary that attaches to another verb's stem.
///
/// Two attach points:
///   - **Renyou (masu) base** — Hajimeru, Tsuzukeru, Owaru, Dasu, Sugiru,
///     Tagaru. Caller composes `<base>.stem_renyou()` + `<aux>.lemma()`.
///   - **Te base** — Teiru, Teoru, Teiku, Tekuru, Tearu, Teoku, Tekureru,
///     Teageru, Temorau, Teshimau. Caller composes `<base>.te_form()` +
///     `<aux>.lemma()`.
///
/// Each variant carries its own [`VerbClass`] so the resulting compound
/// can be conjugated forward via [`Verb::conjugate_axes`] using the aux's
/// own paradigm (e.g., `食べてしまう` → 食べてしまった via godan-u past).
///
/// **Single-level only** for now. Multi-level chains (`食べさせてもらう`
/// = causative + benefactive) are explicitly out of scope per the
/// continuation plan; pursue once Phase 1 ships.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuxKind {
    // ── Te-base aux ────────────────────────────────────────────────
    /// 〜ている — progressive / resultative-state.
    Teiru,
    /// 〜ておる — humble form of 〜ている.
    Teoru,
    /// 〜ていく — going-to (movement away from speaker).
    Teiku,
    /// 〜てくる — coming-to (movement toward speaker).
    Tekuru,
    /// 〜てある — resultative (transitive verb's effect lingers).
    Tearu,
    /// 〜ておく — preparative ("for now").
    Teoku,
    /// 〜てくれる — benefactive (someone does for me).
    Tekureru,
    /// 〜てあげる — benefactive (I do for someone).
    Teageru,
    /// 〜てもらう — benefactive (have someone do).
    Temorau,
    /// 〜てしまう — completive ("end up", "regrettably").
    Teshimau,
    // ── Renyou-base aux ────────────────────────────────────────────
    /// 〜すぎる — excessive ("too much").
    Sugiru,
    /// 〜たがる — third-person desiderative.
    Tagaru,
    /// 〜始める — inchoative ("start V-ing").
    Hajimeru,
    /// 〜続ける — continuative ("continue V-ing").
    Tsuzukeru,
    /// 〜終わる — completive ("finish V-ing").
    Owaru,
    /// 〜出す — sudden inception ("burst into V-ing").
    Dasu,

    // i-adj-producing aux (Yasui, Nikui, NaruI, SuruI, …) intentionally
    // omitted. Their forward path needs `IAdjective::conjugate_axes`
    // (a 5-axis enumerator over i-adj forms) which doesn't exist yet
    // — the deferred plan flags it as "Out (defer to follow-up)".
    // The deconjugator-side rules for `やすい` / `にくい` exist (see
    // `data/deconjugation_rules.json`) so surfaces like 食べやすい
    // deconjugate cleanly to a lemma; ChainSpec::from_process will
    // fail strict (unrecognised process labels) and morph_resolution
    // _from_token falls back to no chain — better than rendering a
    // broken chain that doesn't round-trip on the past form
    // (食べやすかった).
}

impl AuxKind {
    /// The verb class of the aux itself, used to drive forward
    /// conjugation of `<base>.stem + aux.lemma()` through the right
    /// paradigm via [`Verb::conjugate_axes`].
    pub fn verb_class(self) -> VerbClass {
        match self {
            Self::Teiru => VerbClass::Ichidan,
            Self::Teoru => VerbClass::GodanRu,
            Self::Teiku => VerbClass::GodanKuIku,
            Self::Tekuru => VerbClass::Kuru,
            Self::Tearu => VerbClass::GodanRuAru,
            Self::Teoku => VerbClass::GodanKu,
            Self::Tekureru => VerbClass::IchidanKureru,
            Self::Teageru => VerbClass::Ichidan,
            Self::Temorau => VerbClass::GodanU,
            Self::Teshimau => VerbClass::GodanU,
            Self::Sugiru => VerbClass::Ichidan,
            Self::Tagaru => VerbClass::GodanRu,
            Self::Hajimeru => VerbClass::Ichidan,
            Self::Tsuzukeru => VerbClass::Ichidan,
            Self::Owaru => VerbClass::GodanRu,
            Self::Dasu => VerbClass::GodanSu,
        }
    }

    /// Default kanji-form lemma — the surface that appears after the
    /// base verb's stem in canonical writing. Te-stacking aux include
    /// the connecting て/で in the stem, NOT here.
    pub fn lemma(self) -> &'static str {
        match self {
            Self::Teiru => "いる",
            Self::Teoru => "おる",
            Self::Teiku => "行く",
            Self::Tekuru => "来る",
            Self::Tearu => "ある",
            Self::Teoku => "おく",
            Self::Tekureru => "くれる",
            Self::Teageru => "あげる",
            Self::Temorau => "もらう",
            Self::Teshimau => "しまう",
            Self::Sugiru => "すぎる",
            Self::Tagaru => "たがる",
            Self::Hajimeru => "始める",
            Self::Tsuzukeru => "続ける",
            Self::Owaru => "終わる",
            Self::Dasu => "出す",
        }
    }

    /// Whether this aux attaches to the base verb's te-form (vs. its
    /// renyou/masu stem). Used by [`Verb::conjugate_chain`] to pick
    /// the right base-verb stem before composing.
    pub fn attaches_to_te(self) -> bool {
        matches!(
            self,
            Self::Teiru
                | Self::Teoru
                | Self::Teiku
                | Self::Tekuru
                | Self::Tearu
                | Self::Teoku
                | Self::Tekureru
                | Self::Teageru
                | Self::Temorau
                | Self::Teshimau
        )
    }

    /// Map a deconjugator process label to an [`AuxKind`], if the
    /// label is the marker for a verb-producing aux. Non-aux labels
    /// (single-axis like "past", "polite") return `None` so the
    /// caller can route them through the existing
    /// [`apply_process_label`] dispatch.
    ///
    /// `garu` is intentionally NOT mapped to [`AuxKind::Tagaru`]
    /// here — the existing dispatcher treats it as a single-axis
    /// `Mood::DesiderativeOther` marker, and triggering aux mode on
    /// `garu` would conflict. `Tagaru` exists in the enum for future
    /// data shapes that might emit it explicitly; today it's never
    /// produced by [`ChainSpec::from_process`].
    pub fn from_process_label(label: &str) -> Option<Self> {
        Some(match label {
            // Te-stacking
            "teiru" | "teru" => Self::Teiru,
            "teoru" | "toru" => Self::Teoru,
            "teiku" | "teku" => Self::Teiku,
            "tekuru" => Self::Tekuru,
            "tearu" => Self::Tearu,
            "toku (for now)" | "for now" => Self::Teoku,
            "do for me" => Self::Tekureru,
            "do for someone" => Self::Teageru,
            "have someone do" => Self::Temorau,
            "finish/completely/end up" => Self::Teshimau,
            // Renyou-stacking
            "excess V-ing" => Self::Sugiru,
            "start V-ing" => Self::Hajimeru,
            "continue V-ing" => Self::Tsuzukeru,
            "finish V-ing" => Self::Owaru,
            "burst into V-ing" => Self::Dasu,
            _ => return None,
        })
    }
}

/// One auxiliary attachment in a [`ChainSpec`]. Single-level by design
/// — the deferred plan covers multi-level (`食べさせてもらう`) only
/// after this lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuxStep {
    /// Which aux is attached.
    pub kind: AuxKind,
    /// Conjugation applied to the AUX verb itself. The base verb's
    /// own conjugation lives on [`ChainSpec::base`].
    pub conjugation: Conjugation,
}

/// A multi-level conjugation request: a base [`Conjugation`] for
/// the head verb, plus an ordered chain of [`AuxStep`] attachments
/// stacked outermost-from-base. Mirrors the deconjugator's
/// `Form.process` shape in typed form — every chain that reaches
/// a verb lemma decomposes into "base axes + N aux levels + each
/// aux's own axes".
///
/// Multi-level chains compose via te-form / renyou-stem stacking:
///
/// ```text
/// 思い続けている  (思う v1)
///   base = dict()
///   aux  = [Tsuzukeru @ dict, Teiru @ dict]
///                                   │
///   forward = 思う → 思い続ける ─┘ → 思い続けて → 思い続けている
///                  └ Tsuzukeru        ↓ te-form        └ Teiru
/// ```
///
/// The `Vec` is ordered: `aux[0]` attaches to the base verb's stem,
/// `aux[1]` to the te-form / renyou stem of the `aux[0]` compound,
/// etc. Empty Vec = no aux (the [`Verb::conjugate_axes`] fast path).
///
/// `From<Conjugation>` and `Default` cover the no-aux fast path:
/// `ChainSpec::from(c)` is `ChainSpec { base: c, aux: vec![] }`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChainSpec {
    /// Conjugation applied to the head verb BEFORE any aux attaches.
    /// Usually [`Conjugation::dictionary`] — most aux attach straight
    /// off the lemma's stem. Non-default values handle base-side
    /// transforms like causative (`食べさせてもらう` = base.voice =
    /// Causative + aux[0] = Temorau).
    pub base: Conjugation,
    /// Ordered aux stack, outermost-from-base. `aux[i]` attaches to
    /// the te-form or renyou stem of `aux[i-1]`'s compound surface
    /// (or the base verb's stem when `i == 0`).
    ///
    /// Empty Vec for direct conjugation (no aux).
    pub aux: Vec<AuxStep>,
}

impl From<Conjugation> for ChainSpec {
    fn from(base: Conjugation) -> Self {
        Self {
            base,
            aux: Vec::new(),
        }
    }
}

impl ChainSpec {
    /// Build a [`ChainSpec`] from a deconjugator [`crate::Form`]'s
    /// `process` chain, splitting at every aux marker into
    /// successive levels.
    ///
    /// Walking algorithm:
    /// 1. Reverse `process` so we walk forward (deconjugator order
    ///    is backwards).
    /// 2. On an aux marker (`teiru` / `start V-ing` / etc.) push a
    ///    fresh [`AuxStep`] onto `aux`. Subsequent axis labels apply
    ///    to that aux's conjugation, NOT the base.
    /// 3. **Structural-(te) skip**: when `(te)` or `te` appears
    ///    immediately before a te-stacking aux marker, the (te) is
    ///    structural — it's how the next aux attaches to the previous
    ///    compound, not a Mood::Te axis. Skip it.
    /// 4. Apply axis labels via [`apply_process_label`] to the most
    ///    recent aux's conjugation (or the base if `aux` is empty).
    /// 5. Stem markers (`(masu stem)`, `(stem)`, etc.) are no-ops —
    ///    handled inside `apply_process_label`.
    ///
    /// Returns `None` when any non-aux, non-stem label fails to
    /// axis-decompose (compound predicates the deconjugator emits
    /// from rules outside the canonical 5-axis grammar).
    pub fn from_process(process: &[String]) -> Option<Self> {
        let mut spec = ChainSpec::default();
        let labels: Vec<&str> = process.iter().rev().map(String::as_str).collect();
        let mut i = 0;
        while i < labels.len() {
            let label = labels[i];

            // Aux marker — start a new aux level.
            if let Some(kind) = AuxKind::from_process_label(label) {
                spec.aux.push(AuxStep {
                    kind,
                    conjugation: Conjugation::dictionary(),
                });
                i += 1;
                continue;
            }

            // Structural (te) before a te-stacking aux: skip — this
            // is the connector that lets the next aux attach to the
            // previous compound, not a Mood::Te axis on the current
            // level.
            if (label == "(te)" || label == "te")
                && i + 1 < labels.len()
                && AuxKind::from_process_label(labels[i + 1])
                    .is_some_and(|k| k.attaches_to_te())
            {
                i += 1;
                continue;
            }

            // Otherwise: axis label routed to the active side.
            let target = match spec.aux.last_mut() {
                Some(last) => &mut last.conjugation,
                None => &mut spec.base,
            };
            apply_process_label(target, label)?;
            i += 1;
        }
        Some(spec)
    }

    /// Lenient variant — partial aux split that ignores unknown
    /// labels rather than short-circuiting. Mirrors
    /// [`Conjugation::from_process_lenient`].
    pub fn from_process_lenient(process: &[String]) -> Self {
        let mut spec = ChainSpec::default();
        let labels: Vec<&str> = process.iter().rev().map(String::as_str).collect();
        let mut i = 0;
        while i < labels.len() {
            let label = labels[i];
            if let Some(kind) = AuxKind::from_process_label(label) {
                spec.aux.push(AuxStep {
                    kind,
                    conjugation: Conjugation::dictionary(),
                });
                i += 1;
                continue;
            }
            if (label == "(te)" || label == "te")
                && i + 1 < labels.len()
                && AuxKind::from_process_label(labels[i + 1])
                    .is_some_and(|k| k.attaches_to_te())
            {
                i += 1;
                continue;
            }
            let target = match spec.aux.last_mut() {
                Some(last) => &mut last.conjugation,
                None => &mut spec.base,
            };
            let _ = apply_process_label(target, label);
            i += 1;
        }
        spec
    }
}

/// A fully-resolved conjugation: the final surface plus the chain of
/// intermediate surfaces. Returned by [`Verb::conjugate_axes`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainedConjugation {
    /// The terminal surface — same as `chain.last().surface` if the
    /// chain is non-empty, else the dictionary form.
    pub surface: String,
    /// The originating conjugation record (what was asked for).
    pub conjugation: Conjugation,
    /// Per-axis steps in canonical application order. Empty for
    /// `Conjugation::dictionary()`.
    pub chain: Vec<ChainStep>,
}

impl ChainedConjugation {
    fn dict_only(surface: String, conjugation: Conjugation) -> Self {
        Self { surface, conjugation, chain: Vec::new() }
    }
}

// ════════════════════════════════════════════════════════════════════
// Verb::conjugate_axes — the new dispatch
// ════════════════════════════════════════════════════════════════════

impl Verb {
    /// Apply a [`Conjugation`] record to this verb. Returns the final
    /// surface and the per-axis chain of intermediate surfaces.
    ///
    /// The pipeline applies axes in canonical order (Voice → Mood →
    /// Politeness → Polarity → Tense), threading each axis's output
    /// surface into the next. Validation ([`Conjugation::validate`])
    /// runs first; invalid combinations return `None`.
    ///
    /// ```ignore
    /// use sudachi_morphology::{Verb, VerbClass};
    /// use sudachi_morphology::conjugation::Conjugation;
    ///
    /// let v = Verb::new("食べる", VerbClass::Ichidan);
    /// let c = Conjugation::dictionary().with_polite().with_negative().with_past();
    /// let result = v.conjugate_axes(c).unwrap();
    /// assert_eq!(result.surface, "食べませんでした");
    /// // chain = [Polite → 食べます, Negative → 食べません, Past → 食べませんでした]
    /// assert_eq!(result.chain.len(), 3);
    /// ```
    pub fn conjugate_axes(&self, target: Conjugation) -> Option<ChainedConjugation> {
        target.validate().ok()?;

        // Dictionary form fast-path.
        if target == Conjugation::dictionary() {
            return Some(ChainedConjugation::dict_only(self.dict_form.clone(), target));
        }

        let mut chain = Vec::new();

        // ── Stage A: Voice (verb transform) ──────────────────────────
        //
        // Voice operations transform the verb itself, producing a new
        // verb of a different (usually ichidan) class. After this
        // stage, the working verb is whatever Voice produced (or the
        // original verb if Voice::None).
        let working_verb = if target.voice != Voice::None {
            let voice_conjugated = self.conjugate(voice_to_conjform(target.voice))?;
            if voice_conjugated.surface.is_empty() {
                return None;
            }
            chain.push(ChainStep {
                axis: Axis::Voice(target.voice),
                surface: voice_conjugated.surface.clone(),
                formal: false,
            });
            Verb::new(&voice_conjugated.surface, voice_to_resulting_class(target.voice))
        } else {
            self.clone()
        };

        // ── Stage B: Mood + (Politeness, Polarity, Tense) ────────────
        //
        // Most non-Indicative moods are *terminating* — they consume
        // some of the politeness/polarity/tense slots into a single
        // ConjForm variant. Indicative composes politeness × polarity
        // × tense the usual way. We delegate both cases to a single
        // dispatcher: build cumulative AxesSoFar including mood, walk
        // canonical order, dispatch each step through ConjForm.
        let mut current_axes = AxesSoFar {
            mood: target.mood,
            ..AxesSoFar::default()
        };
        let mut formal_propagating = false;

        let axes_in_order: &[(AxisToggle, Axis, bool)] = &[
            (
                AxisToggle::Politeness(target.politeness),
                Axis::Politeness,
                target.politeness == Politeness::Polite,
            ),
            (
                AxisToggle::Polarity(target.polarity),
                Axis::Polarity,
                false,
            ),
            (
                AxisToggle::Tense(target.tense),
                Axis::Tense,
                false,
            ),
        ];

        // Special case: a non-Indicative mood with all other axes
        // default emits a single mood step (e.g., Volitional →
        // 食べよう). Detect that here.
        let no_other_axes_active = !axes_in_order.iter().any(|(t, _, _)| t.is_active());
        if no_other_axes_active && current_axes.mood != Mood::Indicative {
            let conjform = current_axes.to_conjform()?;
            let conjugated = working_verb.conjugate(conjform)?;
            if conjugated.surface.is_empty() {
                return None;
            }
            chain.push(ChainStep {
                axis: Axis::Mood(target.mood),
                surface: conjugated.surface,
                formal: false,
            });
        } else {
            for (toggle, axis, sets_formal) in axes_in_order {
                if !toggle.is_active() {
                    continue;
                }
                current_axes.apply(*toggle);
                let conjform = current_axes.to_conjform()?;
                let conjugated = working_verb.conjugate(conjform)?;
                if conjugated.surface.is_empty() {
                    return None;
                }
                if *sets_formal {
                    formal_propagating = true;
                }
                chain.push(ChainStep {
                    axis: *axis,
                    surface: conjugated.surface,
                    formal: formal_propagating,
                });
            }
        }

        if chain.is_empty() {
            // All axes default — handled above, defensive.
            return Some(ChainedConjugation::dict_only(self.dict_form.clone(), target));
        }

        let final_surface = chain.last().unwrap().surface.clone();
        Some(ChainedConjugation {
            surface: final_surface,
            conjugation: target,
            chain,
        })
    }

    /// Apply a [`ChainSpec`] — a base [`Conjugation`] followed by zero
    /// or more [`AuxStep`] attachments — to this verb. Returns the
    /// final surface and the per-step chain, with a bridge
    /// `Axis::Aux(kind)` step at every aux boundary.
    ///
    /// Multi-level: each aux in `spec.aux` stacks on the previous
    /// compound. `aux[0]` attaches to this verb's te-form or renyou
    /// stem; `aux[1]` attaches to `aux[0]`'s compound's te-form / renyou
    /// stem; and so on. The returned chain is ordered base → bridge
    /// → aux[0] axes → bridge → aux[1] axes → ... → final tense.
    ///
    /// Algorithm per level:
    /// 1. Stage 1 (once): apply `spec.base` to `self` via
    ///    [`Self::conjugate_axes`]. Captures rare base-side axes
    ///    like Voice::Causative for `食べさせてもらう`.
    /// 2. For each aux in order:
    ///    - Compute the stem of the current working surface
    ///      (te-form or renyou) under the working class.
    ///    - Concat stem + aux.lemma to form the compound surface.
    ///    - Fresh Verb with the compound surface + aux.kind.verb_class().
    ///    - Run `conjugate_axes(aux.conjugation)` for the aux
    ///      chain (Politeness, Tense, etc. on the aux verb).
    ///    - Splice: working chain + bridge `Axis::Aux(kind)` +
    ///      aux chain. Update the working class for the next
    ///      level's stem computation.
    pub fn conjugate_chain(&self, spec: &ChainSpec) -> Option<ChainedConjugation> {
        // Stage 1: base verb conjugation. With no aux this is the
        // fast path — same result as `conjugate_axes(spec.base)`. With
        // aux present, captures any base-side voice transform (e.g.
        // `食べさせてもらう` = `Voice::Causative` on base, then
        // Temorau aux on the causative compound).
        let mut working = self.conjugate_axes(spec.base)?;
        // Track the verb class of the current working compound. After
        // a voice transform on the base, the resulting class changes
        // (Causative → Ichidan, CausativeShort → GodanSu, etc.).
        let mut working_class = if spec.base.voice == Voice::None {
            self.class
        } else {
            voice_to_resulting_class(spec.base.voice)
        };

        for aux in &spec.aux {
            // Build the stem from the current working surface +
            // class. Each aux level treats `working.surface` as a
            // dictionary-form input (each aux conjugation typically
            // defaults to dict in multi-level chains); te-stacking
            // aux take the te-form, renyou-stacking aux take the
            // masu-stem.
            let stem_verb = Verb::new(&working.surface, working_class);
            let stem_surface = if aux.kind.attaches_to_te() {
                stem_verb.te_form().surface
            } else {
                stem_verb.stem_renyou()
            };
            if stem_surface.is_empty() {
                return None;
            }
            let compound_surface = format!("{}{}", stem_surface, aux.kind.lemma());
            let compound_verb = Verb::new(&compound_surface, aux.kind.verb_class());
            let aux_result = compound_verb.conjugate_axes(aux.conjugation)?;

            // Splice: working chain + bridge Aux step + aux chain.
            let mut chain = working.chain;
            chain.push(ChainStep {
                axis: Axis::Aux(aux.kind),
                surface: compound_surface.clone(),
                formal: false,
            });
            chain.extend(aux_result.chain);

            let final_surface = chain
                .last()
                .map(|s| s.surface.clone())
                .unwrap_or(compound_surface);
            working = ChainedConjugation {
                surface: final_surface,
                // Preserve the original base conjugation as the
                // "headline" record. Each aux's own conjugation is
                // captured in the chain steps; reconstructing it is
                // a render-time concern.
                conjugation: spec.base,
                chain,
            };
            working_class = aux.kind.verb_class();
        }

        Some(working)
    }
}

/// Map a Voice value to its bare-form ConjForm variant. Used in the
/// Voice-transform stage of `conjugate_axes`.
fn voice_to_conjform(v: Voice) -> ConjForm {
    match v {
        Voice::None => ConjForm::Dictionary,
        Voice::Causative => ConjForm::Causative,
        Voice::CausativeShort => ConjForm::CausativeShort,
        Voice::Passive => ConjForm::Passive,
        Voice::Honorific => ConjForm::Honorific,
        Voice::CausativePassive => ConjForm::CausativePassive,
        Voice::Potential => ConjForm::Potential,
    }
}

/// What verb class is the result of a voice transform? Causative /
/// Passive / CausativePassive / Potential / Honorific all produce
/// ichidan verbs. CausativeShort produces godan-su (-す ending).
fn voice_to_resulting_class(v: Voice) -> VerbClass {
    match v {
        Voice::None => VerbClass::Ichidan, // unreachable in practice
        Voice::Causative
        | Voice::Passive
        | Voice::Honorific
        | Voice::CausativePassive
        | Voice::Potential => VerbClass::Ichidan,
        // CausativeShort: 食べる → 食べさす (godan-su class).
        // 書く → 書かす (godan-su).
        Voice::CausativeShort => VerbClass::GodanSu,
    }
}

// ════════════════════════════════════════════════════════════════════
// Deconjugator alignment — Form → Conjugation
// ════════════════════════════════════════════════════════════════════
//
// The backward direction (`crate::deconjugate`) already returns the
// axis-decomposed shape: `Form { process: Vec<String> }` like
// `["polite", "negative", "past"]`. The asymmetry was that *forward*
// emitted cartesian-product enums and *backward* emitted feature
// lists. Now both speak `Conjugation`.
//
// `Conjugation::from_process(&form.process)` walks the rule labels in
// canonical (forward) order and accumulates axis updates. Returns
// `None` when the chain involves a process step that doesn't
// axis-decompose (compound predicates like `teiru`, `tearu`, `toku`;
// dialect/register markers like `ksb`, `slurred`, `casual`). Those
// constructions live above the basic-conjugation layer and are
// represented in the optimizer / matcher pipeline, not here.

impl Conjugation {
    /// Build a [`Conjugation`] from a deconjugator [`crate::Form`]'s
    /// `process` chain. Process labels are deconjugator order
    /// (reverse of forward composition); we reverse internally so
    /// applying axes left-to-right matches the forward pipeline.
    ///
    /// Returns `None` when any step in the chain isn't an
    /// axis-decomposable rule (compound te-aux predicates, dialect
    /// markers, etc.). Use [`Conjugation::from_process_lenient`] when
    /// you want best-effort decomposition that ignores unknown
    /// labels.
    pub fn from_process(process: &[String]) -> Option<Self> {
        let mut c = Conjugation::dictionary();
        // Reverse: deconjugator emits in undo order; forward order is
        // the reverse.
        for label in process.iter().rev() {
            apply_process_label(&mut c, label)?;
        }
        Some(c)
    }

    /// Like [`Conjugation::from_process`] but returns the partial
    /// `Conjugation` even when some labels couldn't be decomposed.
    /// Useful for callers that want "the axis facts we can extract,
    /// ignore the rest".
    pub fn from_process_lenient(process: &[String]) -> Self {
        let mut c = Conjugation::dictionary();
        for label in process.iter().rev() {
            let _ = apply_process_label(&mut c, label);
        }
        c
    }
}

/// Apply one process-label step to a `Conjugation` accumulator.
/// Returns `Err(())` for labels that don't axis-decompose so the
/// strict caller can short-circuit.
fn apply_process_label(c: &mut Conjugation, label: &str) -> Option<()> {
    match label {
        // ── Single-axis ────────────────────────────────────────────
        "past" => c.tense = Tense::Past,
        "negative" | "archaic negative" | "archaic attributive negative"
        | "adverbial negative" | "slurred negative" => c.polarity = Polarity::Negative,
        "polite" => c.politeness = Politeness::Polite,
        "causative" | "short causative" => c.voice = Voice::Causative,
        "passive" | "passive/potential" | "passive/potential/honorific" => {
            c.voice = Voice::Passive
        }
        "causative passive" => c.voice = Voice::CausativePassive,
        "potential" => c.voice = Voice::Potential,
        "honorific" | "honorific (ksb)" => c.voice = Voice::Honorific,

        // ── Mood markers ────────────────────────────────────────────
        "imperative" | "imperative (ksb)" | "polite imperative"
        | "casual polite imperative" | "polite request" => c.mood = Mood::Imperative,
        "imperative negative" => c.mood = Mood::ImperativeNegative,
        "volitional" => c.mood = Mood::Volitional,
        "polite volitional" => {
            c.mood = Mood::Volitional;
            c.politeness = Politeness::Polite;
        }
        "want" => c.mood = Mood::Desiderative,
        "garu" => c.mood = Mood::DesiderativeOther,
        "(te)" | "te" => c.mood = Mood::Te,
        "polite te" => {
            c.mood = Mood::Te;
            c.politeness = Politeness::Polite;
        }
        "conditional" | "provisional conditional" | "classical hypothetical conditional"
        | "negative conditional" | "polite conditional" | "formal conditional"
        | "colloquial negative conditional" => c.mood = Mood::ConditionalBa,

        // ── Multi-axis (one label encodes multiple axis updates) ───
        "polite past" => {
            c.politeness = Politeness::Polite;
            c.tense = Tense::Past;
        }
        "polite negative" | "formal negative" => {
            c.politeness = Politeness::Polite;
            c.polarity = Polarity::Negative;
        }
        "polite past negative" | "formal negative past" => {
            c.politeness = Politeness::Polite;
            c.polarity = Polarity::Negative;
            c.tense = Tense::Past;
        }

        // ── Stem / intermediate (no axis state, skip) ──────────────
        "(stem)" | "(masu stem)" | "(mizenkei)" | "(izenkei)" | "(adverbial stem)"
        | "(ka stem)" | "(ke stem)" | "('a' stem)" | "(unstressed infinitive)" | "" => {
            // Pure stem markers — internal deconjugator scaffolding.
            // They don't change any axis but also don't fail
            // decomposition.
        }

        // ── Out-of-axis (compound predicates, registers, particles) ─
        // Returning None tells the strict caller to short-circuit.
        // The lenient caller skips and continues.
        _ => return None,
    }
    Some(())
}
//
// The pipeline walks axes in canonical order, and at each step needs
// to know "what's the surface for the cumulative axes set so far?".
// For Phase 1, we lean on the existing legacy `ConjForm` enum + the
// `Verb::conjugate(form)` dispatcher (which routes irregulars through
// `irregular::lookup_irregular`). The mapping table here translates
// (axes-so-far) → ConjForm, leaving Phase 2 to replace the dispatcher
// internals with axis-native rule application.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AxisToggle {
    Politeness(Politeness),
    Polarity(Polarity),
    Tense(Tense),
}

impl AxisToggle {
    fn is_active(self) -> bool {
        match self {
            AxisToggle::Politeness(p) => p == Politeness::Polite,
            AxisToggle::Polarity(p) => p == Polarity::Negative,
            AxisToggle::Tense(t) => t == Tense::Past,
        }
    }
}

/// Cumulative axis state walked by the dispatcher. Voice is removed
/// from this struct because it's handled by the verb-transform stage
/// (Stage A in `conjugate_axes`); by the time we walk Stage B, voice
/// is already absorbed into the working verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AxesSoFar {
    mood: Mood,
    politeness: Politeness,
    polarity: Polarity,
    tense: Tense,
}

impl Default for AxesSoFar {
    fn default() -> Self {
        Self {
            mood: Mood::Indicative,
            politeness: Politeness::Plain,
            polarity: Polarity::Affirmative,
            tense: Tense::Nonpast,
        }
    }
}

impl AxesSoFar {
    fn apply(&mut self, t: AxisToggle) {
        match t {
            AxisToggle::Politeness(p) => self.politeness = p,
            AxisToggle::Polarity(p) => self.polarity = p,
            AxisToggle::Tense(t) => self.tense = t,
        }
    }

    /// Map the cumulative axes (mood × politeness × polarity × tense)
    /// to the legacy `ConjForm` enum variant. Returns `None` for axis
    /// combinations that don't have a corresponding ConjForm.
    ///
    /// Most non-Indicative moods are *terminating* — they consume
    /// politeness/polarity/tense slots into a single ConjForm and
    /// don't compose with extra past/negative on top.
    fn to_conjform(self) -> Option<ConjForm> {
        match self.mood {
            Mood::Indicative => self.indicative_to_conjform(),
            Mood::Imperative => self.expect_terminating(ConjForm::Imperative),
            Mood::ImperativeNegative => self.expect_terminating(ConjForm::ImperativeNegative),
            Mood::Volitional => {
                // Volitional combines with Politeness only.
                if self.polarity != Polarity::Affirmative || self.tense != Tense::Nonpast {
                    return None;
                }
                Some(match self.politeness {
                    Politeness::Plain => ConjForm::Volitional,
                    Politeness::Polite => ConjForm::PoliteVolitional,
                })
            }
            Mood::VolitionalNegative => self.expect_terminating(ConjForm::VolitionalNegative),
            Mood::ConditionalBa => {
                // Conditional ば: combines only with Polarity.
                if self.politeness != Politeness::Plain || self.tense != Tense::Nonpast {
                    return None;
                }
                Some(match self.polarity {
                    Polarity::Affirmative => ConjForm::ConditionalBa,
                    Polarity::Negative => ConjForm::NegativeBa,
                })
            }
            Mood::ConditionalTara => {
                // たら is past-tense conditional; tense is implicit.
                if self.politeness != Politeness::Plain || self.polarity != Polarity::Affirmative {
                    return None;
                }
                Some(ConjForm::ConditionalTara)
            }
            Mood::ProvisionalNara => self.expect_terminating(ConjForm::ProvisionalNara),
            Mood::Te => {
                // Te-form combines with Polarity OR Politeness, not Tense.
                if self.tense != Tense::Nonpast {
                    return None;
                }
                Some(match (self.politeness, self.polarity) {
                    (Politeness::Plain, Polarity::Affirmative) => ConjForm::Te,
                    (Politeness::Plain, Polarity::Negative) => ConjForm::NegativeTe,
                    (Politeness::Polite, Polarity::Affirmative) => ConjForm::PoliteTe,
                    (Politeness::Polite, Polarity::Negative) => return None,
                })
            }
            Mood::Desiderative => self.expect_terminating(ConjForm::Desiderative),
            Mood::DesiderativeOther => self.expect_terminating(ConjForm::DesiderativeOther),
        }
    }

    /// Indicative mood: pure (politeness × polarity × tense) cube.
    fn indicative_to_conjform(self) -> Option<ConjForm> {
        Some(match (self.politeness, self.polarity, self.tense) {
            (Politeness::Plain,  Polarity::Affirmative, Tense::Nonpast) => ConjForm::Dictionary,
            (Politeness::Plain,  Polarity::Affirmative, Tense::Past)    => ConjForm::Past,
            (Politeness::Plain,  Polarity::Negative,    Tense::Nonpast) => ConjForm::Negative,
            (Politeness::Plain,  Polarity::Negative,    Tense::Past)    => ConjForm::NegativePast,
            (Politeness::Polite, Polarity::Affirmative, Tense::Nonpast) => ConjForm::Polite,
            (Politeness::Polite, Polarity::Affirmative, Tense::Past)    => ConjForm::PolitePast,
            (Politeness::Polite, Polarity::Negative,    Tense::Nonpast) => ConjForm::PoliteNegative,
            (Politeness::Polite, Polarity::Negative,    Tense::Past)    => ConjForm::PoliteNegativePast,
        })
    }

    /// Helper: a terminating mood requires politeness/polarity/tense
    /// to all be at default, otherwise the combo isn't valid.
    fn expect_terminating(self, form: ConjForm) -> Option<ConjForm> {
        if self.politeness == Politeness::Plain
            && self.polarity == Polarity::Affirmative
            && self.tense == Tense::Nonpast
        {
            Some(form)
        } else {
            None
        }
    }
}

// Suppress dead-code warnings on the WorkingKind / State scaffolding
// — they're scaffolding for Phase 2's axis-native rule path. Not yet
// reachable in Phase 1 since we dispatch through ConjForm.
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct State {
    surface: String,
    class: VerbClass,
    kind: WorkingKind,
    formal: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkingKind {
    Verb,
    MasuVerb,
    NegativeAuxIAdj,
}

#[allow(dead_code)]
fn apply_negative(state: State) -> Option<State> {
    match state.kind {
        WorkingKind::Verb => {
            let v = Verb::new(&state.surface, state.class);
            let c = v.negative();
            if c.surface.is_empty() {
                return None;
            }
            Some(State {
                surface: c.surface,
                class: state.class,
                kind: WorkingKind::NegativeAuxIAdj,
                formal: state.formal,
            })
        }
        WorkingKind::MasuVerb => {
            let stem = state.surface.strip_suffix("ます")?;
            let mut s = String::with_capacity(stem.len() + "ません".len());
            s.push_str(stem);
            s.push_str("ません");
            Some(State { surface: s, kind: WorkingKind::MasuVerb, ..state })
        }
        WorkingKind::NegativeAuxIAdj => None,
    }
}

// ════════════════════════════════════════════════════════════════════
// Tests
// ════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn v1(dict: &str) -> Verb {
        Verb::new(dict, VerbClass::Ichidan)
    }
    fn v5k(dict: &str) -> Verb {
        Verb::new(dict, VerbClass::GodanKu)
    }

    #[test]
    fn dictionary_form_returns_empty_chain() {
        let v = v1("食べる");
        let r = v.conjugate_axes(Conjugation::dictionary()).unwrap();
        assert_eq!(r.surface, "食べる");
        assert!(r.chain.is_empty());
    }

    #[test]
    fn negative_alone() {
        let v = v1("食べる");
        let c = Conjugation::dictionary().with_negative();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "食べない");
        assert_eq!(r.chain.len(), 1);
        assert_eq!(r.chain[0].axis, Axis::Polarity);
        assert!(!r.chain[0].formal);
    }

    #[test]
    fn negative_past_chain_two_steps() {
        let v = v1("食べる");
        let c = Conjugation::dictionary().with_negative().with_past();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "食べなかった");
        assert_eq!(r.chain.len(), 2);
        assert_eq!(r.chain[0].axis, Axis::Polarity);
        assert_eq!(r.chain[0].surface, "食べない"); // intermediate!
        assert_eq!(r.chain[1].axis, Axis::Tense);
        assert_eq!(r.chain[1].surface, "食べなかった");
    }

    #[test]
    fn polite_negative_past_chain_three_steps_with_intermediates() {
        let v = v1("食べる");
        let c = Conjugation::dictionary().with_polite().with_negative().with_past();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "食べませんでした");
        assert_eq!(r.chain.len(), 3);
        assert_eq!(r.chain[0].axis, Axis::Politeness);
        assert_eq!(r.chain[0].surface, "食べます");
        assert!(r.chain[0].formal);
        assert_eq!(r.chain[1].axis, Axis::Polarity);
        assert_eq!(r.chain[1].surface, "食べません");
        assert!(r.chain[1].formal); // formal propagates through subsequent steps
        assert_eq!(r.chain[2].axis, Axis::Tense);
        assert_eq!(r.chain[2].surface, "食べませんでした");
        assert!(r.chain[2].formal);
    }

    #[test]
    fn godan_polite_negative_past() {
        let v = v5k("書く");
        let c = Conjugation::dictionary().with_polite().with_negative().with_past();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "書きませんでした");
        assert_eq!(r.chain[0].surface, "書きます");
        assert_eq!(r.chain[1].surface, "書きません");
    }

    #[test]
    fn godan_past_alone() {
        let v = v5k("書く");
        let r = v.conjugate_axes(Conjugation::dictionary().with_past()).unwrap();
        assert_eq!(r.surface, "書いた");
    }

    #[test]
    fn voice_alone_works() {
        // Voice + no other axes is in Phase 1 scope (single legacy
        // ConjForm variant available).
        let v = v1("食べる");
        let r = v.conjugate_axes(Conjugation::dictionary().with_voice(Voice::Causative)).unwrap();
        assert_eq!(r.surface, "食べさせる");
        let r = v.conjugate_axes(Conjugation::dictionary().with_voice(Voice::CausativePassive)).unwrap();
        assert_eq!(r.surface, "食べさせられる");
        let r = v.conjugate_axes(Conjugation::dictionary().with_voice(Voice::Potential)).unwrap();
        assert_eq!(r.surface, "食べられる");
    }

    #[test]
    fn voice_potential_negative_works() {
        // Special-cased: PotentialNegative IS a legacy ConjForm variant.
        let v = v1("食べる");
        let c = Conjugation::dictionary()
            .with_voice(Voice::Potential)
            .with_negative();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "食べられない");
    }

    #[test]
    fn voice_composed_with_other_axes_works_in_phase_2() {
        // Phase 2 closed the gap: voice + (politeness | polarity |
        // tense) composes by transforming the verb in Stage A, then
        // applying remaining axes to the resulting verb in Stage B.
        let v = v1("食べる");

        // Causative + Negative: 食べる → 食べさせる → 食べさせない
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_voice(Voice::Causative).with_negative())
            .unwrap();
        assert_eq!(r.surface, "食べさせない");
        assert_eq!(r.chain.len(), 2);
        assert_eq!(r.chain[0].axis, Axis::Voice(Voice::Causative));
        assert_eq!(r.chain[0].surface, "食べさせる");
        assert_eq!(r.chain[1].axis, Axis::Polarity);
        assert_eq!(r.chain[1].surface, "食べさせない");

        // Causative-Passive + Past: 食べる → 食べさせられる → 食べさせられた
        let r = v
            .conjugate_axes(
                Conjugation::dictionary()
                    .with_voice(Voice::CausativePassive)
                    .with_past(),
            )
            .unwrap();
        assert_eq!(r.surface, "食べさせられた");
        assert_eq!(r.chain[0].axis, Axis::Voice(Voice::CausativePassive));
        assert_eq!(r.chain[0].surface, "食べさせられる");
        assert_eq!(r.chain[1].axis, Axis::Tense);
        assert_eq!(r.chain[1].surface, "食べさせられた");
    }

    #[test]
    fn voice_composed_with_polite_negative_past() {
        // Full stack: Causative + Polite + Negative + Past =
        // 食べる → 食べさせる → 食べさせます → 食べさせません → 食べさせませんでした
        let v = v1("食べる");
        let c = Conjugation::dictionary()
            .with_voice(Voice::Causative)
            .with_polite()
            .with_negative()
            .with_past();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "食べさせませんでした");
        assert_eq!(r.chain.len(), 4);
        assert_eq!(r.chain[0].surface, "食べさせる");
        assert_eq!(r.chain[1].surface, "食べさせます");
        assert!(r.chain[1].formal);
        assert_eq!(r.chain[2].surface, "食べさせません");
        assert!(r.chain[2].formal);
        assert_eq!(r.chain[3].surface, "食べさせませんでした");
        assert!(r.chain[3].formal);
    }

    // ── Mood tests ───────────────────────────────────────────────

    #[test]
    fn mood_imperative() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::Imperative))
            .unwrap();
        assert_eq!(r.surface, "食べろ");
        assert_eq!(r.chain.len(), 1);
        assert_eq!(r.chain[0].axis, Axis::Mood(Mood::Imperative));
    }

    #[test]
    fn mood_imperative_negative() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::ImperativeNegative))
            .unwrap();
        assert_eq!(r.surface, "食べるな");
    }

    #[test]
    fn mood_volitional() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::Volitional))
            .unwrap();
        assert_eq!(r.surface, "食べよう");
    }

    #[test]
    fn mood_volitional_polite_composes() {
        let v = v1("食べる");
        let c = Conjugation::dictionary().with_mood(Mood::Volitional).with_polite();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "食べましょう");
    }

    #[test]
    fn mood_volitional_negative() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::VolitionalNegative))
            .unwrap();
        assert_eq!(r.surface, "食べるまい");
    }

    #[test]
    fn mood_te() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::Te))
            .unwrap();
        assert_eq!(r.surface, "食べて");
    }

    #[test]
    fn mood_te_negative() {
        let v = v1("食べる");
        let c = Conjugation::dictionary().with_mood(Mood::Te).with_negative();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "食べなくて");
    }

    #[test]
    fn mood_conditional_ba() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::ConditionalBa))
            .unwrap();
        assert_eq!(r.surface, "食べれば");
    }

    #[test]
    fn mood_conditional_ba_negative() {
        let v = v1("食べる");
        let c = Conjugation::dictionary().with_mood(Mood::ConditionalBa).with_negative();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "食べなければ");
    }

    #[test]
    fn mood_conditional_tara() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::ConditionalTara))
            .unwrap();
        assert_eq!(r.surface, "食べたら");
    }

    #[test]
    fn mood_provisional_nara() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::ProvisionalNara))
            .unwrap();
        assert_eq!(r.surface, "食べるなら");
    }

    #[test]
    fn mood_desiderative() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::Desiderative))
            .unwrap();
        assert_eq!(r.surface, "食べたい");
    }

    #[test]
    fn mood_desiderative_other() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::DesiderativeOther))
            .unwrap();
        assert_eq!(r.surface, "食べたがる");
    }

    #[test]
    fn suru_imperative_via_irregular() {
        let v = Verb::new("する", VerbClass::Suru);
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::Imperative))
            .unwrap();
        assert_eq!(r.surface, "しろ");
    }

    #[test]
    fn kuru_volitional_via_irregular() {
        let v = Verb::new("来る", VerbClass::Kuru);
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::Volitional))
            .unwrap();
        assert_eq!(r.surface, "来よう");
    }

    #[test]
    fn godan_te_form_sound_changes_correctly() {
        let v = v5k("書く");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_mood(Mood::Te))
            .unwrap();
        assert_eq!(r.surface, "書いて");
    }

    #[test]
    fn voice_short_causative_ichidan() {
        let v = v1("食べる");
        let r = v
            .conjugate_axes(Conjugation::dictionary().with_voice(Voice::CausativeShort))
            .unwrap();
        assert_eq!(r.surface, "食べさす");
    }

    #[test]
    fn validate_rejects_imperative_past() {
        let c = Conjugation::dictionary().with_mood(Mood::Imperative).with_past();
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_imperative_negative_polarity() {
        let c = Conjugation::dictionary().with_mood(Mood::Imperative).with_negative();
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_volitional_past() {
        let c = Conjugation::dictionary().with_mood(Mood::Volitional).with_past();
        assert!(c.validate().is_err());
    }

    #[test]
    fn double_negation_returns_none() {
        // Trying to negate a negative — not in scope for Phase 1.
        // (Real usage would express this via a different construction.)
        let _v = v1("食べる");
        let state = State {
            surface: "食べない".to_string(),
            class: VerbClass::Ichidan,
            kind: WorkingKind::NegativeAuxIAdj,
            formal: false,
        };
        let r = apply_negative(state);
        assert!(r.is_none());
    }

    // ── Irregulars (Suru / Kuru) — these were broken with the legacy
    //    direct verb.past() / verb.te_form() calls. Verifying the new
    //    axis API gets them right via the irregular dispatch in the
    //    underlying Verb methods. ────────────────────────────────────

    #[test]
    fn suru_polite_negative_past() {
        let v = Verb::new("する", VerbClass::Suru);
        let c = Conjugation::dictionary().with_polite().with_negative().with_past();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "しませんでした");
        assert_eq!(r.chain[0].surface, "します");
        assert_eq!(r.chain[1].surface, "しません");
    }

    #[test]
    fn kuru_polite_negative_past() {
        let v = Verb::new("来る", VerbClass::Kuru);
        let c = Conjugation::dictionary().with_polite().with_negative().with_past();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "来ませんでした");
        assert_eq!(r.chain[0].surface, "来ます");
        assert_eq!(r.chain[1].surface, "来ません");
    }

    #[test]
    fn suru_past_alone() {
        let v = Verb::new("する", VerbClass::Suru);
        let r = v.conjugate_axes(Conjugation::dictionary().with_past()).unwrap();
        assert_eq!(r.surface, "した");
    }

    #[test]
    fn kuru_negative_past() {
        let v = Verb::new("来る", VerbClass::Kuru);
        let c = Conjugation::dictionary().with_negative().with_past();
        let r = v.conjugate_axes(c).unwrap();
        assert_eq!(r.surface, "来なかった");
        assert_eq!(r.chain[0].surface, "来ない");
    }

    // ── Deconjugator alignment: backward → Conjugation ─────────────

    #[test]
    fn from_process_simple_past() {
        let p = vec!["past".to_string()];
        let c = Conjugation::from_process(&p).unwrap();
        assert_eq!(c, Conjugation::dictionary().with_past());
    }

    #[test]
    fn from_process_negative_past() {
        // Deconjugator emits in undo order: ["past", "negative"]
        // because to undo 食べなかった you first undo past (→ 食べない)
        // then undo negative (→ 食べる).
        let p = vec!["past".to_string(), "negative".to_string()];
        let c = Conjugation::from_process(&p).unwrap();
        assert_eq!(c, Conjugation::dictionary().with_negative().with_past());
    }

    #[test]
    fn from_process_polite_negative_past_via_compound_label() {
        let p = vec!["polite past negative".to_string()];
        let c = Conjugation::from_process(&p).unwrap();
        assert_eq!(
            c,
            Conjugation::dictionary().with_polite().with_negative().with_past()
        );
    }

    #[test]
    fn from_process_causative_passive() {
        let p = vec!["causative passive".to_string()];
        let c = Conjugation::from_process(&p).unwrap();
        assert_eq!(
            c,
            Conjugation::dictionary().with_voice(Voice::CausativePassive)
        );
    }

    #[test]
    fn from_process_volitional_polite_compound() {
        let p = vec!["polite volitional".to_string()];
        let c = Conjugation::from_process(&p).unwrap();
        assert_eq!(
            c,
            Conjugation::dictionary().with_mood(Mood::Volitional).with_polite()
        );
    }

    #[test]
    fn from_process_skips_stem_intermediates() {
        let p = vec!["polite".to_string(), "(masu stem)".to_string()];
        let c = Conjugation::from_process(&p).unwrap();
        assert_eq!(c, Conjugation::dictionary().with_polite());
    }

    #[test]
    fn from_process_strict_rejects_compound_predicate() {
        let p = vec!["teiru".to_string()];
        assert!(Conjugation::from_process(&p).is_none());
    }

    #[test]
    fn from_process_lenient_skips_unknown() {
        let p = vec!["past".to_string(), "teiru".to_string()];
        let c = Conjugation::from_process_lenient(&p);
        assert_eq!(c, Conjugation::dictionary().with_past());
    }

    #[test]
    fn round_trip_forward_then_backward_polite_negative_past() {
        // Structural test: forward(C) → surface → deconjugate →
        // from_process_lenient → C. Should equal the original.
        use crate::deconjugate;
        let v = v1("食べる");
        let original = Conjugation::dictionary().with_polite().with_negative().with_past();
        let forward = v.conjugate_axes(original).unwrap();
        assert_eq!(forward.surface, "食べませんでした");

        let candidates = deconjugate(&forward.surface);
        let matches_original = candidates.iter().any(|f| {
            f.text == "食べる"
                && Conjugation::from_process_lenient(&f.process) == original
        });
        assert!(
            matches_original,
            "expected at least one deconjugator candidate to round-trip to {:?}",
            original
        );
    }

    #[test]
    fn round_trip_negative_past() {
        use crate::deconjugate;
        let v = v1("食べる");
        let original = Conjugation::dictionary().with_negative().with_past();
        let forward = v.conjugate_axes(original).unwrap();
        assert_eq!(forward.surface, "食べなかった");

        let candidates = deconjugate(&forward.surface);
        let matches = candidates.iter().any(|f| {
            f.text == "食べる"
                && Conjugation::from_process_lenient(&f.process) == original
        });
        assert!(matches, "round-trip failed for negative past");
    }

    // ─── ChainSpec / AuxStep round trips (Step 2.10) ──────────────────

    /// Reusable: deconjugate `surface` to `lemma`, build a ChainSpec
    /// from the first matching chain, run `conjugate_chain`, assert
    /// the produced surface equals the original.
    fn assert_aux_round_trip(surface: &str, lemma: &str, lemma_class: VerbClass) {
        let chains = crate::deconjugate_to_lemma(surface, lemma);
        assert!(
            !chains.is_empty(),
            "deconjugate_to_lemma({surface:?}, {lemma:?}) produced no chains",
        );
        // Find the FIRST chain whose process is fully decomposable
        // by ChainSpec::from_process — strict mode. If none, the
        // round-trip is unreachable.
        let chain_spec = chains
            .iter()
            .find_map(|f| ChainSpec::from_process(&f.process))
            .unwrap_or_else(|| {
                let lenient: Vec<_> = chains
                    .iter()
                    .map(|f| (ChainSpec::from_process_lenient(&f.process), &f.process))
                    .collect();
                panic!(
                    "no chain for {surface:?} → {lemma:?} fully decomposed; \
                     lenient candidates: {lenient:?}",
                );
            });
        let v = Verb::new(lemma, lemma_class);
        let result = v.conjugate_chain(&chain_spec).unwrap_or_else(|| {
            panic!("conjugate_chain returned None for {surface:?} → {lemma:?}")
        });
        assert_eq!(
            result.surface, surface,
            "round-trip surface mismatch for {surface:?} → {lemma:?} \
             (got {:?}, chain spec {chain_spec:?})",
            result.surface,
        );
    }

    #[test]
    fn chain_round_trip_taberu_hajimeta() {
        assert_aux_round_trip("食べ始めた", "食べる", VerbClass::Ichidan);
    }

    #[test]
    fn chain_round_trip_taberu_tsuzuketeiru() {
        // Multi-level: Tsuzukeru on renyou, then Teiru on te-form of
        // 続ける. ChainSpec is now Vec<AuxStep> so this round-trips.
        assert_aux_round_trip("食べ続けている", "食べる", VerbClass::Ichidan);
    }

    #[test]
    fn chain_round_trip_taberu_teshimatta() {
        assert_aux_round_trip("食べてしまった", "食べる", VerbClass::Ichidan);
    }

    #[test]
    fn chain_round_trip_taberu_temoratta() {
        assert_aux_round_trip("食べてもらった", "食べる", VerbClass::Ichidan);
    }

    #[test]
    fn chain_round_trip_nomu_owatta() {
        assert_aux_round_trip("飲み終わった", "飲む", VerbClass::GodanMu);
    }

    #[test]
    fn chain_round_trip_hashiru_dashita() {
        assert_aux_round_trip("走り出した", "走る", VerbClass::GodanRu);
    }

    #[test]
    fn chain_round_trip_omoi_tsuzuketeiru() {
        // Two-level multi-aux: Tsuzukeru + Teiru on a v5u verb.
        assert_aux_round_trip("思い続けている", "思う", VerbClass::GodanU);
    }

    #[test]
    fn chain_round_trip_taberu_tsuzuketeita_with_past() {
        // Three-level depth: Tsuzukeru + Teiru + Past — the past
        // applies to the OUTER aux's conjugation, not base.
        assert_aux_round_trip("食べ続けていた", "食べる", VerbClass::Ichidan);
    }

    #[test]
    fn chain_spec_from_process_routes_axes_to_aux() {
        // 食べ始めた process (deconjugator order, reversed during walk):
        //   ["past", "start V-ing", "(masu stem)"]
        // After reverse for forward: "(masu stem)" → "start V-ing" → "past"
        // Stem skipped, aux marker promotes, then "past" goes to aux[0].
        let process = vec![
            "past".to_string(),
            "start V-ing".to_string(),
            "(masu stem)".to_string(),
        ];
        let spec = ChainSpec::from_process(&process).expect("strict decompose");
        assert_eq!(spec.base, Conjugation::dictionary());
        assert_eq!(spec.aux.len(), 1);
        let aux = &spec.aux[0];
        assert_eq!(aux.kind, AuxKind::Hajimeru);
        assert_eq!(aux.conjugation.tense, Tense::Past);
        assert_eq!(aux.conjugation.polarity, Polarity::Affirmative);
    }

    #[test]
    fn chain_spec_from_process_no_aux_returns_base_only() {
        // Plain past without aux should produce no aux side.
        let process = vec!["past".to_string(), "polite".to_string()];
        let spec = ChainSpec::from_process(&process).expect("strict decompose");
        assert!(spec.aux.is_empty());
        assert_eq!(spec.base.tense, Tense::Past);
        assert_eq!(spec.base.politeness, Politeness::Polite);
    }

    #[test]
    fn chain_spec_from_process_skips_structural_te_before_te_aux() {
        // 食べ続けている process (deconjugator order):
        //   ["teiru", "(te)", "continue V-ing", "(masu stem)"]
        // Forward: "(masu stem)" → "continue V-ing" → "(te)" → "teiru"
        // The "(te)" sits between two aux markers; the second one
        // (Teiru) attaches_to_te so the (te) is structural — should
        // be skipped, NOT applied as Mood::Te on aux[0].
        let process = vec![
            "teiru".to_string(),
            "(te)".to_string(),
            "continue V-ing".to_string(),
            "(masu stem)".to_string(),
        ];
        let spec = ChainSpec::from_process(&process).expect("strict decompose");
        assert_eq!(spec.aux.len(), 2, "two aux levels expected");
        assert_eq!(spec.aux[0].kind, AuxKind::Tsuzukeru);
        assert_eq!(spec.aux[0].conjugation, Conjugation::dictionary());
        assert_eq!(spec.aux[1].kind, AuxKind::Teiru);
        assert_eq!(spec.aux[1].conjugation, Conjugation::dictionary());
    }

    #[test]
    fn chain_spec_from_process_keeps_terminal_te_as_mood() {
        // Bare 食べて (no following aux) — the (te) IS Mood::Te.
        let process = vec!["(te)".to_string()];
        let spec = ChainSpec::from_process(&process).expect("strict decompose");
        assert!(spec.aux.is_empty());
        assert_eq!(spec.base.mood, Mood::Te);
    }

    #[test]
    fn aux_kind_attaches_to_te_classification() {
        // Te-stacking aux must report attaches_to_te = true.
        for k in [
            AuxKind::Teiru,
            AuxKind::Teoru,
            AuxKind::Teiku,
            AuxKind::Tekuru,
            AuxKind::Tearu,
            AuxKind::Teoku,
            AuxKind::Tekureru,
            AuxKind::Teageru,
            AuxKind::Temorau,
            AuxKind::Teshimau,
        ] {
            assert!(k.attaches_to_te(), "{k:?} should attach to te-form");
        }
        // Renyou-stacking aux must report attaches_to_te = false.
        for k in [
            AuxKind::Sugiru,
            AuxKind::Tagaru,
            AuxKind::Hajimeru,
            AuxKind::Tsuzukeru,
            AuxKind::Owaru,
            AuxKind::Dasu,
        ] {
            assert!(!k.attaches_to_te(), "{k:?} should attach to renyou stem");
        }
    }
}
