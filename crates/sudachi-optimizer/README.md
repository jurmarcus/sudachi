# sudachi-optimizer

Token-stream rewriter that fixes known Sudachi mis-tokenisations.

## What this is

Sudachi's UniDic tokenization is correct against the dictionary but
has known weaknesses: compound auxiliary verbs, colloquial
inflections, fused interjection+particle pairs, vowel elongation,
and so on. This crate runs a sequence of small named transformations
on the raw Sudachi morpheme stream to produce an *optimised* stream
that's better suited for grammar/vocab span matching downstream.

```
sudachi raw morphemes → OPTIMIZER PIPELINE → post-optimised morphemes
                              ↓
                     applied_rules tracked per morpheme
```

## Why a separate crate

Every other crate in this workspace (`sudachi-search`, `sudachi-sqlite`,
`sudachi-tantivy`, `sudachi-wasm`) imports Sudachi types through this
crate's [`sudachi`](src/sudachi.rs) re-export module — never the
upstream `sudachi` crate directly. That gives one place to apply the
optimization rules so all consumers see the same canonical morpheme
stream.

## Phases

Mirrors the categorisation in
[Sirush/Jiten Stages/](https://github.com/Sirush/Jiten/tree/master/Jiten.Parser/Stages)
where the initial rule set was ported from:

| Phase           | Purpose                                              |
|-----------------|------------------------------------------------------|
| Split           | Break apart over-merged Sudachi morphemes            |
| Repair          | Fix specific known mis-tokenisations                 |
| Combine         | Glue together morphemes that should have been one    |
| Cleanup         | Reclassify orphans, filter misparses                 |
| Disambiguation  | Fix reading ambiguity using neighbouring context     |

## Quick start

```rust,ignore
use std::sync::Arc;
use sudachi_optimizer::{Optimizer, Pipeline};

let dict = Arc::new(sudachi_optimizer::load_dictionary("/path/to/system_full.dic")?);
let optimizer = Optimizer::new(dict).with_pipeline(Pipeline::analysis());
let morphemes = optimizer.tokenize("食べてしまった")?;
for m in &morphemes {
    println!(
        "{}\t{}\t{:?}\t{:?}",
        m.surface, m.reading_form, m.pos, m.applied_rules,
    );
}
```

For search consumers that want raw Sudachi output (no rules):

```rust,ignore
let optimizer = Optimizer::new(dict).with_pipeline(Pipeline::search());
let morphemes = optimizer.tokenize("東京都立大学")?;
```

## Type tour for Sudachi users

| Sudachi-optimizer type     | Maps to in Sudachi                              |
|----------------------------|-------------------------------------------------|
| [`Morpheme`]               | Owned mirror of `sudachi::Morpheme<'_, T>`     |
| `Morpheme::surface`        | `m.surface()`                                  |
| `Morpheme::reading_form`   | `m.reading_form()`                             |
| `Morpheme::dictionary_form`| `m.dictionary_form()`                          |
| `Morpheme::normalized_form`| `m.normalized_form()`                          |
| `Morpheme::part_of_speech` | `m.part_of_speech()`                           |
| `Morpheme::char_range`     | `m.begin_c()..m.end_c()`                       |
| [`Pos`]                    | Closed enum collapse of `part_of_speech[0]`    |
| [`Optimizer`]              | High-level wrapper of `StatelessTokenizer`     |
| [`Lexicon`]                | Optional vocab callback (consumer-supplied)    |
| `sudachi::*` re-exports    | The only place to reach upstream `sudachi`     |

## Adding a new rule

1. Pick a phase subdirectory (`split/`, `repair/`, `combine/`,
   `cleanup/`, `disambiguation/`).
2. Add a new `<rule_name>.rs` file (one rule per file).
3. Define `pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme>`.
4. Register it in [`pipeline::canonical_stages`](src/pipeline.rs)
   with the right [`Phase`] and [`MorphemeFeatures`] gate.
5. Write unit tests in the same file.

## Source attribution

Initial rule set ported from
[Sirush/Jiten](https://github.com/Sirush/Jiten) (MIT). Each rule's
docstring links back to its C# original so future audits can verify
behaviour. Bodies are filled in incrementally — one rule per commit,
TDD-driven from Jiten's own test cases.

## Status

- [x] Framework: `Morpheme`, `Pos`, `MorphemeFeatures`, `Stage`,
      `Phase`, `Pipeline`, pipeline runner, `Optimizer`, `Lexicon`
- [x] Sudachi gateway re-exports (`sudachi::*` accessed through this
      crate by every other workspace member)
- [x] All 28 rule scaffolds (no-op stubs)
- [ ] Port rule bodies from Jiten with TDD-style ports of Jiten's
      test suite (in progress, one rule per commit)
