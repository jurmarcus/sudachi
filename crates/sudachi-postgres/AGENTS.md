# AGENTS.md - sudachi-postgres

Context for AI agents working on this codebase.

## Project Summary

**sudachi-postgres** is a ParadeDB fork with Sudachi Japanese tokenizer integration for PostgreSQL BM25 full-text search.

| Attribute | Value |
|-----------|-------|
| Complexity | Medium (~500 LOC additions to ParadeDB) |
| Pattern | Feature addition to existing extension |
| Key Feature | `pdb.sudachi` tokenizer type |
| Upstream | ParadeDB, sudachi-tantivy |

## The Problem

Japanese compound words break traditional PostgreSQL full-text search:

```
Document: "東京都立大学で研究しています"
Query: "大学"

ParadeDB Lindera: ["東京都立大学", ...]
Result: NO MATCH - "大学" trapped inside compound

Sudachi (B+C):
  pos 0: "東京都立大学" (compound)
  pos 0: "東京"         (colocated)
  pos 0: "都立"         (colocated)
  pos 0: "大学"         (colocated)  ← NOW SEARCHABLE
Result: MATCH!
```

## File Structure

```
sudachi-postgres/                      # ParadeDB fork
├── pg_search/
│   ├── Cargo.toml                    # MODIFIED: sudachi feature
│   └── src/api/tokenizers/
│       ├── definitions.rs            # MODIFIED: define_tokenizer_type!(Sudachi)
│       ├── mod.rs                    # MODIFIED: SearchTokenizer::Sudachi
│       └── typmod/
│           └── definitions.rs        # MODIFIED: SudachiTypmod
├── tokenizers/
│   ├── Cargo.toml                    # MODIFIED: sudachi-tantivy dependency
│   ├── src/
│   │   ├── lib.rs                    # MODIFIED: export sudachi module
│   │   ├── manager.rs                # MODIFIED: SearchTokenizer::Sudachi variant
│   │   └── sudachi.rs                # NEW: SudachiTokenizer wrapper
├── CLAUDE.md
├── SUDACHI.md
└── AGENTS.md
```

## Critical Implementation Details

### Tokenizer Registration (definitions.rs)

```rust
// pg_search/src/api/tokenizers/definitions.rs
define_tokenizer_type!(
    Sudachi,
    "sudachi",
    oid = 4200,                       // Unique OID for pdb.sudachi type
    default = TokenizerDefault::Sudachi(SudachiTypmod::default()),
    typmod_in = sudachi_typmod_in,
    typmod_out = sudachi_typmod_out,
    category = TypeCategory::User
);
```

### SudachiTypmod (typmod/definitions.rs)

```rust
#[derive(Clone, Debug, Default)]
pub struct SudachiTypmod {
    pub mode: SudachiMode,            // 'a', 'b', 'c', 'search'
    pub normalized: bool,             // Default: true
}

impl SudachiTypmod {
    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        let mode = match s.to_lowercase().as_str() {
            "a" => SudachiMode::A,
            "b" => SudachiMode::B,
            "c" => SudachiMode::C,
            "search" | "" => SudachiMode::Search,
            _ => return Err(ParseError::InvalidMode),
        };
        Ok(Self { mode, normalized: true })
    }
}
```

### SearchTokenizer Variant (manager.rs)

```rust
// tokenizers/src/manager.rs
pub enum SearchTokenizer {
    // ... existing variants ...

    #[cfg(feature = "sudachi")]
    Sudachi {
        mode: sudachi::SudachiMode,
        normalized: bool,
        filters: SearchTokenizerFilters,
    },
}

impl SearchTokenizer {
    pub fn sudachi(
        mode: sudachi::SudachiMode,
        normalized: bool,
        filters: SearchTokenizerFilters,
    ) -> Self {
        Self::Sudachi { mode, normalized, filters }
    }
}
```

### SudachiTokenizer Wrapper (sudachi.rs)

```rust
// tokenizers/src/sudachi.rs
use sudachi_tantivy::{SudachiTokenizer as Inner, SplitMode};

#[derive(Clone)]
pub struct SudachiTokenizer {
    inner: Inner,
    mode: SudachiMode,
}

impl SudachiTokenizer {
    pub fn new(mode: SudachiMode, normalized: bool) -> Result<Self, SudachiError> {
        let split_mode = mode.to_split_mode();
        let mut inner = Inner::new(split_mode)?;
        if !normalized {
            inner = inner.with_surface_form();
        }
        Ok(Self { inner, mode })
    }
}

impl Tokenizer for SudachiTokenizer {
    type TokenStream<'a> = <Inner as Tokenizer>::TokenStream<'a>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        self.inner.token_stream(text)
    }
}
```

## Integration Points

