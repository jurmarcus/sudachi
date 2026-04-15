# Sudachi for ParadeDB

**Sudachi Japanese tokenizer for PostgreSQL full-text search.**

This is [ParadeDB](https://github.com/paradedb/paradedb) with Sudachi support added, providing `pdb.sudachi` tokenizer for Japanese full-text search with B+C multi-granularity.

For general ParadeDB documentation, see the [official ParadeDB docs](https://docs.paradedb.com/).

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

## Features

| Feature | Description |
|---------|-------------|
| **B+C Multi-Granularity** | Search mode emits compound AND sub-tokens at same position |
| **Four Split Modes** | A (finest), B (medium), C (coarsest), Search (B+C) |
| **Normalized Forms** | 食べた → 食べる (verb conjugation) |
| **BM25 Ranking** | Full ParadeDB BM25 search support |
| **High-Quality Dictionary** | 1M+ entries from Sudachi |

---

## Installation

### Prerequisites

```bash
# PostgreSQL 17
brew install postgresql@17

# Install pgrx
cargo install cargo-pgrx@0.16.1
cargo pgrx init --pg17=$(brew --prefix postgresql@17)/bin/pg_config
```

### Dictionary Setup

```bash
# Download Sudachi dictionary
mkdir -p ~/.sudachi
curl -L https://github.com/WorksApplications/SudachiDict/releases/download/v20241021/sudachi-dictionary-20241021-small.zip -o /tmp/sudachi-dict.zip
unzip /tmp/sudachi-dict.zip -d ~/.sudachi/

# Set environment variable (REQUIRED)
export SUDACHI_DICT_PATH=~/.sudachi/sudachi-dictionary-20241021/system_small.dic
```

### Build

```bash
cd pg_search
SUDACHI_DICT_PATH=~/.sudachi/sudachi-dictionary-20241021/system_small.dic \
RUSTFLAGS="-Clink-arg=-Wl,-undefined,dynamic_lookup" \
cargo pgrx install --no-default-features --features sudachi,pg17
```

### Run PostgreSQL

**Important**: PostgreSQL must have `SUDACHI_DICT_PATH` set:

```bash
SUDACHI_DICT_PATH=~/.sudachi/sudachi-dictionary-20241021/system_small.dic \
pg_ctl -D /opt/homebrew/var/postgresql@17 start
```

---

## Quick Start

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

-- Search - finds documents 1 AND 2!
SELECT id, content, paradedb.score(id) as score
FROM documents
WHERE id @@@ 'content:大学'
ORDER BY score DESC;
```

---

## Split Modes

| Mode | Description | "東京都立大学" Output |
|------|-------------|----------------------|
| `a` | Finest granularity | ["東京", "都", "立", "大学"] |
| `b` | Medium granularity | ["東京", "都立", "大学"] |
| `c` | Coarsest granularity | ["東京都立大学"] |
| **`search`** | **B+C (Default)** | ["東京都立大学", "東京"\*, "都立"\*, "大学"\*] |

\* Colocated tokens (same position as compound word)

```sql
-- Default: Search mode (B+C)
content::pdb.sudachi

-- Explicit mode
content::pdb.sudachi('search')
content::pdb.sudachi('c')
content::pdb.sudachi('a')
```

---

## Search Mode

Search mode emits **both** compound words AND sub-tokens at the same position:

```sql
-- With Search mode, all these queries match "東京都立大学で研究":
SELECT * FROM documents WHERE id @@@ 'content:東京都立大学';  -- Exact compound
SELECT * FROM documents WHERE id @@@ 'content:大学';          -- Sub-token
SELECT * FROM documents WHERE id @@@ 'content:東京';          -- Sub-token
SELECT * FROM documents WHERE id @@@ 'content:都立';          -- Sub-token
```

This enables:
- **Exact matching**: Query "東京都立大学" finds exact phrase
- **Partial matching**: Query "大学" also finds "東京都立大学"
- **BM25 ranking**: All matches properly ranked

---

## Normalization

Normalized form (default) improves recall by matching conjugated forms:

| Surface | Normalized | Type |
|---------|------------|------|
| 食べた | 食べる | Verb conjugation |
| 美しかった | 美しい | Adjective inflection |
| 附属 | 付属 | Variant kanji |
| ＳＵＭＭＥＲ | サマー | Fullwidth conversion |

```sql
-- Search for base form finds all conjugations
INSERT INTO documents (content) VALUES
    ('昨日ご飯を食べた'),
    ('今ご飯を食べている'),
    ('明日ご飯を食べます');

-- Finds all 3 documents!
SELECT * FROM documents WHERE id @@@ 'content:食べる';
```

---

## Complete Example

```sql
-- Create extension
CREATE EXTENSION pg_search;

-- Create table
CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    title TEXT,
    content TEXT
);

-- Insert documents
INSERT INTO articles (title, content) VALUES
    ('東京の大学', '東京都立大学で日本語を研究しています'),
    ('大学院生活', '大学院で授業を受けている毎日です'),
    ('旅行記', '東京駅から京都に向かいました');

-- Create BM25 index with Sudachi
CREATE INDEX articles_idx ON articles
USING bm25(id, title, (content::pdb.sudachi))
WITH (key_field='id');

-- Search for 大学 - finds articles 1 AND 2!
SELECT id, title, paradedb.score(id) as score
FROM articles
WHERE id @@@ 'content:大学'
ORDER BY score DESC;

-- Boolean search
SELECT * FROM articles WHERE id @@@ 'content:東京 AND content:大学';

-- Multi-field search
SELECT * FROM articles WHERE id @@@ 'title:東京 OR content:東京';
```

---

## API Reference

### Tokenizer Syntax

```sql
-- Default: Search mode (B+C), normalized
column::pdb.sudachi

-- With explicit mode
column::pdb.sudachi('search')  -- B+C (default)
column::pdb.sudachi('a')       -- Finest
column::pdb.sudachi('b')       -- Medium
column::pdb.sudachi('c')       -- Coarsest
```

### Creating Indexes

```sql
-- Single field with Sudachi
CREATE INDEX idx ON table
USING bm25(id, (content::pdb.sudachi))
WITH (key_field='id');

-- Multiple fields
CREATE INDEX idx ON table
USING bm25(id, title, (content::pdb.sudachi), (body::pdb.sudachi))
WITH (key_field='id');
```

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `SUDACHI_DICT_PATH` | **Yes** | Path to Sudachi dictionary file |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      sudachi-postgres                            │
│              (ParadeDB + Sudachi tokenizer)                     │
│                                                                  │
│  pdb.sudachi type → SearchTokenizer::Sudachi                    │
└──────────────────────────────┬──────────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────────┐
│                      sudachi-tantivy                             │
│                  (Tantivy tokenizer adapter)                    │
│                                                                  │
│  SplitMode::Search → colocated token positions                  │
└──────────────────────────────┬──────────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────────┐
│                       sudachi-search                             │
│                 (B+C multi-granularity core)                    │
│                                                                  │
│  Mode C + Mode B → SearchToken { is_colocated }                 │
└──────────────────────────────┬──────────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────────┐
│                        sudachi.rs                                │
│              (Morphological analyzer by WAP)                    │
└─────────────────────────────────────────────────────────────────┘
```

---

## Comparison: Sudachi vs Lindera

ParadeDB includes Lindera for Japanese. Here's why you might prefer Sudachi:

| Feature | Sudachi | Lindera |
|---------|---------|---------|
| B+C multi-granularity | ✅ Yes | ❌ No |
| Compound word partial matching | ✅ Yes | ❌ No |
| Normalized forms | ✅ Yes | ❌ No |
| Dictionary entries | 1M+ | ~400K |
| Memory usage | ~70MB | ~30MB |

**Recommendation**: Use Sudachi for Japanese search quality. Use Lindera for lower memory footprint.

---

## Related Projects

| Project | Description |
|---------|-------------|
| [ParadeDB](https://github.com/paradedb/paradedb) | Base project - PostgreSQL full-text search |
| [sudachi-search](https://github.com/jurmarcus/sudachi-search) | B+C multi-granularity core |
| [sudachi-tantivy](https://github.com/jurmarcus/sudachi-tantivy) | Tantivy tokenizer adapter |
| [sudachi-sqlite](https://github.com/jurmarcus/sudachi-sqlite) | SQLite FTS5 |
| [sudachi.rs](https://github.com/WorksApplications/sudachi.rs) | Upstream morphological analyzer |

---

## License

Same as ParadeDB (AGPL-3.0).
