# sudachi-sqlite

**Sudachi Japanese tokenizer for SQLite FTS5 full-text search.**

A loadable SQLite extension that registers `sudachi_tokenizer` for Japanese full-text search with B+C multi-granularity.

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
| **B+C Multi-Granularity** | Indexes compound words AND sub-tokens at same position |
| **FTS5_TOKEN_COLOCATED** | Uses SQLite's native colocated token support |
| **Normalized Forms** | 食べた → 食べる (verb conjugation) |
| **Surface Form Option** | Use original text when needed |
| **High-Quality Dictionary** | 1M+ entries from Sudachi |

---

## Installation

### Build and Install

```bash
# Clone the monorepo
git clone https://github.com/jurmarcus/sudachi.git
cd sudachi

# Setup dictionary and build (using just)
just dict-setup
just build

# Or build the extension directly
cargo build -p sudachi-sqlite --release
```

The dictionary is auto-discovered from `~/.sudachi/` - no environment variable needed.

---

## Quick Start

```sql
-- Load extension
.load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init

-- Create FTS5 table with Sudachi tokenizer
CREATE VIRTUAL TABLE documents USING fts5(
    content,
    tokenize='sudachi_tokenizer'
);

-- Insert Japanese text
INSERT INTO documents(content) VALUES
    ('東京都立大学で研究しています'),
    ('大学院で授業を受けている'),
    ('東京駅から出発します');

-- Search - finds documents 1 AND 2!
SELECT * FROM documents WHERE documents MATCH '大学';
```

---

## Split Modes

The Sudachi tokenizer uses **Search mode** (B+C) by default:

| Mode | Description | "東京都立大学" Output |
|------|-------------|----------------------|
| A | Finest granularity | ["東京", "都", "立", "大学"] |
| B | Medium granularity | ["東京", "都立", "大学"] |
| C | Coarsest granularity | ["東京都立大学"] |
| **Search** | **B+C (Default)** | ["東京都立大学", "東京"\*, "都立"\*, "大学"\*] |

\* Colocated tokens (same position as compound word)

---

## Search Mode

Search mode emits **both** compound words AND sub-tokens at the same position using `FTS5_TOKEN_COLOCATED`:

```
Input: "東京都立大学で"

FTS5 Token Emissions:
  xToken(0, "東京都立大学", ...)              → Position 0
  xToken(FTS5_TOKEN_COLOCATED, "東京", ...)   → Position 0 (same!)
  xToken(FTS5_TOKEN_COLOCATED, "都立", ...)   → Position 0 (same!)
  xToken(FTS5_TOKEN_COLOCATED, "大学", ...)   → Position 0 (same!)
  xToken(0, "で", ...)                        → Position 1
```

This enables:
- **Exact matching**: Query "東京都立大学" finds exact phrase
- **Partial matching**: Query "大学" also finds "東京都立大学"
- **Phrase queries**: Work correctly with colocated tokens

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
-- Normalized form (default, better recall)
CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer');

-- Surface form (original text)
CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer surface');
```

---

## Complete Example

```sql
-- Set environment variable before starting SQLite
-- export SUDACHI_DICT_PATH=~/.sudachi/sudachi-dictionary-20241021/system_small.dic

-- Load extension
.load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init

-- Create table with Sudachi tokenizer
CREATE VIRTUAL TABLE articles USING fts5(
    title,
    content,
    tokenize='sudachi_tokenizer'
);

-- Insert documents
INSERT INTO articles(title, content) VALUES
    ('東京の大学', '東京都立大学で日本語を研究しています'),
    ('大学院生活', '大学院で授業を受けている毎日です'),
    ('旅行記', '東京駅から京都に向かいました');

-- Search for 大学 - finds articles 1 AND 2!
SELECT title, snippet(articles, 1, '<b>', '</b>', '...', 10) as match
FROM articles
WHERE articles MATCH '大学'
ORDER BY rank;

-- Boolean search
SELECT * FROM articles WHERE articles MATCH '東京 AND 大学';
SELECT * FROM articles WHERE articles MATCH '東京 OR 京都';

-- Phrase search
SELECT * FROM articles WHERE articles MATCH '"東京都立大学"';

-- Ranked results
SELECT title, bm25(articles) as score
FROM articles
WHERE articles MATCH '大学'
ORDER BY score;
```

---

## API Reference

### Loading the Extension

```sql
-- SQLite CLI
.load /path/to/libsudachi_sqlite sudachi_fts5_tokenizer_init

-- From C code
sqlite3_load_extension(db, "/path/to/libsudachi_sqlite", "sudachi_fts5_tokenizer_init", &error);

-- From Python
conn.enable_load_extension(True)
conn.load_extension("/path/to/libsudachi_sqlite", "sudachi_fts5_tokenizer_init")
```

### Tokenizer Options

| Syntax | Description |
|--------|-------------|
| `tokenize='sudachi_tokenizer'` | Normalized form (default) |
| `tokenize='sudachi_tokenizer surface'` | Surface form (original text) |

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `SUDACHI_DICT_PATH` | **Yes** | Path to Sudachi dictionary file |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     SQLite FTS5                                 │
│  .load libsudachi_sqlite sudachi_fts5_tokenizer_init           │
└────────────────────────────┬────────────────────────────────────┘
                             │ calls entry point
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│              sudachi_fts5_tokenizer_init                       │
│  - Retrieves FTS5 API function pointers                        │
│  - Registers 'sudachi_tokenizer' tokenizer                     │
│  - Sets up xCreate, xDelete, xTokenize callbacks               │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Fts5Tokenizer                               │
│  - Wraps sudachi-search SearchTokenizer                        │
│  - Loads dictionary from SUDACHI_DICT_PATH                     │
│  - is_colocated → FTS5_TOKEN_COLOCATED flag                   │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    sudachi-search                              │
│               (B+C multi-granularity core)                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Integrations with SQLite

### Python

```python
import sqlite3

conn = sqlite3.connect(':memory:')
conn.enable_load_extension(True)
conn.load_extension('./target/release/libsudachi_sqlite', 'sudachi_fts5_tokenizer_init')

conn.execute('''
    CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer')
''')
conn.execute("INSERT INTO docs VALUES ('東京都立大学で研究')")

for row in conn.execute("SELECT * FROM docs WHERE docs MATCH '大学'"):
    print(row)
```

### Node.js (better-sqlite3)

```javascript
const Database = require('better-sqlite3');
const db = new Database(':memory:');

db.loadExtension('./target/release/libsudachi_sqlite', 'sudachi_fts5_tokenizer_init');

db.exec(`CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer')`);
db.exec(`INSERT INTO docs VALUES ('東京都立大学で研究')`);

const rows = db.prepare("SELECT * FROM docs WHERE docs MATCH '大学'").all();
console.log(rows);
```

---

## Related Projects

| Project | Description |
|---------|-------------|
| [`sudachi-search`](../sudachi-search/) | B+C core (this monorepo) |
| [`sudachi-tantivy`](../sudachi-tantivy/) | Tantivy integration (this monorepo) |
| [jurmarcus/paradedb](https://github.com/jurmarcus/paradedb) | PostgreSQL (ParadeDB fork) |
| [`sudachi-wasm`](../sudachi-wasm/) | WebAssembly (this monorepo) |
| [sudachi.rs](https://github.com/WorksApplications/sudachi.rs) | Upstream analyzer |

---

## License

Apache-2.0 (same as Sudachi)