### Upstream: sudachi-tantivy

```rust
use sudachi_tantivy::{SudachiTokenizer, SplitMode};

// Mode mapping
impl SudachiMode {
    pub fn to_split_mode(&self) -> SplitMode {
        match self {
            SudachiMode::A => SplitMode::A,
            SudachiMode::B => SplitMode::B,
            SudachiMode::C => SplitMode::C,
            SudachiMode::Search => SplitMode::Search,
        }
    }
}
```

### ParadeDB Integration Pattern

ParadeDB uses a define macro pattern for tokenizer types:

```rust
// Follow existing patterns like Lindera:
define_tokenizer_type!(
    Lindera,
    "lindera",
    oid = 4100,
    ...
);

// Sudachi follows same pattern:
define_tokenizer_type!(
    Sudachi,
    "sudachi",
    oid = 4200,  // Next available OID
    ...
);
```

## SQL Interface

### Basic Usage

```sql
-- Default: Search mode (B+C), normalized
CREATE INDEX idx ON tbl USING bm25(id, (content::pdb.sudachi)) WITH (key_field='id');

-- Explicit mode
CREATE INDEX idx ON tbl USING bm25(id, (content::pdb.sudachi('search'))) WITH (key_field='id');
CREATE INDEX idx ON tbl USING bm25(id, (content::pdb.sudachi('c'))) WITH (key_field='id');
```

### Available Modes

| Mode | SQL | Description |
|------|-----|-------------|
| A | `'a'` | Finest granularity |
| B | `'b'` | Medium granularity |
| C | `'c'` | Coarsest granularity |
| Search | `'search'` | B+C multi-granularity (default) |

## Commands

```bash
# Setup
just dict-setup       # Download Sudachi dictionary (one-time)
just install-deps     # Install cargo-pgrx

# Build & Install
just build            # Build pg_search with Sudachi feature
just install          # Install extension to PostgreSQL

# PostgreSQL
just pg-start         # Start PostgreSQL
just pg-stop          # Stop PostgreSQL

# Testing
just test             # Run Rust tests
just test-sql         # Interactive SQL test session

# Development
just fix              # Format and lint code
just env              # Show environment info

# Full workflow
just setup            # Complete setup from scratch
```

All commands use `just` (task runner). Run `just --list` to see all available commands.

The dictionary is auto-discovered from `~/.sudachi/` - no environment variable needed.

## Common Issues

| Symptom | Cause | Fix |
|---------|-------|-----|
| `pdb.sudachi` type not found | Built without `sudachi` feature | Run `just build` (includes sudachi feature) |
| Empty search results | Dictionary not found | Run `just dict-setup` to download dictionary |
| Build fails on macOS | Missing linker flag | Use `just build` (includes RUSTFLAGS) |
| OID conflict | Duplicate OID in definitions.rs | Choose unused OID (4200+) |

## Debugging

### Check Tokenizer Registered

```sql
SELECT typname FROM pg_type
WHERE typnamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'pdb')
AND typname = 'sudachi';
-- Should return 'sudachi'
```

### Test Tokenization

```bash
# Run tests
just test

# Interactive SQL testing
just test-sql
```

### Check Dictionary Loading

The dictionary is auto-discovered from `~/.sudachi/`. Run `just env` to verify the detected path.

## When Modifying

### Adding New Tokenizer Options

1. Add field to `SudachiTypmod` struct
2. Update `from_str()` parser
3. Pass to `SudachiTokenizer::new()`
4. Update SQL interface documentation

### Changing Mode Behavior

1. Mode mapping is in `SudachiMode::to_split_mode()`
2. Actual tokenization logic is in sudachi-tantivy → sudachi-search
3. Don't modify tokenization here - modify upstream

### Updating sudachi-tantivy Dependency

1. Update version in `tokenizers/Cargo.toml`
2. Check for API changes in `SudachiTokenizer`
3. Run integration tests
4. Test with Japanese text examples

## Dependencies

```toml
# tokenizers/Cargo.toml
[dependencies]
sudachi-tantivy = { path = "../sudachi-tantivy", optional = true }

[features]
sudachi = ["sudachi-tantivy"]

# pg_search/Cargo.toml
[features]
sudachi = ["tokenizers/sudachi"]
```

## Feature Flag Structure

```
pg_search --features sudachi
    ↓
tokenizers --features sudachi
    ↓
sudachi-tantivy (path dependency)
    ↓
sudachi-search (git dependency)
    ↓
sudachi.rs (git dependency, develop branch)
```

## Performance Notes

- Dictionary loading: ~200ms (lazy, once per process)
- Tokenization: ~10K chars/sec
- Memory overhead: ~70MB for dictionary
- Dictionary is shared across all connections via lazy_static
