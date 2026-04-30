# sudachi-tantivy

**Sudachi tokenizer for [Tantivy](https://github.com/quickwit-oss/tantivy).**

Implements `tantivy::tokenizer::Tokenizer` backed by Sudachi morphological analysis. Supports four split modes — A, B, C, and Search (B+C, default). Used by `pg_search` (the ParadeDB Postgres extension) as a git dependency to provide the `pdb.sudachi` cast.

---

## Features

| Feature                    | Description                                                                 |
| -------------------------- | --------------------------------------------------------------------------- |
| **B+C multi-granularity**  | Search mode emits compound + sub-tokens at the same Tantivy position        |
| **Four split modes**       | A (finest), B (medium), C (coarsest), Search (B+C, default)                 |
| **Normalised forms**       | 食べた → 食べる, 附属 → 付属, ＳＵＭＭＥＲ → サマー, パーティー → パーティ   |
| **Shared dictionary**      | `Arc<JapaneseDictionary>` — one dict can back many tokenizers + modes       |

---

## Install

Inside this workspace:

```toml
sudachi-tantivy.workspace = true
```

As a git dep (downstream consumers — e.g. ParadeDB):

```toml
sudachi-tantivy = { git = "https://github.com/jurmarcus/sudachi" }
```

The dictionary is read from `SUDACHI_DICT_PATH` at construction time (or pre-loaded via `with_dictionary`).

---

## Quick start

```rust
use sudachi_tantivy::{SudachiTokenizer, SplitMode};
use tantivy::tokenizer::Tokenizer;

// Loads dictionary from SUDACHI_DICT_PATH
let tokenizer = SudachiTokenizer::new(SplitMode::Search)?;

let mut stream = tokenizer.token_stream("東京都立大学");
// Tokens emitted:
//   "東京都立大学"  position=0  position_length=1
//   "東京"          position=0  position_length=1   ← colocated
//   "都立"          position=0  position_length=1   ← colocated
//   "大学"          position=0  position_length=1   ← colocated

// Register with Tantivy
let manager = tantivy::tokenizer::TokenizerManager::default();
manager.register("sudachi", tokenizer);
```

Or share a pre-loaded dictionary across tokenizer instances:

```rust
let tokenizer = SudachiTokenizer::with_dictionary(dictionary, SplitMode::Search);
```

---

## Split modes

| Mode      | Granularity | "東京都立大学" output                                   |
| --------- | ----------- | ------------------------------------------------------- |
| A         | Finest      | `["東京", "都", "立", "大学"]`                          |
| B         | Medium      | `["東京", "都立", "大学"]`                              |
| C         | Coarsest    | `["東京都立大学"]`                                      |
| **Search**| **B+C**     | `["東京都立大学", "東京"*, "都立"*, "大学"*]` *= colocated |

In Search mode, colocated tokens land at the same Tantivy position as the preceding compound — Tantivy's position arithmetic doesn't advance.

---

## Normalisation

Default. Surface form is opt-in.

```rust
// Normalised (default — better recall)
let tokenizer = SudachiTokenizer::new(SplitMode::Search)?;

// Surface form (raw input)
let tokenizer = SudachiTokenizer::new(SplitMode::Search)?.with_surface_form();

// Or set explicitly
let tokenizer = SudachiTokenizer::new(SplitMode::Search)?.with_normalized_form(false);
```

---

## API

```rust
impl SudachiTokenizer {
    pub fn new(mode: SplitMode) -> Result<Self, SudachiError>;
    pub fn with_dictionary(dictionary: Arc<JapaneseDictionary>, mode: SplitMode) -> Self;

    pub fn with_surface_form(self) -> Self;
    pub fn with_normalized_form(self, enabled: bool) -> Self;
    pub fn uses_normalized_form(&self) -> bool;
    pub fn mode(&self) -> SplitMode;
}

impl Tokenizer for SudachiTokenizer {
    type TokenStream<'a> = SudachiTokenStream<'a>;
    fn token_stream<'a>(&'a mut self, text: &'a str) -> SudachiTokenStream<'a>;
}

pub enum SplitMode { A, B, C, Search }
```

---

## Architecture

```
SudachiTokenizer
  ├── TokenizerInner::Standard(StatelessTokenizer)   ← modes A, B, C
  └── TokenizerInner::Search(SearchTokenizer)        ← Search (B+C) via sudachi-search

SudachiTokenStream<'a>
  └── advances over a pre-collected Vec<TokenData>
      └── if token_data.is_colocated:
              token.position stays the same (no increment)
          else:
              token.position += 1
```

`token_stream` collects all tokens up front into a `Vec<TokenData>` (an owned, lifetime-free representation). This sidesteps a borrow-checker issue: Tantivy's `TokenStream` requires `&mut self`, but `StatelessTokenizer::tokenize` returns a struct that borrows the tokenizer. Pre-collection makes the stream trivially independent.

All Sudachi types come from `sudachi_optimizer::sudachi::*` — the workspace's single Sudachi gateway.

---

## Use in ParadeDB

`pg_search`'s `tokenizers/` crate has a `sudachi` Cargo feature gated on this dep:

```toml
[features]
sudachi = ["dep:sudachi-tantivy"]

[dependencies]
sudachi-tantivy = { git = "https://github.com/jurmarcus/sudachi", optional = true }
```

ParadeDB's workspace uses `[patch.crates-io]` to redirect `tantivy-tokenizer-api` to its own forked tantivy, so types unify across the crate boundary.

Build:

```bash
just pgrx-build   # cd ~/CODE/paradedb && cargo pgrx build -p pg_search --features icu,sudachi
just pgrx-check
```

SQL:

```sql
CREATE INDEX docs_idx ON documents
USING bm25(id, (content::pdb.sudachi))
WITH (key_field='id');

SELECT * FROM documents WHERE id @@@ 'content:大学';
```

---

## License

Apache-2.0
