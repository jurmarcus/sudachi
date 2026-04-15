# justfile - sudachi monorepo
# https://github.com/jurmarcus/sudachi

default:
    @just --list

# ============================================================================
# Crate modules
# ============================================================================

mod search 'crates/sudachi-search'
mod tantivy 'crates/sudachi-tantivy'
mod sqlite 'crates/sudachi-sqlite'
mod wasm 'crates/sudachi-wasm'

# ============================================================================
# Workspace builds (excludes pgrx)
# ============================================================================

# Build all workspace crates (release)
build:
    cargo build --release

# Build all workspace crates (dev)
build-dev:
    cargo build

# Check all workspace crates
check:
    cargo check

# ============================================================================
# Testing
# ============================================================================

# Run all workspace tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# ============================================================================
# Code quality
# ============================================================================

# Format all workspace crates
fmt:
    cargo fmt --all

# Lint all workspace crates
lint:
    cargo clippy --all -- -D warnings

# Format and lint
fix: fmt lint

# ============================================================================
# Dictionary setup (shared across crates)
# ============================================================================

export SUDACHI_DICT_PATH := env_var_or_default("SUDACHI_DICT_PATH", `find ~/.sudachi -name "system_full.dic" 2>/dev/null | head -1 || find ~/.sudachi -name "system_small.dic" 2>/dev/null | head -1 || echo ""`)

# Download and install Sudachi dictionary
dict-setup:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p ~/.sudachi
    if [ ! -d ~/.sudachi/sudachi-dictionary-* ]; then
        echo "Downloading Sudachi dictionary..."
        curl -L https://github.com/WorksApplications/SudachiDict/releases/download/v20251022/sudachi-dictionary-20251022-full.zip -o /tmp/sudachi-dict.zip
        unzip -o /tmp/sudachi-dict.zip -d ~/.sudachi/
        rm /tmp/sudachi-dict.zip
        echo "Dictionary installed to ~/.sudachi/"
    else
        echo "Dictionary already exists"
    fi

# Show dictionary path
dict-path:
    @echo "SUDACHI_DICT_PATH: ${SUDACHI_DICT_PATH:-not found}"

# ============================================================================
# pgrx (own nested workspace - run from crates/sudachi-postgres/)
# ============================================================================

# Build sudachi-postgres (pgrx)
pgrx-build:
    cargo build --release --manifest-path crates/sudachi-postgres/Cargo.toml

# Check sudachi-postgres
pgrx-check:
    cargo check --manifest-path crates/sudachi-postgres/Cargo.toml

# ============================================================================
# Utilities
# ============================================================================

# Show environment
env:
    @echo "SUDACHI_DICT_PATH: ${SUDACHI_DICT_PATH:-not set}"
    @echo "PWD: $(pwd)"
    @cargo --version
    @rustc --version

# Clean all build artifacts
clean:
    cargo clean
    rm -rf crates/sudachi-wasm/pkg/

# Watch workspace for changes
watch:
    cargo watch -x check
