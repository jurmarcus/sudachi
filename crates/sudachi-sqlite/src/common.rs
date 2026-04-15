//! Common types and constants shared across the extension.
//!
//! This module defines the fundamental types and constants used for FFI communication
//! between Rust and SQLite's C API.

use core::convert::TryFrom;
use libc::{c_char, c_int, c_void};
use sudachi_search::SearchTokenizer;

/// SQLite success status code.
pub const SQLITE_OK: c_int = 0;

/// SQLite internal error status code.
pub const SQLITE_INTERNAL: c_int = 2;

/// SQLite misuse error status code.
pub const SQLITE_MISUSE: c_int = 21;

/// FTS5 token flag: colocated token (same position as previous).
/// Used for multi-granularity indexing (e.g., both compound word and its parts).
pub const FTS5_TOKEN_COLOCATED: c_int = 0x0001;

/// Wrapper for Sudachi tokenizer used in FTS5.
pub struct Fts5Tokenizer {
    pub tokenizer: SearchTokenizer,
}

/// Convenience wrapper around SQLite's token callback.
pub struct TokenCallback {
    context: *mut c_void,
    function: TokenFunction,
}

impl TokenCallback {
    /// Creates a new callback wrapper.
    pub const fn new(context: *mut c_void, function: TokenFunction) -> Self {
        Self { context, function }
    }

    /// Emits a token back to SQLite.
    pub fn emit(&self, token: &[u8], byte_start: usize, byte_end: usize) -> Result<(), c_int> {
        self.emit_with_flags(token, byte_start, byte_end, 0)
    }

    /// Emits a colocated token (same position as previous token).
    /// Used for multi-granularity indexing where both compound words
    /// and their sub-tokens are indexed at the same position.
    pub fn emit_colocated(
        &self,
        token: &[u8],
        byte_start: usize,
        byte_end: usize,
    ) -> Result<(), c_int> {
        self.emit_with_flags(token, byte_start, byte_end, FTS5_TOKEN_COLOCATED)
    }

    /// Emits a token with custom flags.
    fn emit_with_flags(
        &self,
        token: &[u8],
        byte_start: usize,
        byte_end: usize,
        flags: c_int,
    ) -> Result<(), c_int> {
        let token_len = usize_to_c_int(token.len())?;
        let start = usize_to_c_int(byte_start)?;
        let end = usize_to_c_int(byte_end)?;

        let status = (self.function)(
            self.context,
            flags,
            token.as_ptr() as *const c_char,
            token_len,
            start,
            end,
        );

        if status == SQLITE_OK {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Convert usize to c_int with overflow checking.
#[inline]
pub fn usize_to_c_int(value: usize) -> Result<c_int, c_int> {
    c_int::try_from(value).map_err(|_| SQLITE_INTERNAL)
}

/// Runs an operation behind a panic boundary suitable for the SQLite FFI.
pub fn ffi_panic_boundary<F>(operation: F) -> c_int
where
    F: FnOnce() -> Result<(), c_int>,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation)) {
        Ok(Ok(())) => SQLITE_OK,
        Ok(Err(code)) => code,
        Err(_) => {
            eprintln!("sudachi-sqlite: panic caught at FFI boundary");
            SQLITE_INTERNAL
        }
    }
}

/// Token callback function type.
pub type TokenFunction = extern "C" fn(
    p_ctx: *mut c_void,
    t_flags: c_int,
    p_token: *const c_char,
    n_token: c_int,
    i_start: c_int,
    i_end: c_int,
) -> c_int;
