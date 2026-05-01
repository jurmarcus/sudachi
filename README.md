# sudachi

**A Rust ecosystem for Japanese morphological analysis — search-engine tokenizers, conjugation, deconjugation, and KWJA-style structural parsing.**

Seven crates that share one tokenizer core, one optimisation pipeline, one bidirectional morphology library, and a pure-Rust port of [KWJA](https://github.com/ku-nlp/kwja) (Kyoto-Waseda Japanese Analyzer) inference. Targets SQLite FTS5, Tantivy, ParadeDB (PostgreSQL), WebAssembly, and gRPC inference services on CUDA / Metal / CPU.

---

## What's in here

```
crates/
├── sudachi-kwja/         # Pure-Rust KWJA v2.4 inference (DeBERTa-v2 base, candle backend)
├── sudachi-morphology/   # Bidirectional: forward conjugation + backward deconjugation
├── sudachi-optimizer/    # Token-stream rewriter; the single Sudachi gateway
├── sudachi-search/       # B+C multi-granularity tokenizer (engine-agnostic)
├── sudachi-sqlite/       # SQLite FTS5 loadable extension (cdylib)
├── sudachi-tantivy/      # tantivy::tokenizer::Tokenizer adapter
└── sudachi-wasm/         # wasm-bindgen tokenizer for browser + Node.js

docker/
└── postgres/             # Docker image building ParadeDB + pg_search with the Sudachi tokenizer
```

All seven are workspace members. `sudachi-wasm` is also driven by `wasm-pack`. `sudachi-kwja` has a `cuda` feature for production GPU inference and a `metal` default for Apple Silicon dev. `docker/postgres/` is build infra; the pgrx Rust source it consumes lives in `~/CODE/paradedb` (see [Postgres integration](#postgres-via-paradedb)).

---

## The three problems this solves

### 1. Compound words trap sub-words

```text
Document: 東京都立大学で研究
Query:    大学

Single-mode tokenizer (Mode C):    ["東京都立大学", "で", "研究"]   → no match
B+C multi-granularity (this repo):
  pos 0  東京都立大学   primary
  pos 0  東京           colocated
  pos 0  都立           colocated
  pos 0  大学           colocated   → match
  pos 1  で
  pos 2  研究
```

`sudachi-search` emits the Mode C compound first, then any Mode B sub-tokens that fall inside it at the **same position**. The `SearchToken::is_colocated` flag carries that information across the FFI / engine boundary.

### 2. Conjugations and inflections fragment the index

Surface forms like 食べた, 食べている, 食べます all normalise to 食べる. Variant kanji (附属 → 付属), fullwidth ASCII (ＳＵＭＭＥＲ → サマー), and long-vowel marks (パーティー → パーティ) are folded the same way. Documents and queries get the same treatment, so a query in any conjugation finds all related documents.

### 3. Tokenizer output is *correct against the dictionary*, not against text

Sudachi's UniDic output is dictionary-correct but ships known weaknesses on compound auxiliaries (てしまう), colloquial forms (食べねえ), fused interjection+particle pairs (じゃあ), vowel elongation (おはよー), and similar surface phenomena. `sudachi-optimizer` runs a five-phase pipeline (Split → Repair → Combine → Cleanup → Disambiguation) over the raw morpheme stream and produces the corrected stream all downstream crates work from.

---

## How the crates compose

The workspace hosts **two complementary stacks** that share no crate-level dependencies. They live together because of shared authorship, the Japanese-NLP problem domain, and a shared deployment story (jisho consumes both):

### Search / FTS stack

```
                    ┌─────────────────────────────┐
                    │   sudachi-morphology        │   forward Verb::*() / Form::*
                    │   (standalone — no deps)    │   backward deconjugate()
                    └─────────────┬───────────────┘
                                  │ used by optimizer rules
                                  ▼
┌──────────────────────────────────────────────────────────────┐
│   sudachi-optimizer                                          │
│   - the single re-export module for upstream sudachi types   │
│   - five-phase rewriter pipeline                             │
│   - Optimizer wraps Sudachi + chosen pipeline                │
└──────────────────────────────┬───────────────────────────────┘
                               │ everyone reaches Sudachi through here
        ┌──────────────────────┼──────────────────────┐
        ▼                      ▼                      ▼
┌────────────────┐    ┌────────────────┐    ┌────────────────┐
│ sudachi-search │    │ sudachi-sqlite │    │ sudachi-tantivy│
│ B+C core       │    │ FTS5 cdylib    │    │ Tokenizer impl │
│ SearchToken    │◄───┤                │    │                │
│ is_colocated   │    └────────────────┘    └────────────────┘
└───────┬────────┘                                   ▲
        │                                            │ git dep
        ▼                                            │
┌────────────────┐                          ┌────────────────┐
│ sudachi-wasm   │                          │ paradedb +     │
│ JS bindings    │                          │ pg_search      │
└────────────────┘                          │ (separate repo)│
                                            └────────────────┘
```

**Key invariant:** every crate that needs `JapaneseDictionary`, `Mode`, `StatelessTokenizer`, `Tokenize`, or `SudachiError` imports them from `sudachi_optimizer::sudachi::*` — never from upstream `sudachi` directly. This gives one place to swap revs, apply optimisation rules, or change tokenizer behaviour for the whole ecosystem.

### Structural NLP stack

```
                  ┌──────────────────────────────────────────────┐
                  │   sudachi-kwja                               │
                  │   - DeBERTa-v2 base (KWJA v2.4) backbone     │
                  │   - 12 word-module heads + char + typo       │
                  │   - candle 0.8 (metal | cuda | cpu)          │
                  │   - fp16 on CUDA via vendored patches        │
                  └──────────────────┬───────────────────────────┘
                                     │ relative-path dep
                                     ▼
                          ┌──────────────────────────┐
                          │   jisho-monorepo         │
                          │   services/jisho-parse   │  gRPC service on 4090
                          │   (separate sl repo)     │
                          └──────────────────────────┘
```

`sudachi-kwja` accepts pre-tokenized morphemes (`SudachiMorpheme`) and produces a structural `Document → Sentence → Phrase → BasePhrase → Morpheme` tree plus cross-sentence discourse relations. The Sudachi tokenization happens in the consumer (`jisho-parse` calls `jisho-core`'s Sudachi pipeline before invoking sudachi-kwja). The two stacks meet in the consumer, never in this workspace's dep graph.

---

## Crates

### `sudachi-kwja` — KWJA v2.4 inference port

Pure-Rust port of [KWJA](https://github.com/ku-nlp/kwja) — Kyoto-Waseda's multi-task Japanese analyzer. DeBERTa-v2 **base** backbone, candle 0.8 runtime, argmax-identical with KWJA-Python on the heads we consume.

```rust
use sudachi_kwja::{Pipeline, SudachiMorpheme};

let pipeline = Pipeline::load("/checkpoints".as_ref())?;
let docs = pipeline.parse_morphemes(&[vec![
    SudachiMorpheme { surface: "今日".into(), reading: "きょう".into(), /* ... */ },
    // ...
]])?;

for item in docs {
    let sudachi_kwja::ParseItem::Tree(doc) = item else { continue };
    for sent in &doc.sentences {
        for bp in &sent.base_phrases {
            println!("BP {}: {} → head {} ({})", bp.id, bp.surface, bp.head, bp.dep_type);
        }
    }
}
```

| Module | Backbone (KWJA v2.4) | Status |
| ------ | -------------------- | ------ |
| char (sentence segmentation) | `deberta-v2-base-japanese-char-wwm` | shipped |
| word (12 heads: dependency / BasePhrase / cohesion / discourse / NE / features / ...) | `deberta-v2-base-japanese` | shipped |
| typo (kdr + ins) | `deberta-v2-base-japanese-char-wwm` | shipped (opt-in) |

Hot path is `parse_morphemes` — caller has already tokenized with Sudachi, KWJA's word module runs on the pre-tokenized stream and emits the structural heads. Sudachi data wins for `surface` / `reading` / `lemma` / `conjtype` / `conjform`; KWJA wins for dependency, BP tree, cohesion, discourse, NE, and per-word feature flags.

See [crates/sudachi-kwja/README.md](crates/sudachi-kwja/README.md) for the full architecture, head taxonomy, fp16 patches, and checkpoint conversion workflow.

### `sudachi-search` — B+C tokenizer core

Engine-agnostic. Emits `SearchToken { surface, byte_start, byte_end, is_colocated }`. The `is_colocated: true` tokens are sub-words at the same position as the immediately preceding compound — every adapter crate's job is to translate that into its engine's "same position" mechanism.

```rust
let tokenizer = SearchTokenizer::new(dictionary);
for tok in tokenizer.tokenize("東京都立大学")? {
    println!("{:12} colocated={}", tok.surface, tok.is_colocated);
}
```

| Engine        | Translation of `is_colocated: true`             |
| ------------- | ----------------------------------------------- |
| SQLite FTS5   | `FTS5_TOKEN_COLOCATED` flag (0x0001)            |
| Tantivy       | Position increment = 0                          |
| Lucene/ES     | `PositionIncrementAttribute = 0`                |
| ParadeDB      | (uses `sudachi-tantivy`)                        |

### `sudachi-sqlite` — FTS5 loadable extension

```sql
.load ./libsudachi_sqlite sudachi_fts5_tokenizer_init

CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer');
INSERT INTO docs VALUES ('東京都立大学で研究しています');

SELECT * FROM docs WHERE docs MATCH '大学';   -- finds the document
```

`crate-type = ["cdylib", "rlib"]`. The cdylib is the `.dylib`/`.so`; the rlib lets `cargo test` link the test binary. All FFI entry points are wrapped in a `catch_unwind` panic boundary — `panic = "abort"` would defeat that and is deliberately not set.

### `sudachi-tantivy` — Tantivy `Tokenizer` impl

```rust
let tokenizer = SudachiTokenizer::new(SplitMode::Search)?;
let mut stream = tokenizer.token_stream("東京都立大学");
// Tokens at position 0: 東京都立大学, 東京 (colocated), 都立 (colocated), 大学 (colocated)
```

Four split modes: A (finest), B (medium), C (coarsest), Search (B+C). Standard modes go straight through `StatelessTokenizer`; Search mode delegates to `sudachi-search`. Used by ParadeDB's `pg_search` extension as a git dep.

### `sudachi-optimizer` — token-stream rewriter

```rust
let dict = Arc::new(sudachi_optimizer::load_dictionary(&dict_path)?);
let optimizer = Optimizer::new(dict).with_pipeline(Pipeline::analysis());

for m in optimizer.tokenize("食べてしまった")? {
    println!("{}\t{:?}\t{:?}", m.surface, m.pos, m.applied_rules);
}
```

Five-phase pipeline over the raw morpheme stream:

| Phase            | Purpose                                                  |
| ---------------- | -------------------------------------------------------- |
| Split            | Break apart over-merged Sudachi morphemes                |
| Repair           | Fix specific known mis-tokenisations                     |
| Combine          | Glue together morphemes that should have been one        |
| Cleanup          | Reclassify orphans, filter misparses                     |
| Disambiguation   | Fix reading ambiguity using neighbouring context         |

Pipelines are configurable: `Pipeline::analysis()` runs everything (the default), `Pipeline::search()` is a hook for search-friendly subsets, `Pipeline::empty()` is a test fixture.

### `sudachi-morphology` — forward + backward Japanese morphology

Standalone crate (no Sudachi dependency). Two complementary surfaces sharing one tag taxonomy:

```rust
// Forward: I have a verb, give me a form
let taberu = Verb::new("食べる", VerbClass::Ichidan);
assert_eq!(taberu.negative().surface, "食べない");
assert_eq!(taberu.causative_passive().surface, "食べさせられる");

// Backward: I see an arbitrary surface, what could it derive from?
let forms = deconjugate("食べさせられた");
// → contains { text: "食べる", class: Ichidan, chain: ["causative", "passive", "past"] }
```

Rules live in `data/` classified by linguistic role (stems, verb, auxiliary, adjective, copula, colloquial, dialect, keigo, irregular, negative_chain). The deconjugator builds an Aho-Corasick automaton over rule suffixes for linear-time matching. Validated against ~4,800 golden test cases covering every modern verb / adjective / copula class.

### `sudachi-wasm` — browser + Node.js bindings

```js
const dictBytes = new Uint8Array(await (await fetch('/system_full.dic')).arrayBuffer());
const tokenizer = new SudachiTokenizer(dictBytes);

tokenizer.tokenize("東京都立大学で研究");
// [{ surface: "東京都立大学", isColocated: false }, ...]
```

Built with `wasm-pack`. Targets: `web` (ES module), `nodejs`, `bundler` (webpack/vite). The crate is a thin wrapper around `sudachi-search` exposed via `wasm-bindgen`.

---

## Postgres via ParadeDB

The PostgreSQL surface is `pg_search` (a pgrx extension) with a Sudachi feature. The Rust source lives in a separate repo at `~/CODE/paradedb`; this monorepo provides:

- `sudachi-tantivy` as a git dep that `pg_search --features sudachi` consumes
- `docker/postgres/` — Docker infrastructure that clones paradedb and builds the image

```sql
CREATE EXTENSION pg_search;

CREATE INDEX docs_idx ON documents
USING bm25(id, (content::pdb.sudachi))
WITH (key_field='id');

SELECT * FROM documents WHERE id @@@ 'content:大学';

-- Mode selection via type cast argument
content::pdb.sudachi              -- Search (B+C, default)
content::pdb.sudachi('search')    -- explicit
content::pdb.sudachi('c')         -- Mode C
content::pdb.sudachi('a')         -- Mode A
```

```bash
just pgrx-build   # cargo pgrx build -p pg_search --features icu,sudachi (in ~/CODE/paradedb)
just pgrx-check   # cargo check, same target
```

---

## Quick start

### Dictionary (one-time)

```bash
just dict-setup   # downloads to ~/.sudachi/system_full.dic
```

Or set `SUDACHI_DICT_PATH=/abs/path/to/system_full.dic` explicitly.

### Build & test

```bash
just build        # release build, all workspace crates
just test         # workspace tests (no dictionary required)
just ci           # fmt check + clippy -D warnings + tests — must pass before commit
just fix          # fmt + lint
```

### SQLite

```bash
just build
sqlite3 test.db ".load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init"
```

### Tantivy

```toml
[dependencies]
sudachi-tantivy = { git = "https://github.com/jurmarcus/sudachi" }
```

### WASM

```bash
just wasm-build         # ES module for browsers
just wasm-build-node    # Node.js
just wasm-build-bundler # webpack / vite
just wasm-serve         # serve the example demo at http://localhost:3000
```

### ParadeDB (Docker)

```bash
cd docker/postgres
docker compose up
```

---

## Commands reference

```text
just                     List all commands
just build               Release build (workspace)
just build-dev           Dev build (workspace)
just check               cargo check
just test                Tests (no dictionary)
just test-verbose        Tests with stdout
just fmt                 cargo fmt --all
just lint                cargo clippy --all -D warnings
just fix                 fmt + lint
just ci                  fmt check + clippy + test

just dict-setup          Download Sudachi dictionary to ~/.sudachi/
just dict-path           Show resolved dictionary path

just wasm-build          wasm-pack build --target web
just wasm-build-node     wasm-pack build --target nodejs
just wasm-build-bundler  wasm-pack build --target bundler
just wasm-build-dev      wasm-pack build --dev (faster)
just wasm-serve          Serve the WASM demo

just pgrx-build          Build pg_search --features icu,sudachi (in ~/CODE/paradedb)
just pgrx-check          Check the same target

just env                 Show environment info
just clean               cargo clean + remove WASM pkg/
```

---

## Split modes

```text
Mode A  (finest)     ["東京", "都", "立", "大学"]
Mode B  (medium)     ["東京", "都立", "大学"]
Mode C  (coarsest)   ["東京都立大学"]
Search  (B+C)        ["東京都立大学", "東京"*, "都立"*, "大学"*]
                     * = colocated (same position as preceding compound)
```

Search mode is the default for all consumers. It indexes both compound and sub-tokens at the same position, so exact-phrase queries and sub-word queries both succeed against the same document.

---

## Normalization

Default. Applied to both indexed text and queries.

| Surface       | Normalised   | Reason                       |
| ------------- | ------------ | ---------------------------- |
| 食べた        | 食べる       | Verb (past → dictionary)     |
| 食べている    | 食べる       | Verb (progressive)           |
| 美しかった    | 美しい       | i-adjective past             |
| 附属病院      | 付属病院     | Variant kanji                |
| ＳＵＭＭＥＲ   | サマー       | Fullwidth ASCII → katakana   |
| パーティー    | パーティ     | Long-vowel mark              |

Surface form is available on every tokenizer when raw text is needed.

---

## Workspace layout

| Crate                | Type                  | Workspace? | Output                              |
| -------------------- | --------------------- | ---------- | ----------------------------------- |
| `sudachi-kwja`       | rlib                  | yes        | KWJA v2.4 inference (DeBERTa-v2 base, candle) |
| `sudachi-morphology` | rlib                  | yes        | Forward + backward morphology       |
| `sudachi-optimizer`  | rlib                  | yes        | Optimised token streams + Sudachi gateway |
| `sudachi-search`     | rlib                  | yes        | B+C SearchToken stream              |
| `sudachi-sqlite`     | cdylib + rlib         | yes        | `libsudachi_sqlite.{so,dylib}`      |
| `sudachi-tantivy`    | rlib                  | yes        | `tantivy::Tokenizer` impl           |
| `sudachi-wasm`       | cdylib + rlib         | yes (also wasm-pack) | `pkg/sudachi_wasm.{wasm,js}` |
| `docker/postgres/`   | Docker only           | n/a        | `paradedb-sudachi` image            |

Edition 2024 throughout. Rust 1.85+. Workspace `panic` policy is the default (unwind) — sudachi-sqlite's FFI panic boundary depends on it. `sudachi-kwja` requires checkpoints at runtime (~1 GB total at `~/.local/share/jisho/checkpoints/`); see [crates/sudachi-kwja/README.md](crates/sudachi-kwja/README.md) for setup.

---

## License

Apache-2.0 for the search/sqlite/tantivy/wasm/optimizer adapters.
MIT for `sudachi-morphology`.
MIT OR Apache-2.0 for `sudachi-kwja` (matches KWJA upstream). Model weights remain under their original KWJA / HuggingFace license terms.
See per-crate `Cargo.toml` for authoritative licenses.
