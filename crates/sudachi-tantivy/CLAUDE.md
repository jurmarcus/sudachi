# CLAUDE.md — sudachi-tantivy

Tantivy tokenizer adapter for Sudachi Japanese morphological analysis.

## What This Is

Implements `tantivy::tokenizer::Tokenizer` backed by `sudachi-search`. Used by
`jurmarcus/paradedb` as a git dependency to power the `pdb.sudachi` PostgreSQL tokenizer.

## Architecture

```
SudachiTokenizer
  ├── Inner::Standard(StatelessTokenizer)  ← Modes A, B, C
  └── Inner::Search(SearchTokenizer)       ← Search mode (B+C, from sudachi-search)

SudachiTokenStream<'a>
  └── Iterates Vec<TokenData> (pre-collected, no borrow issues)
      └── is_colocated: true → position doesn't advance
```

The pre-collection pattern (`collect_tokens()`) exists because Tantivy's `TokenStream`
trait requires `&mut self`, but `StatelessTokenizer::tokenize()` returns a struct
borrowing the tokenizer. Collecting into an owned `Vec<TokenData>` sidesteps this.

## Key Types

```rust
pub struct SudachiTokenizer {
    inner: Arc<TokenizerInner>,  // Standard or Search
    mode: SplitMode,
    token: Token,                // Tantivy token (reset per stream)
    use_normalized: bool,
}

pub enum SplitMode { A, B, C, Search }

pub struct TokenData {
    pub text: String,
    pub offset_from: usize,
    pub offset_to: usize,
    pub is_colocated: bool,
}
```

## Colocated Token Translation

```rust
// In SudachiTokenStream::advance():
self.token.position += if token_data.is_colocated { 0 } else { 1 };
```

Tantivy uses position arithmetic — colocated tokens keep the same position so the
search engine can match compound words AND their sub-tokens at the same document offset.

## Dictionary Loading

```rust
// SudachiTokenizer::new() reads SUDACHI_DICT_PATH from env
let dict_path = std::env::var("SUDACHI_DICT_PATH")?;

// SudachiTokenizer::with_dictionary() accepts a pre-loaded dict (preferred for paradedb)
pub fn with_dictionary(dictionary: Arc<JapaneseDictionary>, mode: SplitMode) -> Self
```

In paradedb, the `SudachiTok` wrapper in `tokenizers/src/sudachi.rs` uses
`Lazy<Option<Arc<SudachiTokenizer>>>` statics per mode — one initialization, shared.

## Tantivy Version Compatibility

This crate uses `tantivy-tokenizer-api = "0.6.0"`. When used from `jurmarcus/paradedb`,
the workspace `[patch.crates-io]` redirects this to paradedb's forked version:

```toml
[patch.crates-io]
tantivy-tokenizer-api = { git = "https://github.com/paradedb/tantivy.git", ... }
```

This means all tokenizer types unify — no duplicate `Token` struct confusion.

## Commands

```bash
just build       # Build from repo root
just test        # Workspace tests
just ci          # fmt check + clippy + tests
```

`sudachi-tantivy` is a workspace member so `just` at root covers it.

## Files

```
src/
├── lib.rs        # Re-exports: SudachiTokenizer, SudachiTokenStream, SplitMode, TokenData
├── tokenizer.rs  # SudachiTokenizer, SplitMode, SudachiError
└── stream.rs     # SudachiTokenStream, position arithmetic
```
