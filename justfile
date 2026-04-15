# justfile - sudachi monorepo
# https://github.com/jurmarcus/sudachi
#
# Workspace crates (sudachi-search, sudachi-sqlite) use plain cargo.
# Excluded crates have their own justfiles:
#   crates/sudachi-wasm/   — wasm-pack build (just wasm <recipe>)
#   crates/sudachi-postgres/ — pgrx (use pgrx-* recipes below)

default:
    @just --list

# ============================================================================
# Wasm module (wasm-pack, excluded from workspace)
# ============================================================================

mod wasm 'crates/sudachi-wasm'

# ============================================================================
# Workspace builds
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

# Run all workspace tests (unit tests, no dictionary required)
test:
    cargo test

# Run workspace tests with output
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
# CI
# ============================================================================

# Full CI pass: fmt check + clippy + tests
ci:
    cargo fmt --all --check
    cargo clippy --all -- -D warnings
    cargo test

# ============================================================================
# Dictionary setup (shared across workspace crates)
# ============================================================================

export SUDACHI_DICT_PATH := env_var_or_default("SUDACHI_DICT_PATH", `find ~/.sudachi -name "system_full.dic" 2>/dev/null | head -1 || find ~/.sudachi -name "system_small.dic" 2>/dev/null | head -1 || echo ""`)

# Download and install Sudachi dictionary to ~/.sudachi/
dict-setup:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p ~/.sudachi
    if [ ! -f ~/.sudachi/system_full.dic ]; then
        echo "Downloading Sudachi dictionary..."
        curl -L https://github.com/WorksApplications/SudachiDict/releases/download/v20251022/sudachi-dictionary-20251022-full.zip -o /tmp/sudachi-dict.zip
        unzip -o /tmp/sudachi-dict.zip -d /tmp/sudachi-temp/
        cp /tmp/sudachi-temp/*/system_full.dic ~/.sudachi/
        cp /tmp/sudachi-temp/*/char.def ~/.sudachi/ 2>/dev/null || true
        cp /tmp/sudachi-temp/*/rewrite.def ~/.sudachi/ 2>/dev/null || true
        cp /tmp/sudachi-temp/*/unk.def ~/.sudachi/ 2>/dev/null || true
        rm -rf /tmp/sudachi-temp /tmp/sudachi-dict.zip
        echo "Dictionary installed to ~/.sudachi/"
    else
        echo "Dictionary already exists at ~/.sudachi/"
    fi

# Show resolved dictionary path
dict-path:
    @echo "SUDACHI_DICT_PATH: ${SUDACHI_DICT_PATH:-not found}"

# ============================================================================
# pgrx (lives in ~/CODE/paradedb — cargo pgrx required)
# ============================================================================

PARADEDB := env_home() + "/CODE/paradedb"

# Build pg_search with Sudachi via pgrx (requires: cargo install cargo-pgrx && cargo pgrx init)
pgrx-build:
    cd {{PARADEDB}} && cargo pgrx build -p pg_search --features icu,sudachi

# Check pg_search with Sudachi
pgrx-check:
    cd {{PARADEDB}} && cargo check -p pg_search --features icu,sudachi

# ============================================================================
# Utilities
# ============================================================================

# Show environment
env:
    @echo "SUDACHI_DICT_PATH: ${SUDACHI_DICT_PATH:-not set}"
    @echo "PWD: $(pwd)"
    @cargo --version
    @rustc --version
    @wasm-pack --version 2>/dev/null || echo "wasm-pack: not installed"

# Clean all build artifacts
clean:
    cargo clean
    rm -rf crates/sudachi-wasm/wasm/pkg/
