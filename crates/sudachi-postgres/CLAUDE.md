# CLAUDE.md — sudachi-postgres

Docker infrastructure for running ParadeDB with the Sudachi Japanese tokenizer.

## What This Is

This directory contains ONLY Docker infrastructure. The Rust source lives at:

- **`~/CODE/paradedb`** — `jurmarcus/paradedb` fork with Sudachi integration

## Structure

```
docker/
├── Dockerfile             Clones jurmarcus/paradedb from GitHub, builds pg_search
├── bootstrap.sh           Postgres init: CREATE EXTENSION pg_search, set search_path
├── docker-compose.yml     Production compose
├── docker-compose.dev.yml Dev compose
├── Dockerfile.config      Config image
├── manifests/             Kubernetes manifests
└── pg_search--0.20.6.sql  Pre-generated SQL schema (bypasses pgrx package UTF-8 bug)
```

## Dockerfile Design

The Dockerfile clones `jurmarcus/paradedb` from GitHub rather than copying local source.
This eliminates sibling-COPY hacks that were needed when tokenizers used path deps.

```dockerfile
# Clean: git clone from GitHub
RUN git clone --depth 1 https://github.com/jurmarcus/paradedb /workspace

# No more COPY sudachi-tantivy/ or COPY sudachi-search/ tricks needed
# Cargo resolves the git dep automatically
```

Build context: `crates/sudachi-postgres/docker/`

## Commands

```bash
# From monorepo root — builds pg_search locally
just pgrx-build   # cd ~/CODE/paradedb && cargo pgrx build -p pg_search --features icu,sudachi
just pgrx-check   # cd ~/CODE/paradedb && cargo check -p pg_search --features icu,sudachi

# Docker
cd crates/sudachi-postgres/docker
docker compose up
```

## Docker Build

```bash
cd crates/sudachi-postgres/docker
docker build \
  --build-arg PG_SEARCH_FEATURES="icu,sudachi" \
  -t paradedb-sudachi .
```

With Sudachi enabled, the Dockerfile will:
1. Clone `jurmarcus/paradedb`
2. Download the Sudachi dictionary
3. Build `pg_search --features icu,sudachi`
4. Install `.so` + `.control` + pre-generated `.sql`
5. Set `SUDACHI_DICT_PATH` for runtime

## SQL Interface

```sql
-- Create BM25 index with Sudachi
CREATE INDEX docs_idx ON documents
USING bm25(id, (content::pdb.sudachi))
WITH (key_field='id');

-- Search mode (B+C, default)
SELECT * FROM documents WHERE id @@@ 'content:大学';

-- Explicit modes
content::pdb.sudachi          -- Search (default)
content::pdb.sudachi('search') -- explicit
content::pdb.sudachi('c')     -- Mode C (longest)
content::pdb.sudachi('a')     -- Mode A (finest)
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `SUDACHI_DICT_PATH` | Yes (Postgres process) | Path to `system_full.dic` |

The Dockerfile sets `SUDACHI_DICT_PATH=/opt/sudachi/sudachi-dictionary-20251022/system_full.dic`
in the runtime image.

## Updating ParadeDB Upstream

```bash
cd ~/CODE/paradedb
gh repo sync --source paradedb/paradedb   # Pull upstream changes
# Apply any conflicts in tokenizers/src/manager.rs manually
sl commit -m "chore: sync upstream paradedb"
sl push --to main
```

The Dockerfile always clones `--depth 1` from main, so a push is all that's needed.

## Key Dependency Chain

```
pg_search (pgrx extension)
  └── tokenizers --features sudachi
        └── sudachi-tantivy = { git = "https://github.com/jurmarcus/sudachi" }
              └── sudachi-search (path dep in sudachi monorepo)
                    └── sudachi.rs (upstream morphological analyzer)
```
