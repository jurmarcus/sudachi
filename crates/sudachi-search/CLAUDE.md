# CLAUDE.md — sudachi-search

B+C multi-granularity Japanese tokenization core. Engine-agnostic. Single-file library; everything is in `src/lib.rs`.

## What this is

Produces a `Vec<SearchToken>` where compound words and their sub-tokens share the same position via the `is_colocated: bool` flag. Adapter crates (`sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm`) translate that flag into their engine's position semantics.

## The problem

```text
Document: 東京都立大学で研究
Query:    大学

Single-mode tokenizer (Mode C):  ["東京都立大学", "で", "研究"]   → no match (大学 trapped)

This crate (Search mode):
  pos 0  東京都立大学    is_colocated: false
  pos 0  東京            is_colocated: true   ← same position
  pos 0  都立            is_colocated: true   ← same position
  pos 0  大学            is_colocated: true   → match
  pos 1  で
  pos 2  研究
```

## Architecture

```
                        SearchTokenizer::tokenize(input)
                                    │
                ┌───────────────────┴───────────────────┐
                │                                       │
   sudachi_optimizer::sudachi::StatelessTokenizer       │
   (the single Sudachi gateway — never use upstream     │
   `sudachi` directly)                                  │
                │                                       │
                ▼                                       ▼
   tokenize(input, Mode::C)                tokenize(input, Mode::B)
        ["東京都立大学"]                  ["東京", "都立", "大学"]
                │                                       │
                └───────────────────┬───────────────────┘
                                    │
                                    ▼
                        For each Mode C morpheme:
                          emit { is_colocated: false }
                          for each Mode B inside C's byte span
                          where b.text ≠ c.text:
                              emit { is_colocated: true }
                                    │
                                    ▼
                            Vec<SearchToken>
                                    │
              ┌─────────────────────┼─────────────────────┐
              ▼                     ▼                     ▼
        sudachi-sqlite        sudachi-tantivy       sudachi-wasm
        FTS5_TOKEN_COLOCATED  position increment=0  isColocated field
```

## Imports must go through `sudachi-optimizer`

```rust
// CORRECT
use sudachi_optimizer::sudachi::{JapaneseDictionary, Mode, StatelessTokenizer, Tokenize};

// WRONG — only sudachi-optimizer is allowed to do this
use sudachi::dic::dictionary::JapaneseDictionary;
```

The workspace `Cargo.toml` enforces this by convention. If you need a Sudachi type that isn't yet re-exported, add it to `crates/sudachi-optimizer/src/sudachi.rs` first.

## Key types

```rust
pub struct SearchToken {
    pub surface: String,       // normalised or raw text
    pub byte_start: usize,     // offset in input
    pub byte_end: usize,
    pub is_colocated: bool,    // true → same position as previous
}

pub struct CompoundWord {
    pub surface: String,
    pub components: Vec<String>,  // Mode B parts of this compound
    pub byte_start: usize,
    pub byte_end: usize,
}

pub enum SearchError { /* wraps sudachi::SudachiError */ }
```

## API

```rust
impl SearchTokenizer {
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self;
    pub fn from_tokenizer(t: StatelessTokenizer<Arc<JapaneseDictionary>>) -> Self;

    pub fn with_surface_form(self) -> Self;
    pub fn with_normalized_form(self, enabled: bool) -> Self;
    pub fn uses_normalized_form(&self) -> bool;

    pub fn tokenize(&self, input: &str) -> Result<Vec<SearchToken>, SearchError>;
    pub fn tokenize_with_normalization(&self, input: &str, normalize: bool)
        -> Result<Vec<SearchToken>, SearchError>;

    pub fn detect_compounds(&self, input: &str)
        -> Result<Vec<CompoundWord>, SearchError>;
    pub fn tokenize_with_compounds(&self, input: &str)
        -> Result<(Vec<SearchToken>, Vec<CompoundWord>), SearchError>;

    pub fn inner(&self) -> &StatelessTokenizer<Arc<JapaneseDictionary>>;
}

pub fn extract_compounds(tokens: &[SearchToken]) -> Vec<CompoundWord>;
```

## Critical implementation rules

### Emission order

1. Mode C morpheme → `is_colocated: false` (advances position).
2. Mode B morphemes inside the C span → `is_colocated: true` (same position) — but only if their text differs from the C morpheme.
3. Move on to the next Mode C morpheme.

```rust
if b_text != &c_text {
    result.push(SearchToken { is_colocated: true, .. });
}
```

Do not emit a colocated token whose text equals the compound — it would duplicate the index entry.

### Normalisation default

`with_normalized_form(true)` is the default. This is what users want for search recall: 食べた → 食べる, 美しかった → 美しい, 附属 → 付属, ＳＵＭＭＥＲ → サマー. Surface form (`with_surface_form()`) is for callers that need to highlight or echo the original text.

### Byte offsets, not char offsets

`byte_start` / `byte_end` are byte offsets into the original input string. Do not convert to char counts — Sudachi's morpheme API is byte-indexed and the FTS5/Tantivy callbacks need byte offsets too.

## Dependencies

```toml
[dependencies]
sudachi-optimizer.workspace = true
```

That's it. The optimiser crate transitively re-exports everything Sudachi-related the search crate needs.

## File structure

```
src/
└── lib.rs    # Everything: SearchTokenizer, SearchToken, CompoundWord, extract_compounds, SearchError
```

Single-file library, ~750 LOC.

## Testing

```bash
cargo test -p sudachi-search                          # unit tests, no dictionary
cargo test -p sudachi-search -- --include-ignored     # integration tests, requires SUDACHI_DICT_PATH
```

Or from the workspace root:

```bash
just test         # workspace unit tests
just test-verbose # with stdout
```

### Key test cases to keep passing

| Input                                | Expected                                                     |
| ------------------------------------ | ------------------------------------------------------------ |
| `"東京都立大学"`                     | compound + 3 sub-tokens, all at position 0                   |
| `"今日は天気がいい"`                 | no colocated tokens (no compound needs splitting)            |
| `"東京都立大学で国会議事堂"`         | two compounds, each with its own sub-tokens                  |
| `"食べた"` (normalised)              | `["食べる"]` — surface morphemes folded to dictionary form    |
| `""`                                 | empty `Vec`                                                  |

## Debugging

```rust
let tokens = tokenizer.tokenize("東京都立大学")?;
for (i, t) in tokens.iter().enumerate() {
    println!("{i}: {:12} colocated={} bytes={}..{}",
             t.surface, t.is_colocated, t.byte_start, t.byte_end);
}
```

Common symptoms:

| Symptom                        | Likely cause                                  | Fix                                       |
| ------------------------------ | --------------------------------------------- | ----------------------------------------- |
| No sub-tokens emitted          | Dictionary missing entries for the compound   | Verify dictionary path; reinstall full dict |
| Wrong byte offsets             | Char vs byte confusion in caller              | Use the offsets verbatim from `SearchToken` |
| Surface form when expecting normalised | `with_surface_form()` left set         | Construct without it, or call `with_normalized_form(true)` |
| Same compound emits twice      | Logic error: skipped the `b_text != c_text` check | Restore the inequality guard            |

## Performance

- Dictionary: ~70MB, share via `Arc<JapaneseDictionary>`.
- Each call does two Sudachi tokenisations (Mode C + Mode B). Cost ≈ 2× single-mode.
- Allocations per call: 2× `Vec<Morpheme>` (from Sudachi) + 1× `Vec<SearchToken>` (returned). Reuse the tokenizer instance.
- `extract_compounds` is a pure pass over the existing token vector — cheap.
