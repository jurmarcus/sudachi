# CLAUDE.md - sudachi-sqlite

Sudachi Japanese tokenizer for SQLite FTS5 full-text search.

## What This Is

A SQLite FTS5 loadable extension (~400 LOC) that registers `sudachi_tokenizer` for Japanese full-text search with B+C multi-granularity.

## Why This Exists

Japanese compound words break traditional FTS5:

```
Document: "東京都立大学で研究"
Query: "大学"

Default FTS5: ["東京都立大学", "で", "研究"]
→ "大学" NOT FOUND - trapped inside compound

Sudachi FTS5:
  pos 0: "東京都立大学" (flag: 0)
  pos 0: "東京"         (flag: FTS5_TOKEN_COLOCATED)  ← SAME POSITION
  pos 0: "都立"         (flag: FTS5_TOKEN_COLOCATED)  ← SAME POSITION
  pos 0: "大学"         (flag: FTS5_TOKEN_COLOCATED)  ← NOW SEARCHABLE!
  pos 1: "で"
  pos 2: "研究"
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     SQLite FTS5                                 │
│  CREATE VIRTUAL TABLE docs USING fts5(                         │
│      content, tokenize='sudachi_tokenizer'                     │
│  );                                                            │
└────────────────────────────┬────────────────────────────────────┘
                             │ loads extension, calls tokenizer
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│              sudachi_fts5_tokenizer_init                       │
│  - Entry point (#[no_mangle] extern "C")                       │
│  - Retrieves FTS5 API via special SQL query                    │
│  - Registers xCreate, xDelete, xTokenize callbacks             │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Fts5Tokenizer                               │
│  - Wraps SearchTokenizer from sudachi-search                   │
│  - is_colocated: true → FTS5_TOKEN_COLOCATED flag             │
│  - is_colocated: false → flag = 0                             │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    sudachi-search                              │
│              (B+C multi-granularity core)                      │
└─────────────────────────────────────────────────────────────────┘
```

## Key Files

| File | Purpose | LOC |
|------|---------|-----|
| `src/lib.rs` | Entry point, tokenization, dictionary loading | ~200 |
| `src/extension.rs` | FTS5 API retrieval, tokenizer registration | ~100 |
| `src/common.rs` | Panic boundary, SQLite constants, callbacks | ~100 |

## Critical Implementation Details

### FTS5 Token Callback

```rust
type TokenFunction = extern "C" fn(
    p_ctx: *mut c_void,
    t_flags: c_int,        // 0 or FTS5_TOKEN_COLOCATED
    p_token: *const c_char,
    n_token: c_int,
    i_start: c_int,        // Byte offset
    i_end: c_int,          // Byte offset
) -> c_int;
```

### Translation from sudachi-search

```rust
for token in tokens {
    if token.is_colocated {
        callback.emit_colocated(...)?;  // FTS5_TOKEN_COLOCATED = 0x0001
    } else {
        callback.emit(...)?;  // flags = 0
    }
}
```

### Panic Safety (CRITICAL)

Rust panics MUST NOT cross the FFI boundary. ALL three callbacks (xCreate, xDelete, xTokenize)
are wrapped in `ffi_panic_boundary`:

```rust
pub fn ffi_panic_boundary<F>(operation: F) -> c_int
where F: FnOnce() -> Result<(), c_int> + UnwindSafe
{
    match std::panic::catch_unwind(operation) {
        Ok(Ok(())) => SQLITE_OK,
        Ok(Err(code)) => code,
        Err(_) => SQLITE_INTERNAL,
    }
}
```

Do NOT add `panic = "abort"` to Cargo.toml — it would disable `catch_unwind` and cause UB.

### crate-type = ["cdylib", "rlib"]

- `cdylib` — produces the loadable SQLite extension (`.dylib`/`.so`)
- `rlib` — required for `#[test]` to work with `cargo test -p sudachi-sqlite`

### Dictionary Loading

```rust
pub fn load_tokenizer(use_surface_form: bool) -> Result<SearchTokenizer, c_int> {
    let dict_path = std::env::var("SUDACHI_DICT_PATH")?;
    // ...
}
```

## SQL Usage

```sql
-- Load extension
.load ./libsudachi_sqlite sudachi_fts5_tokenizer_init

-- Normalized form (default)
CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer');

-- Surface form
CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer surface');

-- Search
INSERT INTO docs VALUES ('東京都立大学で研究');
SELECT * FROM docs WHERE docs MATCH '大学';  -- FOUND!
```

## Commands

```bash
just build        # Build extension (from repo root)
just test         # Run workspace tests
just fix          # Format and lint
just dict-setup   # Download dictionary
```

Run all commands from the repo root (workspace `just`).

## Dependencies

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
libc.workspace = true
sqlite-loadable.workspace = true
sudachi.workspace = true
sudachi-search.workspace = true
```

Note: `sqlite-loadable` re-exports all needed FTS5 types — no need for `sqlite3ext-sys` separately.

## SQLite Error Codes

| Code | Constant | Meaning |
|------|----------|---------|
| 0 | SQLITE_OK | Success |
| 2 | SQLITE_INTERNAL | Internal error |
| 21 | SQLITE_MISUSE | API misuse |

## FTS5 API Constants

```rust
const FTS5_TOKEN_COLOCATED: c_int = 0x0001;
```

## Common Issues

| Issue | Cause | Fix |
|-------|-------|-----|
| Extension won't load | Missing symbol | Check `#[no_mangle]` on entry point |
| No results | Dictionary not loaded | Check SUDACHI_DICT_PATH |
| Panic on bad input | UTF-8 handling | Invalid UTF-8 returns SQLITE_OK |
