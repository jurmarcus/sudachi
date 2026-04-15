# AGENTS.md — sudachi monorepo

Context for AI agents working on this codebase.

## Repository Overview

Rust monorepo for Sudachi-based Japanese tokenization across multiple search engines.

| Crate | Type | Purpose |
|-------|------|---------|
| `sudachi-search` | lib | Core B+C tokenization, engine-agnostic |
| `sudachi-sqlite` | cdylib + rlib | SQLite FTS5 loadable extension |
| `sudachi-tantivy` | lib | Tantivy `Tokenizer` impl (used by paradedb fork) |
| `sudachi-wasm` | wasm-pack | Browser/Node.js WASM (excluded from workspace) |
| `sudachi-postgres` | docker only | Infra for ParadeDB; Rust source is in `jurmarcus/paradedb` |

Root workspace members: `sudachi-search`, `sudachi-sqlite`, `sudachi-tantivy`.

## The Core Concept

B+C multi-granularity: emit the Mode C compound word FIRST, then Mode B sub-tokens at
the SAME position. The `is_colocated` field signals "don't advance position":

```
tokenize("東京都立大学"):
  SearchToken { surface: "東京都立大学", is_colocated: false }  ← new position
  SearchToken { surface: "東京",         is_colocated: true  }  ← same position
  SearchToken { surface: "都立",         is_colocated: true  }  ← same position
  SearchToken { surface: "大学",         is_colocated: true  }  ← same position
```

Every adapter crate's job is to translate `is_colocated` for its engine:
- `sudachi-sqlite`: → `FTS5_TOKEN_COLOCATED` flag (= 0x0001)
- `sudachi-tantivy`: → Tantivy position increment stays 0

## File Map

```
Cargo.toml                   Workspace root (edition 2024, workspace.dependencies)
justfile                     Task runner for all crates
rust-toolchain.toml          Pins stable channel
crates/
  sudachi-search/src/lib.rs  Everything: SearchTokenizer, SearchToken, CompoundWord
  sudachi-sqlite/src/
    lib.rs                   Entry point, tokenization loop, dict loading
    extension.rs             FTS5 API retrieval, tokenizer registration, callbacks
    common.rs                ffi_panic_boundary, constants, callback types
  sudachi-tantivy/src/
    lib.rs                   Re-exports: SudachiTokenizer, SudachiTokenStream, SplitMode
    tokenizer.rs             SudachiTokenizer implementing Tantivy's Tokenizer trait
    stream.rs                SudachiTokenStream implementing Tantivy's TokenStream trait
  sudachi-postgres/docker/
    Dockerfile               Clones jurmarcus/paradedb from GitHub (no local source)
    bootstrap.sh             Postgres init script (CREATE EXTENSION pg_search)
    pg_search--0.20.6.sql    Pre-generated SQL schema (bypasses pgrx package UTF-8 bug)
```

## Critical Rules

### DO NOT violate these:

1. **`panic = "abort"` MUST NOT appear in `sudachi-sqlite`** — this disables `catch_unwind`
   and causes undefined behavior when Rust panics cross the FFI boundary into SQLite.
   Panics ARE caught by `ffi_panic_boundary` using `std::panic::catch_unwind`. Leave it.

2. **`crate-type = ["cdylib", "rlib"]` in sudachi-sqlite** — `cdylib` produces the loadable
   `.dylib`/`.so`; `rlib` is required for `cargo test` to link test code. Both are needed.

3. **`sudachi.rs` rev must stay pinned** — all workspace crates use the same git rev for the
   upstream sudachi dependency. Changing it in one place without the others breaks types.

4. **`is_colocated` ordering** — Mode C token FIRST (`is_colocated: false`), then Mode B
   sub-tokens (`is_colocated: true`). Never reorder. Search engines rely on this sequence.

5. **Sapling, not git** — `sl commit`, `sl push`, `sl addremove`. Never run bare `git` commands
   on this repo.

## Build & Test

```bash
just ci           # fmt check + clippy -D warnings + tests — must pass before committing
just build        # Release build
just test         # Unit tests (no dictionary required)
just dict-setup   # Download dictionary (one-time, for integration tests)
```

## Dependency Graph

```
sudachi.rs (upstream git)
    ↑
sudachi-search    (pinned rev, workspace dep)
    ↑
sudachi-sqlite    (path dep via workspace)
sudachi-tantivy   (path dep via workspace) → tantivy-tokenizer-api

jurmarcus/paradedb → sudachi-tantivy (git dep from this repo)
```

## Algorithm (sudachi-search)

```rust
fn tokenize_internal(input: &str) -> Vec<SearchToken> {
    // Two tokenization passes
    let morphemes_c = tokenizer.tokenize(input, Mode::C)?;  // compounds
    let morphemes_b = tokenizer.tokenize(input, Mode::B)?;  // sub-tokens

    for each morpheme_c:
        emit SearchToken { is_colocated: false }   // compound word

        for each morpheme_b within morpheme_c's byte span:
            if morpheme_b.text != morpheme_c.text:
                emit SearchToken { is_colocated: true }  // sub-token
}
```

## ParadeDB Integration

The paradedb fork lives at `~/CODE/paradedb` (`jurmarcus/paradedb`).
Key files there:
- `tokenizers/src/sudachi.rs` — `SudachiTokenizer` wrapping `sudachi-tantivy`
- `tokenizers/src/manager.rs` — `SearchTokenizer::Sudachi` variant (feature-gated)
- `tokenizers/Cargo.toml` — `sudachi = ["dep:sudachi-tantivy"]` feature

The `[patch.crates-io]` for `tantivy-tokenizer-api` in paradedb's Cargo.toml
redirects all transitive deps to paradedb's forked tantivy API. This is how
`sudachi-tantivy = "0.6.0"` resolves to the same type as paradedb's internal tantivy.

Build paradedb locally:
```bash
just pgrx-build   # → cd ~/CODE/paradedb && cargo pgrx build -p pg_search --features icu,sudachi
```

## Common Tasks

**Adding a tokenizer feature to sudachi-search:**
1. Add method to `SearchTokenizer` in `crates/sudachi-search/src/lib.rs`
2. Run `just ci` to verify

**Adding a tokenizer option to sudachi-sqlite:**
1. Parse in `xCreate` callback (`src/extension.rs`)
2. Store in `Fts5Tokenizer` struct (`src/lib.rs`)
3. Test with `just test`

**Updating the Tantivy integration in paradedb:**
1. Edit `crates/sudachi-tantivy/src/tokenizer.rs` or `stream.rs`
2. `sl commit && sl push` to push to GitHub
3. In `~/CODE/paradedb`: `cargo update -p sudachi-tantivy`
4. `just pgrx-check` to verify

**Testing the SQLite extension manually:**
```bash
SUDACHI_DICT_PATH=~/.sudachi/system_full.dic sqlite3 test.db
.load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init
CREATE VIRTUAL TABLE t USING fts5(c, tokenize='sudachi_tokenizer');
INSERT INTO t VALUES ('東京都立大学で研究');
SELECT * FROM t WHERE t MATCH '大学';
```

## Performance Notes

- Dictionary is ~70MB, shared via `Arc<JapaneseDictionary>`
- B+C tokenization does 2 passes — ~2× the cost of single-mode tokenization
- `sudachi-sqlite` loads dictionary at `xCreate` time (once per FTS5 table)
- `sudachi-tantivy` uses `Lazy<Option<Arc<SudachiTokenizer>>>` per mode — loaded once
