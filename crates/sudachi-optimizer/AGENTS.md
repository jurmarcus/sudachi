# AGENTS.md — sudachi-optimizer

Context for AI agents working on this crate.

## Purpose

1. **Token-stream rewriter pipeline** — five phases of corrective rules over raw Sudachi morphemes.
2. **Sudachi gateway** — the only crate in the workspace that imports upstream `sudachi` directly. All other crates reach Sudachi types through `sudachi_optimizer::sudachi::*`.

| Attribute   | Value                                                              |
| ----------- | ------------------------------------------------------------------ |
| Type        | rlib                                                               |
| Phases      | Split, Repair, Combine, Cleanup, Disambiguation                    |
| Rules       | 28 stages (one Rust file each)                                     |
| Deps        | `sudachi` (upstream), `sudachi-morphology`, `bitflags`, `thiserror` |

## Hard rules

1. **This is the ONLY crate that imports `sudachi` directly.** Every other workspace crate consumes Sudachi types via `sudachi_optimizer::sudachi::*`. Adding a new Sudachi import elsewhere is a workspace-level smell — add the re-export here instead.
2. **One rule per file.** `src/<phase>/<rule_name>.rs`. Don't bundle.
3. **Stage signature is rigid:**
   ```rust
   pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme>
   ```
   Don't change this — `Stage::new` enforces it via boxed closure.
4. **`MorphemeFeatures` is a perf optimisation, not correctness.** Stages must produce correct output even when their gate is open; the gate just lets them skip work cheaply.
5. **`applied_rules` is the audit trail.** Every stage that fires must push its `name` onto the modified morphemes' `applied_rules` vec. This is how downstream debugging works.

## File map

```
src/lib.rs            Public re-exports + module decls + load_dictionary helper
src/sudachi.rs        ★ The gateway — pub use lines for upstream sudachi types
src/optimizer.rs      Optimizer struct + tokenize/tokenize_raw entry points
src/pipeline.rs       Pipeline + optimize() runner + canonical_stages()
src/stage.rs          Stage struct + Phase enum
src/token.rs          Morpheme owned mirror + Pos closed enum + from_sudachi()
src/token_features.rs MorphemeFeatures bitflags + scan()
src/lookup.rs         Lexicon trait + EmptyLexicon
src/data.rs           Static rule data

src/{split,repair,combine,cleanup,disambiguation}/
                      One rule per file, organised by phase
```

## Sudachi gateway

`src/sudachi.rs` is the entire gateway:

```rust
pub use ::sudachi::analysis::Mode;
pub use ::sudachi::analysis::Tokenize;
pub use ::sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
pub use ::sudachi::config::Config;
pub use ::sudachi::dic::dictionary::JapaneseDictionary;
pub use ::sudachi::dic::storage::{Storage, SudachiDicData};
pub use ::sudachi::error::SudachiError;
pub use ::sudachi::prelude::Morpheme;
```

Need a new Sudachi type elsewhere? Add a `pub use` line here, then import from `sudachi_optimizer::sudachi::*` in the consumer.

## Pipeline runner

```rust
pub fn optimize<L: Lexicon>(
    mut morphemes: Vec<Morpheme>,
    pipeline: &Pipeline,
    lexicon: &L,
) -> Vec<Morpheme> {
    let lexicon_dyn: &dyn Lexicon = lexicon;
    let mut features = MorphemeFeatures::scan(&morphemes);

    for stage in &pipeline.stages {
        if !stage.required_features.is_empty()
            && (features & stage.required_features).is_empty() {
            continue;
        }
        let prev_len = morphemes.len();
        let next = stage.apply(morphemes, lexicon_dyn);
        let changed = next.len() != prev_len;
        morphemes = next;
        if changed {
            features = MorphemeFeatures::scan(&morphemes);
        }
    }
    morphemes
}
```

The gate (`features & required_features` empty → skip) is the perf trick. Re-scanning only when length changes is cheap because most stages either no-op or modify in-place.

## Canonical stage ordering

