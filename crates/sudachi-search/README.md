# sudachi-search

**Sudachi B+C multi-granularity tokenization for search engines.**

The core library that enables Japanese compound word search. Search-engine agnostic - provides `SearchToken` with `is_colocated` flag that adapter crates translate for each platform.

---

## Why Sudachi?

Three problems break Japanese full-text search. Sudachi solves all of them:

### 1. Compound Words

```
Document: "東京都立大学で研究しています"
Query: "大学"

Traditional tokenizer: ❌ NO MATCH
  → "大学" is trapped inside "東京都立大学"

Sudachi (B+C mode): ✅ MATCH
  → Indexes both "東京都立大学" AND "大学" at same position
```

### 2. Verb Conjugations & Adjective Inflections

Both documents AND queries are normalized to the dictionary form:

```
Documents contain:           Normalized to:
  "食べた" (ate)        →    食べる
  "食べている" (eating)  →    食べる
  "食べます" (will eat)  →    食べる

Query "食べる" → matches all 3 documents  ✅
Query "食べた" → ALSO matches all 3!      ✅  (query is normalized too)
```

This means users can search in **any conjugated form** and find all related documents.

### 3. Character Normalization

```
Documents with variant forms:
  - "ＳＵＭＭＥＲ"  (fullwidth)
  - "附属病院"     (variant kanji 附)
  - "パーティー"   (long vowel mark)

Sudachi normalizes:
  - ＳＵＭＭＥＲ → サマー
  - 附属 → 付属
  - パーティー → パーティ
```

### The Solution: B+C Multi-Granularity

Sudachi's Search mode indexes text at **multiple granularities simultaneously**:

```
Input: "東京都立大学"

Indexed tokens (all at position 0):
  "東京都立大学"  (compound word)
  "東京"          (sub-token)
  "都立"          (sub-token)
  "大学"          (sub-token)
```

Now searches for "大学", "東京", or "東京都立大学" all match.

---

## Features

| Feature | Description |
|---------|-------------|
| **B+C Multi-Granularity** | Emits compound words AND sub-tokens at same position |
| **`is_colocated` Flag** | Adapters translate this to platform-specific position handling |
| **Normalized Forms** | 食べた → 食べる (verb conjugation) |
| **Compound Detection** | Analyze compound word structure separately |
| **High-Quality Dictionary** | 1M+ entries from Sudachi |

---

## Installation

```toml
[dependencies]
sudachi-search = { git = "https://github.com/jurmarcus/sudachi" }
```

### Dictionary Setup

```bash
# Using just (recommended)
just dict-setup

# Or manually
mkdir -p ~/.sudachi
curl -L https://github.com/WorksApplications/SudachiDict/releases/download/v20251022/sudachi-dictionary-20251022-full.zip -o /tmp/sudachi-dict.zip
unzip /tmp/sudachi-dict.zip -d ~/.sudachi/
```

The dictionary is auto-discovered from `~/.sudachi/` - no environment variable needed.

---

## Quick Start

```rust
use sudachi_search::{SearchTokenizer, SearchToken};
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load dictionary
    let dict_path = std::env::var("SUDACHI_DICT_PATH")?;
    let dict_bytes = std::fs::read(&dict_path)?;
    let storage = SudachiDicData::new(Storage::Owned(dict_bytes));
    let config = Config::minimal_at(std::path::Path::new(&dict_path).parent().unwrap());
    let dictionary = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage)?;

    // Create search tokenizer
    let tokenizer = SearchTokenizer::new(Arc::new(dictionary));

    // Tokenize with B+C multi-granularity
    let tokens = tokenizer.tokenize("東京都立大学で研究")?;

    for token in &tokens {
        println!("{:12} colocated={}", token.surface, token.is_colocated);
    }

    // Output:
    // 東京都立大学 colocated=false  ← Position 0
    // 東京         colocated=true   ← Position 0 (same!)
    // 都立         colocated=true   ← Position 0 (same!)
    // 大学         colocated=true   ← Position 0 (same!)
    // で           colocated=false  ← Position 1
    // 研究         colocated=false  ← Position 2

    Ok(())
}
```

---

## Split Modes

Sudachi provides three base modes. This crate adds **Search mode** (B+C):

| Mode | Description | "東京都立大学" Output |
|------|-------------|----------------------|
| A | Finest granularity | ["東京", "都", "立", "大学"] |
| B | Medium granularity | ["東京", "都立", "大学"] |
| C | Coarsest granularity | ["東京都立大学"] |
| **Search** | **B+C (Recommended)** | ["東京都立大学", "東京"\*, "都立"\*, "大学"\*] |

\* Colocated tokens (`is_colocated: true`) - same position as compound

---

## Search Mode

Search mode emits **both** compound words AND sub-tokens at the same position:

```rust
let tokens = tokenizer.tokenize("東京都立大学")?;

// tokens:
// SearchToken { surface: "東京都立大学", is_colocated: false }  ← Position 0
// SearchToken { surface: "東京",         is_colocated: true }   ← Position 0
// SearchToken { surface: "都立",         is_colocated: true }   ← Position 0
// SearchToken { surface: "大学",         is_colocated: true }   ← Position 0
```

**How adapters use `is_colocated`:**

| Engine | Translation |
|--------|-------------|
| Tantivy | Position increment = 0 |
| SQLite FTS5 | `FTS5_TOKEN_COLOCATED` flag |
| PostgreSQL | Via Tantivy (ParadeDB) |
| Lucene/ES | `PositionIncrementAttribute = 0` |

---

## Normalization

Normalized form (default) improves recall by matching conjugated forms:

