# sudachi-search

**B+C multi-granularity Japanese tokenization for search engines.**

The engine-agnostic core of the workspace. Emits a stream of `SearchToken { surface, byte_start, byte_end, is_colocated }`. Every search-engine adapter — `sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm` — translates `is_colocated` into its engine's "same position" mechanism.

---

## Three problems, one crate

### Compound words trap sub-words

```text
Document: 東京都立大学で研究
Query:    大学

Single-mode tokenizer (Mode C): ["東京都立大学", "で", "研究"]   → no match

This crate (Search mode):
  pos 0  東京都立大学   primary
  pos 0  東京           colocated
  pos 0  都立           colocated
  pos 0  大学           colocated   → match
  pos 1  で
  pos 2  研究
```

### Conjugations / inflections fragment the index

| Surface       | Normalised   | Type                       |
| ------------- | ------------ | -------------------------- |
| 食べた        | 食べる       | Verb (past)                |
| 食べている    | 食べる       | Verb (progressive)         |
| 美しかった    | 美しい       | i-adjective past           |

Both indexed text and queries get normalised, so a search in any conjugation matches all related documents.

### Character variants

| Surface       | Normalised   | Reason                    |
| ------------- | ------------ | ------------------------- |
| 附属病院      | 付属病院     | Variant kanji             |
| ＳＵＭＭＥＲ   | サマー       | Fullwidth ASCII           |
| パーティー    | パーティ     | Long-vowel mark           |

---

## Install

```toml
[dependencies]
sudachi-search = { git = "https://github.com/jurmarcus/sudachi" }
```

Inside this workspace, depend on the path member:

```toml
sudachi-search.workspace = true
```

### Dictionary

```bash
just dict-setup       # downloads to ~/.sudachi/system_full.dic
```

Or set `SUDACHI_DICT_PATH=/abs/path/to/system_full.dic`.

---

## Quick start

```rust
use std::sync::Arc;
use sudachi_search::{SearchTokenizer, SearchToken};
use sudachi_optimizer::sudachi::{
    Config, JapaneseDictionary, Storage, SudachiDicData,
};

let dict_path = std::env::var("SUDACHI_DICT_PATH")?;
let dict_bytes = std::fs::read(&dict_path)?;
let storage = SudachiDicData::new(Storage::Owned(dict_bytes));
let config = Config::minimal_at(std::path::Path::new(&dict_path).parent().unwrap());
let dictionary = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage)?;

let tokenizer = SearchTokenizer::new(Arc::new(dictionary));

for tok in tokenizer.tokenize("東京都立大学で研究")? {
    println!("{:12} colocated={}", tok.surface, tok.is_colocated);
}
// 東京都立大学   colocated=false
// 東京           colocated=true
// 都立           colocated=true
// 大学           colocated=true
// で             colocated=false
// 研究           colocated=false
```

> All Sudachi types come from `sudachi_optimizer::sudachi::*` — the workspace's single Sudachi gateway. Don't depend on the upstream `sudachi` crate directly.

---

## Split modes

| Mode      | Granularity | "東京都立大学" output                                  |
| --------- | ----------- | ------------------------------------------------------ |
| A         | Finest      | `["東京", "都", "立", "大学"]`                         |
| B         | Medium      | `["東京", "都立", "大学"]`                             |
| C         | Coarsest    | `["東京都立大学"]`                                     |
| **Search**| **B+C**     | `["東京都立大学", "東京"*, "都立"*, "大学"*]` *= colocated |

This crate exposes Search mode. Use `sudachi-tantivy::SudachiTokenizer` if you need plain A/B/C streams.

---

## How `is_colocated` is translated downstream

| Engine      | Translation                                |
| ----------- | ------------------------------------------ |
| SQLite FTS5 | `FTS5_TOKEN_COLOCATED` flag (0x0001)       |
| Tantivy     | Position increment = 0                     |
| Lucene/ES   | `PositionIncrementAttribute = 0`           |

