# sudachi-kwja-optimizer

**Document-tree rewriter that fixes known KWJA mis-tagging.**

A pipeline of rules that runs over a raw KWJA `Document` tree (bunsetsu / BasePhrase / dependency / NE / features) and produces a cleaned tree better suited for downstream comprehension consumers.

This is the **layer (2)** of the comprehension pipeline. See [`COMPREHENSION_PIPELINE.md`](../../COMPREHENSION_PIPELINE.md) at the workspace root for the full architectural context.

---

## What it fixes

KWJA's structural analysis is mostly correct when fed clean Sudachi+optimizer morphemes, but the model has known type-specific failure modes — particularly in NE tagging, where common nouns get false-fired as named entities. Concrete examples:

| Phenomenon                            | KWJA produces                  | This crate emits        |
| ------------------------------------- | ------------------------------ | ----------------------- |
| Pure-hiragana common noun → PERSON    | `NE = "PERSON:やまだ"`         | NE feature dropped      |
| Single-kanji common noun → ARTIFACT   | `NE = "ARTIFACT:本"`           | NE feature dropped      |
| Malformed NE value                    | `NE = "PERSON山田"`            | NE feature dropped      |
| Real proper noun                      | `NE = "PERSON:山田太郎"`       | preserved               |
| Temporal expression in hiragana       | `NE = "DATE:きのう"`           | preserved (DATE allows) |

The rules are **mechanical surface heuristics** — no vocab corroboration, no learner state, no jisho-specific knowledge. Layer (2) is generic KWJA cleanup that any KWJA consumer could use.

---

## Quick start

```rust
use sudachi_kwja_optimizer::{Optimizer, document::Document};

// raw_doc came from sudachi-kwja's Pipeline::parse_morphemes(...)
let raw_doc: Document = /* ... */;

let optimizer = Optimizer::new();         // canonical pipeline
let clean_doc = optimizer.optimize(raw_doc);
```

For consumers that want a custom rule set:

```rust
use sudachi_kwja_optimizer::{Optimizer, Pipeline};

let optimizer = Optimizer::new().with_pipeline(Pipeline::empty()); // passthrough
```

---

## The three phases

| Phase     | Purpose                                                                  |
| --------- | ------------------------------------------------------------------------ |
| Filter    | Drop spurious / low-confidence annotations                               |
| Validate  | Check structural invariants (BIO sequences, dep arcs)                    |
| Normalize | Canonicalise label spellings (`敬語=尊敬` vs `敬語=尊敬語`)               |

Today only Filter has a stage (NE filter). Validate and Normalize are reserved for future rules — added when concrete failure cases emerge in the regression corpus, not speculatively.

---

## Where this fits in the comprehension pipeline

```
text → sudachi+optimizer (1) → clean morphemes
     → KWJA on (1) → raw tree → sudachi-kwja-optimizer (2) → clean tree
                                              │                      │
                                              └──────────┬───────────┘
                                                         ▼
                                            jisho-core comprehension layer (3)
```

Each layer is independently persisted, independently testable, independently consumable. (1) and (2) are reusable by non-jisho consumers; (3) is where all jisho-specific opinion lives.

---

## Adding a rule

1. Pick a phase: `src/filter/`, `src/validate/`, or `src/normalize/`.
2. Add `<rule_name>.rs`. One rule per file.
3. Implement:
   ```rust
   pub fn apply(doc: Document, _lexicon: &dyn Lexicon) -> Document {
       // mutate doc in place; return it
   }
   pub fn stage() -> Stage {
       Stage::new(NAME, Phase::Filter, FEATURE_GATE, |d, l| apply(d, l))
   }
   ```
4. Register it in `pipeline::canonical_stages`.
5. Inline unit tests for each heuristic + a Document-level integration test.

---

## Testing

```bash
cargo test -p sudachi-kwja-optimizer       # unit tests, fast, no checkpoints needed
cargo clippy -p sudachi-kwja-optimizer --no-deps -- -D warnings   # lint
```

Tests are CPU-only with synthetic Documents — no GPU, no KWJA inference, no checkpoints. Each rule's heuristics are tested in isolation before the Document-level integration test.

---

## Cargo.toml

```toml
[dependencies]
sudachi-kwja.workspace = true
bitflags = "2"
thiserror = "2"
```

The KWJA Document tree types are re-exported via this crate's `document` module so consumers don't need a direct `sudachi-kwja` dep.

---

## License

MIT
