# CLAUDE.md — docker/postgres

Docker infrastructure for running ParadeDB with the Sudachi Japanese tokenizer.

## What this is

This directory contains **only Docker infrastructure** — no Rust source. The pgrx Rust extension (`pg_search`) it builds lives in a separate repo at `~/CODE/paradedb` (`jurmarcus/paradedb`).

The image clones paradedb at build time, builds `pg_search` with the `sudachi` cargo feature enabled, downloads the Sudachi dictionary, and produces a Postgres image where `CREATE EXTENSION pg_search` and `pdb.sudachi` casts work out of the box.

## Files

```
docker/postgres/
├── Dockerfile              Clones jurmarcus/paradedb, builds pg_search --features icu,sudachi
├── bootstrap.sh            Postgres init script: CREATE EXTENSION pg_search, set search_path
├── docker-compose.yml      Production compose
├── docker-compose.dev.yml  Dev compose
├── Dockerfile.config       Config image
├── manifests/              Kubernetes manifests
└── pg_search--0.20.6.sql   Pre-generated SQL schema (workaround for a pgrx package UTF-8 bug)
```

## Dockerfile design

The Dockerfile clones `jurmarcus/paradedb` from GitHub rather than copying local source:

```dockerfile
RUN git clone --depth 1 https://github.com/jurmarcus/paradedb /workspace
```

This sidesteps the sibling-COPY hacks needed when the workspace used path deps for `sudachi-tantivy`. Cargo resolves the git dep automatically.

Build context: `docker/postgres/`.

## Build

```bash
cd docker/postgres
docker build \
  --build-arg PG_SEARCH_FEATURES="icu,sudachi" \
  -t paradedb-sudachi .
```

With `sudachi` enabled, the image will:

1. Clone `jurmarcus/paradedb`
2. Download the Sudachi dictionary
3. Build `pg_search --features icu,sudachi` via `cargo pgrx package`
4. Install `.so` + `.control` + the pre-generated `.sql`
5. Set `SUDACHI_DICT_PATH` for the runtime postgres process

## Run

```bash
cd docker/postgres
docker compose up                           # production compose
docker compose -f docker-compose.dev.yml up # dev compose
```

## Build pg_search locally (no Docker)

```bash
just pgrx-build   # cd ~/CODE/paradedb && cargo pgrx build -p pg_search --features icu,sudachi
just pgrx-check   # cargo check, same target
```

Requires `cargo install cargo-pgrx && cargo pgrx init` first.

## SQL surface

```sql
CREATE EXTENSION pg_search;

-- Create a BM25 index using the Sudachi tokenizer
CREATE INDEX docs_idx ON documents
USING bm25(id, (content::pdb.sudachi))
WITH (key_field='id');

-- Search — finds compound words AND sub-tokens
SELECT * FROM documents WHERE id @@@ 'content:大学';

-- Mode selection via cast argument
content::pdb.sudachi              -- Search (B+C, default)
content::pdb.sudachi('search')    -- explicit
content::pdb.sudachi('c')         -- Mode C (longest tokens)
content::pdb.sudachi('a')         -- Mode A (finest)
```

## Environment

| Variable             | Required                      | Purpose                              |
| -------------------- | ----------------------------- | ------------------------------------ |
| `SUDACHI_DICT_PATH`  | Yes (in the postgres process) | Absolute path to `system_full.dic`   |

The Dockerfile sets `SUDACHI_DICT_PATH=/opt/sudachi/sudachi-dictionary-20251022/system_full.dic` in the runtime image.

## Key dependency chain

```
docker/postgres/Dockerfile
  └── git clone jurmarcus/paradedb
        └── pg_search (pgrx extension)
              └── tokenizers --features sudachi
                    └── sudachi-tantivy = { git = "https://github.com/jurmarcus/sudachi" }
                          ├── sudachi-search (path dep in this monorepo)
                          └── sudachi-optimizer (the Sudachi gateway)
                                └── sudachi.rs (upstream morphological analyzer, pinned rev)
```

ParadeDB's workspace `[patch.crates-io]` redirects `tantivy-tokenizer-api` to its forked tantivy, so types unify across the crate boundary.

## Updating ParadeDB upstream

```bash
cd ~/CODE/paradedb
gh repo sync --source paradedb/paradedb     # pull upstream changes
# resolve any conflicts in tokenizers/src/manager.rs manually
sl commit -m "chore: sync upstream paradedb"
sl push --to main
```

The Dockerfile always clones `--depth 1` from main, so a fresh `sl push --to main` is all that's needed for the next image build to pick up the change.

## Pre-generated SQL schema

`pg_search--0.20.6.sql` is committed as a workaround for a pgrx package UTF-8 bug that occasionally corrupts the schema during `cargo pgrx package`. The Dockerfile installs this file directly instead of regenerating, ensuring deterministic schema across builds. When pgrx fixes the bug upstream, this file can be regenerated and committed.

## Common issues

| Symptom                                 | Cause                                                      | Fix                                                |
| --------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------- |
| `CREATE EXTENSION pg_search` fails      | `.control` or `.sql` missing in install dir                | Verify Dockerfile `COPY` lines for the build output |
| Tokenizer cast `pdb.sudachi` not found  | pg_search built without `--features sudachi`               | Rebuild with `--build-arg PG_SEARCH_FEATURES="icu,sudachi"` |
| Empty search results                    | `SUDACHI_DICT_PATH` not set in postgres process            | Inspect runtime env; check Dockerfile ENV line     |
| pgrx panic in postgres logs             | Sudachi panic crossed FFI                                  | Check pg_search's tokenizer wrapper for `catch_unwind` |
| Image build hangs on Sudachi rev fetch  | Network issue or rev moved                                  | Re-pin `sudachi.rs` rev in this monorepo's `Cargo.toml` |
