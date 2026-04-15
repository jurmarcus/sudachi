# sudachi monorepo

Japanese morphological analysis ecosystem — Sudachi tokenizers for PostgreSQL, SQLite, Tantivy, and WASM.

## Structure

```
crates/
├── sudachi-search/    # Core: B+C multi-granularity tokenization (search-engine agnostic)
├── sudachi-tantivy/   # Adapter: SearchToken → Tantivy position stream
├── sudachi-sqlite/    # Adapter: SearchToken → SQLite FTS5 colocated tokens
├── sudachi-wasm/      # WASM: sudachi.rs compiled for browser/Node.js
└── sudachi-postgres/  # pgrx: ParadeDB fork + Sudachi (own nested workspace)
```

## Commands

```bash
just              # List all commands
just build        # Build workspace crates
just test         # Run workspace tests
just fix          # Format + lint
just dict-setup   # Install Sudachi dictionary (~400MB)
just dict-path    # Show dictionary path

just search build # Build sudachi-search only
just sqlite build # Build sudachi-sqlite only
just wasm build   # Build WASM (wasm-pack)

just pgrx-build   # Build sudachi-postgres (pgrx)
```

## Dictionary

Required at runtime. Auto-discovered from `~/.sudachi/system_full.dic`:

```bash
just dict-setup
```

Or set `SUDACHI_DICT_PATH=/path/to/system.dic` explicitly.

## Workspace

Root workspace includes `crates/sudachi-{search,tantivy,sqlite,wasm}`.

`crates/sudachi-postgres/` is **excluded** — it has its own nested pgrx workspace and must be built separately via `just pgrx-build` or `cargo build --manifest-path crates/sudachi-postgres/Cargo.toml`.

## Version Control

This repo uses **Sapling (sl)**, not git.

```bash
sl status       # Status
sl add          # Stage
sl commit       # Commit
sl push         # Push to GitHub
```
