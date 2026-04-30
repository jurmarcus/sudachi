# AGENTS.md — sudachi-search

Context for AI agents working on this crate.

## Purpose

The B+C multi-granularity tokenisation core. Engine-agnostic. Produces `Vec<SearchToken>` with the `is_colocated: bool` flag that downstream adapters translate into engine-specific position semantics.

| Attribute   | Value                                                     |
| ----------- | --------------------------------------------------------- |
| Type        | rlib (library)                                            |
| Size        | ~750 LOC, single file (`src/lib.rs`)                      |
| Public API  | `SearchTokenizer`, `SearchToken`, `CompoundWord`, `extract_compounds`, `SearchError` |
| Dependency  | `sudachi-optimizer` (which re-exports upstream `sudachi`) |

## Architectural invariant

**All Sudachi imports go through `sudachi-optimizer::sudachi::*`.** Workspace `Cargo.toml` documents this rule. If a Sudachi type isn't yet re-exported, add it to `crates/sudachi-optimizer/src/sudachi.rs` first — never depend on upstream `sudachi` directly here.

```rust
use sudachi_optimizer::sudachi::{
    JapaneseDictionary, Mode, StatelessTokenizer, Tokenize,
};
```

## File map

```
src/lib.rs    Everything (~750 LOC)
              ├─ SearchToken        (struct)
              ├─ CompoundWord       (struct + impl)
              ├─ SearchTokenizer    (struct + impl)
              ├─ extract_compounds  (free fn)
              └─ SearchError        (enum + Display + Error)
Cargo.toml    Single dep: sudachi-optimizer (workspace path dep)
README.md     User-facing documentation
CLAUDE.md     AI assistant context
AGENTS.md     This file
```

## Tokenisation algorithm

```rust
fn tokenize_internal(&self, input: &str, use_normalized: bool)
    -> Result<Vec<SearchToken>, SearchError>
{
    let morphemes_c = self.inner.tokenize(input, Mode::C, false)?;
    let morphemes_b = self.inner.tokenize(input, Mode::B, false)?;

    // Mode B lookup keyed by byte range
    let mode_b: Vec<(usize, usize, String)> = morphemes_b
        .iter()
        .map(|m| (m.begin(), m.end(), text_for(m, use_normalized)))
        .collect();

    let mut out = Vec::new();
    for c in morphemes_c.iter() {
        let c_text = text_for(c, use_normalized);
        let c_start = c.begin();
        let c_end = c.end();

        // 1. Emit the compound (advances position)
        out.push(SearchToken {
            surface: c_text.clone(),
            byte_start: c_start,
            byte_end: c_end,
            is_colocated: false,
        });

        // 2. Emit sub-tokens within C's span (same position)
        for (b_start, b_end, b_text) in &mode_b {
            if *b_start >= c_start && *b_end <= c_end && b_text != &c_text {
                out.push(SearchToken {
                    surface: b_text.clone(),
                    byte_start: *b_start,
                    byte_end: *b_end,
                    is_colocated: true,
                });
            }
        }
    }
    Ok(out)
}
```

Key invariants:
1. Compound first (`is_colocated: false`), then sub-tokens (`is_colocated: true`).
2. Skip sub-tokens whose text equals the compound — would create a duplicate index entry.
3. Byte offsets, not char offsets.

## When changing this crate

### Adding a new method on `SearchTokenizer`

Just append to the `impl SearchTokenizer` block. Keep the `&self` (immutable) shape — downstream callers may share a single tokenizer behind `Arc<…>`.

### Adding a field to `SearchToken`

Will break every adapter (`sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm`). Update them in the same change.

### Changing default normalisation

Default is `with_normalized_form(true)`. If you change this, update the corresponding defaults in `sudachi-tantivy::SudachiTokenizer::new` and `sudachi-sqlite`'s `xCreate` parsing.

### Changing the tokenisation algorithm

Be careful with:
- Order of emission (C first, then B's inside C's span)
- Inequality check (`b_text != c_text`)
- Byte offsets passed through unchanged
- Borrow lifetimes — `morphemes_b` must outlive the lookup

## Testing

```bash
cargo test -p sudachi-search                       # unit tests, no dictionary
cargo test -p sudachi-search -- --include-ignored  # integration, requires SUDACHI_DICT_PATH
```

### Test cases that must keep passing

| Input                            | Expected output                                    |
| -------------------------------- | -------------------------------------------------- |
| `"東京都立大学"`                 | 1 compound + 3 sub-tokens at position 0            |
| `"今日は天気がいい"`             | No colocated tokens                                |
| `"食べた"` (normalised)          | `["食べる"]`                                       |
| `""`                             | Empty `Vec`                                        |
| `"予約困難店を探す"`             | 1 compound (`予約困難店`) with 3 components       |
| Two compounds in one input       | Each gets its own colocated sub-tokens             |

## Integration points

### Upstream

```rust
sudachi_optimizer::sudachi::{
    JapaneseDictionary,    // dictionary handle
    Mode,                  // A | B | C
    StatelessTokenizer,    // takes Arc<JapaneseDictionary>
    Tokenize,              // trait providing tokenize()
}
```

### Downstream

| Crate              | What they import        | What they do with `is_colocated`               |
| ------------------ | ----------------------- | ---------------------------------------------- |
| `sudachi-sqlite`   | `SearchTokenizer`, `SearchToken` | Set `FTS5_TOKEN_COLOCATED` flag (0x0001) on the token callback |
| `sudachi-tantivy`  | `SearchTokenizer` (only Search mode); falls back to upstream `StatelessTokenizer` for A/B/C | Skip incrementing Tantivy position |
| `sudachi-wasm`     | `SearchTokenizer`, `SearchToken`, `CompoundWord` | Pass through as `isColocated` JSON field |

## Commands

Run from the repo root (workspace `just`):

```bash
just dict-setup    # download dictionary (one-time)
just build         # workspace release build
just test          # workspace unit tests
just fix           # fmt + clippy
just ci            # fmt check + clippy -D warnings + tests
```

## Performance characteristics

- **Dictionary**: ~70MB, shared via `Arc`. One dict can back many tokenizers.
- **Per-call cost**: 2× single-mode tokenisation (Mode C + Mode B passes).
- **Per-call allocs**: 2× `Vec<Morpheme>` (Sudachi internal) + 1× `Vec<SearchToken>` (returned).
- **Optimisation**: Reuse a single `SearchTokenizer` across calls. Don't reconstruct per request.
