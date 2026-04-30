# CLAUDE.md — sudachi-wasm

`wasm-bindgen` wrapper around `sudachi-search` for browsers and Node.js.

## What this is

A thin `wasm-bindgen` shim (~145 LOC in `src/lib.rs`) that exposes `sudachi_search::SearchTokenizer` to JavaScript. Built with `wasm-pack`, not plain Cargo.

## Architecture

```
JavaScript / TypeScript caller
        │
        ▼
SudachiTokenizer (wasm-bindgen)
  ├── new(dict_bytes: &[u8])
  │     └── JapaneseDictionary::from_system_bytes
  │     └── SearchTokenizer::new(Arc::new(dict))
  ├── tokenize(text)            → Vec<JsToken>  → JS Array
  ├── tokenize_surfaces(text)   → Vec<&str>     → JS Array (only !is_colocated)
  ├── detect_compounds(text)    → Vec<JsCompound>
  └── with_surface_form()       → SudachiTokenizer (consumes self)
        │
        ▼
sudachi-search::SearchTokenizer  (B+C two-pass tokenisation)
        │
        ▼
sudachi_optimizer::sudachi::*    (single Sudachi gateway)
```

## Why a separate crate

1. **Build target**: needs `wasm-pack` (not plain cargo). Different build command, different output (`pkg/` directory with `.wasm` + JS glue).
2. **Cargo workspace participation**: it IS a workspace member so `cargo check` / `cargo clippy` cover it on the host platform. The wasm32 build is invoked separately via `just wasm-build*`.
3. **JS-friendly types**: `JsToken` and `JsCompound` use `serde(rename_all = "camelCase")` so JS callers see `byteStart`, `isColocated` instead of snake_case.

## Files

```
src/lib.rs           SudachiTokenizer wasm-bindgen exports + JsToken/JsCompound serde shims
example/index.html   Browser demo
example/node.mjs     Node.js demo
example/package.json
Cargo.toml           crate-type = ["cdylib", "rlib"], wasm-bindgen + serde + serde-wasm-bindgen
```

## The libloading patch

Upstream `sudachi.rs` does not compile for `wasm32-unknown-unknown` because it uses `libloading` for DSO plugin loading. The workspace `Cargo.toml` redirects to a fork that gates the plugin loader behind `cfg(not(target_family = "wasm"))`:

```toml
[patch."https://github.com/WorksApplications/sudachi.rs"]
sudachi = { git = "https://github.com/jurmarcus/sudachi.rs", rev = "..." }
```

Without this patch, `wasm-pack build crates/sudachi-wasm` fails with link errors on the `libloading` crate. The patch is invisible to non-wasm crates.

## Imports

```rust
// CORRECT — through the gateway
use sudachi_optimizer::sudachi::JapaneseDictionary;
use sudachi_search::{CompoundWord, SearchToken, SearchTokenizer};

// WRONG — never reach upstream sudachi directly
use sudachi::dic::dictionary::JapaneseDictionary;
```

## JS type contract

```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsToken {
    surface: String,
    byte_start: usize,
    byte_end: usize,
    is_colocated: bool,
}
```

Renamed fields produce JS objects with camelCase keys. `serde-wasm-bindgen::to_value` converts to a real JS object (not a JSON string).

## Build matrix

```bash
just wasm-build           # wasm-pack --target web        → ES module
just wasm-build-node      # wasm-pack --target nodejs     → CommonJS
just wasm-build-bundler   # wasm-pack --target bundler    → webpack / vite
just wasm-build-dev       # wasm-pack --dev               → unoptimised, fast iteration
just wasm-serve           # serve example/ at :3000
```

Each target produces a different `pkg/` layout — pick the one that matches your bundler.

## Cargo.toml constraints

```toml
[lib]
crate-type = ["cdylib", "rlib"]   # cdylib for wasm output, rlib for cargo test/check

[dependencies]
sudachi-search.workspace = true
sudachi-optimizer.workspace = true
wasm-bindgen = "0.2"
serde = { version = "1", features = ["derive"] }
serde-wasm-bindgen = "0.6"

[dev-dependencies]
wasm-bindgen-test = "0.3"

publish = false   # not published to crates.io — built with wasm-pack
```

## Running tests

Host-only unit tests:

```bash
cargo test -p sudachi-wasm    # rlib test target on host platform
just test                     # workspace tests including this one
```

For wasm-specific tests (using `wasm-bindgen-test`), use `wasm-pack test`:

```bash
wasm-pack test crates/sudachi-wasm --headless --chrome
```

## Dictionary loading semantics

```rust
#[wasm_bindgen(constructor)]
pub fn new(dict_bytes: &[u8]) -> Result<SudachiTokenizer, JsError> {
    let dictionary = JapaneseDictionary::from_system_bytes(dict_bytes.to_vec())?;
    Ok(SudachiTokenizer { inner: SearchTokenizer::new(Arc::new(dictionary)) })
}
```

`from_system_bytes` parses the dictionary in-memory. The caller hands over bytes (`Uint8Array`); the wasm module owns them after the call. ~70MB dictionary → expect a noticeable allocation spike at construction.

## Common JS pitfalls

| Symptom                          | Cause                                              | Fix                                  |
| -------------------------------- | -------------------------------------------------- | ------------------------------------ |
| `tokenize` returns a Promise     | Forgot to `await init()`                           | `await init()` before instantiation  |
| Empty result                     | Bad UTF-8 input                                    | JS strings → wasm: should always be valid; check the dict path |
| `Failed to load Sudachi dictionary` | Wrong dict file (e.g. small vs full)            | Rebuild dict via `just dict-setup` and use the resulting bytes |
| Browser CORS error on dict fetch | Serving from a different origin without headers   | Use `just wasm-serve` (sets `--cors`) |

## Performance notes

- Construction parses the whole dictionary (~70MB) into wasm linear memory. Allocate once, share the tokenizer across calls.
- Per-tokenise cost: same as `sudachi-search` (two Sudachi passes for Search mode).
- Each token cross-the-FFI is one `serde-wasm-bindgen::to_value` conversion. For huge inputs, prefer `tokenize_surfaces` over `tokenize` (only emits primary tokens).
- The wasm binary itself is small (~few MB compiled with `lto`); the dictionary dominates payload size.
