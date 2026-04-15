---
type: plan
project: sudachi
status: ready
created: 2026-04-16
tags: [rust, monorepo, cleanup, best-practices]
---

# Panel Report: sudachi monorepo cleanup

7-expert panel review. Experts: rust-architecture, software-architecture, devils-advocate,
code-review, test-architect, cli-tui, performance-profiler.

---

## Cross-Expert Consensus (4+ experts)

These are not opinions — they are facts confirmed by multiple independent reviewers:

| Finding | Experts |
|---------|---------|
| `sudachi-tantivy` stub must go (empty crate, false dependency claim) | arch×2, devils, code-review |
| `CLAUDE.md` is wrong on day 1 (wasm excluded, panic guidance dangerous) | arch×2, devils, code-review, perf |
| `is_non_independent_verb` is called but never defined — **compile error** | code-review, test-architect |
| wasm package named `sudachi` collides with upstream git dep | rust-arch, code-review |
| Per-crate justfiles for workspace members duplicate root with zero benefit | devils, cli-tui |

---

## Critical Bugs (ship-blockers)

### BUG-1 — Compile error: `is_non_independent_verb` undefined
`crates/sudachi-search/src/lib.rs:908,912` — `cargo test` does not compile.

**Fix:** Define `fn is_non_independent_verb(pos: &[String]) -> bool` (1–3 lines),
or remove the two dead assertions from `test_non_independent_verbs_not_filtered`.

### BUG-2 — FFI panic boundary is half-wired
`crates/sudachi-sqlite/crates/extension.rs:239-256` — `fts5_create_sudachi_tokenizer` and
`fts5_delete_sudachi_tokenizer` are NOT wrapped in `ffi_panic_boundary`. Only the tokenize
callback is protected. `load_tokenizer` allocates ~70MB and can panic. Any panic in create/delete
is UB across the C boundary.

**Fix:** Extract an inner `Result`-returning helper; wrap the outer `extern "C"` in
`ffi_panic_boundary`. Same for the delete callback.

### BUG-3 — `just wasm *` and `just tantivy *` are broken
Root justfile has `mod wasm 'crates/sudachi-wasm'` and `mod tantivy 'crates/sudachi-tantivy'`
but neither crate has a `justfile`. Every `just wasm <cmd>` fails at runtime.

**Fix:** Create `crates/sudachi-wasm/justfile` (wasm-pack build/test recipes).
For tantivy: either create a minimal justfile or remove `mod tantivy` from root until the crate is real.

### BUG-4 — `just search example` targets a nonexistent example
`sudachi-search/justfile` runs `cargo run --example basic` but the examples directory has
`tokenize.rs` and `full_info.rs`, not `basic`. Fails immediately.

**Fix:** Change to `cargo run --example tokenize`.

### BUG-5 — `sudachi-wasm` include_bytes! paths are broken post-migration
The wasm crate's source embeds resources via paths like `../../resources/sudachi.json`.
In the original layout (`sudachi-wasm/sudachi/src/`), these resolved to `sudachi-wasm/resources/`.
In the current layout (`crates/sudachi-wasm/src/`), they resolve to `crates/resources/` — which
doesn't exist. Excluding the crate hides the error from `cargo check` but `wasm-pack build` fails.

**Fix (option C — only option that survives upstream merges):**
- Move the wasm crate to `crates/sudachi-wasm/wasm/` (the actual Rust crate)
- Put resources at `crates/sudachi-wasm/resources/` (sibling of the crate dir)
- Paths like `../../resources/sudachi.json` from `wasm/src/` resolve correctly
- Zero source edits required, upstream-merge-safe

---

## P0 — Day-one fixes (before next commit)

### P0-A: Delete `sudachi-tantivy` from workspace
The stub crate is a lie: it declares a public crate with keywords/categories, exports a
workspace dep that nothing can use, and documents an architecture that doesn't exist.

