# CLAUDE.md — sudachi-kwja-optimizer

Document-tree rewriter operating on KWJA's structural output.

## Single responsibility

Take a raw `Document` tree from `sudachi-kwja` and apply a sequence of rule-based corrections that fix mechanical KWJA mis-tagging (over-tagged NE spans, malformed BIO sequences, label-spelling drift). Produce a cleaned `Document` for the comprehension layer downstream.

## Layer (2) in the pipeline

Per [`COMPREHENSION_PIPELINE.md`](../../COMPREHENSION_PIPELINE.md):

```
(1) sudachi-optimizer        — Sudachi morpheme cleanup
(2) sudachi-kwja-optimizer   — KWJA tree cleanup  ← THIS CRATE
(3) jisho-core comprehension — combines (1) and (2) with jisho data
```

The discriminator that decides whether a rule belongs in (2) vs (3): **does it need jisho-specific data** (vocab table, grammar table, learner state)?

- **No** → (2). Mechanical KWJA cleanup, reusable by any KWJA consumer.
- **Yes** → (3). Lives in jisho-core; consumes (2)'s cleaned output.

Examples:
- "Drop NE entries where the surface is pure hiragana" → (2). Surface heuristic only.
- "Override KWJA's reading when corroborated by a vocab-table entry" → (3). Needs vocab data.

## Architecture

```
                              Optimizer
                                 │
                                 ▼
                        Pipeline (Vec<Stage>)
                                 │
                                 ▼
                       DocumentFeatures::scan(&doc)
                                 │
                                 ▼
              for each stage: skip if gate doesn't match
                                 │
                                 ▼
                    stage.apply(doc, &dyn Lexicon)
                                 │
                                 ▼
              re-scan features iff doc shape changed
                                 │
                                 ▼
                          cleaned Document
```

Each `Stage` is `Fn(Document, &dyn Lexicon) -> Document` (aliased as `StageFn`) plus metadata (`name`, `Phase`, `DocumentFeatures` gate). The runner scans features once, then runs each stage whose required features intersect with what's present — re-scanning only when a stage actually changed the document's structural shape (sentence/phrase/BP/feature/morpheme counts).

## File map

```
src/lib.rs           Public re-exports + module decls + document gateway
src/optimizer.rs     Optimizer struct + entry points
src/pipeline.rs      Pipeline + optimize() runner + canonical_stages()
src/stage.rs         Stage struct + Phase enum + StageFn type alias
src/doc_features.rs  DocumentFeatures bitflags + scan()
src/lookup.rs        Lexicon trait + EmptyLexicon

src/filter/          Drop spurious annotations
  ├── mod.rs
  └── ne.rs          NE span filter (per-tag heuristics)
src/validate/        Check structural invariants (currently empty)
src/normalize/       Canonicalise label spellings (currently empty)
```

One rule per file, organised by phase. Tests live alongside the rule.

## Pipeline runner contract

```rust
pub fn optimize<L: Lexicon>(mut doc: Document, pipeline: &Pipeline, lexicon: &L) -> Document {
    let lexicon_dyn: &dyn Lexicon = lexicon;
    let mut features = DocumentFeatures::scan(&doc);

    for stage in &pipeline.stages {
        if !stage.required_features.is_empty()
            && (features & stage.required_features).is_empty()
        {
            continue;  // gate: stage's features absent → skip
        }
        let prev_shape = doc_shape(&doc);
        let next = stage.apply(doc, lexicon_dyn);
        let changed = doc_shape(&next) != prev_shape;
        doc = next;
        if changed {
            features = DocumentFeatures::scan(&doc);
        }
    }
    doc
}
```

The shape signature catches BPs/morphemes/features added or removed; pure value rewrites (a Normalize rule changing a label spelling without altering counts) skip the re-scan, which is fine because subsequent stages' gates aren't sensitive to the value, only to presence.

## Phase enum

| Phase     | Purpose                                                                  |
| --------- | ------------------------------------------------------------------------ |
| Filter    | Drop spurious / low-confidence annotations                               |
| Validate  | Check structural invariants (BIO well-formedness, dep arcs)              |
| Normalize | Canonicalise label spellings (`敬語=尊敬` vs `敬語=尊敬語`)               |

KWJA-side cleanup operations have a different vocabulary than sudachi-optimizer's `Split / Combine / Repair` because we don't reorder morphemes here — we work with the tree KWJA produced.

## DocumentFeatures bitflags

