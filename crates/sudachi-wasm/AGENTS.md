# AGENTS.md — sudachi-wasm

Context for AI agents working on this crate.

## Purpose

Browser + Node.js bindings for `sudachi-search`. A `wasm-bindgen` shim that exposes B+C tokenisation as a JavaScript class.

| Attribute   | Value                                                    |
| ----------- | -------------------------------------------------------- |
| Type        | cdylib + rlib (~145 LOC)                                 |
| Build tool  | `wasm-pack` (not plain cargo build)                      |
| Targets     | `web` (ES module), `nodejs` (CJS), `bundler` (webpack/vite/rollup) |
| Output      | `crates/sudachi-wasm/pkg/`                               |
| Publish     | `publish = false` — distributed via `pkg/` artefacts, not crates.io |

## File map

```
src/lib.rs           SudachiTokenizer wasm-bindgen exports + JsToken/JsCompound shims
example/index.html   Browser demo
example/node.mjs     Node.js demo
example/package.json
Cargo.toml           crate-type cdylib+rlib, wasm-bindgen, serde-wasm-bindgen
```

## Hard rules

1. **Imports through the gateway.** `sudachi_optimizer::sudachi::JapaneseDictionary`, never `sudachi::*` directly.
2. **The wasm patch is not optional.** Workspace root has `[patch.…]` redirecting upstream `sudachi.rs` to a fork that gates `libloading` behind `cfg(not(wasm))`. Removing it breaks `wasm-pack build`.
3. **JS-facing types use camelCase.** `#[serde(rename_all = "camelCase")]` on `JsToken` / `JsCompound`. JS callers expect `byteStart`, `isColocated`, etc.
4. **`crate-type = ["cdylib", "rlib"]`.** cdylib for the wasm output; rlib so the crate participates in `cargo test` / `cargo clippy` on the host platform.

## API surface (exported to JS)

```rust
#[wasm_bindgen]
pub struct SudachiTokenizer { inner: SearchTokenizer }

#[wasm_bindgen]
impl SudachiTokenizer {
    #[wasm_bindgen(constructor)]
    pub fn new(dict_bytes: &[u8]) -> Result<SudachiTokenizer, JsError>;

    pub fn tokenize(&self, text: &str) -> Result<JsValue, JsError>;
    pub fn tokenize_surfaces(&self, text: &str) -> Result<JsValue, JsError>;
    pub fn detect_compounds(&self, text: &str) -> Result<JsValue, JsError>;

    pub fn with_surface_form(self) -> SudachiTokenizer;
}
```

## Build commands

```bash
just wasm-build           # --target web (ES module, default for browsers)
just wasm-build-node      # --target nodejs
just wasm-build-bundler   # --target bundler (webpack, vite)
just wasm-build-dev       # --dev (unoptimised, faster iteration)
just wasm-serve           # serve example/ at http://localhost:3000 with --cors
```

Each target produces a different `pkg/` layout — there's no single "universal" build.

## When changing this crate

### Add a new method

1. Add `pub fn` to the `#[wasm_bindgen] impl SudachiTokenizer` block.
2. Convert the result via `serde_wasm_bindgen::to_value` to a JS object.
3. Update `example/index.html` and/or `example/node.mjs` to demonstrate it.
4. Update the README's API section.

### Add a new JS-facing struct

1. Define a `#[derive(serde::Serialize)] #[serde(rename_all = "camelCase")] struct JsXxx { ... }`.
2. Implement `From<DomainStruct> for JsXxx`.
3. Use `serde_wasm_bindgen::to_value(&js_xxx)` to hand to JS.

### Update the wasm patch

If upstream `sudachi.rs` merges the wasm-friendly version, the patch in workspace `Cargo.toml` can be removed. Test by running `just wasm-build-dev` against an unpatched `sudachi.rs` rev.

### Test in a browser

```bash
just wasm-build
just wasm-serve
# open http://localhost:3000/example/
```

The demo lets you upload a `.dic` file or paste a URL and tokenise text in real time.

## Cargo.toml

```toml
[package]
name = "sudachi-wasm"
publish = false

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
sudachi-search.workspace = true
sudachi-optimizer.workspace = true
wasm-bindgen = "0.2"
serde = { version = "1", features = ["derive"] }
serde-wasm-bindgen = "0.6"

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

## Dictionary handling

```rust
let dictionary = JapaneseDictionary::from_system_bytes(dict_bytes.to_vec())
    .map_err(|e| JsError::new(&format!("Failed to load Sudachi dictionary: {e}")))?;
```

`from_system_bytes` parses the in-memory bytes. The wasm module then owns them. ~70MB dictionary → noticeable allocation spike at constructor time. Callers should construct the tokenizer once and reuse.

## Common issues

| Symptom                         | Cause                                       | Fix                                     |
| ------------------------------- | ------------------------------------------- | --------------------------------------- |
| `wasm-pack build` fails on `libloading` | Patch missing or rev mismatch       | Verify root `Cargo.toml` `[patch]` block |
| `pkg/sudachi_wasm.js` not generated | Wrong target flag                       | Use `--target web`/`nodejs`/`bundler`   |
| JS sees snake_case fields       | `#[serde(rename_all = "camelCase")]` missing | Add the attribute to the `JsXxx` struct |
| `init()` not awaited            | Caller used the export before init resolved | `await init()` before constructing      |
| CORS error fetching dict        | Browser cross-origin without right headers  | `just wasm-serve` sets `--cors`         |

## Performance characteristics

- Constructor: parses ~70MB dictionary. Allocate once, reuse.
- `tokenize`: ~2× single-mode cost (B+C two passes) + one `serde-wasm-bindgen::to_value` per call.
- Prefer `tokenize_surfaces` for high-volume calls if you only need the primary surfaces — drops colocated tokens, less serialisation work.
- Built with workspace `[profile.release]` (`opt-level=3`, `lto`, `codegen-units=1`, `strip`). Release wasm binary is small; dictionary dominates payload.
