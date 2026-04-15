# sudachi monorepo

Japanese morphological analysis ecosystem — Sudachi tokenizers for SQLite, PostgreSQL (ParadeDB), Tantivy, and WebAssembly.

## Structure

```
crates/
├── sudachi-search/    # Core: B+C multi-granularity tokenization (search-engine agnostic)
├── sudachi-sqlite/    # Adapter: SearchToken → SQLite FTS5 colocated tokens (cdylib)
├── sudachi-tantivy/   # Adapter: SearchToken → Tantivy (used by jurmarcus/paradedb)
└── sudachi-wasm/      # Adapter: SearchToken → WebAssembly via wasm-bindgen

docker/
└── postgres/          # Docker infrastructure for ParadeDB + Sudachi (no Rust source here)
```

## Workspace

Root workspace members: `sudachi-search`, `sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm`.

Separate build targets:
- `docker/postgres/` — Docker infra; Rust pgrx extension lives in `~/CODE/paradedb`

## WASM Patch

The upstream `sudachi` crate doesn't compile for `wasm32-unknown-unknown` due to `libloading`
(DSO plugin loading). We apply a patch from `jurmarcus/sudachi.rs` that gates the plugin loader
behind `#[cfg(not(target_family = "wasm"))]`. See `[patch]` in root `Cargo.toml`.

When https://github.com/WorksApplications/sudachi.rs/pull/313 merges upstream, remove the
`[patch]` block and update the `sudachi` workspace dep back to the upstream URL.

## ParadeDB Integration

The Postgres extension lives at `~/CODE/paradedb` (`jurmarcus/paradedb`).
`sudachi-tantivy` is pulled in as a git dep from this monorepo:

```toml
sudachi-tantivy = { git = "https://github.com/jurmarcus/sudachi", optional = true }
```

## Commands

```bash
just              # List all commands
just build        # Build workspace crates (release)
just test         # Run workspace tests
just fix          # Format + lint
just ci           # Full CI: fmt check + clippy + tests

just wasm-build       # Build WASM for browser (ES module) from crates/sudachi-wasm
just wasm-build-node  # Build WASM for Node.js

just dict-setup   # Install Sudachi dictionary to ~/.sudachi/
just dict-path    # Show resolved dictionary path

just pgrx-build   # Build pg_search in ~/CODE/paradedb --features icu,sudachi
just pgrx-check   # Check pg_search in ~/CODE/paradedb --features icu,sudachi
```

## Dictionary

Required at runtime. Auto-discovered from `~/.sudachi/system_full.dic`:

```bash
just dict-setup
```

Or set `SUDACHI_DICT_PATH=/path/to/system_full.dic` explicitly.

## Key Design Facts

- `sudachi-search` is the only pure-logic crate — all engine adapters depend on it
- `is_colocated: bool` is the adapter contract; every engine translates it differently
- `panic = "abort"` must NOT be in `sudachi-sqlite` — breaks `catch_unwind` FFI safety
- `crate-type = ["cdylib", "rlib"]` in sudachi-sqlite — cdylib for SQLite, rlib for tests
- `sudachi.rs` dep is pinned to a specific rev in workspace so all crates share types

## Version Control

This repo uses **Sapling (sl)**, not git.

```bash
sl status       # Status
sl add          # Stage new files
sl commit       # Commit
sl push         # Push to GitHub
sl addremove    # Detect added/removed files
```