This crate only produces the abstract flag. Downstream adapters do the engine-specific work.

---

## Normalization

Default. Surface form is opt-in:

```rust
// Normalised (default — better recall)
let tokenizer = SearchTokenizer::new(dictionary);

// Surface form (original text)
let tokenizer = SearchTokenizer::new(dictionary).with_surface_form();

// Or toggle explicitly
let tokenizer = SearchTokenizer::new(dictionary).with_normalized_form(true);
```

---

## Compound detection (separate from search)

Sometimes you want to *analyse* compound structure, not just produce search tokens:

```rust
use sudachi_search::extract_compounds;

let tokens = tokenizer.tokenize("予約困難店を探す")?;
let compounds = extract_compounds(&tokens);

for c in &compounds {
    println!("{} = {:?}", c.surface, c.components);
}
// 予約困難店 = ["予約", "困難", "店"]
```

Or fused with tokenisation:

```rust
let (tokens, compounds) = tokenizer.tokenize_with_compounds(input)?;
```

---

## API reference

### Types

```rust
pub struct SearchToken {
    pub surface: String,    // normalised or surface text
    pub byte_start: usize,  // byte offset in original input
    pub byte_end: usize,
    pub is_colocated: bool, // true → same position as previous token
}

pub struct CompoundWord {
    pub surface: String,
    pub components: Vec<String>,  // Mode B parts
    pub byte_start: usize,
    pub byte_end: usize,
}

impl CompoundWord {
    pub fn is_compound(&self) -> bool;       // components.len() > 1
    pub fn component_count(&self) -> usize;
}
```

### Tokenizer

```rust
impl SearchTokenizer {
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self;
    pub fn from_optimizer(optimizer: Arc<sudachi_optimizer::Optimizer>) -> Self;

    pub fn with_surface_form(self) -> Self;
    pub fn with_normalized_form(self, enabled: bool) -> Self;
    pub fn uses_normalized_form(&self) -> bool;

    pub fn tokenize(&self, input: &str)
        -> Result<Vec<SearchToken>, SearchError>;

    pub fn tokenize_with_normalization(&self, input: &str, normalize: bool)
        -> Result<Vec<SearchToken>, SearchError>;

    pub fn detect_compounds(&self, input: &str)
        -> Result<Vec<CompoundWord>, SearchError>;

    pub fn tokenize_with_compounds(&self, input: &str)
        -> Result<(Vec<SearchToken>, Vec<CompoundWord>), SearchError>;

    pub fn optimizer(&self) -> &Arc<sudachi_optimizer::Optimizer>;
}

pub fn extract_compounds(tokens: &[SearchToken]) -> Vec<CompoundWord>;
```

---

## Architecture

```
SearchTokenizer::tokenize("東京都立大学")
  │
  └─► sudachi_optimizer::Optimizer::tokenize_raw_multi_mode(
          input, &[Mode::C, Mode::B]
      )
      // Single shared lattice build, ~1.7× faster than two
      // sequential `tokenize` calls.
        │
        ├─► morphemes_c (Mode::C) → ["東京都立大学"]
        ├─► morphemes_b (Mode::B) → ["東京", "都立", "大学"]
        │
        └─► For each Mode C morpheme:
              emit SearchToken { is_colocated: false }
              for Mode B morpheme inside its byte span where text ≠ C:
                  emit SearchToken { is_colocated: true }
```

All access goes through `sudachi_optimizer::Optimizer` (constructed with an empty `Pipeline` since sudachi-search does its own post-processing) — i.e. the single Sudachi gateway. Dictionaries are shared via `Arc` so a single dict can back many tokenizers.

---

## Performance

- Dictionary: ~70MB, share via `Arc<JapaneseDictionary>`
- Tokenisation cost: ~2× single-mode (two Sudachi passes)
- Allocations per call: 2× `Vec<Morpheme>` from Sudachi + 1× `Vec<SearchToken>` result

Reuse a single `SearchTokenizer` instance across calls.

---

## License

Apache-2.0
