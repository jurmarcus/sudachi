# CLAUDE.md - sudachi-postgres

A ParadeDB fork with Sudachi Japanese tokenizer integration for PostgreSQL full-text search.

## Why This Exists

Japanese full-text search has a fundamental problem: **compound words**.

```
Document: "東京都立大学で研究しています"
         (I'm doing research at Tokyo Metropolitan University)

Traditional tokenizer (Mode C): ["東京都立大学", "で", "研究", "し", "て", "い", "ます"]
Search query: "大学"

Result: NO MATCH - "大学" is trapped inside "東京都立大学"
```

This fork adds **Sudachi with B+C multi-granularity** - the gold standard for Japanese search:

```
Sudachi Search mode:
  pos 0: "東京都立大学" (compound)
  pos 0: "東京" (sub-token)     ← SAME position!
  pos 0: "都立" (sub-token)     ← SAME position!
  pos 0: "大学" (sub-token)     ← SAME position!
  pos 1: "で"
  pos 2: "研究"
  ...

Search query: "大学"
Result: MATCH! Both exact and partial matching work.
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           sudachi-postgres                                   │
│                      (ParadeDB fork + Sudachi)                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  pg_search/src/api/tokenizers/                                              │
│  ├── definitions.rs    ← define_tokenizer_type!(Sudachi, ...)              │
│  ├── mod.rs            ← SearchTokenizer::Sudachi case handling            │
│  └── typmod/           ← SudachiTypmod (mode, normalized options)          │
│                                                                              │
│  tokenizers/src/                                                            │
│  ├── sudachi.rs        ← SudachiTokenizer wrapper                          │
│  └── manager.rs        ← SearchTokenizer::Sudachi variant                  │
│                                                                              │
└────────────────────────────────────────┬────────────────────────────────────┘
                                         │
┌────────────────────────────────────────▼────────────────────────────────────┐
│                         sudachi-tantivy                                      │
│                    (Tantivy tokenizer adapter)                              │
│                                                                              │
│  SplitMode::Search → uses SearchTokenizer from sudachi-search              │
│  is_colocated: true → position stays same for sub-tokens                   │
└────────────────────────────────────────┬────────────────────────────────────┘
                                         │
┌────────────────────────────────────────▼────────────────────────────────────┐
│                          sudachi-search                                      │
│                   (B+C multi-granularity core)                              │
│                                                                              │
│  1. Tokenize with Mode C (compounds)                                        │
│  2. Tokenize with Mode B (sub-tokens)                                       │
│  3. Emit both, marking sub-tokens as is_colocated: true                    │
└────────────────────────────────────────┬────────────────────────────────────┘
                                         │
┌────────────────────────────────────────▼────────────────────────────────────┐
│                            sudachi.rs                                        │
│               (WorksApplications morphological analyzer)                    │
│                                                                              │
│  - High-quality dictionary with 1M+ entries                                │
│  - Three base modes: A (finest), B (medium), C (coarsest)                  │
│  - Normalized forms for better recall                                       │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Commands

```bash
# Setup dictionary (one-time)
just dict-setup

# Install prerequisites
just install-deps

# Build pg_search with Sudachi
just build

# Install extension to PostgreSQL
just install

# Start PostgreSQL
just pg-start

# Run tests
just test

# Interactive SQL test
just test-sql

# Full setup from scratch
just setup

# Show environment
just env
```

All commands use `just` (task runner). Run `just --list` to see all available commands.

The dictionary is auto-discovered from `~/.sudachi/` - no environment variable needed.

### Prerequisites

```bash
# Install PostgreSQL 17
brew install postgresql@17

# Install cargo-pgrx and initialize
just install-deps
```

### Test

```sql
-- Create extension
CREATE EXTENSION pg_search;

-- Create test table
CREATE TABLE documents (id SERIAL PRIMARY KEY, content TEXT);
INSERT INTO documents (content) VALUES
    ('東京都立大学は東京にある公立大学です'),
    ('大学院で研究しています'),
    ('私は大学で日本語を勉強しました');

