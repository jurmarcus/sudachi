# AGENTS.md - sudachi-search

Context for AI agents working on this codebase.

## Project Summary

**sudachi-search** is the core B+C multi-granularity tokenization library.

| Attribute | Value |
|-----------|-------|
| Complexity | Low (~750 LOC, single file) |
| Pattern | Tokenization strategy |
| Key Feature | B+C multi-granularity |
| Output | `Vec<SearchToken>` with `is_colocated` flag |

## The Problem

Japanese compound words are invisible to traditional search:

```
Document: "東京都立大学で研究"
Query: "大学"

Mode C: ["東京都立大学", "で", "研究"]
→ "大学" not found (trapped inside compound)

B+C (this crate):
  pos 0: "東京都立大学" (is_colocated: false)
  pos 0: "東京"         (is_colocated: true)
  pos 0: "都立"         (is_colocated: true)
  pos 0: "大学"         (is_colocated: true) ← NOW SEARCHABLE
  pos 1: "で"
  pos 2: "研究"
```

## File Structure

```
sudachi-search/
├── src/
│   └── lib.rs    # Everything (~750 LOC)
├── Cargo.toml
├── README.md
├── CLAUDE.md
└── AGENTS.md
```

## Core Data Structures

### SearchToken

```rust
pub struct SearchToken {
    pub surface: String,      // Token text
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

## Algorithm (tokenize_internal)

```rust
fn tokenize_internal(&self, input: &str, use_normalized: bool) -> Result<Vec<SearchToken>> {
    // 1. Tokenize with Mode C (compounds)
    let morphemes_c = self.inner.tokenize(input, Mode::C, false)?;

    // 2. Tokenize with Mode B (sub-tokens)
    let morphemes_b = self.inner.tokenize(input, Mode::B, false)?;

    // 3. Build lookup of Mode B tokens by byte position
    let mode_b_tokens: Vec<(byte_start, byte_end, text)> = ...;

    let mut result = Vec::new();

    // 4. For each Mode C token:
    for morpheme_c in morphemes_c {
        // 4a. Emit compound (NOT colocated)
        result.push(SearchToken {
            surface: morpheme_c.text(),
            is_colocated: false,
            ..
        });

        // 4b. Find Mode B tokens within this span
        for (b_start, b_end, b_text) in mode_b_tokens.within(morpheme_c.span()) {
            // Only emit if different from Mode C
            if b_text != morpheme_c.text() {
                result.push(SearchToken {
                    surface: b_text,
                    is_colocated: true,  // ← SAME POSITION
                    ..
                });
            }
        }
    }

    Ok(result)
}
```

## API Quick Reference

```rust
impl SearchTokenizer {
    pub fn new(dictionary: Arc<JapaneseDictionary>) -> Self;
    pub fn from_tokenizer(tokenizer: StatelessTokenizer<...>) -> Self;
    pub fn with_surface_form(self) -> Self;
    pub fn with_normalized_form(self, enabled: bool) -> Self;
    pub fn uses_normalized_form(&self) -> bool;
    pub fn tokenize(&self, input: &str) -> Result<Vec<SearchToken>, SearchError>;
    pub fn tokenize_with_normalization(&self, input: &str, normalize: bool) -> Result<...>;
    pub fn detect_compounds(&self, input: &str) -> Result<Vec<CompoundWord>, SearchError>;
    pub fn tokenize_with_compounds(&self, input: &str) -> Result<(Vec<SearchToken>, Vec<CompoundWord>)>;
    pub fn inner(&self) -> &StatelessTokenizer<Arc<JapaneseDictionary>>;
}

pub fn extract_compounds(tokens: &[SearchToken]) -> Vec<CompoundWord>;
```

## Colocated Token Rules

**Critical for correctness:**

1. **Mode C token** → `is_colocated: false` (new position)
2. **Mode B sub-tokens within Mode C span** → `is_colocated: true` (same position)
3. **Only emit different tokens** - skip if Mode B == Mode C

```rust
// CORRECT: only emit when different
if b_text != &text_c {
    result.push(SearchToken { is_colocated: true, .. });
}
```

## Integration Points

### Downstream Adapters

| Crate | Translation |
|-------|-------------|
| sudachi-tantivy | `is_colocated` → `position` stays same |
| sudachi-sqlite | `is_colocated` → `FTS5_TOKEN_COLOCATED` |
| sudachi-postgres | Via sudachi-tantivy |

### Upstream

```rust
use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::analysis::{Mode, Tokenize};
use sudachi::dic::dictionary::JapaneseDictionary;
```

## Commands

```bash
just dict-setup   # Download dictionary (one-time)
just build        # Build library
just test         # Unit tests (no dictionary)
just test-all     # All tests (requires dictionary)
just fix          # Format and lint
just env          # Show environment
```

## Testing

### Unit Tests (No Dictionary)

```bash
just test
```

### Integration Tests (Requires Dictionary)

```bash
just test-all
```

### Key Test Cases

1. **Simple compound**: "東京都立大学" → compound + 3 sub-tokens
2. **No compound**: "今日は" → single tokens, no colocated
3. **Multiple compounds**: "東京都立大学で国会議事堂" → 2 compounds
4. **Normalization**: "食べた" → "食べる" (when normalized)
5. **Empty input**: "" → empty Vec

## Debugging

### Verify Colocated Logic

```rust
let tokens = tokenizer.tokenize("東京都立大学")?;
for (i, token) in tokens.iter().enumerate() {
    println!("{}: {} colocated={}", i, token.surface, token.is_colocated);
}
// Expected:
// 0: 東京都立大学 colocated=false
// 1: 東京 colocated=true
// 2: 都立 colocated=true
// 3: 大学 colocated=true
```

### Common Issues

| Symptom | Cause | Fix |
|---------|-------|-----|
| No sub-tokens | Mode B == Mode C | Check dictionary quality |
| Wrong offsets | Char vs byte | Use `m.begin()` directly (bytes) |
| Missing compounds | Threshold too strict | Check `components.len() > 1` |

## When Modifying

### Adding Features

1. **New output format**: Add to `SearchToken` struct
2. **New detection mode**: Add method like `detect_compounds()`
3. **Performance**: Consider caching Mode B tokens

### Changing Algorithm

Be careful with:
- Colocated flag logic
- Byte offset handling
- Token emission order (Mode C first, then Mode B)

### Testing Changes

Always test with real Japanese text:
- Compound words (東京都立大学, 予約困難店)
- Simple text (今日は天気がいい)
- Mixed (会議が東京で開催)

## Performance

- Dictionary: Shared via Arc, ~70MB
- Tokenization: 2x standard (Mode C + Mode B)
- Allocations: 2 Vec per input + result Vec

Optimize by reusing tokenizer instance.
