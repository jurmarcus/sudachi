# AGENTS.md — sudachi monorepo

Context for AI agents operating across the workspace.

## Repository purpose

A six-crate Rust workspace producing Japanese morphological tooling:

| Crate                | Type                  | Purpose                                                  |
| -------------------- | --------------------- | -------------------------------------------------------- |
| `sudachi-morphology` | rlib                  | Bidirectional morphology — forward conjugation + backward deconjugation. Standalone. |
| `sudachi-optimizer`  | rlib                  | Token-stream rewriter + the single Sudachi gateway re-export. |
| `sudachi-search`     | rlib                  | B+C multi-granularity tokenizer. Engine-agnostic.        |
| `sudachi-sqlite`     | cdylib + rlib         | SQLite FTS5 loadable extension.                          |
| `sudachi-tantivy`    | rlib                  | `tantivy::tokenizer::Tokenizer` adapter.                 |
| `sudachi-wasm`       | cdylib + rlib         | wasm-bindgen tokenizer for browser + Node.js.            |

Plus `docker/postgres/` — Docker infra for ParadeDB; the consumed Rust source (pg_search) lives in `~/CODE/paradedb`.

Workspace members: all six crates listed above. Edition 2024, Rust 1.85+.

## Two load-bearing concepts

### 1. B+C multi-granularity (`is_colocated`)

```text
tokenize("東京都立大学"):
  SearchToken { surface: "東京都立大学", is_colocated: false }   ← new position
  SearchToken { surface: "東京",         is_colocated: true  }   ← same position
  SearchToken { surface: "都立",         is_colocated: true  }   ← same position
  SearchToken { surface: "大学",         is_colocated: true  }   ← same position
```

Each adapter crate translates `is_colocated`:
- `sudachi-sqlite` → `FTS5_TOKEN_COLOCATED` flag (0x0001)
- `sudachi-tantivy` → Tantivy position increment stays at 0
- `sudachi-wasm` → emitted as a JSON field for JS callers

### 2. The single Sudachi gateway

`sudachi-optimizer::sudachi` re-exports every upstream Sudachi type that any consumer needs. Workspace `Cargo.toml` makes the rule explicit:

> Direct use of upstream `sudachi` is restricted to `sudachi-optimizer`; everything else imports through the gateway so post-tokenisation rules apply uniformly across consumers.

```rust
// Rule: everything except sudachi-optimizer uses this
use sudachi_optimizer::sudachi::{JapaneseDictionary, Mode, StatelessTokenizer, Tokenize};
```

If you find yourself adding `sudachi = "..."` to a crate other than `sudachi-optimizer`, stop and add the missing re-export to `sudachi-optimizer/src/sudachi.rs` instead.

## File map

