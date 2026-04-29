# sudachi-optimizer

Token-stream rewriter that fixes known Sudachi mis-tokenisations.

## What this is

Sudachi's UniDic tokenization is correct against the dictionary but has
known weaknesses: compound auxiliary verbs, colloquial inflections,
fused interjection+particle pairs, vowel elongation, and so on. This
crate runs a sequence of small named transformations on the raw
Sudachi token stream to produce an *optimised* stream that's better
suited for grammar/vocab span matching downstream.

```
sudachi raw tokens → OPTIMIZER PIPELINE → post-optimised tokens
                          ↓
                   applied_rules tracked per token
```

## Why a separate crate

Every other crate in this workspace (`sudachi-search`, `sudachi-sqlite`,
`sudachi-tantivy`, `sudachi-wasm`) imports Sudachi types through this
crate's [`sudachi`](src/sudachi.rs) re-export module — never the
upstream `sudachi` crate directly. That gives one place to apply the
optimization rules so all consumers see the same canonical token
stream.

## Stage groups

Mirrors the categorisation in
[Sirush/Jiten](https://github.com/Sirush/Jiten/tree/master/Jiten.Parser/Stages)
where the initial rule set was ported from:

| Group           | Purpose                                              |
|-----------------|------------------------------------------------------|
| Split           | Break apart over-merged Sudachi tokens               |
| Repair          | Fix specific known mis-tokenisations                 |
| Combine         | Glue together tokens that should have been one       |
| Cleanup         | Reclassify orphans, filter misparses                 |
| Disambiguation  | Fix reading ambiguity using neighbouring context     |

## Quick start

```rust,ignore
use sudachi_optimizer::{Tokenizer, RuleSet};
use std::sync::Arc;

let dict = sudachi_optimizer::load_dictionary("/path/to/system_full.dic")?;
let tokenizer = Tokenizer::new(Arc::new(dict)).with_rules(RuleSet::analysis());
let tokens = tokenizer.tokenize("食べてしまった")?;
```

For search consumers that want raw Sudachi output (no rules):

```rust,ignore
let tokenizer = Tokenizer::new(Arc::new(dict)).with_rules(RuleSet::search());
let tokens = tokenizer.tokenize("東京都立大学")?;
```

## Adding a new rule

1. Pick a category subdirectory (`split/`, `repair/`, `combine/`,
   `cleanup/`, `disambiguation/`).
2. Add a new `<rule_name>.rs` file (one rule per file).
3. Define `pub fn apply(tokens: Vec<OptimizerToken>, lookup: &dyn OptimizerLookup) -> Vec<OptimizerToken>`.
4. Register it in [`pipeline::canonical_stages`](src/pipeline.rs) with the
   right [`StageGroup`] and [`TokenFeatures`] gate.
5. Write a unit test in the same file.

## Source attribution

Initial rule set ported from
[Sirush/Jiten](https://github.com/Sirush/Jiten) (MIT). Each rule's
docstring links back to its C# original so future audits can verify
behaviour. The rule scaffolds currently no-op; bodies are filled in
incrementally so each port lands as its own reviewable commit.

## Status

- [x] Framework: `OptimizerToken`, `SemanticPos`, `TokenFeatures`,
      `Stage`, `RuleSet`, pipeline runner, `Tokenizer` wrapper
- [x] Sudachi gateway re-exports (`sudachi::*` accessed through this
      crate by every other workspace member)
- [x] All 28 rule scaffolds (no-op stubs)
- [ ] Port rule bodies from Jiten (in progress)