```rust
HAS_NE_FEATURES        // some BP has an "NE" key
HAS_BP_FEATURES        // some BP has a non-empty features list
HAS_DEP_ARCS           // some Phrase or BP has head >= 0
HAS_MORPHEME_FEATURES  // some Morpheme has a non-empty features list
HAS_RELATIONS          // some BP has a non-empty relations list (PAS)
HAS_DISCOURSE          // document has any discourse_relations
```

Add a flag whenever a new stage needs to gate on a particular signal type. The scan is a single linear pass over the document; called once per pipeline run, then again only when a stage changes the document shape.

## Lexicon trait

```rust
pub trait Lexicon {
    fn knows(&self, _surface: &str) -> Option<bool> { None }
}
pub struct EmptyLexicon;
impl Lexicon for EmptyLexicon {}
```

Intentionally minimal compared to sudachi-optimizer's verb-form-rich version. (2) rules are mostly mechanical surface heuristics; rules that need rich vocab queries belong in (3) as hybrid rules. The trait exists for parity so any rule that DOES want optional refinement from a consumer-supplied catalog can reach for it.

## Document gateway

`src/lib.rs` re-exports the KWJA tree types via the `document` module:

```rust
pub mod document {
    pub use sudachi_kwja::document::tree::{
        BasePhrase, Document, KeyValue, Morpheme, Phrase, Relation, Sentence,
    };
}
```

Consumers reach the tree types via `sudachi_kwja_optimizer::document::*` rather than importing `sudachi-kwja` directly. Mirrors how `sudachi-optimizer::sudachi::*` gateways the upstream Sudachi types — same architectural lever applied to the KWJA upstream.

## When changing this crate

### Add a new rule

1. Pick a phase: `src/filter/`, `src/validate/`, `src/normalize/`.
2. Create `src/<phase>/<rule_name>.rs`. One rule per file.
3. Define the rule:
   ```rust
   use crate::doc_features::DocumentFeatures;
   use crate::document::Document;
   use crate::lookup::Lexicon;
   use crate::stage::{Phase, Stage};

   pub const NAME: &str = "<rule_name>";

   pub fn stage() -> Stage {
       Stage::new(NAME, Phase::Filter, DocumentFeatures::HAS_X, |d, l| apply(d, l))
   }

   pub fn apply(doc: Document, _lexicon: &dyn Lexicon) -> Document {
       // mutate doc in place
       doc
   }
   ```
4. Register in `pipeline::canonical_stages`.
5. Add the module to `src/<phase>/mod.rs`.
6. Inline unit tests:
   - Unit tests for each helper / heuristic in isolation
   - Document-level integration test with a synthetic input
   - Test the no-op case (don't fire when feature is absent)

### Add a new DocumentFeatures flag

Edit `src/doc_features.rs`. Add a `bitflags` variant. Update `DocumentFeatures::scan` to set it when the predicate holds. Existing stages keep working; new stages can gate on the new flag.

### Add a new Phase

Edit `src/stage.rs`. Add to the `Phase` enum. Update the `analysis()` doc to mention the new phase. The runner doesn't enforce phase ordering — `canonical_stages()` is the source of truth for ordering.

## Cargo.toml

```toml
[dependencies]
sudachi-kwja.workspace = true
bitflags = "2"
thiserror = "2"
```

No upstream `sudachi` dep — this crate works on KWJA output, not on Sudachi morphemes directly.

## Testing

```bash
cargo test -p sudachi-kwja-optimizer       # unit tests
cargo clippy -p sudachi-kwja-optimizer --no-deps -- -D warnings  # lint
just test                                   # workspace tests
just ci                                     # gate before commit
```

Tests are CPU-only synthetic Documents — no GPU, no KWJA checkpoints, no Sudachi dictionary required. Each rule's heuristics get unit tests; the rule's `apply()` gets Document-level integration tests.

## Performance

- Stage gating via `DocumentFeatures` short-circuits no-op stages cheaply.
- `DocumentFeatures::scan` is a single linear pass; called once per pipeline run, then again only when a stage changes the document shape.
- The shape signature avoids deep equality — counts at each nesting level only.
- The `Lexicon` trait is `&dyn`; vtable lookups are negligible compared to tree-walking cost.
- `Optimizer` is `Clone + Send + Sync` — the inner pipeline is `Arc`'d. Cheap to share across threads.

## Cross-references

- [`COMPREHENSION_PIPELINE.md`](../../COMPREHENSION_PIPELINE.md) — the three-layer architecture this crate sits inside
- [`crates/sudachi-optimizer/CLAUDE.md`](../sudachi-optimizer/CLAUDE.md) — the (1)-layer crate this mirrors structurally
- [`crates/sudachi-kwja/CLAUDE.md`](../sudachi-kwja/CLAUDE.md) — the upstream that produces our input
