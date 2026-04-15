# AGENTS.md - sudachi-sqlite

Context for AI agents working on this codebase.

## Project Summary

**sudachi-sqlite** is a SQLite FTS5 tokenizer extension using Sudachi with B+C multi-granularity.

| Attribute | Value |
|-----------|-------|
| Complexity | Medium (~400 LOC) |
| Pattern | C ABI extension with FFI |
| Key Feature | FTS5_TOKEN_COLOCATED flag |
| Upstream | sudachi-search |

## The Problem

Japanese compound words break FTS5 search:

```
Document: "東京都立大学で研究"
Query: "大学"

Default FTS5: ["東京都立大学", "で", "研究"]
→ "大学" not found (trapped inside compound)

Sudachi FTS5:
  pos 0: "東京都立大学" (flag: 0)
  pos 0: "東京"         (flag: FTS5_TOKEN_COLOCATED)
  pos 0: "都立"         (flag: FTS5_TOKEN_COLOCATED)
  pos 0: "大学"         (flag: FTS5_TOKEN_COLOCATED) ← NOW SEARCHABLE
  pos 1: "で"
  pos 2: "研究"
```

## File Structure

```
sudachi-sqlite/
├── src/
│   ├── lib.rs          # Entry point, tokenization (~200 LOC)
│   ├── extension.rs    # FTS5 API retrieval (~100 LOC)
│   └── common.rs       # Panic boundary, callbacks (~100 LOC)
├── Cargo.toml
├── README.md
├── CLAUDE.md
└── AGENTS.md
```

## Critical Implementation Details

### Entry Point Pattern

```rust
#[unsafe(no_mangle)]
pub extern "C" fn sudachi_fts5_tokenizer_init(
    db: *mut sqlite3,
    _pz_err_msg: *mut *mut c_char,
    p_api: *mut fts5_api,
) -> c_int {
    ffi_panic_boundary(|| {
        // 1. Get FTS5 API
        let fts5_api = get_fts5_api(db, p_api)?;

        // 2. Register tokenizer
        register_tokenizer(fts5_api)?;

        Ok(())
    })
}
```

### FTS5 Callback Structure

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

### Colocated Token Translation

```rust
// sudachi-search gives us:
SearchToken { is_colocated: true, .. }

// We translate to:
callback.emit_colocated(...)  // Sets FTS5_TOKEN_COLOCATED = 0x0001
```

### Panic Boundary (MUST USE)

```rust
pub fn ffi_panic_boundary<F>(operation: F) -> c_int
where F: FnOnce() -> Result<(), c_int> + UnwindSafe
{
    match std::panic::catch_unwind(operation) {
        Ok(Ok(())) => SQLITE_OK,
        Ok(Err(code)) => code,
        Err(_) => SQLITE_INTERNAL,  // Panic caught
    }
}
```

### Memory Management

```rust
// xCreate: Allocate on heap, return raw pointer
let tokenizer = Box::new(Fts5Tokenizer::new(use_surface)?);
*pp_out = Box::into_raw(tokenizer) as *mut c_void;

// xDelete: Take ownership and drop
if !p_tokenizer.is_null() {
    drop(unsafe { Box::from_raw(p_tokenizer as *mut Fts5Tokenizer) });
}
```

## API Quick Reference

### SQL Usage

```sql
-- Load extension
.load ./libsudachi_sqlite sudachi_fts5_tokenizer_init

-- Normalized form (default)
CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer');

-- Surface form
CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer surface');

-- Search
SELECT * FROM docs WHERE docs MATCH '大学';
```

### Tokenizer Options

| Option | Description |
|--------|-------------|
| (none) | Normalized form (default) |
| `surface` | Surface form |

## Integration Points

### Upstream: sudachi-search

```rust
use sudachi_search::SearchTokenizer;

let tokens = tokenizer.tokenize(input)?;
for token in tokens {
    if token.is_colocated {
        callback.emit_colocated(...)?;
    } else {
        callback.emit(...)?;
    }
}
```

### FTS5 API Retrieval

```rust
// Special SQL query to get FTS5 function pointers
let query = "SELECT fts5(?1)";
// Parse result to get fts5_api pointer
```

## Commands

```bash
just dict-setup   # Download dictionary (one-time)
just build        # Build extension
just install      # Install to ~/.local/lib/
just test         # Run tests
just test-sqlite  # Interactive SQLite test
just rebuild      # Clean, build, and reinstall
just fix          # Format and lint
just env          # Show environment
```

## Testing

### Interactive SQLite Test

```bash
just test-sqlite
```

### Build & Test

```bash
just build
just test
```

## Common Issues

| Symptom | Cause | Fix |
|---------|-------|-----|
| Extension won't load | Missing `#[no_mangle]` | Add to entry point |
| No results | SUDACHI_DICT_PATH not set | Set environment variable |
| Crash on load | Panic crossing FFI | Wrap in `ffi_panic_boundary` |
| UTF-8 errors | Invalid input | Returns SQLITE_OK (intentional) |

## Debugging

```bash
# Check extension has correct symbol
nm -gU target/release/libsudachi_sqlite.dylib | grep sudachi
# Should show: sudachi_fts5_tokenizer_init

# Verbose SQLite loading
sqlite3 -cmd ".load ./libsudachi_sqlite sudachi_fts5_tokenizer_init" test.db
```

## When Modifying

### Adding Options

1. Parse in `xCreate` callback
2. Store in `Fts5Tokenizer` struct
3. Pass to `load_tokenizer()`

### Changing Token Output

1. Modify `emit_tokens()` in `lib.rs`
2. Keep `is_colocated` → `FTS5_TOKEN_COLOCATED` translation

### FFI Safety Rules

1. ALL entry points wrapped in `ffi_panic_boundary`
2. NO panics crossing FFI boundary
3. All memory manually managed with Box::into_raw / Box::from_raw
4. `panic = "abort"` in release profile

## Dependencies

```toml
[lib]
crate-type = ["cdylib", "rlib"]  # rlib required for cargo test

[dependencies]
libc.workspace = true
sqlite3ext-sys = "0.0.1"
sqlite-loadable.workspace = true
sudachi-search = { path = "../sudachi-search" }
sudachi.workspace = true

# CRITICAL: Do NOT add panic = "abort"
# It disables catch_unwind, breaking ffi_panic_boundary safety
```

## SQLite Constants

| Code | Constant | Value |
|------|----------|-------|
| SQLITE_OK | Success | 0 |
| SQLITE_INTERNAL | Internal error | 2 |
| SQLITE_MISUSE | API misuse | 21 |
| FTS5_TOKEN_COLOCATED | Colocated flag | 0x0001 |
