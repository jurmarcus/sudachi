# CLAUDE.md - sudachi-search

B+C multi-granularity Japanese tokenization for search engines.

## What This Is

The core library that solves Japanese compound word search. Search-engine agnostic - provides `SearchToken` with `is_colocated` flag that adapter crates translate for each engine.

## The Problem Being Solved

Japanese compound words hide sub-words from search:

```
Document: "東京都立大学で研究"
Query: "大学"

Traditional (Mode C): ["東京都立大学", "で", "研究"]
→ "大学" NOT FOUND - trapped inside compound

B+C (this crate):
  pos 0: "東京都立大学" (is_colocated: false)
  pos 0: "東京"         (is_colocated: true)  ← SAME POSITION
  pos 0: "都立"         (is_colocated: true)  ← SAME POSITION
  pos 0: "大学"         (is_colocated: true)  ← NOW SEARCHABLE!
  pos 1: "で"
  pos 2: "研究"
```

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
│  Output: Vec<SearchToken>                                                   │
│      ├── { surface: "東京都立大学", is_colocated: false }                   │
│      ├── { surface: "東京", is_colocated: true }                            │
│      ├── { surface: "都立", is_colocated: true }                            │
│      └── { surface: "大学", is_colocated: true }                            │
│                                                                              │
└───────────────────────────────────────────────────────────────────────────────┘
                │                     │                     │
                ▼                     ▼                     ▼
┌───────────────────────┐ ┌───────────────────────┐ ┌───────────────────────┐
│   sudachi-tantivy     │ │   sudachi-sqlite      │ │   sudachi-postgres    │
│                       │ │                       │ │                       │
│ is_colocated →        │ │ is_colocated →        │ │ Uses sudachi-tantivy  │
│ position_inc=0        │ │ FTS5_TOKEN_COLOCATED  │ │ for ParadeDB          │
└───────────────────────┘ └───────────────────────┘ └───────────────────────┘
```

## Key Data Structures

### SearchToken

```rust
pub struct SearchToken {
    pub surface: String,      // Token text (normalized or surface)
    pub byte_start: usize,    // Byte offset start
    pub byte_end: usize,      // Byte offset end
    pub is_colocated: bool,   // Same position as previous?
}
```

### CompoundWord

```rust
pub struct CompoundWord {
    pub surface: String,           // Full compound
    pub components: Vec<String>,   // Mode B parts
    pub byte_start: usize,
    pub byte_end: usize,
}

impl CompoundWord {
    pub fn is_compound(&self) -> bool;      // components.len() > 1
    pub fn component_count(&self) -> usize; // Number of parts
}
```

## API

```rust
impl SearchTokenizer {
    // Construction
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self;
    pub fn from_tokenizer(tokenizer: StatelessTokenizer<...>) -> Self;

    // Configuration
    pub fn with_surface_form(self) -> Self;           // Disable normalization
    pub fn with_normalized_form(self, bool) -> Self;  // Configure normalization
    pub fn uses_normalized_form(&self) -> bool;       // Check current setting

    // Core tokenization
    pub fn tokenize(&self, input: &str) -> Result<Vec<SearchToken>, SearchError>;
    pub fn tokenize_with_normalization(&self, input: &str, normalize: bool)
        -> Result<Vec<SearchToken>, SearchError>;

    // Compound detection
    pub fn detect_compounds(&self, input: &str) -> Result<Vec<CompoundWord>, SearchError>;
    pub fn tokenize_with_compounds(&self, input: &str)
        -> Result<(Vec<SearchToken>, Vec<CompoundWord>), SearchError>;

    // Internal access
    pub fn inner(&self) -> &StatelessTokenizer<Arc<JapaneseDictionary>>;
}

// Extract compounds without re-tokenizing
pub fn extract_compounds(tokens: &[SearchToken]) -> Vec<CompoundWord>;
```

## Critical Implementation Rules

### Colocated Token Logic

```rust
// 1. Mode C token → is_colocated: false (NEW position)
// 2. Mode B sub-tokens within C span → is_colocated: true (SAME position)
// 3. Only emit B tokens that DIFFER from C text

// CORRECT:
if b_text != &text_c {
    result.push(SearchToken { is_colocated: true, .. });
}

// WRONG: emitting duplicate
result.push(SearchToken { is_colocated: true, .. }); // Even if same text!
```

### Normalization

Default is normalized (better recall):

| Surface | Normalized | Type |
|---------|------------|------|
| 食べた | 食べる | Conjugation |
| 美しかった | 美しい | Inflection |
| 附属 | 付属 | Variant kanji |
| ＳＵＭＭＥＲ | サマー | Fullwidth |

### Byte Offsets

Sudachi returns byte offsets directly:

```rust
// NO conversion needed - already bytes
pub byte_start: usize,  // m.begin()
pub byte_end: usize,    // m.end()
```

## Commands

```bash
# Setup dictionary (one-time)
just dict-setup

# Build
just build

# Test (unit tests, no dictionary)
just test

# Test (all tests, requires dictionary)
just test-all

# Format and lint
just fix

# Show environment
just env
```

All commands use `just` (task runner). Run `just --list` to see all available commands.

## File Structure

```
src/
└── lib.rs    # Everything: SearchTokenizer, SearchToken, CompoundWord, extract_compounds()
```

Single-file library, ~750 LOC.

## Dependencies

```toml
[dependencies]
sudachi = { git = "https://github.com/WorksApplications/sudachi.rs", branch = "develop" }
```

## Testing

### Unit Tests (No Dictionary)

```bash
cargo test
```

Tests `SearchToken` equality, `CompoundWord` methods, `extract_compounds()`.

### Integration Tests (Requires Dictionary)

```bash
SUDACHI_DICT_PATH=/path/to/system.dic cargo test -- --ignored
```

Tests actual tokenization.

## Downstream Adapters

| Crate | Translation |
|-------|-------------|
| sudachi-tantivy | `is_colocated` → position stays same |
| sudachi-sqlite | `is_colocated` → `FTS5_TOKEN_COLOCATED` |
| sudachi-postgres | Via sudachi-tantivy |

## Why This Exists Separately

- **Sudachi's modes** = linguistic granularity (how to analyze)
- **B+C** = search output format (emitting multiple granularities)

Orthogonal concerns. This crate bridges them for search use cases.