```text
Split:          CompoundAuxiliaryVerbs, TatteParticle, TanSuffix, TawakeNoun
Repair:         HasaNoun, NTokenisation, VowelElongation, ProcessSpecialCases,
                ColloquialNegativeNee, ColloquialRanNai
Combine:        Prefixes, Inflections, Amounts, Tte, AuxiliaryVerbStem, Suffix
Cleanup:        ReclassifyOrphanedSuffixes
Combine:        ConjunctiveParticle, Auxiliary, ToNaru
Repair:         FusedInterjectionParticle, OrphanedAuxiliary
Combine:        AdverbialParticle, VerbDependant, Particles, Final
Repair:         TankaToTaNKa
Cleanup:        FilterMisparse
Disambiguation: FixReadingAmbiguity
```

Phases interleave because some Combine rules depend on earlier Repair output, and the second Cleanup catches orphans the second Combine produces. Don't reorder lightly.

## When changing this crate

### Add a new rule

1. Pick a phase. The phase determines _when_ the rule runs in the canonical pipeline.
2. Create `src/<phase>/<rule_name>.rs`.
3. Implement:
   ```rust
   use crate::lookup::Lexicon;
   use crate::token::Morpheme;

   pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
       morphemes.into_iter().map(|m| {
           if /* trigger condition */ {
               let mut m = m;
               m.applied_rules.push("rule_name");
               // ... transformation
               m
           } else {
               m
           }
       }).collect()
   }

   #[cfg(test)]
   mod tests {
       use super::*;
       // ... unit tests
   }
   ```
4. Add to `src/<phase>/mod.rs`: `pub mod <rule_name>;`
5. Register in `pipeline::canonical_stages`:
   ```rust
   Stage::new(
       "<rule_name>",
       Phase::<Phase>,
       MorphemeFeatures::<gate>,
       <phase>::<rule_name>::apply,
   ),
   ```
6. `cargo test -p sudachi-optimizer` to verify.

### Add a new MorphemeFeature

Edit `src/token_features.rs`:

```rust
bitflags::bitflags! {
    pub struct MorphemeFeatures: u32 {
        // ... existing flags
        const MY_NEW_FEATURE = 0b...;
    }
}

impl MorphemeFeatures {
    pub fn scan(morphemes: &[Morpheme]) -> Self {
        let mut features = Self::empty();
        for m in morphemes {
            // ... existing checks
            if /* my predicate */ {
                features |= Self::MY_NEW_FEATURE;
            }
        }
        features
    }
}
```

### Add a new Pipeline preset

```rust
impl Pipeline {
    pub fn my_preset() -> Self {
        Self::new(vec![ /* Stage::new(...) */ ])
    }
}
```

### Skip the pipeline for a consumer

Use `Optimizer::tokenize_raw()` or `tokenize_raw_in()` to get raw Sudachi output. `sudachi-search` does this — it wants the unoptimised stream so its B+C two-pass logic can pair Mode C compounds with Mode B sub-tokens.

## Testing

```bash
cargo test -p sudachi-optimizer    # unit tests
just test                          # workspace tests
just ci                            # gate before commit
```

Each rule file has inline unit tests. Cover at least:
- The trigger case (rule fires, produces expected output)
- The no-op case (rule sees stream that doesn't trigger, returns unchanged)
- Edge cases (empty input, single morpheme, boundary conditions)

## Cargo.toml

```toml
[dependencies]
sudachi.workspace = true             # the gateway — only this crate
sudachi-morphology.workspace = true
bitflags = "2"
thiserror = "2"
```

## Performance

- `MorphemeFeatures::scan` is a single linear pass over morphemes, called once per pipeline run plus once per stage that changes the stream length.
- Stage gating short-circuits cheaply — no-op stages don't re-iterate.
- Owned `Morpheme` allocations dominate cost. Most rules `into_iter().map().collect()` — that's one `Vec<Morpheme>` allocation per stage that fires.
- `Optimizer` is `Send + Sync`. Cheap to clone (Arc'd dict + Arc'd pipeline).
