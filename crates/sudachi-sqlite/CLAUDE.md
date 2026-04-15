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

// Emit regular token
callback.emit(surface.as_bytes(), byte_start, byte_end)?;

// Emit colocated token
callback.emit_colocated(surface.as_bytes(), byte_start, byte_end)?;
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

Rust panics must NOT cross FFI boundary:

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

### Dictionary Loading

```rust
pub fn load_tokenizer(use_surface_form: bool) -> Result<SearchTokenizer, c_int> {
    let dict_path = std::env::var("SUDACHI_DICT_PATH")?;
    let dict_bytes = std::fs::read(&dict_path)?;
    let storage = SudachiDicData::new(Storage::Owned(dict_bytes));
    let config = Config::minimal_at(dict_path.parent()?);
    let dictionary = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage)?;

    let tokenizer = SearchTokenizer::new(Arc::new(dictionary));
    if use_surface_form {
        Ok(tokenizer.with_surface_form())
    } else {
        Ok(tokenizer)  // Normalized by default
    }
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
# Setup dictionary (one-time)
just dict-setup

# Build extension
just build

# Install to ~/.local/lib/
just install

# Run tests
just test

# Interactive SQLite test
just test-sqlite

# Format and lint
just fix

# Full rebuild and reinstall
just rebuild

# Show environment
just env
```

All commands use `just` (task runner). Run `just --list` to see all available commands.

## Dependencies

```toml
[lib]
crate-type = ["cdylib"]  # Loadable extension

[dependencies]
libc = "0.2"
sqlite3ext-sys = "0.0.1"
sudachi-search = { git = "https://github.com/jurmarcus/sudachi-search" }

[profile.release]
panic = "abort"  # Required for FFI
```

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
| Panic on Japanese text | UTF-8 handling | Invalid UTF-8 returns SQLITE_OK |
