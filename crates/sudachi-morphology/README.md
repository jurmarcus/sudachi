# sudachi-morphology

**Bidirectional Japanese morphology — forward conjugation + backward deconjugation.**

A standalone library (no Sudachi dependency) covering every modern Japanese verb / adjective / copula paradigm plus the classical residues JMdict catalogues. Two complementary surfaces share one tag taxonomy and one rule corpus, so a verb's forward output round-trips through `deconjugate()` back to the original.

---

## Two paradigms, one library

| Direction                 | Use case                                                    | API                                              |
| ------------------------- | ----------------------------------------------------------- | ------------------------------------------------ |
| **Forward** (typed)       | I have a known verb, give me a specific form                | `Verb::negative()`, `Verb::past()`, `Verb::te_form()`, … |
| **Backward** (rule-table) | I see an arbitrary surface, what could it derive from?      | `deconjugate()`                                  |

Both share the `ConjForm` tag taxonomy and the same rule corpus. A typed forward call's output is recoverable by the backward call.

---

## Forward — typed conjugation

```rust
use sudachi_morphology::{Verb, VerbClass, IAdjective, NaAdjective};

let taberu = Verb::new("食べる", VerbClass::Ichidan);

assert_eq!(taberu.negative().surface,        "食べない");
assert_eq!(taberu.past().surface,            "食べた");
assert_eq!(taberu.te_form().surface,         "食べて");
assert_eq!(taberu.causative().surface,       "食べさせる");
assert_eq!(taberu.passive().surface,         "食べられる");
assert_eq!(taberu.causative_passive().surface, "食べさせられる");
assert_eq!(taberu.imperative().surface,      "食べろ");
assert_eq!(taberu.volitional().surface,      "食べよう");
assert_eq!(taberu.potential().surface,       "食べられる");
assert_eq!(taberu.conditional_eba().surface, "食べれば");
assert_eq!(taberu.conditional_tara().surface, "食べたら");
```

Every conjugation paradigm has its own `VerbClass` variant. Variant names are linguistic descriptors — `Ichidan`, `GodanBu`, `GodanKuIku`, `Suru` — not abbreviated codes. JMdict's codes (`v1`, `v5b`, `v5k-s`, `vs-i`) live in `serde(rename = "...")` attributes for JSON interop.

```rust
let kau   = Verb::new("買う",  VerbClass::GodanU);    // 買った
let iku   = Verb::new("行く",  VerbClass::GodanKuIku); // 行った (irregular vs other godan-ku)
let aru   = Verb::new("ある",  VerbClass::GodanRuAru); // ない (irregular negative)
let suru  = Verb::new("する",  VerbClass::Suru);       // した
let kuru  = Verb::new("来る",  VerbClass::Kuru);       // 来た (with reading change)
```

### Adjectives

```rust
let utsukushii = IAdjective::new("美しい");
assert_eq!(utsukushii.past().surface,     "美しかった");
assert_eq!(utsukushii.negative().surface, "美しくない");
assert_eq!(utsukushii.te_form().surface,  "美しくて");

let shizuka = NaAdjective::new("静か");
assert_eq!(shizuka.attributive().surface, "静かな");
```

### Copula

```rust
use sudachi_morphology::{conjugate_copula, CopulaForm};

let polite_past = conjugate_copula(CopulaForm::Polite, /* past */ true);
// → でした
```

### Composite axes

A Japanese conjugated verb is the cartesian product of independent axes — Voice × Mood × Politeness × Polarity × Tense — not a flat enumeration. `Conjugation` lets you compose them:

```rust
use sudachi_morphology::{
    Conjugation, Voice, Mood, Politeness, Polarity, Tense,
};

let conj = Conjugation::default()
    .with_voice(Voice::CausativePassive)
    .with_politeness(Politeness::Polite)
    .with_polarity(Polarity::Negative)
    .with_tense(Tense::Past);

let chained = conj.apply(&taberu);
// → 食べさせられませんでした
// chained.steps gives intermediate forms: 食べる → 食べさせる → 食べさせられる →
//                                          食べさせられます → 食べさせられません → 食べさせられませんでした
```

Invalid combinations (e.g. `Mood::Imperative` + `Tense::Past`) return `None`.

### Honorific prefixes

Keigo constructions choose `お` for native (wago) verbs and `ご` for Sino-Japanese (kango):

```rust
use sudachi_morphology::HonorificPrefix;

let yomu = Verb::new("読む", VerbClass::GodanMu);
assert_eq!(
    yomu.honorific_oninaru_with_prefix(HonorificPrefix::O).surface,
    "お読みになる"
);

let setsumei = Verb::new("説明する", VerbClass::Suru);
assert_eq!(
    setsumei.honorific_oninaru_with_prefix(HonorificPrefix::Go).surface,
    "ご説明になる"
);
```

---

## Backward — rule-table deconjugation

```rust
use sudachi_morphology::deconjugate;

let forms = deconjugate("食べさせられた");

// Among the candidates:
//   text: "食べる",        class: "v1",     process: ["causative", "passive", "past"]
//   text: "食べさせる",    class: "v1",     process: ["passive", "past"]
//   text: "食べさせられる", class: "v1",     process: ["past"]
```

