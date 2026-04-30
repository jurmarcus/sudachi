# AGENTS.md — sudachi-tantivy

Context for AI agents working on this crate.

## Purpose

Implements `tantivy::tokenizer::Tokenizer` over Sudachi. Translates `is_colocated` into Tantivy's position arithmetic. The primary downstream consumer is `pg_search` (the ParadeDB Postgres extension) which pulls this crate as a git dep.

| Attribute  | Value                                                       |
| ---------- | ----------------------------------------------------------- |
| Type       | rlib                                                        |
| Output     | `tantivy::tokenizer::Tokenizer` impl                        |
| Backends   | `StatelessTokenizer` (modes A/B/C), `SearchTokenizer` (B+C) |
| Deps       | `tantivy-tokenizer-api 0.6.0`, `sudachi-optimizer`, `sudachi-search` |

## File map

```
src/lib.rs        Re-exports: SudachiTokenizer, SudachiTokenStream, SplitMode, TokenData, SudachiError
src/tokenizer.rs  SudachiTokenizer struct, SplitMode enum, TokenizerInner private enum, token_stream() impl
src/stream.rs     SudachiTokenStream, advance() — the position-arithmetic core
Cargo.toml        tantivy-tokenizer-api = "0.6.0" + sudachi-optimizer + sudachi-search
```

## Hard rules

1. **Don't increment position on colocated tokens.** The single line in `advance()`:
   ```rust
   self.token.position += if td.is_colocated { 0 } else { 1 };
   ```
   Touch this and you break the entire compound-word search behaviour.

2. **Pre-collect tokens.** `token_stream` collects into `Vec<TokenData>` before returning. Don't refactor this to be lazy — `StatelessTokenizer::tokenize` returns a borrow-tied iterator that conflicts with Tantivy's `&mut self`.

3. **Imports through the gateway.** Use `sudachi_optimizer::sudachi::*`, never `sudachi::*` directly.

4. **Don't change `tantivy-tokenizer-api` version casually.** ParadeDB's workspace patches this dep to a forked version; both must agree on the API shape.

## Search mode vs Standard modes

```rust
enum TokenizerInner {
    Standard(StatelessTokenizer<Arc<JapaneseDictionary>>),  // A, B, C
    Search(SearchTokenizer),                                 // Search (B+C)
}
```

Standard modes call `StatelessTokenizer::tokenize(text, mode, false)` directly — single-pass.
Search mode delegates to `sudachi_search::SearchTokenizer` which does the B+C two-pass logic and produces `is_colocated` tokens.

## Pre-collection contract

```rust
fn token_stream<'a>(&'a mut self, text: &'a str) -> SudachiTokenStream<'a> {
    let token_data: Vec<TokenData> = match &*self.inner {
        TokenizerInner::Standard(tok) => collect_standard(tok, text, self.mode),
        TokenizerInner::Search(tok)   => collect_search(tok, text),
    };

    self.token.reset();
    SudachiTokenStream {
        tokens: token_data,
        token: &mut self.token,
        index: 0,
    }
}
```

`TokenData` is owned strings — once the vec is built, the stream has no further dependency on the tokenizer.

## When changing this crate

### Add a new `SplitMode` variant

1. Variant in `SplitMode` enum (`tokenizer.rs`).
2. Arm in `to_sudachi_mode()` if it maps to a built-in Sudachi mode.
3. Otherwise add a new `TokenizerInner` variant and constructor arms in `new()` and `with_dictionary()`.
4. Test: token a sample input, verify the produced `Vec<TokenData>` shape.

### Change normalisation default

Both `new()` and `with_dictionary()` set `use_normalized: true`. If you flip this default, audit `pg_search`'s `tokenizers/src/sudachi.rs` — its `SudachiTok` wrapper assumes normalised output.

### Debug wrong positions

```rust
// In SudachiTokenStream::advance()
eprintln!("pos={:>3} colocated={} {:?}",
          self.token.position, td.is_colocated, td.text);
```

Run a tokenisation, eyeball the output. Colocated lines must repeat the previous position.

### Update what paradedb sees

After committing here:

```bash
sl push
cd ~/CODE/paradedb
cargo update -p sudachi-tantivy
just pgrx-check
```

Paradedb pulls this crate by git rev — `cargo update` fetches the new HEAD.

## Tantivy API version

`tantivy-tokenizer-api = "0.6.0"` works with both crates.io tantivy and ParadeDB's forked tantivy (because the fork preserves the same `Token` struct shape). Don't introduce dependencies on tantivy features that aren't in 0.6.0.

## Common tasks

### Reproduce a position bug

```rust
let mut tok = SudachiTokenizer::new(SplitMode::Search)?;
let mut stream = tok.token_stream("東京都立大学");
while stream.advance() {
    let t = stream.token();
    eprintln!("{:?} pos={}", t.text, t.position);
}
// Expected: position 0 four times (compound + 3 colocated), then 1 advances.
```

### Verify the patch chain in paradedb

```bash
cd ~/CODE/paradedb
cargo tree -p pg_search --features sudachi | grep -E "tantivy|sudachi"
```

The `tantivy-tokenizer-api` line should resolve to the patched git URL, not crates.io.

## Performance

- One `Vec<TokenData>` allocation per `token_stream` call.
- Each `TokenData` owns its `String` — no reference back to the input.
- For high-volume pipelines (paradedb indexing), `Lazy<Option<Arc<SudachiTokenizer>>>` per mode amortises construction.
