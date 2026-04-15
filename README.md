# sudachi

**Japanese morphological analysis ecosystem — Sudachi tokenizers for SQLite, PostgreSQL (ParadeDB), Tantivy, and WebAssembly.**

Built on [sudachi.rs](https://github.com/WorksApplications/sudachi.rs) by Works Applications.

---

## The Problem

Japanese full-text search has a fundamental problem: **compound words trap sub-words**.

```
Document: "東京都立大学で研究しています"
Query: "大学"

Traditional tokenizer (Mode C): ["東京都立大学", "で", "研究", ...]
Result: NO MATCH — "大学" is trapped inside "東京都立大学"
```

This ecosystem solves it with **B+C multi-granularity tokenization**:

```
Sudachi Search mode:
  pos 0: "東京都立大学"  ← compound (as written)
  pos 0: "東京"          ← sub-token (same position!)
  pos 0: "都立"          ← sub-token (same position!)
  pos 0: "大学"          ← sub-token (same position!)
  pos 1: "で"
  pos 2: "研究"

Query "大学" → MATCH ✅
Query "東京都立大学" → MATCH ✅
Query "東京" → MATCH ✅
```

It also handles **verb conjugations** and **character variants**:

```
食べた / 食べている / 食べます → all normalize to 食べる
附属病院 → 付属病院 (variant kanji)
ＳＵＭＭＥＲ → サマー (fullwidth)
```

---

## Repository Structure

```
crates/
├── sudachi-search/    # Core B+C tokenization (search-engine agnostic)
├── sudachi-sqlite/    # SQLite FTS5 loadable extension
├── sudachi-tantivy/   # Tantivy tokenizer adapter (used by ParadeDB)
├── sudachi-wasm/      # WebAssembly (wasm-pack, browser/Node.js)
└── sudachi-postgres/  # Docker infrastructure for ParadeDB + Sudachi
```

The **root workspace** includes `sudachi-search`, `sudachi-sqlite`, and `sudachi-tantivy`.
The **ParadeDB integration** lives at `~/CODE/paradedb` (`jurmarcus/paradedb`).

---

## Crates

### `sudachi-search` — Core Library

The engine-agnostic B+C tokenization core. Every other crate depends on this.

**How it works:**

```rust
use sudachi_search::SearchTokenizer;

let tokens = tokenizer.tokenize("東京都立大学で研究")?;

// tokens:
// SearchToken { surface: "東京都立大学", is_colocated: false }  ← pos 0
// SearchToken { surface: "東京",         is_colocated: true }   ← pos 0
// SearchToken { surface: "都立",         is_colocated: true }   ← pos 0
// SearchToken { surface: "大学",         is_colocated: true }   ← pos 0
// SearchToken { surface: "で",           is_colocated: false }  ← pos 1
// SearchToken { surface: "研究",         is_colocated: false }  ← pos 2
```

The `is_colocated` flag is the adapter contract. Each downstream crate translates it:

| Engine | Translation |
|--------|-------------|
| SQLite FTS5 | `FTS5_TOKEN_COLOCATED` flag |
| Tantivy | Position increment = 0 |
| PostgreSQL | Via Tantivy (ParadeDB) |

Also provides `CompoundWord` detection and `extract_compounds()` for compound analysis.

---

### `sudachi-sqlite` — SQLite FTS5 Extension

A cdylib loadable extension that registers `sudachi_tokenizer` for SQLite FTS5.

```sql
-- Load extension
.load ./libsudachi_sqlite sudachi_fts5_tokenizer_init

-- Create FTS5 table with Sudachi tokenizer
CREATE VIRTUAL TABLE documents USING fts5(
    content,
    tokenize='sudachi_tokenizer'
);

-- Insert and search
INSERT INTO documents VALUES ('東京都立大学で研究しています');
SELECT * FROM documents WHERE documents MATCH '大学';  -- FOUND ✅
```

**Tokenizer options:**

| Syntax | Description |
|--------|-------------|
| `tokenize='sudachi_tokenizer'` | Normalized form (default, better recall) |
| `tokenize='sudachi_tokenizer surface'` | Surface form (original text) |

Works with Python, Node.js, Go, or any language with SQLite bindings.

---

### `sudachi-tantivy` — Tantivy Adapter

Implements `tantivy::tokenizer::Tokenizer` backed by Sudachi. Used by `jurmarcus/paradedb`
as a git dependency — provides the `SudachiTokenizer` and `SudachiTokenStream` that
integrate with Tantivy's position-based indexing.

**Split modes:**

| Mode | Output for "東京都立大学" |
|------|--------------------------|
| A | ["東京", "都", "立", "大学"] |
| B | ["東京", "都立", "大学"] |
| C | ["東京都立大学"] |
| **Search** | ["東京都立大学", "東京"\*, "都立"\*, "大学"\*] ← **default** |

\* Colocated tokens (same Tantivy position)

---

### `sudachi-wasm` — WebAssembly

Compiles sudachi.rs for browser and Node.js use via wasm-pack. Excluded from the
root workspace (wasm-pack incompatible with standard Cargo). Built with `just wasm build`.

```bash
just wasm dict-setup   # Populate resource files (one-time)
just wasm build        # wasm-pack build → ES module
```

---

### `sudachi-postgres` — ParadeDB + Sudachi

Docker infrastructure for deploying ParadeDB with the Sudachi tokenizer. The actual
Rust source lives at `~/CODE/paradedb` (`jurmarcus/paradedb`).

```sql
-- Create BM25 index with Sudachi tokenizer
CREATE INDEX docs_idx ON documents
USING bm25(id, (content::pdb.sudachi))
WITH (key_field='id');

-- Search — finds both compound words and sub-tokens
SELECT * FROM documents WHERE id @@@ 'content:大学';
```

**Modes via type cast:**

```sql
content::pdb.sudachi           -- Search mode (B+C, default)
content::pdb.sudachi('search') -- explicit
content::pdb.sudachi('c')      -- Mode C (longest tokens)
content::pdb.sudachi('a')      -- Mode A (finest)
```

---

## Quick Start

### Dictionary Setup (required for all crates)

```bash
just dict-setup   # Downloads to ~/.sudachi/system_full.dic
```

Or manually:

```bash
mkdir -p ~/.sudachi
curl -L https://github.com/WorksApplications/SudachiDict/releases/download/v20251022/sudachi-dictionary-20251022-full.zip -o /tmp/sudachi-dict.zip
unzip /tmp/sudachi-dict.zip -d /tmp/sudachi-temp/
cp /tmp/sudachi-temp/*/system_full.dic ~/.sudachi/
rm -rf /tmp/sudachi-temp /tmp/sudachi-dict.zip
```

### Build

```bash
just build        # Release build (workspace crates)
just test         # Run tests (unit tests, no dictionary required)
just ci           # Full CI: fmt check + clippy + tests
```

### SQLite

```bash
just build
sqlite3 test.db ".load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init"
```

### Tantivy (as a library dep)

```toml
[dependencies]
sudachi-tantivy = { git = "https://github.com/jurmarcus/sudachi" }
```

### ParadeDB (Docker)

```bash
cd crates/sudachi-postgres/docker
docker compose up
```

---

## Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                  jurmarcus/paradedb                              │
│   (ParadeDB fork — pg_search + Sudachi tokenizer feature)        │
│                                                                   │
│   pdb.sudachi → SearchTokenizer::Sudachi                         │
└─────────────────────────┬────────────────────────────────────────┘
                          │ git dep
┌─────────────────────────▼────────────────────────────────────────┐
│                   sudachi-tantivy                                 │
│   SudachiTokenizer → SudachiTokenStream                          │
│   is_colocated: true → position stays same                       │
└─────────────────────────┬────────────────────────────────────────┘
                          │ path dep
┌─────────────────────────▼────────────────────────────────────────┐
│                   sudachi-search                                  │
│   SearchTokenizer: B+C core                                      │
│   Mode C + Mode B → Vec<SearchToken { is_colocated }>            │
└─────────────────────────┬────────────────────────────────────────┘
                          │ workspace dep
┌─────────────────────────▼────────────────────────────────────────┐
│                     sudachi.rs (upstream)                         │
│   WorksApplications morphological analyzer, 1M+ dictionary       │
└──────────────────────────────────────────────────────────────────┘

  sudachi-sqlite  ──────── sudachi-search (separate path)
  sudachi-wasm    ──────── sudachi.rs (excluded from workspace)
```

---

## Commands Reference

```bash
just                  # List all commands
just build            # Build workspace (release)
just build-dev        # Build workspace (dev)
just check            # cargo check
just test             # Tests (no dictionary needed)
just test-verbose     # Tests with output
just fmt              # Format
just lint             # Clippy (deny warnings)
just fix              # fmt + lint
just ci               # Full CI pass

just dict-setup       # Download Sudachi dictionary
just dict-path        # Show resolved dictionary path

just pgrx-build       # Build pg_search in ~/CODE/paradedb --features icu,sudachi
just pgrx-check       # Check pg_search in ~/CODE/paradedb --features icu,sudachi

just wasm build       # Build WebAssembly (wasm-pack)
just wasm dict-setup  # Populate WASM resource files

just env              # Show environment info
just clean            # Clean build artifacts
```

---

## Split Modes Explained

Sudachi has three base analysis modes. This ecosystem adds Search:

| Mode | Granularity | Use Case |
|------|-------------|----------|
| A | Finest — splits compounds to smallest units | Character-level analysis |
| B | Medium — preserves named entities | Named entity recognition |
| C | Coarsest — longest tokens | Simple tokenization |
| **Search** | **B+C simultaneously** | **Full-text search (default)** |

Search mode is the key innovation: it emits the Mode C compound word first, then
the Mode B sub-tokens at the **same position**. This lets both exact phrase queries
AND sub-word queries match the same document.

---

## Normalization

All tokenizers default to **normalized form** for better recall:

| Surface | Normalized | Reason |
|---------|------------|--------|
| 食べた | 食べる | Verb (past → dictionary) |
| 食べている | 食べる | Verb (progressive → dictionary) |
| 美しかった | 美しい | Adjective (past → dictionary) |
| 附属病院 | 付属病院 | Variant kanji |
| ＳＵＭＭＥＲ | サマー | Fullwidth ASCII → katakana |
| パーティー | パーティ | Long vowel normalization |

This means searching for `食べる` finds documents containing `食べた`, `食べていた`,
`食べます`, etc. — and vice versa.

Surface form is available on all tokenizers when normalization is unwanted.

---

## Dependency Graph

```
sudachi-search    ──► sudachi.rs (git)
sudachi-sqlite    ──► sudachi-search (path)
sudachi-tantivy   ──► sudachi-search (path)
                  ──► sudachi.rs (git, same rev)
                  ──► tantivy-tokenizer-api
jurmarcus/paradedb ─► sudachi-tantivy (git)
```

Root workspace Cargo.toml pins `sudachi.rs` to a specific git rev so all crates
see the same dictionary types. `jurmarcus/paradedb` patches `tantivy-tokenizer-api`
via `[patch.crates-io]` to ensure its forked tantivy API is used consistently.

---

## Related

| Project | Description |
|---------|-------------|
| [sudachi.rs](https://github.com/WorksApplications/sudachi.rs) | Upstream morphological analyzer |
| [SudachiDict](https://github.com/WorksApplications/SudachiDict) | Dictionary releases |
| [jurmarcus/paradedb](https://github.com/jurmarcus/paradedb) | ParadeDB fork with Sudachi |
| [paradedb/paradedb](https://github.com/paradedb/paradedb) | Upstream ParadeDB |

---

## License

Apache-2.0
