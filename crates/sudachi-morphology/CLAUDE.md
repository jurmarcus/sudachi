# CLAUDE.md — sudachi-morphology

Bidirectional Japanese morphology — forward conjugation + backward deconjugation. Standalone (no Sudachi dependency).

## What this is

Two complementary surfaces sharing one tag taxonomy and one rule corpus:

- **Forward** (`Verb::*`, `IAdjective::*`, `NaAdjective::*`, `conjugate_copula`): I have a known verb / adjective / copula, give me a specific form.
- **Backward** (`deconjugate`): I see an arbitrary surface, return all candidate dictionary forms with derivation chains.

The forward output of any well-formed verb round-trips through `deconjugate()` back to the original verb — guaranteed by the round-trip test.

## Architecture

```
                          Verb { surface, class }
                                  │
        ┌─────────────────────────┴─────────────────────────┐
        ▼                                                   ▼
  forward: typed methods                     Conjugation { axes }.apply(verb)
  Verb::negative()                                          │
  Verb::past()                                              ▼
  Verb::causative_passive()                  Voice → Mood → Politeness → Polarity → Tense
                                             pipeline of axis transforms,
                                             each producing Conjugated { surface, class, tags }
                                                          │
                                  ┌───────────────────────┘
                                  ▼
                          Conjugated { surface, ... }


                          surface = "食べさせられた"
                                  │
                                  ▼
                          deconjugate(surface)
                                  │
                                  ├─► Aho-Corasick suffix-match  (daachorse)
                                  ├─► BFS over candidate rules
                                  ├─► Cycle detection by seen-text set
                                  └─► Returns Vec<Form> with process chains

                          Form { text: "食べる", tags: ["v1"], process: ["causative", "passive", "past"] }
```

## File map

```
src/lib.rs            Public API + re-exports + Polite/HonorificPrefix enums
src/verb.rs           Verb<class> + Conjugated<form> typed forward API
src/verb_class.rs     VerbClass enum (every modern paradigm + classical residues)
src/adjective.rs      IAdjective + NaAdjective forward API
src/copula.rs         conjugate_copula + CopulaForm
src/conjugation.rs    Composite axis pipeline (Voice + Mood + Politeness + Polarity + Tense)
src/deconjugate.rs    Backward BFS deconjugator + Form struct
src/rule.rs           Rule struct + RuleKind + load_default_rules
src/rule_index.rs     RuleIndex (daachorse Aho-Corasick over rule.con_end)
src/irregular.rs      Hard-coded paradigms for する / 来る / ある / 行く
src/kana.rs           Hiragana/katakana helpers + small kana →大きいかな normalisation
src/tag.rs            ConjForm shared tag taxonomy

src/irregular/        Per-verb paradigm submodules

data/                 Rule corpus organised by linguistic role
data/deconjugation_rules.json   Compiled rule data

tests/golden.rs       Golden corpus runner
tests/golden/*.rs     Per-class fixture modules (~4,800 cases across 23 classes)
tests/round_trip.rs   Forward → deconjugate round-trip check
benches/deconjugate.rs
```

## Two paradigms aligned on one taxonomy

The legacy `ConjForm` enum enumerated the cartesian product of axes (75+ variants like `PoliteNegativePast`). That's combinatorial — `2 × 2 × 2 × N × 2` axis values squashed into named atoms. It also forced the deconjugator (which has always returned tag chains like `["polite", "negative", "past"]`) and the forward conjugator to use *different* representations for the same information.

The `Conjugation` feature record (in `src/conjugation.rs`) aligns both directions:

```text
食べませんでした  ↔  Conjugation { politeness: Polite, polarity: Negative, tense: Past, ... }
```

Forward composition pipeline applies axes in canonical order:

1. **Voice** — Causative / Passive / Causative-Passive / Potential. Each produces a new ichidan-class verb.
2. **Mood** — selects which stem-form is used. Some moods (Imperative, Volitional, Te) terminate the chain and don't accept Politeness/Polarity/Tense.
3. **Politeness** — inserts ます. Switches the working class to a "masu-verb" with its own negative (ません) and past (ました) forms.
4. **Polarity** — appends ない (or transforms ます → ません). Switches the working class to "i-adjective-like" for the tense step.
5. **Tense** — applies past transformation. Sound changes depend on the current working class.

Invalid combinations (e.g. `Mood::Imperative` + `Tense::Past`, `Mood::VolitionalNegative` + `Polarity::Negative`) return `None`.

## VerbClass design

Variant names are linguistic descriptors — `Ichidan`, `GodanKuIku`, `Suru` — not abbreviated codes. Codes (`v1`, `v5k-s`, `vs-i`) live in `serde(rename = "...")` attributes for JSON interop with JMdict-derived data.

Why so many variants:

```
GodanKu      書く → 書いた
GodanKuIku   行く → 行った            (irregular vs other -く)
GodanRu      走る → 走らない
GodanRuAru   ある → ない              (irregular negative)
GodanU       買う → 買った
GodanUSpecial 請う → 請うた           (classical -う retention)
```

Conflating these is the #1 source of bugs in conjugation libraries. Keep them distinct at the type level.

## Deconjugator algorithm

```
fn deconjugate(input: &str) -> Vec<Form> {
    let mut queue = VecDeque::from([Form::seed(input)]);
    let mut results = Vec::new();
    let mut seen = HashSet::new();

    while let Some(form) = queue.pop_front() {
        // Try every rule whose con_end matches form.text suffix
        for rule in index.candidates(&form.text) {
            if let Some(next_form) = rule.try_apply(&form) {
                if seen.insert(next_form.text.clone()) {
                    if next_form.is_valid_endpoint() {
                        results.push(next_form.clone());
                    }
                    queue.push_back(next_form);
                }
            }
        }
    }
    results
}
```

Key invariants:
- Aho-Corasick (daachorse) over `rule.con_end` strings finds candidates in O(input.len()).
- Cycle detection via `seen` set on text — without this, recursive rules like ない+ない would loop.
- `is_valid_endpoint`: at least one rule applied AND last rule wasn't a `NeverFinal` (stem expansion).
- Length / depth / tag-density limits cap pathological inputs.

## Rule kinds

```rust
pub enum RuleKind {
    Standard,                // applies anywhere; output is valid endpoint
    OnlyFinal,               // applies anywhere; output LOCKED (no chains continue)
    NeverFinal,              // applies only as non-first step; cannot be returned
    Rewrite,                 // single-pair: です → でした
    Context(ContextKind),    // gated on a named context
    Substitution,            // pure surface rewrite; ignores tags
}
```

`OnlyFinal` is the rule kind the legacy "imperative" rule uses — once you've concluded "this is an imperative", no further deinflection can apply.

`NeverFinal` is for stems (izenkei → v1, mizenkei → v5*) that are intermediate roots: they advance the chain but the result isn't a valid dictionary lookup target on its own.

## Data corpus organisation

```
data/
├── stems/           izenkei / mizenkei / renyoukei / shuushikei
├── verb/            negation / past / te / polite / causative / passive /
│                    volitional / imperative / conditional / desiderative
├── auxiliary/       てしまう / ておく / ている / etc.
├── adjective/       i-adj + na-adj forms
├── copula/          だ / です / である / のだ
├── colloquial/      ちゃう / じゃう / ねえ / らん / etc.
├── dialect/         Kansai (へん, やん, とる, …)
├── keigo/           尊敬語 / 謙譲語
├── irregular/       する / 来る / ある / 行く full paradigms
└── negative_chain/  なくて / なければ / ずに
```

The compiled `data/deconjugation_rules.json` uses parallel-array compression — one entry expands into N rule instances at load time. The expansion happens once on first call to `deconjugate()` (via `LazyLock`); subsequent calls reuse the flat rule vec and the daachorse automaton built over it.

## Validation