-- Create index with Sudachi tokenizer
CREATE INDEX docs_idx ON documents
USING bm25(id, (content::pdb.sudachi))
WITH (key_field='id');

-- Search for 大学 - finds ALL documents!
SELECT * FROM documents WHERE id @@@ 'content:大学';
```

## SQL Interface

### Basic Usage

```sql
-- Default: Search mode (B+C), normalized=true
CREATE INDEX idx ON tbl USING bm25(id, (content::pdb.sudachi)) WITH (key_field='id');

-- With explicit mode
CREATE INDEX idx ON tbl USING bm25(id, (content::pdb.sudachi('search'))) WITH (key_field='id');
CREATE INDEX idx ON tbl USING bm25(id, (content::pdb.sudachi('c'))) WITH (key_field='id');

-- Modes: 'a' (finest), 'b' (medium), 'c' (coarsest), 'search' (B+C, default)
```

### Features Demonstrated

| Feature | Query | Matches |
|---------|-------|---------|
| Compound matching | `大学` | 東京都立大学, 大学院, 大学 |
| Exact phrase | `東京都立大学` | Only 東京都立大学 |
| Conjugation normalization | `食べる` | 食べた, 食べます, 食べている |
| Adjective inflection | `美しい` | 美しく, 美しかった, 美しさ |

## Key Files

### pg_search Integration

| File | Purpose |
|------|---------|
| `pg_search/Cargo.toml` | `sudachi` feature flag |
| `pg_search/src/api/tokenizers/definitions.rs` | `define_tokenizer_type!(Sudachi, ...)` |
| `pg_search/src/api/tokenizers/mod.rs` | `SearchTokenizer::Sudachi` handling |
| `pg_search/src/api/tokenizers/typmod/definitions.rs` | `SudachiTypmod` struct |

### tokenizers Crate

| File | Purpose |
|------|---------|
| `tokenizers/Cargo.toml` | `sudachi` feature, sudachi-tantivy dep |
| `tokenizers/src/sudachi.rs` | `SudachiTokenizer`, `SudachiMode` |
| `tokenizers/src/manager.rs` | `SearchTokenizer::Sudachi` variant |

## Debugging

### Check Tokenizer is Available

```sql
SELECT typname FROM pg_type
WHERE typnamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'pdb')
AND typname = 'sudachi';
```

### Dictionary Not Found

If searches return empty results, ensure `SUDACHI_DICT_PATH` is set for the PostgreSQL server process:

```bash
# Restart with dictionary path
SUDACHI_DICT_PATH=/path/to/system.dic pg_ctl -D /path/to/data restart
```

### Test Tokenization Manually

```bash
# In Rust test
SUDACHI_DICT_PATH=/path/to/system.dic cargo test --features sudachi -p tokenizers
```

## Dependencies

```toml
# tokenizers/Cargo.toml
[dependencies]
sudachi-tantivy = { path = "../sudachi-tantivy", optional = true }

[features]
sudachi = ["sudachi-tantivy"]
```

## Comparison with Lindera

ParadeDB includes Lindera for Japanese tokenization. Here's why Sudachi is better for search:

| Feature | Sudachi | Lindera |
|---------|---------|---------|
| B+C multi-granularity | Yes (Search mode) | No |
| Compound detection | Superior (dictionary-based) | Basic |
| Normalized forms | Yes | No |
| Conjugation handling | Better | Good |
| Dictionary quality | 1M+ entries | 400K entries |
| Memory usage | Higher (~70MB) | Lower (~30MB) |

**Recommendation**: Use Sudachi for Japanese full-text search. Use Lindera for simpler use cases or lower memory environments.

## Related Crates

| Crate | Purpose | Location |
|-------|---------|----------|
| sudachi-search | B+C multi-granularity core | ~/CODE/sudachi-search |
| sudachi-tantivy | Tantivy adapter | ~/CODE/sudachi-tantivy |
| sudachi.rs | Upstream morphological analyzer | GitHub (develop branch) |
