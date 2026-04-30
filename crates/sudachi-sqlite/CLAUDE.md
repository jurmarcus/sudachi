# CLAUDE.md — sudachi-sqlite

SQLite FTS5 loadable extension that registers `sudachi_tokenizer`. Wraps `sudachi-search` and translates `is_colocated` into the FTS5 `FTS5_TOKEN_COLOCATED` flag.

## What this is

`crate-type = ["cdylib", "rlib"]`. The cdylib is the loadable extension (`libsudachi_sqlite.{dylib,so}`). The rlib lets `cargo test` link the test binary.

## Why it exists

```text
Default FTS5: ["東京都立大学で研究"]   → query "大学" misses
Sudachi FTS5:
  pos 0  東京都立大学    flag: 0
  pos 0  東京            flag: FTS5_TOKEN_COLOCATED
  pos 0  都立            flag: FTS5_TOKEN_COLOCATED
  pos 0  大学            flag: FTS5_TOKEN_COLOCATED   → match
  pos 1  で
  pos 2  研究
```

## Architecture

```
SQLite FTS5
    │ .load libsudachi_sqlite sudachi_fts5_tokenizer_init
    ▼
sudachi_fts5_tokenizer_init   (entry point — extern "C", #[unsafe(no_mangle)])
    │ Retrieves fts5_api function pointers via SELECT fts5(?1)
    │ Registers our tokenizer struct: { xCreate, xDelete, xTokenize }
    ▼
Fts5Tokenizer                 (heap-allocated, owned by FTS5)
    │ Wraps sudachi-search's SearchTokenizer
    │ All FFI entry points wrapped in ffi_panic_boundary
    ▼
sudachi-search::SearchTokenizer
    │ B+C two-pass tokenisation
    │ Emits Vec<SearchToken> with is_colocated flag
    ▼
sudachi_optimizer::sudachi::*  (the single Sudachi gateway)
```

## Files

| File                | Purpose                                                | LOC  |
| ------------------- | ------------------------------------------------------ | ---- |
| `src/lib.rs`        | Entry point, `Fts5Tokenizer`, tokenization callback, dictionary loader | ~200 |
| `src/extension.rs`  | FTS5 API retrieval, tokenizer registration             | ~100 |
| `src/common.rs`     | `ffi_panic_boundary`, SQLite/FTS5 constants, callback types | ~100 |

## Critical FFI details

### Entry point

```rust
#[unsafe(no_mangle)]
pub extern "C" fn sudachi_fts5_tokenizer_init(
    db: *mut sqlite3,
    _pz_err_msg: *mut *mut c_char,
    p_api: *mut fts5_api,
) -> c_int {
    ffi_panic_boundary(|| {
        let fts5_api = get_fts5_api(db, p_api)?;
        register_tokenizer(fts5_api)?;
        Ok(())
    })
}
```

### Token callback signature

```rust
type TokenFunction = extern "C" fn(
    p_ctx: *mut c_void,
    t_flags: c_int,            // 0 or FTS5_TOKEN_COLOCATED (0x0001)
    p_token: *const c_char,
    n_token: c_int,
    i_start: c_int,            // byte offset
    i_end: c_int,              // byte offset
) -> c_int;
```

### `is_colocated` translation

```rust
for token in tokens {
    let flags = if token.is_colocated { FTS5_TOKEN_COLOCATED } else { 0 };
    callback(ctx, flags, token.surface.as_ptr() as *const c_char,
             token.surface.len() as c_int,
             token.byte_start as c_int, token.byte_end as c_int);
}
```

`FTS5_TOKEN_COLOCATED` is `0x0001` per the SQLite FTS5 ABI.

### Memory management

```rust
// xCreate: heap-allocate, return raw pointer
let tokenizer = Box::new(Fts5Tokenizer::new(use_surface)?);
*pp_out = Box::into_raw(tokenizer) as *mut c_void;

// xDelete: take ownership and drop
if !p_tokenizer.is_null() {
    drop(unsafe { Box::from_raw(p_tokenizer as *mut Fts5Tokenizer) });
}
```