```bash
cargo test -p sudachi-morphology --test golden       # ~4,800 cases across 23 classes
cargo test -p sudachi-morphology --test round_trip   # forward → backward identity
```

Each `tests/golden/<class>.rs` is a per-class fixture: `v1.rs`, `v5_k.rs`, `v5_k_s.rs`, `vs_i.rs`, `cop.rs`, `adj_i.rs`, etc. The runner asserts that `deconjugate(input)` produces a candidate whose `process` chain matches the expected formatted string per the canonical "process to text" rules.

When a deconjugator change goes in, the golden test count in `cargo test -p sudachi-morphology --test golden` is the load-bearing signal: it MUST stay green.

## Performance

- Rule corpus expansion: one-time, on first `deconjugate()` call. `LazyLock` caches.
- daachorse Aho-Corasick: built once over `rule.con_end` suffixes; lookups are linear in input length.
- BFS bound: total form count limited by length × depth × tag-density caps (configured in `deconjugate.rs`).
- Forward conjugation: pure data structure manipulation, no automaton overhead.
- Bench: `cargo bench -p sudachi-morphology --bench deconjugate` (criterion).

## When changing this crate

### Add a forward conjugation form

1. Add the method to `Verb` / `IAdjective` / `NaAdjective` in `src/verb.rs` etc.
2. If it's a new axis combination, extend `Conjugation` in `src/conjugation.rs`.
3. Add cases to `tests/golden/<class>.rs` covering the new form.
4. Add round-trip cases in `tests/round_trip.rs` so forward output decompiles cleanly.
5. `cargo test -p sudachi-morphology` to verify both suites stay green.

### Add a deconjugation rule

1. Edit `data/deconjugation_rules.json` (or the relevant subcorpus). Use parallel-array compression for related rules.
2. If the rule needs a new context, extend `ContextKind` in `src/rule.rs` and the matching switch in the deconjugator.
3. Add round-trip cases for any new endpoint.
4. Add fixture cases to `tests/golden/<class>.rs`.
5. `cargo test -p sudachi-morphology` — golden + round-trip MUST stay green.

### Add a VerbClass variant

This is rare — should only happen when a paradigm currently conflated turns out to differ. Steps:

1. Add the variant to `VerbClass` in `src/verb_class.rs` with its `serde(rename)` JMdict code.
2. Add forward conjugation logic in `src/verb.rs` (probably a new arm in the master match).
3. Add a fixture file `tests/golden/<new_class>.rs` with curated test cases.
4. Update any relevant rules in `data/` to assign the right `dec_tag` for the new class.

### Update the data corpus

`data/deconjugation_rules.json` is the source of truth for backward rules. Edit it directly; the loader handles the rest. Don't modify the compiled corpus by hand without keeping the JSON in sync.

## Common pitfalls

| Symptom                         | Likely cause                                                | Fix                                              |
| ------------------------------- | ----------------------------------------------------------- | ------------------------------------------------ |
| Round-trip test fails           | Forward emits a surface no rule recognises                  | Add a rule whose `con_end` matches the new surface |
| Golden test reports missing chain | Rule's `dec_tag` doesn't list the expected class           | Add the class to the rule's `dec_tag` parallel array |
| Deconjugator infinite-looping   | Cycle detection broken or rule produces a seen text         | Verify `seen.insert(...)` is called before recursion |
| Verb conjugates to wrong form   | Wrong `VerbClass` chosen at `Verb::new`                     | Pick the more specific variant (`GodanRuAru`, `GodanUSpecial`, …) |
| `Conjugation::apply` returns None | Invalid axis combination (Imperative + Past, etc.)        | Validate early or split into mutually-exclusive shapes |

## Cargo.toml

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
daachorse = "1"

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "deconjugate"
harness = false
```

No Sudachi dependency. The crate is intentionally standalone — `sudachi-optimizer` consumes it for rule data, but downstream projects can use this crate as a pure morphology library independent of the rest of the workspace.