The deconjugator returns *every* valid endpoint, not just the most likely one. Callers filter by the leading tag (`v1`, `v5*`, `adj-i`, `vk`, `vs-i`, etc.) to find candidate dictionary forms.

### How it works

```text
1. Seed a queue with the input surface (no tags applied yet).
2. Pop forms; for each, try every rule whose con_end suffix matches.
3. If a rule matches, push the resulting form back onto the queue.
4. A form is valid as endpoint if at least one rule has applied AND
   its last rule wasn't a NeverFinal stem expansion.
5. Cycle detection: track every text the chain has produced; reject
   rules that would re-produce a seen text.
6. Length / depth / tag-density limits keep the search bounded.
```

Rules are indexed by their `con_end` suffix at startup using an Aho-Corasick automaton ([`daachorse`](https://github.com/daac-tools/daachorse)) so each step finds candidate rules in linear time.

### Six rule kinds

| Kind         | Behaviour                                                                 |
| ------------ | ------------------------------------------------------------------------- |
| Standard     | Applies anywhere; output is a valid endpoint                              |
| OnlyFinal    | Applies anywhere; output is **locked** — no further rules apply           |
| NeverFinal   | Applies only as **non-first** step; output must be extended by another rule |
| Rewrite      | Single-pair surface rewrite (e.g., です → でした). No array expansion       |
| Context      | Requires a named contextual condition (e.g., する's irregular short causative) |
| Substitution | Pure surface rewrite; ignores tags (e.g., ０-９ ↔ 0-9)                    |

---

## Data layout

Rules live in `data/`, organised by what they linguistically encode:

```
data/
├── stems/              izenkei / mizenkei / renyoukei / shuushikei
├── verb/               negation / past / te / polite / causative /
│                       passive / volitional / imperative / conditional /
│                       desiderative
├── auxiliary/          てしまう / ておく / ている / etc.
├── adjective/          i-adjective + na-adjective forms
├── copula/             だ / です / である / のだ
├── colloquial/         ちゃう / じゃう / ねえ / らん / etc.
├── dialect/            Kansai (へん, やん, とる, …)
├── keigo/              尊敬語 / 謙譲語 constructions
├── irregular/          full paradigms for する / 来る / ある / 行く
├── negative_chain/     なくて / なければ / ずに
└── deconjugation_rules.json   Compiled rule corpus
```

The JSON uses parallel-array compression: one entry like
```
{ "con_end": ["く", "す", "ぐ"], "dec_end": ["いた", "した", "いだ"], "con_tag": "...", "dec_tag": [...] }
```
expands into N concrete rule instances at load time. The rule loader handles the expansion and produces a flat `Vec<Rule>` for the deconjugator.

---

## Verb classes

Modern Japanese verb conjugation is mostly regular but has roughly twelve systematic godan classes (one per consonant) plus six special-case classes that conjugate slightly differently. Conflating these is the most common source of bugs in conjugation libraries; this crate keeps the distinctions explicit at the type level.

```rust
pub enum VerbClass {
    // Ichidan
    Ichidan,           // v1     食べる, 見る
    IchidanKureru,     // v1-s   くれる (irregular imperative)

    // Godan, by consonant row
    GodanBu,           // v5b    飛ぶ
    GodanGu,           // v5g    泳ぐ
    GodanKu,           // v5k    書く
    GodanKuIku,        // v5k-s  行く (irregular past)
    GodanMu,           // v5m    飲む
    GodanNu,           // v5n    死ぬ
    GodanRu,           // v5r    走る
    GodanRuAru,        // v5r-i  ある (irregular negative)
    GodanSu,           // v5s    話す
    GodanTsu,          // v5t    立つ
    GodanU,            // v5u    買う
    GodanUSpecial,     // v5u-s  請う (irregular past)

    // Suru / Kuru / Zuru
    Suru,              // vs-i   する
    SuruSpecial,       // vs-s   愛する
    SuruClassical,     // vs-c   す (classical)
    Kuru,              // vk     来る
    Zuru,              // vz     感ずる

    // Classical residue
    Yodan,             // v4r    classical four-grade
    // ... (see src/verb_class.rs for the full list)
}
```

---

## Validation

A golden corpus of ~4,800 hand-curated test cases covers every verb / adjective / copula class. Each test asserts that `deconjugate(input)` produces a candidate with the expected dictionary form, class tag, and process chain.

```bash
cargo test -p sudachi-morphology --test golden
```

When a deconjugator change goes in, this is the binary signal for whether it preserves correctness across thousands of cases.

A round-trip test (`tests/round_trip.rs`) confirms forward conjugation feeds back through deconjugation cleanly:

```bash
cargo test -p sudachi-morphology --test round_trip
```

---

## Cargo.toml

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
daachorse = "1"     # Aho-Corasick over rule.con_end suffixes

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "deconjugate"
harness = false
```

No Sudachi dependency. The crate is self-contained.

---

## Benchmarks

```bash
cargo bench -p sudachi-morphology --bench deconjugate
```

The deconjugator builds its rule index once via `LazyLock`, so amortised per-call cost is dominated by the BFS over candidate rules — typically a handful of suffix-trie hits and a few dozen form expansions per input.

---

## License

MIT
