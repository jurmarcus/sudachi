# sudachi-optimizer

**Token-stream rewriter that fixes known Sudachi mis-tokenisations.**

A five-phase pipeline (Split → Repair → Combine → Cleanup → Disambiguation) that runs over a raw Sudachi morpheme stream and produces a corrected stream — better suited for grammar/vocab span matching, dictionary lookup, and downstream NLP.

This crate is also the **single Sudachi gateway** for the workspace: every other crate imports `JapaneseDictionary`, `Mode`, `StatelessTokenizer`, etc. through `sudachi_optimizer::sudachi::*`, never from upstream `sudachi` directly.

---

## What it fixes

Sudachi's UniDic output is correct against the dictionary but ships known weaknesses against running text:

| Phenomenon                  | Example surface       | What Sudachi gives                | What you want                      |
| --------------------------- | --------------------- | --------------------------------- | ---------------------------------- |
| Compound auxiliary verbs    | 食べてしまった        | 食べ / て / しま / っ / た        | 食べる + てしまう (auxiliary chain) |
| Colloquial negative -ねえ   | 食べねえ              | 食べ / ねえ                       | 食べる + negative                  |
| Fused interjection+particle | じゃあ                | じゃ / あ                         | じゃあ (single token)              |
| Vowel elongation            | おはよー              | おはよ / ー                       | おはよう (normalised)              |
| Reading ambiguity           | 行った                | 行 / った  (potentially wrong reading) | 行く / past, with corrected reading |

The pipeline runs over Sudachi's raw output and produces a stream where these are normalised.

---

## Quick start

```rust
use std::sync::Arc;
use sudachi_optimizer::{Optimizer, Pipeline, load_dictionary};

let dict = Arc::new(load_dictionary("/abs/path/to/system_full.dic")?);
let optimizer = Optimizer::new(dict).with_pipeline(Pipeline::analysis());

for m in optimizer.tokenize("食べてしまった")? {
    println!(
        "{}\t{}\t{:?}\t{:?}",
        m.surface, m.reading_form, m.pos, m.applied_rules,
    );
}
```

`Pipeline::analysis()` runs every stage. For search consumers that want raw Sudachi output, use `Pipeline::search()` (currently empty — kept as a hook for future search-friendly rules) or `Pipeline::empty()`.

---

## The five phases

| Phase           | Purpose                                              |
| --------------- | ---------------------------------------------------- |
| Split           | Break apart over-merged Sudachi morphemes            |
| Repair          | Fix specific known mis-tokenisations                 |
| Combine         | Glue together morphemes that should have been one    |
| Cleanup         | Reclassify orphans, filter misparses                 |
| Disambiguation  | Fix reading ambiguity using neighbouring context     |

The canonical pipeline interleaves these phases — Split first, then alternating passes of Repair → Combine → Cleanup → Combine → Repair → Cleanup → Disambiguation. See `pipeline::canonical_stages` for the exact ordering.

---

## Why a separate crate (the Sudachi gateway)

The workspace's other crates — `sudachi-search`, `sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm` — all import Sudachi types through this crate's `sudachi` module:

```rust
use sudachi_optimizer::sudachi::{
    JapaneseDictionary,
    Mode,
    StatelessTokenizer,
    Tokenize,
    SudachiError,
};
```

This gives the workspace one place to:
- Pin the upstream Sudachi rev
- Apply optimisation rules uniformly across all consumers
- Swap the underlying tokenizer if needed (without touching every crate)

The workspace `Cargo.toml` makes the rule explicit:

> Direct use of upstream `sudachi` is restricted to `sudachi-optimizer`; everything else imports through the gateway so post-tokenisation rules apply uniformly across consumers.

---

## API tour

### Top-level types

| Type                | Role                                                                |
| ------------------- | ------------------------------------------------------------------- |
| `Optimizer`         | Wraps a `JapaneseDictionary` + chosen `Pipeline`                    |
| `Pipeline`          | Ordered bundle of `Stage`s + a runner                               |
| `Stage`             | One named transformation: `(Vec<Morpheme>, &dyn Lexicon) -> Vec<Morpheme>` |
| `Phase`             | Enum (`Split`/`Repair`/`Combine`/`Cleanup`/`Disambiguation`) — stage classification |
| `Morpheme`          | Owned mirror of `sudachi::Morpheme<'_, T>` + `applied_rules` audit trail |
| `Pos`               | Closed enum collapse of `part_of_speech[0]` (Noun, Verb, Particle, …) |
| `MorphemeFeatures`  | Bitflags marking traits of the input — gates whether stages run    |
| `Lexicon`           | Trait for vocab callbacks; `EmptyLexicon` is the no-op default     |

