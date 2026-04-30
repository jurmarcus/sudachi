# AGENTS.md — sudachi-morphology

Context for AI agents working on this crate.

## Purpose

Bidirectional Japanese morphology. Forward: typed `Verb`/`IAdjective`/`NaAdjective` API. Backward: rule-table BFS deconjugator. Both share the `ConjForm` taxonomy and round-trip cleanly.

| Attribute   | Value                                                      |
| ----------- | ---------------------------------------------------------- |
| Type        | rlib                                                       |
| Standalone  | Yes — no Sudachi dependency                                |
| Validation  | ~4,800 golden test cases across 23 classes + round-trip suite |
| Deps        | `serde`, `serde_json`, `thiserror`, `daachorse`            |

## Hard rules

1. **Round-trip MUST stay green.** Forward output of any well-formed verb must decompile through `deconjugate()` back to the original verb. `cargo test -p sudachi-morphology --test round_trip`.
2. **Golden test count never decreases.** ~4,800 cases in `tests/golden/`. A change that breaks any case is a regression unless explicitly justified in the commit.
3. **`VerbClass` variants are linguistic descriptors, not abbreviations.** The `serde(rename = "v1")` etc. handles JMdict interop; the Rust API uses `Ichidan`, `GodanKuIku`, etc.
4. **No Sudachi dependency.** This crate is intentionally standalone. Adding `sudachi` here breaks the gateway invariant the rest of the workspace relies on.
5. **Edit `data/deconjugation_rules.json` for rules.** Don't hand-modify compiled artefacts.

## File map

```
src/lib.rs            Public re-exports + Polite/HonorificPrefix enums
src/verb.rs           Verb<class> + Conjugated<form> typed forward API
src/verb_class.rs     VerbClass enum (every paradigm)
src/adjective.rs      IAdjective + NaAdjective forward API
src/copula.rs         Copula forms
src/conjugation.rs    Composite axis pipeline (Voice + Mood + Politeness + Polarity + Tense)
src/deconjugate.rs    BFS deconjugator
src/rule.rs           Rule + RuleKind + load_default_rules
src/rule_index.rs     RuleIndex (daachorse Aho-Corasick over rule.con_end)
src/irregular.rs      Hard-coded paradigms for する / 来る / ある / 行く
src/irregular/        Per-verb submodules
src/kana.rs           Hiragana/katakana helpers
src/tag.rs            ConjForm shared tag taxonomy

data/                 Rule corpus by linguistic role (stems/, verb/, adjective/, ...)
data/deconjugation_rules.json   Compiled rule data (loaded once via LazyLock)

tests/golden.rs       Golden corpus runner + helper formatter
tests/golden/*.rs     Per-class fixtures (v1, v5_*, vs_*, vk, vz, adj_i, cop, ...)
tests/round_trip.rs   Forward → backward identity check
benches/deconjugate.rs
```

## Forward composition pipeline

Order of axis application (`Conjugation::apply`):

1. **Voice** (Causative / Passive / Causative-Passive / Potential) — produces a new ichidan-class surface.
2. **Mood** — selects stem-form. Imperative / Volitional / Te terminate the chain (no Politeness/Polarity/Tense after).
3. **Politeness** — inserts ます; switches working class to "masu-verb".
4. **Polarity** — appends ない or transforms ます → ません; switches working class to "i-adjective-like".
5. **Tense** — applies past transformation. Sound changes depend on current working class.

Invalid combinations return `None`:
- `Mood::Imperative` + `Tense::Past`
- `Mood::Volitional` + `Tense::Past`
- `Mood::VolitionalNegative` + `Polarity::Negative`
- `Mood::Imperative` + `Polarity::Negative` (use `Mood::ImperativeNegative` instead)

## Backward algorithm (BFS)

```rust
fn deconjugate(input: &str) -> Vec<Form> {
    let mut queue = VecDeque::from([Form::seed(input)]);
    let mut results = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    while let Some(form) = queue.pop_front() {
        for rule in index.candidates(&form.text) {
            if let Some(next) = rule.try_apply(&form) {
                if seen.insert(next.text.clone()) {
                    if next.is_valid_endpoint() {
                        results.push(next.clone());
                    }
                    queue.push_back(next);
                }
            }
        }
    }
    results
}
```

`is_valid_endpoint`: at least one rule applied AND last rule wasn't `NeverFinal`. Without the latter, intermediate stems would leak into results.