1. Remove `"crates/sudachi-tantivy"` from `[workspace.members]` in root `Cargo.toml`
2. Remove `sudachi-tantivy` from `[workspace.dependencies]`
3. Delete `crates/sudachi-tantivy/` entirely
4. Remove `mod tantivy` from root `justfile`
5. Add it back the day someone writes real Tantivy tokenizer code

### P0-B: Fix CLAUDE.md accuracy
Root `CLAUDE.md` has two lies:
- Structure diagram lists `sudachi-wasm` as a workspace member (it's excluded)
- Workspace section contradicts itself (structure says 4 members, bottom says 3)

`crates/sudachi-sqlite/CLAUDE.md` says `panic = "abort"  # Required for FFI` — **this is
dangerous**. `panic = "abort"` would silently defeat `catch_unwind`. The correct note is:
> Release profile must remain on the default `panic = "unwind"` so `catch_unwind` works.

### P0-C: Rename wasm package `sudachi` → `sudachi-wasm`
`crates/sudachi-wasm/Cargo.toml` line 2: `name = "sudachi"` collides with the upstream
`WorksApplications/sudachi.rs` git dep. This is a **name collision** that would break the
workspace if wasm ever rejoins.

Also fix: `authors`, `version` (to `0.1.0`), `edition` (to `2024`), `license` (to `MIT`).

---

## P1 — Workspace hygiene

### P1-A: Pin upstream sudachi dep
```toml
# Current — non-reproducible:
sudachi = { git = "https://github.com/WorksApplications/sudachi.rs", branch = "develop" }

# Fix — pin to a specific commit:
sudachi = { git = "https://github.com/WorksApplications/sudachi.rs", rev = "<sha>" }
```
`branch = "develop"` means every `cargo update` silently pulls new upstream code.

### P1-B: Add `[workspace.lints]`
```toml
# Root Cargo.toml
[workspace.lints.rust]
warnings = "deny"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
```
Then add `lints.workspace = true` to each member crate's `[package]`. This is the enforcement
point for the zero-warning clippy policy.

### P1-C: Add `crate-type = ["cdylib", "rlib"]` to sudachi-sqlite
Currently `["cdylib"]` only. `rlib` is required for `#[test]` items to link. Without it,
`cargo test -p sudachi-sqlite` runs but there are zero test items — a false pass.

### P1-D: Hoist FFI deps to workspace.dependencies
```toml
# Root Cargo.toml [workspace.dependencies]
libc = { version = "0.2", default-features = false }
sqlite-loadable = "0.0.5"
```
Drop the `sqlite3ext-sys = "0.0.1"` direct dep in sudachi-sqlite — `sqlite-loadable` already
re-exports the types needed (confirmed by code-review: `sqlite3_stmt` is available via
`sqlite_loadable::prelude::*`).

### P1-E: Add rust-toolchain.toml at repo root
`crates/sudachi-postgres/rust-toolchain.toml` pins `1.90.0`. Hoist a root-level
`rust-toolchain.toml` so all crates use the same toolchain. pgrx's file takes precedence
when building from within that directory (correct behavior).

### P1-F: Delete per-crate justfiles for workspace members
`crates/sudachi-search/justfile` and `crates/sudachi-sqlite/justfile` each re-declare
`build`, `test`, `fmt`, `lint`, `fix`, `dict-setup`, `clean`, `watch` — identical to
the root. Two sources of truth guarantee drift.

**Keep:** `crates/sudachi-wasm/justfile` (needs wasm-pack), `crates/sudachi-postgres/justfile`
(needs cargo pgrx commands).

**Delete:** `crates/sudachi-search/justfile`, `crates/sudachi-sqlite/justfile`

### P1-G: Add `just ci` recipe to root justfile
```just
# Run what CI runs (no dictionary required)
ci: fmt lint test
```

### P1-H: Fix `pgrx-build` recipe
`cargo build --release --manifest-path` produces a `.so`/`.dylib` but doesn't install the
pgrx extension. Use `cargo pgrx build` instead, or add a comment clarifying this is a
compilation check only and add a separate `pgrx-install` recipe.

---

## P2 — Code quality and API

### P2-A: Add unit tests to sudachi-sqlite
After P1-C (adding `rlib`), add `#[cfg(test)]` items for:
- `usize_to_c_int`: overflow → `Err(SQLITE_INTERNAL)`
- `ffi_panic_boundary`: panicking closure → `SQLITE_INTERNAL`
- `InputText::from_raw_parts`: invalid UTF-8 → `Err(SQLITE_OK)` (the intentional swallow)

### P2-B: Replace `eprintln!` with `log` in sudachi-sqlite hot path
`lib.rs:216` writes to stderr on every failed tokenize call. Under batch indexing load
this is unbounded allocation + stderr lock contention. Add `log` dep, use `log::warn!`.

### P2-C: API cleanup in sudachi-search (medium effort)
Four overlapping normalization knobs exist where one `TokenizeOptions` struct should be:
- `with_surface_form()` / `with_normalized_form(bool)` / `tokenize_with_normalization(_, bool)`
Plan: deprecate the per-call override, consolidate into `TokenizeOptions { normalize: bool }`.

`pub fn inner()` exposes the concrete `StatelessTokenizer<Arc<JapaneseDictionary>>` type.
Change to `pub(crate)` unless a documented downstream adapter needs it.

`SearchError` is a single-variant String wrapper. Use `thiserror` with real variants
(`DictionaryLoad`, `TokenizationFailed`, etc.) so callers can pattern-match.

### P2-D: Extract three tokenization loops into one
`tokenize_internal`, `detect_compounds`, and `tokenize_with_compounds` each re-implement
Mode C + Mode B + byte-range nesting (~150 LOC of drift-prone duplication).
Extract `fn analyze(&self, input) -> Result<Analysis>` that all three delegate to.

---

## P3 — Architectural decisions (needs deliberation)

### P3-A: Decide on sudachi-postgres
A full ParadeDB fork is inside what's nominally a "tokenizer crate family." It has:
- Its own pgrx version coupling (Postgres major version matrix)
- Its own release cadence
- Its own CI requirements (postgres server, pgrx toolchain)
- A "product" shape (ships a postgres extension, not a Rust crate)

**Option A (evict):** Move to `jurmarcus/sudachi-postgres`, consume `sudachi-search` via
crates.io or git dep. Cleaner separation, independent versioning.

**Option B (keep):** Justified only if the tokenizer and extension are co-evolving weekly
and atomic cross-crate changes happen regularly.

Current evidence points to Option A, but this needs a deliberate decision, not a cleanup task.

---

## Summary Table

| Expert | Top finding |
|--------|-------------|
| rust-architecture | wasm name collision; unpinned git dep; no workspace.lints |
| software-architecture | Tantivy stub is the worst thing in the repo; API surface too wide |
| devils-advocate | 5/7 decisions OVER-ENGINEERED; delete stub + duplicate justfiles |
| code-review | Compile error in tests; wasm metadata wrong; sqlite dep duplication |
| test-architect | No sqlite tests; compile error confirmed; false-pass in `just test` |
| cli-tui | Missing justfiles break mod dispatch; stale example name |
| performance-profiler | FFI panic boundary half-wired (create/delete unprotected); wasm broken |

## Execution order

```
BUG-1  (compile error)          ← do first, unblocks cargo test
BUG-2  (FFI panic boundary)     ← safety-critical
BUG-3  (missing justfiles)      ← tooling broken
BUG-4  (stale example)          ← trivial, do alongside BUG-3
P0-A   (delete tantivy stub)    ← workspace lies
P0-B   (CLAUDE.md fixes)        ← doc accuracy
P0-C   (rename wasm package)    ← name collision
BUG-5  (wasm resource layout)   ← fix wasm build
P1-A–H (workspace hygiene)      ← one commit each
P2-A–D (code quality)           ← as bandwidth allows
P3-A   (pgrx decision)          ← deliberate, not reactive
```
