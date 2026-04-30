# CLAUDE.md — sudachi-optimizer

Token-stream rewriter + the workspace's Sudachi gateway.

## Two responsibilities

1. **Rewriter pipeline**: takes raw Sudachi morphemes, runs five phases of corrective rules, returns the optimised stream.
2. **Sudachi gateway**: `sudachi-optimizer::sudachi` re-exports every upstream type the workspace needs. All other crates import from here, never from upstream `sudachi` directly.

## Why the gateway matters

Workspace `Cargo.toml`:

```toml
# DIRECT use of this dep is restricted to sudachi-optimizer; everything else
# imports through sudachi-optimizer's re-exports so post-tokenisation rules
# can apply uniformly across all consumers.
sudachi = { git = "https://github.com/WorksApplications/sudachi.rs", rev = "..." }
```

Practical consequences:
- `sudachi-search`, `sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm` all import `JapaneseDictionary`, `Mode`, `StatelessTokenizer`, `Tokenize`, etc. from `sudachi_optimizer::sudachi::*`.
- Adding a new Sudachi type to the surface area means adding a `pub use` line in `src/sudachi.rs` here — not propagating new direct deps.
- The optimisation pipeline can be inserted (or bypassed) uniformly — `Optimizer::tokenize_raw()` exposes the unoptimised path for search consumers.

## Architecture

```
                                  Optimizer
                                      │
                ┌─────────────────────┴─────────────────────┐
                ▼                                           ▼
    StatelessTokenizer.tokenize(text, mode, false)   Pipeline (Vec<Stage>)
                │                                           │
                ▼                                           ▼
        upstream sudachi::Morpheme<'_, T>            scan + run rules
                │                                           │
                ▼                                           │
   crate::Morpheme::from_sudachi(&m, &lexicon)              │
   (owned mirror — no lifetime to the iterator)             │
                └────────────► Vec<Morpheme> ◄──────────────┘
                                      │
                                      ▼
                             optimised stream returned
```

Each `Stage` is `Fn(Vec<Morpheme>, &dyn Lexicon) -> Vec<Morpheme>` plus metadata (`name`, `Phase`, `MorphemeFeatures` gate). The runner scans features once, then runs each stage whose features intersect the current stream — re-scanning only when a stage actually changed the morpheme count.

## File map

```
src/lib.rs            Public API + module declarations + load_dictionary helper
src/sudachi.rs        ★ The gateway — pub use lines for upstream types
src/optimizer.rs      Optimizer struct: dictionary + pipeline + default mode
src/pipeline.rs       Pipeline + optimize() runner + canonical_stages()
src/stage.rs          Stage struct + Phase enum
src/token.rs          Morpheme owned mirror + Pos closed enum
src/token_features.rs MorphemeFeatures bitflags
src/lookup.rs         Lexicon trait + EmptyLexicon
src/data.rs           Static rule data (irregular paradigms, etc.)

src/split/            Phase: break apart over-merged morphemes
  ├── compound_auxiliary_verbs.rs
  ├── tatte_particle.rs
  ├── tan_suffix.rs
  └── tawake_noun.rs

src/repair/           Phase: fix specific known mis-tokenisations
  ├── colloquial_negative_nee.rs
  ├── colloquial_ran_nai.rs
  ├── fused_interjection_particle.rs
  ├── hasa_noun.rs
  ├── n_tokenisation.rs
  ├── orphaned_auxiliary.rs
  ├── process_special_cases.rs
  ├── tanka_to_ta_n_ka.rs
  └── vowel_elongation.rs

src/combine/          Phase: glue together morphemes that should have been one
  ├── adverbial_particle.rs
  ├── amounts.rs
  ├── auxiliary.rs
  ├── auxiliary_verb_stem.rs
  ├── conjunctive_particle.rs
  ├── final.rs
  ├── inflections.rs
  ├── particles.rs
  ├── prefixes.rs
  ├── suffix.rs
  ├── to_naru.rs
  ├── tte.rs
  └── verb_dependant.rs

src/cleanup/          Phase: reclassify orphans, filter misparses
  ├── filter_misparse.rs
  └── reclassify_orphaned_suffixes.rs

src/disambiguation/   Phase: fix reading ambiguity using neighbouring context
  └── fix_reading_ambiguity.rs
```

28 rules total. Each is a single file. One rule, one file. Tests live alongside the rule.

## Canonical pipeline ordering

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

