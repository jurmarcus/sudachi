# CLAUDE.md - sudachi-search

B+C multi-granularity Japanese tokenization for search engines.

## What This Is

The core library that solves Japanese compound word search. Search-engine agnostic — provides `SearchToken` with `is_colocated` flag that adapter crates translate for each engine.

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
└─────────────────────────────────────────────────────────────────────────────┘
                │                     │
                ▼                     ▼
┌───────────────────────┐ ┌───────────────────────┐
│   sudachi-sqlite      │ │   sudachi-postgres    │
│                       │ │                       │
│ is_colocated →        │ │ (pgrx, own workspace) │
│ FTS5_TOKEN_COLOCATED  │ │                       │
└───────────────────────┘ └───────────────────────┘
```

Note: `sudachi-tantivy` is a workspace member; paradedb pulls it as a git dep.

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
```

## API

```rust
impl SearchTokenizer {
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self;
    pub fn with_surface_form(self) -> Self;
    pub fn tokenize(&self, input: &str) -> Result<Vec<SearchToken>, SearchError>;
    pub fn detect_compounds(&self, input: &str) -> Result<Vec<CompoundWord>, SearchError>;
}
```

## Critical Implementation Rules

### Colocated Token Logic

```rust
// 1. Mode C token → is_colocated: false (NEW position)
// 2. Mode B sub-tokens within C span → is_colocated: true (SAME position)
// 3. Only emit B tokens that DIFFER from C text (no duplicates)

if b_text != &text_c {
    result.push(SearchToken { is_colocated: true, .. });
}
```

### Normalization

Default is normalized (better recall):

| Surface | Normalized | Type |
|---------|------------|------|
| 食べた | 食べる | Conjugation |
| 美しかった | 美しい | Inflection |
| 附属 | 付属 | Variant kanji |

## Dependencies

```toml
[dependencies]
sudachi.workspace = true
```

The upstream `sudachi` dep is pinned to a specific rev in the root workspace:

```toml
[workspace.dependencies]
sudachi = { git = "https://github.com/WorksApplications/sudachi.rs", rev = "b568decbbd4ea209dec847821dc1f5069a0f2ff5" }
```

## Commands

```bash
just build        # Build (from repo root)
just test         # Unit tests — no dictionary required
just fix          # Format and lint
just dict-setup   # Download dictionary
```

Run all commands from the repo root (workspace `just`).

## File Structure

```
src/
└── lib.rs    # Everything: SearchTokenizer, SearchToken, CompoundWord, extract_compounds()
```

Single-file library, ~750 LOC.

## Testing

### Unit Tests (No Dictionary)

```bash
cargo test -p sudachi-search
```

Tests `SearchToken` equality, `CompoundWord` methods, `extract_compounds()`.

### Integration Tests (Requires Dictionary)

```bash
SUDACHI_DICT_PATH=~/.sudachi/system_full.dic cargo test -p sudachi-search -- --include-ignored
```
