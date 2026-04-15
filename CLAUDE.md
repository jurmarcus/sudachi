# sudachi monorepo

Japanese morphological analysis ecosystem — Sudachi tokenizers for SQLite, PostgreSQL (ParadeDB), Tantivy, and WebAssembly.

## Structure

```
crates/
├── sudachi-search/    # Core: B+C multi-granularity tokenization (search-engine agnostic)
├── sudachi-sqlite/    # Adapter: SearchToken → SQLite FTS5 colocated tokens (cdylib)
└── sudachi-tantivy/   # Adapter: SearchToken → Tantivy (used by jurmarcus/paradedb)

wasm/                  # WASM: sudachi.rs compiled for browser/Node.js (wasm-pack, own workspace)
docker/
└── postgres/          # Docker infrastructure for ParadeDB + Sudachi (no Rust source here)
```

## Workspace

Root workspace members: `crates/sudachi-search`, `crates/sudachi-sqlite`, `crates/sudachi-tantivy`.

Separate build targets (not workspace members):
- `wasm/` — wasm-pack build; run via `just wasm build`
- `docker/postgres/` — Docker infra; Rust pgrx extension lives in `~/CODE/paradedb`

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

just wasm build       # Build WASM (wasm-pack, browser ES module) — runs from wasm/
just wasm dict-setup  # Install WASM dict resources into wasm/resources/

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