Cycle detection by `seen` set on text. Without this, recursive rules (ない+ない, etc.) loop indefinitely.

## Six rule kinds

| Kind         | Semantics                                                  |
| ------------ | ---------------------------------------------------------- |
| Standard     | Applies anywhere; output is valid endpoint                 |
| OnlyFinal    | Applies anywhere; output LOCKED — chain ends                |
| NeverFinal   | Applies only as non-first step; output cannot be returned   |
| Rewrite      | Single-pair surface rewrite (です → でした); no array expansion |
| Context(k)   | Requires named contextual condition                         |
| Substitution | Pure surface rewrite; ignores tags                          |

## VerbClass invariants

Every paradigm that conjugates differently gets its own variant. Conflations are bugs.

```text
GodanKu        書く → 書いた     (regular -く past)
GodanKuIku     行く → 行った     (irregular -く past — 行く only)

GodanRu        走る → 走らない   (regular -る negative)
GodanRuAru     ある → ない       (irregular -る negative)

GodanU         買う → 買った     (regular -う past)
GodanUSpecial  請う → 請うた     (classical -う retention)
```

## When changing this crate

### Add a forward form

1. Method on `Verb` / `IAdjective` / `NaAdjective` (`src/verb.rs` etc.).
2. If the form is a new axis combination, extend `Conjugation` axes in `src/conjugation.rs`.
3. Fixture cases in `tests/golden/<class>.rs` covering all classes that produce it.
4. Round-trip cases in `tests/round_trip.rs` so forward output decompiles cleanly.
5. `cargo test -p sudachi-morphology` — both golden + round-trip green.

### Add a deconjugation rule

1. Edit `data/deconjugation_rules.json`. Use parallel-array compression for related rules.
2. If the rule needs a new context, extend `ContextKind` in `src/rule.rs` and the matching switch in the deconjugator.
3. Round-trip cases for any new endpoint.
4. Fixture cases in `tests/golden/<class>.rs`.
5. `cargo test -p sudachi-morphology`.

### Add a `VerbClass` variant

Only when a paradigm previously conflated turns out to differ. Steps:

1. Variant in `VerbClass` (`src/verb_class.rs`) with `serde(rename = "<jmdict_code>")`.
2. Forward conjugation logic in `src/verb.rs`.
3. New fixture file `tests/golden/<new_class>.rs` with curated cases.
4. Update relevant rules in `data/` to assign the right `dec_tag` for the new class.

### Update the data corpus

`data/deconjugation_rules.json` is the source of truth for backward rules. The on-disk `data/<role>/` subdirectories are the curated source from which the JSON is built (treat them as documentation of intent).

## Validation suites

```bash
cargo test -p sudachi-morphology --test golden       # ~4,800 fixture cases
cargo test -p sudachi-morphology --test round_trip   # forward ↔ backward identity
cargo test -p sudachi-morphology                     # all unit + integration tests
cargo bench -p sudachi-morphology --bench deconjugate
```

The golden runner (`tests/golden.rs`) formats deconjugator output as `～<chain>` strings and asserts set membership against the expected JL-style strings. A test passes when every expected chain is present in the output (extras are tolerated — the deconjugator may legitimately find more candidates).

## Performance characteristics

- Rule corpus expansion: one-time on first `deconjugate()` call. Cached via `LazyLock`.
- Aho-Corasick automaton (daachorse) over `rule.con_end` suffixes: built once with the rules, lookups are linear in input length.
- BFS bound: total form count limited by length × depth × tag-density caps.
- Forward conjugation: pure data manipulation, no automaton.

## Common pitfalls

| Symptom                       | Likely cause                                            | Fix                                                  |
| ----------------------------- | ------------------------------------------------------- | ---------------------------------------------------- |
| Round-trip fails              | Forward emits surface no rule recognises                | Add rule with matching `con_end`                     |
| Golden test "missing chain"   | Rule's `dec_tag` doesn't include the expected class     | Extend the rule's `dec_tag` parallel array            |
| Infinite loop in BFS          | Cycle detection broken                                  | Confirm `seen.insert` runs before recursion          |
| Wrong past form for Godan-ku  | Used `GodanKu` for 行く                                  | Use `GodanKuIku` for 行く only                       |
| Wrong negative for ある       | Used `GodanRu`                                           | Use `GodanRuAru`                                     |
| `Conjugation::apply` → None   | Invalid axis combination (Imperative + Past, etc.)      | Validate early or split shapes                       |

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

No Sudachi dependency. Standalone library by design.
