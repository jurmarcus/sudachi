# sudachi-tantivy

**Sudachi Japanese tokenizer for [Tantivy](https://github.com/quickwit-oss/tantivy) full-text search.**

Implements `tantivy::tokenizer::Tokenizer` backed by Sudachi morphological analysis with B+C multi-granularity tokenization. Used by [jurmarcus/paradedb](https://github.com/jurmarcus/paradedb) as a git dependency.

---

## Features

| Feature | Description |
|---------|-------------|
| **B+C Multi-Granularity** | Search mode emits compound words AND sub-tokens at the same Tantivy position |
| **Four Split Modes** | A (finest), B (medium), C (coarsest), Search (B+C, default) |
| **Normalized Forms** | 食べた → 食べる (verb conjugation), variant kanji, fullwidth normalization |
| **Shared Dictionary** | `Arc<JapaneseDictionary>` for efficient multi-tokenizer setups |

---

## Quick Start

```rust
use sudachi_tantivy::{SudachiTokenizer, SplitMode};
use tantivy::tokenizer::Tokenizer;

// Load via SUDACHI_DICT_PATH environment variable
let tokenizer = SudachiTokenizer::new(SplitMode::Search)?;

// Or share a pre-loaded dictionary
let tokenizer = SudachiTokenizer::with_dictionary(dictionary, SplitMode::Search);

// Register with Tantivy
let manager = tantivy::tokenizer::TokenizerManager::default();
manager.register("sudachi", tokenizer);
```

---

## Split Modes

| Mode | Description | "東京都立大学" Output |
|------|-------------|----------------------|
| A | Finest granularity | ["東京", "都", "立", "大学"] |
| B | Medium granularity | ["東京", "都立", "大学"] |
| C | Coarsest granularity | ["東京都立大学"] |
| **Search** | **B+C (default)** | ["東京都立大学", "東京"\*, "都立"\*, "大学"\*] |

\* Colocated tokens — Tantivy position does not advance for these tokens.

---

## Search Mode

Search mode emits compound words AND their sub-tokens at the same Tantivy position,
enabling both exact and partial matching:

```rust
let mut tokenizer = SudachiTokenizer::new(SplitMode::Search)?;
let mut stream = tokenizer.token_stream("東京都立大学");

// Tokens (all at position 0):
//   "東京都立大学"  position=0, position_length=1
//   "東京"          position=0, position_length=1  ← colocated
//   "都立"          position=0, position_length=1  ← colocated
//   "大学"          position=0, position_length=1  ← colocated
```

---

## Normalization

Default is normalized form for better recall:

| Surface | Normalized | Type |
|---------|------------|------|
| 食べた | 食べる | Verb conjugation |
| 美しかった | 美しい | Adjective inflection |
| 附属 | 付属 | Variant kanji |
| ＳＵＭＭＥＲ | サマー | Fullwidth conversion |

```rust
// Normalized (default)
let tokenizer = SudachiTokenizer::new(SplitMode::Search)?;

// Surface form
let tokenizer = SudachiTokenizer::new(SplitMode::Search)?.with_surface_form();
```

---

## API

```rust
impl SudachiTokenizer {
    // Construction
    pub fn new(mode: SplitMode) -> Result<Self, SudachiError>;
    pub fn with_dictionary(dictionary: Arc<JapaneseDictionary>, mode: SplitMode) -> Self;

    // Configuration
    pub fn with_surface_form(self) -> Self;
    pub fn with_normalized_form(self, enabled: bool) -> Self;
    pub fn uses_normalized_form(&self) -> bool;
    pub fn mode(&self) -> SplitMode;
}

impl Tokenizer for SudachiTokenizer {
    type TokenStream<'a> = SudachiTokenStream<'a>;
    fn token_stream<'a>(&'a mut self, text: &'a str) -> SudachiTokenStream<'a>;
}
```

---

## Architecture

```
SudachiTokenizer
  ├── TokenizerInner::Standard(StatelessTokenizer)  ← Modes A, B, C
  └── TokenizerInner::Search(SearchTokenizer)       ← Search mode (B+C)
          ↓
  SudachiTokenStream
    ├── advances through Vec<TokenData>
    └── is_colocated: true → position_length=1, position NOT incremented
```

---

## Usage in ParadeDB

This crate is used by `jurmarcus/paradedb` as an optional feature:

```toml
# In paradedb's tokenizers/Cargo.toml
[features]
sudachi = ["dep:sudachi-tantivy"]

[dependencies]
sudachi-tantivy = { git = "https://github.com/jurmarcus/sudachi", optional = true }
```

The paradedb fork's `[patch.crates-io]` redirects `tantivy-tokenizer-api` to
paradedb's forked tantivy, ensuring type compatibility across the crate boundary.

---

## Related

| Project | Description |
|---------|-------------|
| [sudachi-search](../sudachi-search/) | B+C core this crate adapts |
| [jurmarcus/paradedb](https://github.com/jurmarcus/paradedb) | ParadeDB fork using this crate |
| [sudachi.rs](https://github.com/WorksApplications/sudachi.rs) | Upstream morphological analyzer |

---

## License

Apache-2.0
