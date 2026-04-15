# AGENTS.md — sudachi-tantivy

Context for AI agents working on this crate.

## Purpose

Implements `tantivy::tokenizer::Tokenizer` backed by Sudachi. Primary consumer is
`jurmarcus/paradedb` which pulls this crate as a git dep to power `pdb.sudachi`.

## File Map

```
src/lib.rs        Re-exports (SudachiTokenizer, SudachiTokenStream, SplitMode, TokenData)
src/tokenizer.rs  SudachiTokenizer: new(), with_dictionary(), token_stream()
src/stream.rs     SudachiTokenStream: position arithmetic, is_colocated handling
Cargo.toml        tantivy-tokenizer-api, tantivy, sudachi (git), sudachi-search (path)
```

## Critical: Colocated Position Arithmetic

```rust
// stream.rs: advance() must do this EXACTLY
self.token.position += if token_data.is_colocated { 0 } else { 1 };
```

Do NOT increment position for colocated tokens. This is how Tantivy knows compound word
sub-tokens are at the same index position as the compound itself.

## Critical: Pre-Collection Pattern

`token_stream()` collects ALL tokens into a `Vec<TokenData>` before returning the stream.
This is intentional — the `StatelessTokenizer::tokenize()` result borrows the tokenizer,
which conflicts with Tantivy's `&mut self` requirement on `token_stream`. Pre-collecting
into owned data sidesteps the borrow entirely.

Do NOT try to make this lazy/streaming — the borrow checker will fight you.

## Tantivy API Version

`tantivy-tokenizer-api = "0.6.0"` is in Cargo.toml (crates.io version). When this crate
is built inside `jurmarcus/paradedb`, the workspace `[patch.crates-io]` replaces it with
paradedb's forked version. The API surface is compatible — same `Token` fields.

## Search Mode vs Standard Modes

```rust
enum TokenizerInner {
    Standard(StatelessTokenizer<Arc<JapaneseDictionary>>),  // Modes A, B, C
    Search(SearchTokenizer),                                 // Mode Search (B+C)
}
```

Standard modes call `StatelessTokenizer::tokenize(text, mode, false)` directly.
Search mode delegates to `sudachi_search::SearchTokenizer` which does the B+C two-pass logic.

## Common Tasks

**Change normalization default:** modify `use_normalized: true` in `SudachiTokenizer::new()`
and `with_dictionary()`.

**Add a new SplitMode:** add variant to `SplitMode`, add arm to `to_sudachi_mode()`,
add arm to `TokenizerInner` construction in `new()` and `with_dictionary()`.

**Debug wrong positions:** add temp logging in `SudachiTokenStream::advance()` to print
`(token.position, token_data.is_colocated)` — verify colocated tokens don't increment.
