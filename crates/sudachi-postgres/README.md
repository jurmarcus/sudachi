# sudachi-postgres

**[ParadeDB](https://github.com/paradedb/paradedb) with Sudachi Japanese tokenizer support.**

This repository adds the `pdb.sudachi` tokenizer type to ParadeDB's pg_search extension, enabling **B+C multi-granularity** Japanese full-text search in PostgreSQL.

For general ParadeDB documentation, deployment, and usage, see the [official ParadeDB docs](https://docs.paradedb.com/).

For Sudachi-specific documentation, see **[SUDACHI.md](./SUDACHI.md)**.

---

## Why Sudachi?

Three problems break Japanese full-text search. Sudachi solves all of them:

### 1. Compound Words

```
Document: "東京都立大学で研究しています"
Query: "大学"

Traditional tokenizer: ❌ NO MATCH
  → "大学" is trapped inside "東京都立大学"

Sudachi (B+C mode): ✅ MATCH
  → Indexes both "東京都立大学" AND "大学" at same position
```

### 2. Verb Conjugations & Adjective Inflections

Both documents AND queries are normalized to the dictionary form:

```
Documents contain:           Normalized to:
  "食べた" (ate)        →    食べる
  "食べている" (eating)  →    食べる
  "食べます" (will eat)  →    食べる

Query "食べる" → matches all 3 documents  ✅
Query "食べた" → ALSO matches all 3!      ✅  (query is normalized too)
```

This means users can search in **any conjugated form** and find all related documents.

### 3. Character Normalization

```
Documents with variant forms:
  - "ＳＵＭＭＥＲ"  (fullwidth)
  - "附属病院"     (variant kanji 附)
  - "パーティー"   (long vowel mark)

Sudachi normalizes:
  - ＳＵＭＭＥＲ → サマー
  - 附属 → 付属
  - パーティー → パーティ
```

### The Solution: B+C Multi-Granularity

Sudachi's Search mode indexes text at **multiple granularities simultaneously**:

```
Input: "東京都立大学"

Indexed tokens (all at position 0):
  "東京都立大学"  (compound word)
  "東京"          (sub-token)
  "都立"          (sub-token)
  "大学"          (sub-token)
```

Now searches for "大学", "東京", or "東京都立大学" all match.

---

## Quick Start

### Prerequisites

```bash
# Install PostgreSQL 17
brew install postgresql@17
```

### Setup

```bash
# Full setup from scratch (dictionary + deps + build + install)
just setup

# Or step by step:
just dict-setup     # Download dictionary (one-time)
just install-deps   # Install cargo-pgrx
just build          # Build pg_search with Sudachi
just install        # Install extension to PostgreSQL
```

### Run PostgreSQL

```bash
just pg-start       # Start PostgreSQL
just pg-stop        # Stop PostgreSQL
```

### Usage

```sql
-- Load extension
CREATE EXTENSION pg_search;

-- Create table
CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    content TEXT
);

INSERT INTO documents (content) VALUES
    ('東京都立大学で研究しています'),
    ('大学院の授業に出席した'),
    ('東京駅から出発します');

-- Create BM25 index with Sudachi tokenizer
CREATE INDEX docs_idx ON documents
USING bm25(id, (content::pdb.sudachi))
WITH (key_field='id');

-- Search - finds documents 1 and 2!
SELECT id, content, paradedb.score(id) as score
FROM documents
WHERE id @@@ 'content:大学'
ORDER BY score DESC;
```

---

## Features

| Feature | Description |
|---------|-------------|
| **B+C Multi-Granularity** | Compound words AND sub-tokens at same position |
| **Conjugation Normalization** | 食べた → 食べる (ate → eat) |
| **Adjective Inflection** | 美しかった → 美しい (was beautiful → beautiful) |
| **High-Quality Dictionary** | 1M+ entries vs ~400K in Lindera |

---

## Commands

```bash
# Setup
just dict-setup       # Download Sudachi dictionary (one-time)
just install-deps     # Install cargo-pgrx

# Build & Install
just build            # Build pg_search with Sudachi feature
just install          # Install extension to PostgreSQL

# PostgreSQL
just pg-start         # Start PostgreSQL
just pg-stop          # Stop PostgreSQL

# Testing
just test             # Run Rust tests
just test-sql         # Interactive SQL test session

# Development
just fix              # Format and lint code
just env              # Show environment info

# Full workflow
just setup            # Complete setup from scratch
```

All commands use `just` (task runner). Run `just --list` to see all available commands.

The dictionary is auto-discovered from `~/.sudachi/` - no environment variable needed.

---

## Architecture

```
sudachi-postgres (this repo)
        │
        ▼
sudachi-tantivy ─────► Tantivy tokenizer adapter
        │
        ▼
sudachi-search ──────► B+C multi-granularity core
        │
        ▼
sudachi.rs ──────────► Morphological analyzer (WAP)
```

---

## Related Projects

| Project | Description |
|---------|-------------|
| [ParadeDB](https://github.com/paradedb/paradedb) | Base project - PostgreSQL full-text search |
| [sudachi-search](https://github.com/jurmarcus/sudachi-search) | B+C multi-granularity tokenization |
| [sudachi-tantivy](https://github.com/jurmarcus/sudachi-tantivy) | Tantivy tokenizer adapter |
| [sudachi.rs](https://github.com/WorksApplications/sudachi.rs) | Upstream morphological analyzer |

---

## License

Same as ParadeDB (AGPL-3.0).
