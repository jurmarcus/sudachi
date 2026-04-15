# sudachi monorepo

Japanese morphological analysis ecosystem — Sudachi tokenizers for SQLite, PostgreSQL, Tantivy, and WASM.

## Structure

```
crates/
├── sudachi-search/    # Core: B+C multi-granularity tokenization (search-engine agnostic)
├── sudachi-sqlite/    # Adapter: SearchToken → SQLite FTS5 colocated tokens
├── sudachi-tantivy/   # Adapter stub: SearchToken → Tantivy (not yet implemented)
├── sudachi-wasm/      # WASM: sudachi.rs compiled for browser/Node.js (excluded from workspace)
└── sudachi-postgres/  # pgrx: ParadeDB fork + Sudachi (excluded from workspace, own nested workspace)
```

## Workspace

The **root workspace** includes only `crates/sudachi-search` and `crates/sudachi-sqlite`.

Excluded crates:
- `crates/sudachi-tantivy/` — stub, excluded until tantivy dependency is added
- `crates/sudachi-wasm/` — built with wasm-pack, not standard cargo
- `crates/sudachi-postgres/` — pgrx requires its own nested workspace, incompatible with root

## Commands

```bash
just              # List all commands
just build        # Build workspace crates (release)
just test         # Run workspace tests
just fix          # Format + lint
just ci           # Full CI: fmt check + clippy + tests

just wasm build   # Build WASM (wasm-pack, browser ES module)
just wasm dict-setup  # Install WASM dict resources

just dict-setup   # Install Sudachi dictionary to ~/.sudachi/
just dict-path    # Show resolved dictionary path

just pgrx-build   # Build sudachi-postgres (requires cargo-pgrx)
```

## Dictionary

Required at runtime. Auto-discovered from `~/.sudachi/system_full.dic`:

```bash
just dict-setup
```

Or set `SUDACHI_DICT_PATH=/path/to/system_full.dic` explicitly.

## Version Control

This repo uses **Sapling (sl)**, not git.

```bash
sl status       # Status
sl add          # Stage
sl commit       # Commit
sl push         # Push to GitHub
```
