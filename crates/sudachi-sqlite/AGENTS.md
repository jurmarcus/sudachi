# AGENTS.md — sudachi-sqlite

Context for AI agents working on this crate.

## Purpose

A SQLite FTS5 loadable extension that exposes Sudachi B+C tokenisation as the `sudachi_tokenizer` tokeniser. Translates `sudachi-search`'s `SearchToken::is_colocated` into the FTS5 `FTS5_TOKEN_COLOCATED` flag (0x0001).

| Attribute   | Value                                                  |
| ----------- | ------------------------------------------------------ |
| Type        | cdylib + rlib (~400 LOC)                               |
| Output      | `target/release/libsudachi_sqlite.{dylib,so,dll}`      |
| Entry point | `sudachi_fts5_tokenizer_init` (`extern "C"`)           |
| Dependencies | `sudachi-optimizer`, `sudachi-search`, `sqlite-loadable`, `sqlite3ext-sys`, `libc` |

## File map

```
src/lib.rs        Entry point, Fts5Tokenizer, tokenize callback, dictionary loader (~200 LOC)
src/extension.rs  FTS5 API retrieval (via SELECT fts5(?1)), tokenizer registration (~100 LOC)
src/common.rs     ffi_panic_boundary, SQLite/FTS5 constants, callback function types (~100 LOC)
Cargo.toml        crate-type = ["cdylib", "rlib"]
```

## Hard rules

1. **Never add `panic = "abort"`.** The cdylib relies on `std::panic::catch_unwind` in `ffi_panic_boundary` to convert Rust panics into SQLite error codes. `panic = "abort"` defeats `catch_unwind` and any panic becomes UB at the SQLite FFI.
2. **Keep `crate-type = ["cdylib", "rlib"]`.** cdylib produces the loadable extension; rlib is required for `cargo test` to link the test binary. Removing either breaks the build.
3. **All FFI entry points wrapped in `ffi_panic_boundary`.** That's `sudachi_fts5_tokenizer_init`, `xCreate`, `xDelete`, `xTokenize`. Adding a new callback? Wrap it too.
4. **Imports go through `sudachi-optimizer::sudachi::*`.** Never `use sudachi::*` directly here.
5. **Memory rules:** `Box::into_raw` to hand ownership to FTS5; `Box::from_raw` in `xDelete` to drop. Mismatching these leaks or double-frees.

## Entry point pattern

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

The init symbol has to be exactly this name — that's what `.load <library> <symbol>` in SQLite invokes.

## FTS5 callback signature

```rust
type TokenFunction = extern "C" fn(
    p_ctx: *mut c_void,
    t_flags: c_int,         // 0 or FTS5_TOKEN_COLOCATED
    p_token: *const c_char,
    n_token: c_int,
    i_start: c_int,         // BYTE offset
    i_end: c_int,           // BYTE offset
) -> c_int;
```

Byte offsets, not char offsets — `SearchToken::byte_start` / `byte_end` map straight through.

## `is_colocated` translation

```rust
for token in tokens {
    let flags = if token.is_colocated { FTS5_TOKEN_COLOCATED } else { 0 };
    invoke_callback(ctx, flags, &token.surface, token.byte_start, token.byte_end)?;
}
```

That's the whole adapter contract. Everything else is FFI plumbing.

## Panic boundary

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

Any code that can panic must run inside this. Tokenisation can panic (allocations, dictionary errors), so the `xTokenize` callback is wrapped end-to-end.

## When changing this crate

### Adding a tokeniser option

1. Parse it in `xCreate` (string args after the tokenizer name).
2. Store on `Fts5Tokenizer` struct.
3. Pass through to `load_tokenizer` and onward to `SearchTokenizer`.
4. Add a unit test (uses `cargo test` — rlib makes this work).

### Returning a new SQLite error code

Define the constant in `common.rs`. Map an internal `Result<_, c_int>` error to it. Don't return arbitrary integers.

### Touching `xTokenize`

The token callback walks `SearchTokenizer::tokenize(input)` and emits each token. Three failure cases:
- Sudachi error → `SQLITE_INTERNAL`
- Invalid UTF-8 input → `SQLITE_OK` with no tokens (FTS5 expects this on best-effort)
- Out-of-memory in the callback → `SQLITE_NOMEM`

## Manual integration test

```bash
SUDACHI_DICT_PATH=~/.sudachi/system_full.dic sqlite3 test.db
sqlite> .load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init
sqlite> CREATE VIRTUAL TABLE t USING fts5(c, tokenize='sudachi_tokenizer');
sqlite> INSERT INTO t VALUES ('東京都立大学で研究');
sqlite> SELECT * FROM t WHERE t MATCH '大学';
```

## Symbol verification

```bash
# macOS
nm -gU target/release/libsudachi_sqlite.dylib | grep sudachi
# Should show:  T _sudachi_fts5_tokenizer_init

# Linux
nm -D target/release/libsudachi_sqlite.so | grep sudachi
```

If the symbol is missing or undefined, `extern "C"` / `#[unsafe(no_mangle)]` was probably stripped or modified.

## Constants

| Constant               | Value   |
| ---------------------- | ------- |
| `SQLITE_OK`            | 0       |
| `SQLITE_INTERNAL`      | 2       |
| `SQLITE_MISUSE`        | 21      |
| `SQLITE_NOMEM`         | 7       |
| `FTS5_TOKEN_COLOCATED` | 0x0001  |