```
Cargo.toml              Workspace root: members, [workspace.package], [workspace.dependencies], [patch], release profile
justfile                Task runner — single source of truth for build/test/wasm/pgrx commands
rust-toolchain.toml     Stable channel pin

crates/sudachi-morphology/
  src/lib.rs            Re-exports: Verb, VerbClass, IAdjective, NaAdjective, Conjugation, deconjugate, ...
  src/conjugation.rs    Feature-record forward conjugation (Voice + Mood + Politeness + Polarity + Tense)
  src/verb.rs           Verb<class> + Conjugated<form> typed forward API
  src/verb_class.rs     VerbClass enum (every modern paradigm + classical residues)
  src/adjective.rs      IAdjective / NaAdjective forward API
  src/copula.rs         Copula forms (だ / です / である / のだ)
  src/deconjugate.rs    BFS rule-table deconjugator → Vec<Form>
  src/rule.rs           Rule struct + RuleKind (Standard/OnlyFinal/NeverFinal/Rewrite/Context/Substitution)
  src/rule_index.rs     Aho-Corasick index over rule.con_end suffixes (daachorse)
  src/irregular.rs      Hard-coded paradigms for する / 来る / ある / 行く
  src/kana.rs           Hiragana/katakana helpers
  src/tag.rs            ConjForm shared tag taxonomy
  data/                 Rule corpus (JSON) + deconjugation_rules.json
  tests/golden.rs       Golden corpus runner (~4,800 cases across 23 classes)
  tests/golden/*.rs     Per-class fixture modules
  tests/round_trip.rs   Forward → deconjugate round-trip checks
  benches/deconjugate.rs

crates/sudachi-optimizer/
  src/lib.rs            Re-exports: Optimizer, Pipeline, Stage, Phase, Morpheme, Pos, MorphemeFeatures, Lexicon, load_dictionary
  src/sudachi.rs        Upstream re-exports — the gateway
  src/optimizer.rs      Optimizer (StatelessTokenizer + Pipeline + default Mode)
  src/pipeline.rs       Pipeline runner + canonical_stages() (full canonical ordering)
  src/stage.rs          Stage struct + Phase enum (Split/Repair/Combine/Cleanup/Disambiguation)
  src/token.rs          Morpheme owned mirror of sudachi::Morpheme + Pos closed enum
  src/token_features.rs MorphemeFeatures bitflags (gates whether a stage runs)
  src/lookup.rs         Lexicon trait (vocab callback) + EmptyLexicon
  src/data.rs           Static rule data
  src/split/*.rs        Split-phase rules (1 file per rule)
  src/repair/*.rs       Repair-phase rules
  src/combine/*.rs      Combine-phase rules
  src/cleanup/*.rs      Cleanup-phase rules
  src/disambiguation/*.rs  Disambiguation-phase rules

crates/sudachi-search/
  src/lib.rs            Everything: SearchTokenizer, SearchToken, CompoundWord, extract_compounds, SearchError

crates/sudachi-sqlite/
  src/lib.rs            Entry point sudachi_fts5_tokenizer_init, tokenization loop, dict loading
  src/extension.rs      FTS5 API retrieval, tokenizer registration
  src/common.rs         ffi_panic_boundary, SQLite/FTS5 constants, callback types

crates/sudachi-tantivy/
  src/lib.rs            Re-exports: SudachiTokenizer, SudachiTokenStream, SplitMode, TokenData
  src/tokenizer.rs      SudachiTokenizer + SplitMode + TokenizerInner enum
  src/stream.rs         SudachiTokenStream — position arithmetic over pre-collected TokenData

crates/sudachi-wasm/
  src/lib.rs            wasm-bindgen exports — SudachiTokenizer, JsToken, JsCompound
  example/index.html    Browser demo
  example/node.mjs      Node.js demo

docker/postgres/
  Dockerfile            Clones jurmarcus/paradedb, builds pg_search with --features icu,sudachi
  bootstrap.sh          Postgres init: CREATE EXTENSION pg_search, search_path
  docker-compose.yml    Production compose
  docker-compose.dev.yml Dev compose
  pg_search--0.20.6.sql Pre-generated schema (workaround for pgrx package UTF-8 bug)
  manifests/            Kubernetes manifests
```

## Hard rules

These will break things or silently change behaviour. Don't violate them without a deliberate reason.

1. **Do not import `sudachi` directly outside `sudachi-optimizer`.** Add the re-export to `sudachi-optimizer/src/sudachi.rs` and consume it from there.
2. **Do not add `panic = "abort"` to `sudachi-sqlite` or to the workspace.** Sudachi-sqlite's FFI panic boundary depends on `std::panic::catch_unwind`, which `panic = "abort"` disables, producing UB on any Rust panic that crosses the SQLite FFI.
3. **Do not change `sudachi-sqlite`'s `crate-type = ["cdylib", "rlib"]`.** cdylib produces the loadable extension; rlib lets `cargo test` link the test binary. Both are required.
4. **Do not reorder `is_colocated` emission.** The Mode C compound MUST come first (`is_colocated: false`), then any Mode B sub-tokens (`is_colocated: true`). Search engines rely on this sequence to compute positions.
5. **Pin upstream `sudachi.rs` to one rev across the workspace.** Two crates seeing different upstream types do not link.
6. **Use Sapling (`sl`), not `git`.** This repo's history is in `.sl/`. Bare `git` commands will fail or do the wrong thing.
7. **Run `just ci` before committing.** It runs `cargo fmt --all --check`, `cargo clippy --all -- -D warnings`, and `cargo test`. The clippy gate is `-D warnings`, so any new lint fails CI.

## Build & test

```bash
just dict-setup   # one-time — downloads dictionary to ~/.sudachi/
just ci           # fmt check + clippy -D warnings + tests — gate before commit
just build        # release build
just test         # workspace tests
```