Phases interleave deliberately — some Combine rules need an earlier Repair to have run, and the second Cleanup pass catches orphans the second Combine pass produces.

## Pipeline runner contract

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
            continue;   // gate: stage's features absent → skip
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

Re-scanning features only when a stage actually changed the stream is the perf trick — most stages no-op cheaply through the gate without re-iterating.

## Pipeline presets

| Preset                | Contents                                                       | Use case                                  |
| --------------------- | -------------------------------------------------------------- | ----------------------------------------- |
| `Pipeline::analysis()`| All 28 stages in canonical ordering                            | Dictionary lookup, grammar matching       |
| `Pipeline::search()`  | Empty (today) — kept as a hook for search-friendly future rules | FTS consumers want raw Sudachi output    |
| `Pipeline::empty()`   | No stages                                                      | Test fixture                              |
| `Pipeline::new(stages)` | Custom                                                       | Specialised consumers                     |

`Optimizer::tokenize_raw()` skips the pipeline entirely regardless of which preset is configured.

## Owned Morpheme mirror

```rust
pub struct Morpheme {
    pub surface: String,
    pub reading_form: String,
    pub dictionary_form: String,
    pub normalized_form: String,
    pub part_of_speech: Vec<String>,    // full Sudachi POS array
    pub pos: Pos,                       // closed enum collapse of part_of_speech[0]
    pub char_range: Range<usize>,
    pub applied_rules: Vec<&'static str>, // audit trail
}
```

Owned (not borrowed from a Sudachi iterator) so stages can mutate freely. The `applied_rules` field is the audit trail — every stage that fires appends its name, so debugging "why did this morpheme become X?" reduces to printing `applied_rules`.

## Lexicon trait

```rust
pub trait Lexicon {
    fn knows(&self, surface: &str) -> bool;
    fn known_form(&self, surface: &str) -> Option<&str>;
}

pub struct EmptyLexicon;  // returns false / None — the no-op default
```

Vocab-aware stages (e.g., compound recognition for words present in a user vocab list) take `&dyn Lexicon` so they can be context-sensitive. Most stages don't use it; they accept it via the uniform stage signature.

## When changing this crate

### Add a new Sudachi re-export

Edit `src/sudachi.rs`. Add a `pub use sudachi::path::to::Type;` line. Other workspace crates can now reach it via `sudachi_optimizer::sudachi::Type`.

### Add a new rule

1. Pick a phase: `split/`, `repair/`, `combine/`, `cleanup/`, `disambiguation/`.
2. Create `src/<phase>/<rule_name>.rs`. One rule per file.
3. Implement:
   ```rust
   pub fn apply(morphemes: Vec<Morpheme>, _lexicon: &dyn Lexicon) -> Vec<Morpheme> {
       // ... transformation
   }
   ```
4. Register in `pipeline::canonical_stages` with the right `Phase` and `MorphemeFeatures` gate.
5. Unit tests in the same file. Cover both the trigger case and the no-op case.

### Add a new MorphemeFeature

Edit `src/token_features.rs`. Add a `bitflags` flag. Update `MorphemeFeatures::scan` to set it when the predicate holds. Existing stages keep working; new stages can gate on it.

### Add a `Pipeline` preset

```rust
impl Pipeline {
    pub fn my_preset() -> Self {
        Self::new(vec![
            Stage::new("...", Phase::Repair, MorphemeFeatures::empty(), |m, _| m),
            // ...
        ])
    }
}
```

## Cargo.toml

```toml
[dependencies]
sudachi.workspace = true             # the only direct upstream dep in the workspace
sudachi-morphology.workspace = true  # rule data
bitflags = "2"
thiserror = "2"
```

## Testing

```bash
cargo test -p sudachi-optimizer       # unit tests
just test                             # workspace tests
just ci                               # gate before commit
```

Most rule files have unit tests inline. The `tests/` subdirectory currently empty — if you add integration tests, prefer the inline pattern unless cross-rule scenarios warrant separation.

## Performance

- Stage gating via `MorphemeFeatures` short-circuits no-op stages cheaply.
- `MorphemeFeatures::scan` is a single linear pass; called once per pipeline run, then again only when a stage changes the stream length.
- The `Lexicon` trait is `&dyn`; vtable lookups are negligible compared to morpheme allocations.
- `Optimizer` is `Send + Sync` — pipeline is `Arc<Pipeline>`, dictionary is `Arc<JapaneseDictionary>`. Cheap to clone.
