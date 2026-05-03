# AGENTS.md — sudachi-kwja-optimizer

Context for AI agents working on this crate.

## Purpose

Document-tree rewriter that fixes mechanical KWJA mis-tagging (NE over-tagging, BIO drift, label normalisation). Layer (2) of the comprehension pipeline.

| Attribute | Value                                                                       |
| --------- | --------------------------------------------------------------------------- |
| Type      | rlib                                                                        |
| Phases    | Filter, Validate, Normalize                                                 |
| Rules     | 1 today (filter/ne) — minimal at launch; grow when concrete cases emerge    |
| Deps      | `sudachi-kwja` (Document tree types), `bitflags`, `thiserror`               |
| Layer     | (2) — KWJA cleanup. (1) is sudachi-optimizer; (3) is jisho-core             |

## Hard rules

1. **Layer-(2) discriminator: no jisho-specific data allowed.** If a rule needs vocab corroboration, learner state, or any data that lives in jisho-core, it belongs in layer (3) as a hybrid rule, NOT here. The Lexicon trait is here for generic catalog queries (`knows(surface) -> Option<bool>`), not jisho-specific lookups.

2. **Never reorder morphemes or change shape gratuitously.** KWJA gave us a tree; we clean its annotations. Removing a morpheme from a BP, or restructuring the tree, would break the contract with downstream consumers that align spans by char offset against the input text.

3. **Stage signature is rigid:**
   ```rust
   pub fn apply(doc: Document, lexicon: &dyn Lexicon) -> Document
   ```
   Don't change. `Stage::new` enforces it via `Box<StageFn>`.

4. **One rule per file.** `src/<phase>/<rule_name>.rs`. Don't bundle.

5. **Document gateway: re-export, don't add direct deps elsewhere.** `sudachi_kwja_optimizer::document::*` is the only place workspace code should reach KWJA tree types from. Adding a new tree type to consumers means adding a `pub use` line in `src/lib.rs`'s `document` module.

6. **`DocumentFeatures` is a perf optimisation, not correctness.** Stages must produce correct output even when their gate is open; the gate just lets them skip work cheaply.

7. **Hold this crate to a higher lint standard than legacy crates.** `cargo clippy --no-deps -D warnings` must pass clean. sudachi-optimizer has 17 pre-existing warnings; don't propagate that pattern here.

## File map

```
src/lib.rs           Public re-exports + module decls + document gateway
src/optimizer.rs     Optimizer struct (entry point)
src/pipeline.rs      Pipeline + optimize() runner + canonical_stages()
src/stage.rs         Stage struct + Phase enum + StageFn type alias
src/doc_features.rs  DocumentFeatures bitflags + scan()
src/lookup.rs        Lexicon trait + EmptyLexicon

src/filter/{mod,ne}.rs       Filter phase + NE rule
src/validate/mod.rs          Validate phase (empty)
src/normalize/mod.rs         Normalize phase (empty)
```

## Common task: add a new rule

```bash
# 1. Decide phase + name
PHASE=filter   # or validate / normalize
NAME=my_rule

# 2. Create the file
touch src/$PHASE/$NAME.rs

# 3. Implement (template):
cat > src/$PHASE/$NAME.rs <<'EOF'
//! `<RuleName>` — <one-line summary>.
//!
//! <Why this is layer (2) and not (3): mechanical, no jisho data>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::EmptyLexicon;
    // unit tests for each heuristic
    // Document-level integration test
}
EOF

# 4. Register in src/<phase>/mod.rs
echo "pub mod $NAME;" >> src/$PHASE/mod.rs

# 5. Add to canonical_stages in src/pipeline.rs
# vec![..., crate::<phase>::<name>::stage(), ...]

# 6. Verify
cargo test -p sudachi-kwja-optimizer
cargo clippy -p sudachi-kwja-optimizer --no-deps -- -D warnings
```

## Common task: add a new DocumentFeatures flag

1. Edit `src/doc_features.rs`. Add a `bitflags` variant.
2. Update `DocumentFeatures::scan` to set it when the predicate holds.
3. Add a unit test for the scanner.
4. New stages can now gate on the flag. Existing stages keep working.

## Common task: add a new Phase

1. Edit `src/stage.rs`. Add to the `Phase` enum.
2. Update doc comments in `src/lib.rs` and `README.md` to mention it.
3. The runner doesn't enforce phase ordering; `canonical_stages()` is the source of truth.

## What lives in (3) instead

These belong in `~/CODE/jisho/packages/rs/jisho-core/src/analysis/` — NOT here:

- KWJA reading-drift correction gated on vocab-table corroboration → `kwja_reading_refinement.rs`
- KWJA BP feature → sense-pick register bias (gloss selection) → `sense_pick.rs`
- NE-augmented proper-noun span detection (consumes our cleaned NE output, augments proper_noun trie hits) → future hybrid rule

The (3) layer reads our cleaned output; we don't reach into its data.

## Sapling, not git

This monorepo is Sapling. Use `sl status` / `sl commit` / `sl push`. Each task = one logical commit per the workspace convention.