Most workspace tests do not require the dictionary. Tests that do are gated with `#[ignore]` and run via `cargo test -- --include-ignored`.

## Dependency graph (workspace level)

```
sudachi.rs (upstream git, pinned rev, [patch] redirects to wasm-friendly fork)
       ▲
       │ direct dep, only allowed here
sudachi-optimizer  ──► sudachi-morphology
       ▲
       │ everyone else reaches Sudachi through the optimizer's `sudachi::*` re-export
       │
sudachi-search ◄── sudachi-sqlite
              ◄── sudachi-tantivy
              ◄── sudachi-wasm

paradedb/pg_search → sudachi-tantivy (git dep, separate repo)
```

## Common tasks

### Add a feature to `sudachi-search`

1. Add the method/struct to `crates/sudachi-search/src/lib.rs`.
2. If new Sudachi types are needed, add them to `crates/sudachi-optimizer/src/sudachi.rs` first and re-export.
3. `just ci` to verify.

### Add a tokenizer option to `sudachi-sqlite`

1. Parse the option in the `xCreate` callback (`src/extension.rs`).
2. Store on the `Fts5Tokenizer` struct (`src/lib.rs`).
3. Add a unit test; `just test`.

### Add a new optimiser rule

1. Pick a phase: `split/`, `repair/`, `combine/`, `cleanup/`, or `disambiguation/`.
2. Create `crates/sudachi-optimizer/src/<phase>/<rule_name>.rs`. One rule per file.
3. Define `pub fn apply(morphemes: Vec<Morpheme>, lexicon: &dyn Lexicon) -> Vec<Morpheme>`.
4. Register it in `pipeline::canonical_stages` with the appropriate `Phase` and `MorphemeFeatures` gate.
5. Unit tests in the same file.

### Update Tantivy integration (paradedb consumer)

1. Edit `crates/sudachi-tantivy/src/tokenizer.rs` or `src/stream.rs`.
2. `sl commit && sl push` to make the change visible to paradedb's git dep.
3. In `~/CODE/paradedb`: `cargo update -p sudachi-tantivy && just pgrx-check`.

### Test the SQLite extension manually

```bash
SUDACHI_DICT_PATH=~/.sudachi/system_full.dic sqlite3 test.db
sqlite> .load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init
sqlite> CREATE VIRTUAL TABLE t USING fts5(c, tokenize='sudachi_tokenizer');
sqlite> INSERT INTO t VALUES ('東京都立大学で研究');
sqlite> SELECT * FROM t WHERE t MATCH '大学';
```

### Add a forward conjugation form to `sudachi-morphology`

1. Add the method to `Verb` / `IAdjective` / `NaAdjective` in `crates/sudachi-morphology/src/verb.rs` etc.
2. If the form is a new axis combination, extend `Conjugation` axes (`Voice` / `Mood` / `Politeness` / `Polarity` / `Tense`) in `src/conjugation.rs`.
3. Add cases to `tests/golden/<class>.rs` covering the new form.
4. `cargo test --test golden` to verify.

### Add a deconjugation rule

1. Edit `crates/sudachi-morphology/data/deconjugation_rules.json`.
2. Add round-trip test in `tests/round_trip.rs`.
3. Add fixture cases to the relevant `tests/golden/<class>.rs`.
4. `cargo test -p sudachi-morphology` to verify.

## Performance notes

- Dictionary is ~70MB. Share via `Arc<JapaneseDictionary>` between tokenizer instances.
- B+C tokenisation does two Sudachi passes — ~2× single-mode cost.
- `sudachi-sqlite` loads the dictionary once per FTS5 table (in `xCreate`).
- `sudachi-tantivy`'s paradedb consumer uses `Lazy<Option<Arc<SudachiTokenizer>>>` per mode for one-shot init.
- `sudachi-morphology`'s deconjugator builds a daachorse Aho-Corasick automaton once at first use via `LazyLock` and reuses it for every subsequent call.
- `sudachi-optimizer`'s pipeline gates each stage on `MorphemeFeatures`, skipping stages whose triggering features aren't present in the current morpheme stream.

## License

Apache-2.0 for everything except `sudachi-morphology` (MIT). Per-crate `Cargo.toml` is authoritative.