### Sudachi gateway re-exports (`sudachi-optimizer::sudachi`)

```rust
pub use sudachi::analysis::Mode;
pub use sudachi::analysis::Tokenize;
pub use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
pub use sudachi::config::Config;
pub use sudachi::dic::dictionary::JapaneseDictionary;
pub use sudachi::dic::storage::{Storage, SudachiDicData};
pub use sudachi::error::SudachiError;
pub use sudachi::prelude::Morpheme;     // upstream's Morpheme<'_, T> — distinct from this crate's owned Morpheme
```

If a consumer needs a Sudachi type that isn't yet re-exported, add it to `src/sudachi.rs` rather than importing upstream directly.

### Optimizer

```rust
impl Optimizer {
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self;
    pub fn with_pipeline(self, pipeline: Pipeline) -> Self;
    pub fn with_default_mode(self, mode: Mode) -> Self;

    // Tokenise + apply pipeline (default mode, EmptyLexicon)
    pub fn tokenize(&self, text: &str) -> Result<Vec<Morpheme>, OptimizeError>;

    // With custom lexicon
    pub fn tokenize_with<L: Lexicon>(&self, text: &str, lexicon: &L)
        -> Result<Vec<Morpheme>, OptimizeError>;

    // In a specific Sudachi mode
    pub fn tokenize_in<L: Lexicon>(&self, text: &str, mode: Mode, lexicon: &L)
        -> Result<Vec<Morpheme>, OptimizeError>;

    // Skip the pipeline entirely (raw Sudachi output)
    pub fn tokenize_raw(&self, text: &str) -> Result<Vec<Morpheme>, OptimizeError>;
    pub fn tokenize_raw_in(&self, text: &str, mode: Mode)
        -> Result<Vec<Morpheme>, OptimizeError>;
}
```

### Pipeline constructors

```rust
Pipeline::new(stages: Vec<Stage>) -> Pipeline    // custom stage list
Pipeline::analysis() -> Pipeline                  // every stage, canonical ordering
Pipeline::search() -> Pipeline                    // hook for search-friendly subset (empty today)
Pipeline::empty() -> Pipeline                     // no stages — test fixture
```

---

## Mapping to upstream Sudachi types

| `sudachi-optimizer::Morpheme` field | `sudachi::Morpheme<'_, T>` accessor   |
| ----------------------------------- | ------------------------------------- |
| `surface`                           | `m.surface()`                         |
| `reading_form`                      | `m.reading_form()`                    |
| `dictionary_form`                   | `m.dictionary_form()`                 |
| `normalized_form`                   | `m.normalized_form()`                 |
| `part_of_speech`                    | `m.part_of_speech()`                  |
| `char_range`                        | `m.begin_c()..m.end_c()`              |
| `pos`                               | (this crate's closed `Pos` enum)      |
| `applied_rules`                     | (this crate's audit trail)            |

The owned `Morpheme` mirror lets the pipeline mutate freely — append rule names, drop tokens, splice in new ones — without lifetime constraints from the borrowed Sudachi iterator.

---

## Adding a new rule

```
1. Pick a phase: split/, repair/, combine/, cleanup/, disambiguation/
2. Create <rule_name>.rs in that subdirectory
3. Implement:
       pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme>
4. Register in pipeline::canonical_stages with the right Phase + MorphemeFeatures gate
5. Unit tests in the same file
```

Rule files are small and self-contained — one concrete rewrite per file. The `MorphemeFeatures` gate is a bitflags set; stages whose required features don't intersect the current stream's features are skipped, so the canonical pipeline runs all 28 stages but most are no-ops on most inputs.

---

## Cargo.toml

```toml
[dependencies]
sudachi.workspace = true             # the only direct upstream dep in the workspace
sudachi-morphology.workspace = true  # rule data (verb/adjective/auxiliary classes)
bitflags = "2"                       # MorphemeFeatures
thiserror = "2"                      # OptimizeError
```

---

## License

MIT