| Surface | Normalized | Type |
|---------|------------|------|
| 食べた | 食べる | Verb conjugation |
| 美しかった | 美しい | Adjective inflection |
| 附属 | 付属 | Variant kanji |
| ＳＵＭＭＥＲ | サマー | Fullwidth conversion |

```rust
// Default: normalized form (better recall)
let tokenizer = SearchTokenizer::new(dictionary);

// Surface form (exact text)
let tokenizer = SearchTokenizer::new(dictionary).with_surface_form();
```

---

## Complete Example

```rust
use sudachi_search::{SearchTokenizer, extract_compounds};
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load dictionary
    let dict_path = std::env::var("SUDACHI_DICT_PATH")?;
    let dict_bytes = std::fs::read(&dict_path)?;
    let storage = SudachiDicData::new(Storage::Owned(dict_bytes));
    let config = Config::minimal_at(std::path::Path::new(&dict_path).parent().unwrap());
    let dictionary = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage)?;

    let tokenizer = SearchTokenizer::new(Arc::new(dictionary));

    // Tokenize
    let tokens = tokenizer.tokenize("予約困難店を探す")?;

    println!("Tokens:");
    for token in &tokens {
        let marker = if token.is_colocated { " (colocated)" } else { "" };
        println!("  {}{}", token.surface, marker);
    }

    // Extract compound words
    let compounds = extract_compounds(&tokens);
    println!("\nCompounds:");
    for c in &compounds {
        println!("  {} = {:?}", c.surface, c.components);
    }

    // Output:
    // Tokens:
    //   予約困難店
    //   予約 (colocated)
    //   困難 (colocated)
    //   店 (colocated)
    //   を
    //   探す
    //
    // Compounds:
    //   予約困難店 = ["予約", "困難", "店"]

    Ok(())
}
```

---

## API Reference

### SearchToken

```rust
pub struct SearchToken {
    pub surface: String,      // Token text (normalized or surface)
    pub byte_start: usize,    // Byte offset start in original text
    pub byte_end: usize,      // Byte offset end in original text
    pub is_colocated: bool,   // True = same position as previous token
}
```

### CompoundWord

```rust
pub struct CompoundWord {
    pub surface: String,           // Full compound text
    pub components: Vec<String>,   // Mode B sub-tokens
    pub byte_start: usize,
    pub byte_end: usize,
}

impl CompoundWord {
    pub fn is_compound(&self) -> bool;      // components.len() > 1
    pub fn component_count(&self) -> usize;
}
```

### SearchTokenizer

```rust
impl SearchTokenizer {
    // Construction
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self;
    pub fn from_tokenizer(tokenizer: StatelessTokenizer<...>) -> Self;

    // Configuration
    pub fn with_surface_form(self) -> Self;
    pub fn with_normalized_form(self, enabled: bool) -> Self;
    pub fn uses_normalized_form(&self) -> bool;

    // Tokenization
    pub fn tokenize(&self, input: &str) -> Result<Vec<SearchToken>, SearchError>;
    pub fn tokenize_with_normalization(&self, input: &str, normalize: bool)
        -> Result<Vec<SearchToken>, SearchError>;

    // Compound detection
    pub fn detect_compounds(&self, input: &str) -> Result<Vec<CompoundWord>, SearchError>;
    pub fn tokenize_with_compounds(&self, input: &str)
        -> Result<(Vec<SearchToken>, Vec<CompoundWord>), SearchError>;
}

// Extract compounds from existing tokens
pub fn extract_compounds(tokens: &[SearchToken]) -> Vec<CompoundWord>;
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         sudachi-search                                       │
│                    (Search Engine Agnostic)                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  SearchTokenizer::tokenize("東京都立大学")                                   │
│                                                                              │
│  1. tokenize(Mode::C) → ["東京都立大学"]                                    │
│  2. tokenize(Mode::B) → ["東京", "都立", "大学"]                            │
│  3. Emit C token (is_colocated: false)                                      │
│  4. Emit B tokens within C span (is_colocated: true)                        │
│                                                                              │
└───────────────────────────────────────────────────────────────────────────────┘
                │                     │                     │
                ▼                     ▼                     ▼
┌───────────────────────┐ ┌───────────────────────┐ ┌───────────────────────┐
│   sudachi-tantivy     │ │   sudachi-sqlite      │ │   sudachi-postgres    │
│                       │ │                       │ │                       │
│ is_colocated →        │ │ is_colocated →        │ │ Via sudachi-tantivy   │
│ position increment=0  │ │ FTS5_TOKEN_COLOCATED  │ │ for ParadeDB          │
└───────────────────────┘ └───────────────────────┘ └───────────────────────┘
```

---

## Integrations

| Platform | Adapter | Repository |
|----------|---------|------------|
| **Tantivy** | sudachi-tantivy | [`crates/sudachi-tantivy`](../sudachi-tantivy/) in this repo |
| **SQLite FTS5** | sudachi-sqlite | [`crates/sudachi-sqlite`](../sudachi-sqlite/) in this repo |
| **PostgreSQL** | sudachi-postgres | [jurmarcus/paradedb](https://github.com/jurmarcus/paradedb) |
| **WebAssembly** | sudachi-wasm | [`crates/sudachi-wasm`](../sudachi-wasm/) in this repo |

---

## Related Projects

| Project | Description |
|---------|-------------|
| [sudachi.rs](https://github.com/WorksApplications/sudachi.rs) | Upstream morphological analyzer |
| [SudachiDict](https://github.com/WorksApplications/SudachiDict) | Dictionary releases |

---

## License

MIT
