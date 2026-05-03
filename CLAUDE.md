# sudachi monorepo

Seven-crate Rust workspace for Japanese morphological analysis and structural NLP:
- a tokenizer core (`sudachi-search`)
- three search-engine adapters (`sudachi-sqlite`, `sudachi-tantivy`, `sudachi-wasm`)
- a token-stream optimiser that doubles as the upstream-Sudachi gateway (`sudachi-optimizer`)
- a standalone bidirectional morphology library (`sudachi-morphology`)
- a pure-Rust port of [KWJA](https://github.com/ku-nlp/kwja) v2.4 inference for dependency / BasePhrase / cohesion / discourse / NE on a DeBERTa-v2 base backbone (`sudachi-kwja`)

## Workspace structure

```
crates/
├── sudachi-kwja/         KWJA v2.4 inference port. DeBERTa-v2 base. candle 0.8 (metal default, cuda for prod).
├── sudachi-morphology/   Standalone. Forward verb/adj conjugation + backward deconjugation.
├── sudachi-optimizer/    The single Sudachi gateway. Re-exports upstream types + 5-phase rewriter.
├── sudachi-search/       B+C multi-granularity tokenizer. Engine-agnostic SearchToken stream.
├── sudachi-sqlite/       SQLite FTS5 loadable extension (cdylib).
├── sudachi-tantivy/      tantivy::tokenizer::Tokenizer adapter (used by paradedb fork).
└── sudachi-wasm/         wasm-bindgen tokenizer for browser + Node.js.

docker/
└── postgres/             Docker infra for ParadeDB + pg_search (Sudachi feature). No Rust source here.
```

All seven are workspace members. `sudachi-wasm` is additionally driven by `wasm-pack`. `sudachi-kwja` has mutually-exclusive `metal` (default) and `cuda` features for the candle backend. `docker/postgres/` is a separate build target; the pgrx Rust source it consumes lives in `~/CODE/paradedb`.

## Two product surfaces, one workspace

The crates form **two complementary stacks** that share neither dependencies nor consumers — they coexist here because they share authorship, the Sudachi tokenizer, and the same Japanese-NLP problem domain.

```
                    Search / FTS stack                          Structural NLP stack
                    ─────────────────────────                   ───────────────────────────
                    sudachi-optimizer                           sudachi-kwja
                          │                                          │
                          ├── sudachi-search                          (depends on candle, not Sudachi)
                          │       ├── sudachi-sqlite                  Loaded by jisho-parse (gRPC service)
                          │       ├── sudachi-tantivy ── paradedb     in the jisho monorepo via relative
                          │       └── sudachi-wasm                    path: services/jisho-parse/Cargo.toml
                          │                                          → ../../../sudachi/crates/sudachi-kwja
                          └── sudachi-morphology

                    Sudachi UniDic tokenizer +                  KWJA dependency / BasePhrase tree /
                    deconjugation/conjugation +                 cohesion / discourse / NE on a
                    optimised search token streams              pre-tokenized morpheme stream.
```

The KWJA crate does not depend on the Sudachi tokenizer crates — it accepts pre-tokenized input via a `SudachiMorpheme` struct mirroring the proto contract that crosses the gRPC boundary in `jisho-parse`. The Sudachi work happens in the consumer; this crate just consumes the morphemes.

## Dependency invariant: one Sudachi gateway

`sudachi-optimizer` is the **only** crate that imports the upstream `sudachi` crate directly. Every other crate imports `JapaneseDictionary`, `Mode`, `StatelessTokenizer`, `Tokenize`, `SudachiError`, etc. from `sudachi_optimizer::sudachi::*`.

```rust
// CORRECT — every consumer
use sudachi_optimizer::sudachi::{JapaneseDictionary, Mode, StatelessTokenizer};

// WRONG — only sudachi-optimizer may do this
use sudachi::dic::dictionary::JapaneseDictionary;
```

Why: a single gateway is the place where rev pinning, optimiser rules, and any future tokenizer swap can be applied uniformly to the whole ecosystem.

The workspace `Cargo.toml` documents this explicitly:

```toml
# DIRECT use of this dep is restricted to sudachi-optimizer; everything else
# imports through sudachi-optimizer's re-exports so post-tokenisation rules
# can apply uniformly across all consumers.
sudachi = { git = "https://github.com/WorksApplications/sudachi.rs", rev = "..." }
```

## Crate dependency graph

```
sudachi-morphology    (standalone — serde, daachorse, thiserror)
       ▲
       │ used by optimizer rules
       │
sudachi-optimizer  ──► sudachi.rs (upstream, pinned rev)
       ▲
       │ everyone reaches Sudachi through here
       │
sudachi-search ◄── sudachi-sqlite
       ▲       ◄── sudachi-tantivy
       │       ◄── sudachi-wasm
       │
   external: paradedb/pg_search → sudachi-tantivy (git dep)


sudachi-kwja          (independent stack — no Sudachi dep)
       │              candle 0.8 + safetensors + tokenizers
       │              consumes pre-tokenized SudachiMorpheme structs
       ▼
   external: jisho-monorepo/services/jisho-parse → sudachi-kwja (relative path)
```

The KWJA stack is **not** plumbed through `sudachi-optimizer`. It accepts a `Vec<Vec<SudachiMorpheme>>` (mirrored from the proto contract) and emits a `Document` tree. The actual Sudachi tokenization happens in the consumer (`jisho-parse` calls `jisho-core`'s Sudachi pipeline before invoking `sudachi-kwja`).

## Workspace dependencies (root Cargo.toml)

- `edition = "2024"`, Rust 1.85+
- `[patch."https://github.com/WorksApplications/sudachi.rs"]` redirects to a fork that gates `libloading` behind `cfg(not(target_family = "wasm"))` — required for `sudachi-wasm` to compile to `wasm32-unknown-unknown`. The patch is invisible to non-wasm crates.
- Release profile: `opt-level = 3`, `lto = true`, `codegen-units = 1`, `strip = true`. **No `panic = "abort"`** — `sudachi-sqlite`'s FFI boundary uses `catch_unwind`.

## Commands

```bash
just                  # List all
just build            # Release build (workspace)
just build-dev        # Dev build
just check            # cargo check
just test             # Workspace tests (no dictionary required)
just test-verbose     # Tests with stdout
just fmt              # cargo fmt --all
just lint             # cargo clippy --all -- -D warnings
just fix              # fmt + lint
just ci               # fmt check + clippy + tests — gate before commit

just dict-setup       # Install dictionary to ~/.sudachi/
just dict-path        # Show resolved dictionary path

just wasm-build           # wasm-pack --target web
just wasm-build-node      # wasm-pack --target nodejs
just wasm-build-bundler   # wasm-pack --target bundler
just wasm-build-dev       # wasm-pack --dev (faster)
just wasm-serve           # Serve the demo at :3000

just pgrx-build       # cd ~/CODE/paradedb && cargo pgrx build -p pg_search --features icu,sudachi
just pgrx-check       # same crate, cargo check

just env              # Print toolchain + dict path
just clean            # cargo clean + remove WASM pkg/
```

## Dictionary

Required at runtime by every tokenizer-using crate. Auto-discovered from `~/.sudachi/system_full.dic` (or `system_small.dic` as fallback) by the `just` recipes:

```bash
just dict-setup
```

Or override explicitly:

```bash
export SUDACHI_DICT_PATH=/abs/path/to/system_full.dic
```

The dictionary is ~70MB and shared across crates via `Arc<JapaneseDictionary>`.

## Key design facts

- **`is_colocated: bool`** is the contract every search-engine adapter implements. Mode C compound first (`is_colocated: false`), then Mode B sub-tokens at the same position (`is_colocated: true`). Never reorder.
- **B+C tokenisation does two passes** — Mode C for compounds, Mode B for sub-tokens — so it costs ~2× single-mode tokenisation. Acceptable for the recall gain.
- **`sudachi-sqlite` keeps the unwind panic strategy.** Adding `panic = "abort"` would invalidate `std::panic::catch_unwind` and cause UB when Rust code panics across the SQLite FFI boundary.
- **`sudachi-sqlite` has `crate-type = ["cdylib", "rlib"]`.** cdylib produces the loadable extension; rlib is required to link `cargo test`'s test binary. Both are needed.
- **`sudachi-wasm` is excluded from a normal `cargo build` workspace pass for wasm targets** but is a workspace member so `cargo check`/`clippy` cover it on the host platform. Use `just wasm-build*` for the wasm32 target.
- **`sudachi-morphology` has no Sudachi dependency.** It's a self-contained morphology library that the optimiser uses; it can also be used standalone for conjugation tables, deconjugation queries, etc.
- **`sudachi.rs` is pinned** to a specific git rev in the workspace, ensuring all crates see the same dictionary types.
- **`sudachi-kwja` has no Sudachi dependency either.** It accepts pre-tokenized morphemes (`SudachiMorpheme`) and runs the KWJA word module via candle. The structural heads (dependency, BP tree, cohesion, discourse, NE) augment Sudachi's morpheme-level data; reading/lemma/conjtype/conjform from KWJA are computed but discarded in favor of Sudachi's UniDic values in production.
- **`sudachi-kwja` requires checkpoints at runtime.** ~1 GB total (char + word safetensors + HF tokenizer artifacts) loaded once per process. Default search location is `~/.local/share/jisho/checkpoints/`; the typo module (~330 MB more) is opt-in.
- **`sudachi-kwja` is fp16 on CUDA, fp32 elsewhere.** Required vendored patches to `candle-transformers` are documented in `crates/sudachi-kwja/CLAUDE.md`.

## Algorithm: B+C tokenisation (sudachi-search)

```rust
fn tokenize_internal(input: &str) -> Vec<SearchToken> {
    let morphemes_c = stateless.tokenize(input, Mode::C, false)?;
    let morphemes_b = stateless.tokenize(input, Mode::B, false)?;

    for morpheme_c in morphemes_c {
        emit SearchToken { surface: c.text, is_colocated: false, ... };

        for morpheme_b in morphemes_b within morpheme_c.byte_span() {
            if morpheme_b.text != morpheme_c.text {
                emit SearchToken { surface: b.text, is_colocated: true, ... };
            }
        }
    }
}
```

## ParadeDB integration

Source: `~/CODE/paradedb` (separate repo).
This monorepo provides:

- `sudachi-tantivy` as a git dep that paradedb's `tokenizers/` consumes under a `sudachi` feature
- `docker/postgres/` — Docker image building paradedb + pg_search

Build paradedb against this monorepo's `sudachi-tantivy`:

```bash
just pgrx-build   # cd ~/CODE/paradedb && cargo pgrx build -p pg_search --features icu,sudachi
just pgrx-check
```

Paradedb's workspace uses `[patch.crates-io]` to redirect `tantivy-tokenizer-api` to its forked tantivy, ensuring type compatibility across the crate boundary.

## Architecture

- [COMPREHENSION_PIPELINE.md](COMPREHENSION_PIPELINE.md) — **start here** for cross-crate architecture: what `sudachi-optimizer` vs `sudachi-kwja` vs `sudachi-morphology` each do, how they fit into the comprehension-oriented pipeline, the layer-decision rules for new code, and the audit of the existing 28 optimizer rules.

## Per-crate docs

Every crate has its own `CLAUDE.md` and `AGENTS.md` with deeper detail:

- [crates/sudachi-kwja/CLAUDE.md](crates/sudachi-kwja/CLAUDE.md) — KWJA v2.4 inference port (DeBERTa-v2 base, candle)
- [crates/sudachi-morphology/CLAUDE.md](crates/sudachi-morphology/CLAUDE.md)
- [crates/sudachi-optimizer/CLAUDE.md](crates/sudachi-optimizer/CLAUDE.md)
- [crates/sudachi-search/CLAUDE.md](crates/sudachi-search/CLAUDE.md)
- [crates/sudachi-sqlite/CLAUDE.md](crates/sudachi-sqlite/CLAUDE.md)
- [crates/sudachi-tantivy/CLAUDE.md](crates/sudachi-tantivy/CLAUDE.md)
- [crates/sudachi-wasm/CLAUDE.md](crates/sudachi-wasm/CLAUDE.md)
- [docker/postgres/CLAUDE.md](docker/postgres/CLAUDE.md)

## Version control

This repo uses **Sapling (`sl`)**, not git.

```bash
sl status       # Status
sl add          # Stage
sl commit       # Commit
sl push         # Push
sl addremove    # Detect added/removed
```
