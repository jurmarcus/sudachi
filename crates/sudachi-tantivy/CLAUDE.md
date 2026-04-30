# CLAUDE.md — sudachi-tantivy

Tantivy `Tokenizer` adapter for Sudachi. Translates `is_colocated` into Tantivy's position arithmetic (no increment for colocated tokens).

## What this is

An rlib implementing `tantivy::tokenizer::Tokenizer` over Sudachi. Two distinct paths inside one struct:

- **Modes A / B / C** — single-pass `StatelessTokenizer::tokenize(text, mode, false)`
- **Search mode** — delegates to `sudachi_search::SearchTokenizer` for B+C two-pass output

Used by `pg_search` (the ParadeDB Postgres extension) as a git dep to power the `pdb.sudachi` cast.

## Architecture

```
SudachiTokenizer
  ├── inner: Arc<TokenizerInner>
  │     ├── Standard(StatelessTokenizer<Arc<JapaneseDictionary>>)   modes A/B/C
  │     └── Search(SearchTokenizer)                                 mode Search (B+C)
  ├── mode: SplitMode
  ├── token: Token                  (Tantivy's owned token, reset per stream)
  └── use_normalized: bool

SudachiTokenStream<'a>
  ├── tokens: Vec<TokenData>        (pre-collected, owns its strings)
  ├── token: &'a mut Token          (Tantivy's per-stream state)
  ├── index: usize                  (cursor into tokens)
  └── advance(): increments Tantivy position only if !is_colocated
```

## Why pre-collection

Tantivy's `Tokenizer::token_stream` requires `&mut self`. `StatelessTokenizer::tokenize` returns a `MorphemeList<…>` that borrows the underlying tokenizer (so it can iterate morphemes lazily). Combining the two would create a `&mut self` + `&'self … morphemes` overlap.

We sidestep this by collecting all morphemes into an owned `Vec<TokenData>` up front:

```rust
struct TokenData {
    text: String,
    offset_from: usize,
    offset_to: usize,
    is_colocated: bool,
}
```

The stream then walks this owned vec — no borrow tied to the tokenizer.

**Don't try to make this lazy.** The borrow checker will fight you, and the win is small (Sudachi's iterator already buffers internally).

## `is_colocated` translation

```rust
// stream.rs: advance()
self.token.position += if td.is_colocated { 0 } else { 1 };
```

That's the entire adapter contract. Tantivy uses position arithmetic for phrase queries and proximity scoring; colocated tokens stay at the same position so a query for `大学` matches a document containing `東京都立大学`.

## Imports

```rust
// CORRECT — through the gateway
use sudachi_optimizer::sudachi::{
    JapaneseDictionary, Mode, StatelessTokenizer, SudachiError, Tokenize,
};

// WRONG — never reach upstream sudachi directly
use sudachi::dic::dictionary::JapaneseDictionary;
```

If a needed Sudachi type isn't yet re-exported, add it to `sudachi-optimizer/src/sudachi.rs` first.

## Files

```
src/lib.rs        Re-exports: SudachiTokenizer, SudachiTokenStream, SplitMode, TokenData, SudachiError
src/tokenizer.rs  SudachiTokenizer + SplitMode + TokenizerInner enum + token_stream() impl
src/stream.rs     SudachiTokenStream<'a>, position arithmetic, advance()
Cargo.toml        tantivy-tokenizer-api 0.6.0 + sudachi-optimizer + sudachi-search
```

## Tantivy API version compatibility

`Cargo.toml` pins `tantivy-tokenizer-api = "0.6.0"` (crates.io version). When this crate is consumed by `jurmarcus/paradedb`, paradedb's workspace `[patch.crates-io]` redirects to its forked tantivy:

```toml
# In paradedb's Cargo.toml
[patch.crates-io]
tantivy-tokenizer-api = { git = "https://github.com/paradedb/tantivy.git", ... }
```

The API surface is field-compatible — the same `Token` struct shape — so the patch is transparent. Don't introduce `tantivy-tokenizer-api` features that the paradedb fork doesn't carry.

## Default mode and normalisation

```rust
SudachiTokenizer::new(SplitMode::Search)
    .uses_normalized_form()   // → true by default
```

Normalised + Search mode is the recommended default for FTS use. Change defaults only if a downstream consumer's expectations are about to flip.

## Commands

Run from the repo root (workspace `just`):

```bash
just build         # workspace release build
just test          # workspace tests
just ci            # fmt check + clippy + tests
just pgrx-build    # build pg_search with this crate enabled
just pgrx-check    # cargo check the same target
```

## When changing this crate

### Add a new SplitMode

1. Variant in `SplitMode` enum.
2. Arm in `to_sudachi_mode()` (if Standard) or new `TokenizerInner` variant (if it needs a different backend).
3. Constructor arms in both `new()` and `with_dictionary()`.

### Change default normalisation

Modify `use_normalized: true` in `SudachiTokenizer::new()` and `with_dictionary()`. Bump consumer expectations in paradedb the same change.

### Debug wrong positions

Add temporary logging to `SudachiTokenStream::advance()`:

```rust
eprintln!("position={} is_colocated={} text={}",
          self.token.position, td.is_colocated, td.text);
```

Then check that colocated tokens keep the same position and non-colocated tokens advance by 1.

## Common issues

| Symptom                          | Likely cause                                                | Fix                                          |
| -------------------------------- | ----------------------------------------------------------- | -------------------------------------------- |
| Colocated tokens advance position | `advance()` increments unconditionally                      | Restore the `if !is_colocated` guard         |
| Phrase queries miss compounds    | Search mode not selected                                    | Construct with `SplitMode::Search`           |
| Type mismatch on `Token` in paradedb | Patch on `tantivy-tokenizer-api` missing                    | Verify `[patch.crates-io]` in paradedb       |
| Dictionary load fails at construction | `SUDACHI_DICT_PATH` not set                                | Use `with_dictionary(...)` and load yourself |

## Performance

- Dictionary: shared via `Arc<JapaneseDictionary>` — pass the same dict to multiple `SudachiTokenizer` instances if you have several modes.
- Per-stream cost: one Sudachi tokenisation pass (or two for Search mode), then iteration over `Vec<TokenData>`. Allocations: 1× owned vec per stream.
- ParadeDB-side optimisation: `Lazy<Option<Arc<SudachiTokenizer>>>` per mode in `pg_search`'s `tokenizers/src/sudachi.rs` keeps initialisation cost amortised over the connection lifetime.
