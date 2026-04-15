//! # sudachi-sqlite
//!
//! A SQLite FTS5 tokenizer extension using [Sudachi](https://github.com/WorksApplications/sudachi.rs),
//! enabling Japanese full-text search in SQLite databases.
//!
//! ## Features
//!
//! - **B+C multi-granularity**: Indexes both compound words and sub-tokens
//! - **Normalized form by default**: Better recall (附属 matches 付属)
//! - **Surface form option**: Use original text when precision matters
//!
//! ## Usage
//!
//! ### Building the Extension
//!
//! ```bash
//! cargo build --release
//! ```
//!
//! ### Setting Up Dictionary
//!
//! Set the `SUDACHI_DICT_PATH` environment variable to point to your Sudachi dictionary file:
//!
//! ```bash
//! export SUDACHI_DICT_PATH=/path/to/system.dic
//! ```
//!
//! ### Loading in SQLite
//!
//! ```sql
//! .load ./target/release/libsudachi_sqlite sudachi_fts5_tokenizer_init
//! ```
//!
//! ### Creating an FTS5 Table
//!
//! ```sql
//! -- Normalized form (default, better recall)
//! CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer');
//!
//! -- Surface form (use original text, better precision)
//! CREATE VIRTUAL TABLE docs USING fts5(content, tokenize='sudachi_tokenizer surface');
//! ```
//!
//! ### Searching
//!
//! ```sql
//! INSERT INTO docs(content) VALUES ('東京都立大学で会議が開催された');
//! SELECT * FROM docs WHERE content MATCH '大学';
//! ```

extern crate alloc;

mod common;
mod extension;

use libc::{c_char, c_int, c_uchar, c_void};
use std::sync::Arc;

use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use sudachi_search::SearchTokenizer;

pub use crate::common::*;

/// Auto-discover Sudachi dictionary in ~/.sudachi/
///
/// Looks for `~/.sudachi/sudachi-dictionary-*/system_full.dic` or `system_small.dic`.
fn discover_dictionary() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let sudachi_dir = std::path::PathBuf::from(&home).join(".sudachi");

    if !sudachi_dir.exists() {
        return None;
    }

    let entries = std::fs::read_dir(&sudachi_dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name()?.to_string_lossy();
            if name.starts_with("sudachi-dictionary") {
                // Prefer system_full.dic, fall back to system_small.dic
                let full_path = path.join("system_full.dic");
                if full_path.exists() {
                    eprintln!(
                        "sudachi-sqlite: auto-discovered dictionary at {:?}",
                        full_path
                    );
                    return Some(full_path.to_string_lossy().to_string());
                }
                let small_path = path.join("system_small.dic");
                if small_path.exists() {
                    eprintln!(
                        "sudachi-sqlite: auto-discovered dictionary at {:?}",
                        small_path
                    );
                    return Some(small_path.to_string_lossy().to_string());
                }
            }
        }
    }

    None
}

/// Loads and initializes a Sudachi search tokenizer.
///
/// This function creates a new search tokenizer using the dictionary specified
/// by the `SUDACHI_DICT_PATH` environment variable. If not set, it will
/// auto-discover the dictionary in `~/.sudachi/sudachi-dictionary-*/system_full.dic`.
///
/// The tokenizer uses B+C multi-granularity strategy for optimal Japanese full-text search.
///
/// By default, uses **normalized form** for better search recall.
/// Pass `use_surface_form = true` to disable normalization.
#[inline]
pub fn load_tokenizer(use_surface_form: bool) -> Result<SearchTokenizer, c_int> {
    let dict_path = std::env::var("SUDACHI_DICT_PATH")
        .ok()
        .or_else(discover_dictionary)
        .ok_or_else(|| {
            eprintln!(
                "sudachi-sqlite: SUDACHI_DICT_PATH not set and no dictionary found in ~/.sudachi/"
            );
            SQLITE_INTERNAL
        })?;

    let dict_path = std::path::PathBuf::from(&dict_path);

    // Verify dictionary exists
    if !dict_path.exists() {
        eprintln!("sudachi-sqlite: dictionary not found at {:?}", dict_path);
        return Err(SQLITE_INTERNAL);
    }

    // Read dictionary into memory
    let dict_bytes = std::fs::read(&dict_path).map_err(|e| {
        eprintln!("sudachi-sqlite: failed to read dictionary: {}", e);
        SQLITE_INTERNAL
    })?;

    // Create storage from dictionary bytes
    let storage = SudachiDicData::new(Storage::Owned(dict_bytes));

    // Create minimal config with OOV support
    let resource_dir = dict_path.parent().unwrap_or(std::path::Path::new("."));
    let config = Config::minimal_at(resource_dir);

    // Create the dictionary with embedded char.def
    let dictionary = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage)
        .map_err(|e| {
        eprintln!("sudachi-sqlite: failed to load dictionary: {}", e);
        SQLITE_INTERNAL
    })?;

    // Create tokenizer with appropriate form setting
    // Default: normalized form for better recall
    // With "surface" option: use original surface form
    let tokenizer = SearchTokenizer::new(Arc::new(dictionary));
    if use_surface_form {
        Ok(tokenizer.with_surface_form())
    } else {
        Ok(tokenizer)
    }
}