### Panic safety

Every FFI entry point — `xCreate`, `xDelete`, `xTokenize`, the init function — is wrapped in:

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

**Do not add `panic = "abort"` to `Cargo.toml` or to the workspace.** It disables `catch_unwind` and turns any Rust panic crossing the FFI boundary into UB.

## Cargo.toml constraints

```toml
[lib]
crate-type = ["cdylib", "rlib"]   # both are required

[dependencies]
libc.workspace = true
sqlite-loadable.workspace = true
sqlite3ext-sys = "0.0.1"           # for sqlite3_stmt (not in sqlite-loadable's prelude)
sudachi-optimizer.workspace = true
sudachi-search.workspace = true
```

`sudachi-optimizer` is the gateway re-export; `sudachi-search` provides the B+C tokeniser. Don't depend on upstream `sudachi` directly.

## SQL surface

```sql
.load ./libsudachi_sqlite sudachi_fts5_tokenizer_init

-- Normalised form (default)
CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer');

-- Surface form
CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer surface');

INSERT INTO docs VALUES ('東京都立大学で研究');
SELECT * FROM docs WHERE docs MATCH '大学';
```

## Dictionary loading

```rust
fn load_tokenizer(use_surface_form: bool) -> Result<SearchTokenizer, c_int> {
    let dict_path = std::env::var("SUDACHI_DICT_PATH")
        .map_err(|_| SQLITE_INTERNAL)?;
    // ... load JapaneseDictionary, build SearchTokenizer
}
```

Called once per `CREATE VIRTUAL TABLE` (in `xCreate`). Errors map to SQLite return codes.

## Constants reference

| Constant               | Value   | Source           |
| ---------------------- | ------- | ---------------- |
| `SQLITE_OK`            | 0       | SQLite           |
| `SQLITE_INTERNAL`      | 2       | SQLite           |
| `SQLITE_MISUSE`        | 21      | SQLite           |
| `FTS5_TOKEN_COLOCATED` | 0x0001  | SQLite FTS5 ABI  |

## Commands

Run from the repo root (workspace `just`):

```bash
just dict-setup        # one-time
just build             # release build (cdylib lands at target/release/libsudachi_sqlite.{dylib,so})
just test              # workspace tests
just fix               # fmt + clippy
```

## Manual integration test

```bash
SUDACHI_DICT_PATH=~/.sudachi/system_full.dic sqlite3 test.db
sqlite> .load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init
sqlite> CREATE VIRTUAL TABLE t USING fts5(c, tokenize='sudachi_tokenizer');
sqlite> INSERT INTO t VALUES ('東京都立大学で研究');
sqlite> SELECT * FROM t WHERE t MATCH '大学';
```

## Common issues

| Symptom                       | Cause                                                       | Fix                                                |
| ----------------------------- | ----------------------------------------------------------- | -------------------------------------------------- |
| Extension fails to load       | Missing entry-point symbol                                  | Verify `#[unsafe(no_mangle)] extern "C"` on init   |
| `nm -gU libsudachi_sqlite.dylib` shows no `sudachi_*` | Stripped binary or wrong build profile | Build with `--release`; check `crate-type` includes cdylib |
| Returns no rows               | Dictionary not loaded                                       | Set `SUDACHI_DICT_PATH` before invoking sqlite3    |
| Crash on bad input            | Panic crossed FFI boundary                                  | Wrap the new code path in `ffi_panic_boundary`     |
| Invalid UTF-8 input           | Sudachi rejects                                             | Returns `SQLITE_OK` and emits no tokens (intentional) |

## What downstream looks like

```rust
// Inside lib.rs xTokenize callback:
let tokens = self.tokenizer.tokenize(input)?;
for tok in tokens {
    let flags = if tok.is_colocated { FTS5_TOKEN_COLOCATED } else { 0 };
    invoke_callback(ctx, flags, &tok.surface, tok.byte_start, tok.byte_end)?;
}
```

That's the entire `is_colocated` translation. Everything else is FFI plumbing.
