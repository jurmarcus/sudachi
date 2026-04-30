# sudachi-sqlite

**Sudachi Japanese tokenizer for SQLite FTS5.**

A loadable extension (cdylib) that registers `sudachi_tokenizer` for SQLite FTS5 with B+C multi-granularity. Compound words and their sub-tokens get indexed at the same FTS5 position via `FTS5_TOKEN_COLOCATED`, so a query for `大学` matches a document containing `東京都立大学`.

---

## Why

Default FTS5 tokenizers split Japanese on whitespace, which leaves compound words intact and unsearchable:

```text
Document: 東京都立大学で研究
Query:    大学

Default FTS5:                ["東京都立大学で研究"]   → no match
This extension (Search mode):
  pos 0  東京都立大学   flag: 0
  pos 0  東京           flag: FTS5_TOKEN_COLOCATED
  pos 0  都立           flag: FTS5_TOKEN_COLOCATED
  pos 0  大学           flag: FTS5_TOKEN_COLOCATED   → match
  pos 1  で
  pos 2  研究
```

It also normalises:

| Surface       | Normalised   | Type                |
| ------------- | ------------ | ------------------- |
| 食べた        | 食べる       | Verb conjugation    |
| 美しかった    | 美しい       | i-adjective         |
| 附属          | 付属         | Variant kanji       |
| ＳＵＭＭＥＲ   | サマー       | Fullwidth ASCII     |

Both indexed text and queries get normalised, so a query in any conjugation matches all related documents.

---

## Install

```bash
git clone https://github.com/jurmarcus/sudachi.git
cd sudachi
just dict-setup           # downloads dictionary to ~/.sudachi/
just build                # builds the extension at target/release/libsudachi_sqlite.{dylib,so}
```

Or directly:

```bash
cargo build -p sudachi-sqlite --release
```

The compiled artefact is `target/release/libsudachi_sqlite.{dylib,so,dll}`.

---

## Quick start

```sql
.load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init

CREATE VIRTUAL TABLE docs USING fts5(
    content,
    tokenize='sudachi_tokenizer'
);

INSERT INTO docs VALUES
    ('東京都立大学で研究しています'),
    ('大学院で授業を受けている'),
    ('東京駅から出発します');

SELECT * FROM docs WHERE docs MATCH '大学';
-- returns rows 1 and 2
```

`SUDACHI_DICT_PATH` must be set when SQLite loads the extension (or the dictionary must live at `~/.sudachi/system_full.dic`).

---

## Tokenizer options

| Syntax                                    | Meaning                              |
| ----------------------------------------- | ------------------------------------ |
| `tokenize='sudachi_tokenizer'`            | Normalised form (default)            |
| `tokenize='sudachi_tokenizer surface'`    | Surface form (raw input text)        |

---

## Search modes

The extension uses Search mode (B+C) — the default and only mode exposed via SQL today. The underlying `sudachi-search` crate also supports modes A/B/C separately if you need them at the Rust level.

| Mode      | Granularity | "東京都立大学" output                                  |
| --------- | ----------- | ------------------------------------------------------ |
| A         | Finest      | `["東京", "都", "立", "大学"]`                         |
| B         | Medium      | `["東京", "都立", "大学"]`                             |
| C         | Coarsest    | `["東京都立大学"]`                                     |
| **Search**| **B+C**     | `["東京都立大学", "東京"*, "都立"*, "大学"*]` *= colocated |

---

## Examples

### Search ranking

```sql
SELECT title, bm25(articles) AS score
FROM articles
WHERE articles MATCH '大学'
ORDER BY score;
```

### Snippets

```sql
SELECT title, snippet(articles, 1, '<b>', '</b>', '...', 10) AS hit
FROM articles
WHERE articles MATCH '東京';
```

### Boolean and phrase queries

```sql
SELECT * FROM articles WHERE articles MATCH '東京 AND 大学';
SELECT * FROM articles WHERE articles MATCH '東京 OR 京都';
SELECT * FROM articles WHERE articles MATCH '"東京都立大学"';
```

---

## Loading the extension

### sqlite3 CLI

```bash
sqlite3
sqlite> .load /path/to/libsudachi_sqlite sudachi_fts5_tokenizer_init
```

### From C

```c
sqlite3_load_extension(db, "/path/to/libsudachi_sqlite",
                       "sudachi_fts5_tokenizer_init", &errmsg);
```

### From Python

```python
import sqlite3
conn = sqlite3.connect(':memory:')
conn.enable_load_extension(True)
conn.load_extension('./target/release/libsudachi_sqlite',
                    'sudachi_fts5_tokenizer_init')

conn.execute("CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer')")
conn.execute("INSERT INTO docs VALUES ('東京都立大学で研究')")
print(list(conn.execute("SELECT * FROM docs WHERE docs MATCH '大学'")))
```

### From Node.js (better-sqlite3)

```javascript
const Database = require('better-sqlite3');
const db = new Database(':memory:');
db.loadExtension('./target/release/libsudachi_sqlite',
                 'sudachi_fts5_tokenizer_init');
db.exec(`CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer')`);
db.exec(`INSERT INTO docs VALUES ('東京都立大学で研究')`);
console.log(db.prepare("SELECT * FROM docs WHERE docs MATCH '大学'").all());
```

---

## Environment

| Variable            | Required | Default                                   |
| ------------------- | -------- | ----------------------------------------- |
| `SUDACHI_DICT_PATH` | Yes      | (the extension errors out if it's missing)|

The dictionary is loaded once per `CREATE VIRTUAL TABLE` (in the FTS5 `xCreate` callback) and cached on the `Fts5Tokenizer` struct.

---

## Architecture

```
SQLite FTS5
    │ .load libsudachi_sqlite sudachi_fts5_tokenizer_init
    ▼
sudachi_fts5_tokenizer_init   (cdylib entry point)
    │ retrieves fts5_api function pointers
    │ registers xCreate / xDelete / xTokenize callbacks
    ▼
Fts5Tokenizer                 (heap-allocated, owned by FTS5)
    │ holds SearchTokenizer + use_surface_form flag
    │ ALL FFI callbacks wrapped in ffi_panic_boundary
    ▼
sudachi-search                (B+C tokenizer)
    │
    ▼
sudachi_optimizer::sudachi    (the single Sudachi gateway)
```

---

## License

Apache-2.0