/// C-compatible FTS5 tokenization function.
#[unsafe(no_mangle)]
pub extern "C" fn sudachi_fts5_tokenize(
    tokenizer: *mut Fts5Tokenizer,
    p_ctx: *mut c_void,
    _flags: c_int,
    p_text: *const c_char,
    n_text: c_int,
    x_token: TokenFunction,
) -> c_int {
    crate::common::ffi_panic_boundary(|| {
        sudachi_fts5_tokenize_internal(tokenizer, p_ctx, p_text, n_text, x_token)?;
        Ok(())
    })
}

#[inline]
fn sudachi_fts5_tokenize_internal(
    tokenizer: *mut Fts5Tokenizer,
    p_ctx: *mut c_void,
    p_text: *const c_char,
    n_text: c_int,
    x_token: TokenFunction,
) -> Result<(), c_int> {
    if n_text <= 0 {
        return Ok(());
    }

    let input = unsafe { InputText::from_raw_parts(p_text, n_text)? };
    let mut tokenizer = unsafe { TokenizerHandle::new(tokenizer)? };
    let callback = crate::common::TokenCallback::new(p_ctx, x_token);

    tokenizer.emit_tokens(input.as_str(), &callback)
}

struct TokenizerHandle<'a> {
    inner: &'a mut Fts5Tokenizer,
}

impl<'a> TokenizerHandle<'a> {
    unsafe fn new(ptr: *mut Fts5Tokenizer) -> Result<Self, c_int> {
        let inner = unsafe { ptr.as_mut() }.ok_or(SQLITE_INTERNAL)?;
        Ok(Self { inner })
    }

    /// Emit tokens using B+C multi-granularity strategy via sudachi-search.
    ///
    /// Delegates to `SearchTokenizer::tokenize()` and translates `is_colocated`
    /// to the FTS5_TOKEN_COLOCATED flag.
    fn emit_tokens(
        &mut self,
        input: &str,
        callback: &crate::common::TokenCallback,
    ) -> Result<(), c_int> {
        // Use the shared B+C tokenization from sudachi-search
        let tokens = self.inner.tokenizer.tokenize(input).map_err(|e| {
            eprintln!("sudachi-sqlite: tokenization error: {}", e);
            SQLITE_INTERNAL
        })?;

        // Emit each token, translating is_colocated to FTS5 flag
        for token in tokens {
            if token.is_colocated {
                callback.emit_colocated(
                    token.surface.as_bytes(),
                    token.byte_start,
                    token.byte_end,
                )?;
            } else {
                callback.emit(token.surface.as_bytes(), token.byte_start, token.byte_end)?;
            }
        }

        Ok(())
    }
}

struct InputText<'a> {
    text: &'a str,
}

impl<'a> InputText<'a> {
    unsafe fn from_raw_parts(ptr: *const c_char, len: c_int) -> Result<Self, c_int> {
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const c_uchar, len as usize) };
        // Invalid UTF-8 mapped to SQLITE_OK to prevent database inaccessibility
        let text = core::str::from_utf8(slice).map_err(|_| SQLITE_OK)?;
        Ok(Self { text })
    }

    fn as_str(&self) -> &str {
        self.text
    }
}
