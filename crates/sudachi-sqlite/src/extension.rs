//! SQLite FTS5 extension integration.
//!
//! This module provides the C ABI implementation for integrating Sudachi tokenizer
//! as a SQLite FTS5 extension.

use core::ptr::null_mut;
use libc::{c_char, c_int, c_uchar, c_void};

use crate::common::*;
use crate::load_tokenizer;
use crate::sudachi_fts5_tokenize;

/// Opaque SQLite database handle.
pub type Sqlite3 = sqlite_loadable::prelude::sqlite3;

/// Opaque SQLite prepared statement handle.
type Sqlite3Stmt = sqlite3ext_sys::sqlite3_stmt;

/// FTS5 tokenizer API structure.
#[repr(C)]
struct Fts5TokenizerApi {
    x_create: extern "C" fn(
        p_context: *mut c_void,
        az_arg: *const *const c_uchar,
        n_arg: c_int,
        fts5_tokenizer: *mut *mut Fts5Tokenizer,
    ) -> c_int,
    x_delete: extern "C" fn(fts5_tokenizer: *mut Fts5Tokenizer),
    x_tokenize: extern "C" fn(
        tokenizer: *mut Fts5Tokenizer,
        p_ctx: *mut c_void,
        flags: c_int,
        p_text: *const c_char,
        n_text: c_int,
        x_token: TokenFunction,
    ) -> c_int,
}

/// FTS5 API structure.
#[repr(C)]
struct FTS5API {
    i_version: c_int,
    x_create_tokenizer: extern "C" fn(
        fts5_api: *const FTS5API,
        z_name: *const c_uchar,
        p_context: *mut c_void,
        fts5_tokenizer: *mut Fts5TokenizerApi,
        x_destroy: extern "C" fn(module: *mut c_void),
    ) -> c_int,
}

/// SQLite extension API function table.
pub type Sqlite3APIRoutines = sqlite_loadable::prelude::sqlite3_api_routines;

struct SqliteApi<'a> {
    raw: &'a Sqlite3APIRoutines,
}

impl<'a> SqliteApi<'a> {
    unsafe fn new(p_api: *const c_void) -> Result<Self, c_int> {
        let raw =
            unsafe { (p_api as *const Sqlite3APIRoutines).as_ref() }.ok_or(SQLITE_INTERNAL)?;
        Ok(Self { raw })
    }

    fn ensure_supported_version(&self) -> Result<(), c_int> {
        let libversion_number = self.raw.libversion_number.ok_or(SQLITE_INTERNAL)?;
        if unsafe { libversion_number() } < 302000 {
            Err(SQLITE_MISUSE)
        } else {
            Ok(())
        }
    }

    fn prepare_statement(
        &'a self,
        db: *mut Sqlite3,
        sql: *const c_uchar,
    ) -> Result<PreparedStatement<'a>, c_int> {
        let mut stmt = null_mut::<Sqlite3Stmt>();
        let prepare = self.raw.prepare.ok_or(SQLITE_INTERNAL)?;
        let rc = unsafe { prepare(db, sql.cast::<c_char>(), -1, &mut stmt, null_mut()) };
        if rc != SQLITE_OK {
            return Err(rc);
        }
        Ok(PreparedStatement::new(self, stmt))
    }

    fn bind_fts5_pointer(
        &self,
        stmt: *mut Sqlite3Stmt,
        target: &mut *mut FTS5API,
    ) -> Result<(), c_int> {
        let bind_pointer = self.raw.bind_pointer.ok_or(SQLITE_INTERNAL)?;
        let rc = unsafe {
            bind_pointer(
                stmt,
                1,
                target as *mut *mut FTS5API as *mut c_void,
                c"fts5_api_ptr".as_ptr() as *const c_char,
                None,
            )
        };
        if rc == SQLITE_OK { Ok(()) } else { Err(rc) }
    }

    fn step(&self, stmt: *mut Sqlite3Stmt) -> c_int {
        let step = match self.raw.step {
            Some(f) => f,
            None => return SQLITE_INTERNAL,
        };
        unsafe { step(stmt) }
    }

    fn finalize(&self, stmt: *mut Sqlite3Stmt) -> c_int {
        let finalize = match self.raw.finalize {
            Some(f) => f,
            None => return SQLITE_INTERNAL,
        };
        unsafe { finalize(stmt) }
    }
}

struct PreparedStatement<'api> {
    stmt: *mut Sqlite3Stmt,
    api: &'api SqliteApi<'api>,
    finalized: bool,
}

impl<'api> PreparedStatement<'api> {
    fn new(api: &'api SqliteApi<'api>, stmt: *mut Sqlite3Stmt) -> Self {
        Self {
            stmt,
            api,
            finalized: false,
        }
    }

    fn bind_fts5_pointer(&mut self, target: &mut *mut FTS5API) -> Result<(), c_int> {
        self.api.bind_fts5_pointer(self.stmt, target)
    }

    fn step(&mut self) {
        self.api.step(self.stmt);
    }

    fn finalize(mut self) -> Result<(), c_int> {
        let rc = self.api.finalize(self.stmt);
        self.finalized = true;
        if rc == SQLITE_OK { Ok(()) } else { Err(rc) }
    }
}

impl Drop for PreparedStatement<'_> {
    fn drop(&mut self) {
        if !self.finalized
            && !self.stmt.is_null()
            && let Some(finalize) = self.api.raw.finalize
        {
            unsafe {
                finalize(self.stmt);
            }
        }
    }
}

/// Extension initialization entry point.
#[unsafe(no_mangle)]
pub extern "C" fn sudachi_fts5_tokenizer_init(
    db: *mut Sqlite3,
    _pz_err_msg: *mut *mut c_uchar,
    p_api: *const c_void,
) -> c_int {
    crate::common::ffi_panic_boundary(|| {
        sudachi_fts_tokenizer_internal_init(db, p_api)?;
        Ok(())
    })
}

fn sudachi_fts_tokenizer_internal_init(
    db: *mut Sqlite3,
    p_api: *const c_void,
) -> Result<(), c_int> {
    let api = unsafe { SqliteApi::new(p_api)? };
    api.ensure_supported_version()?;

    let mut stmt = api.prepare_statement(db, c"SELECT fts5(?1)".as_ptr() as *const u8)?;
    let mut p_fts5_api = null_mut::<FTS5API>();
    stmt.bind_fts5_pointer(&mut p_fts5_api)?;
    stmt.step();
    stmt.finalize()?;

    let fts5_api = unsafe { p_fts5_api.as_ref() }.ok_or(SQLITE_INTERNAL)?;
    ensure_fts5_api_version(fts5_api)?;
    register_sudachi_tokenizer(fts5_api);

    Ok(())
}

fn ensure_fts5_api_version(fts5_api: &FTS5API) -> Result<(), c_int> {
    // Accept version >= 2 (we only use the basic xCreateTokenizer API)
    if fts5_api.i_version >= 2 {
        Ok(())
    } else {
        eprintln!(
            "sudachi-sqlite: FTS5 API version {} is too old",
            fts5_api.i_version
        );
        Err(SQLITE_MISUSE)
    }
}

fn register_sudachi_tokenizer(fts5_api: &FTS5API) {
    let mut tokenizer = Fts5TokenizerApi {
        x_create: fts5_create_sudachi_tokenizer,
        x_delete: fts5_delete_sudachi_tokenizer,
        x_tokenize: sudachi_fts5_tokenize,
    };

    (fts5_api.x_create_tokenizer)(
        fts5_api,
        c"sudachi_tokenizer".as_ptr() as *const u8,
        null_mut(),
        &mut tokenizer,
        fts5_destroy_module,
    );
}

/// Creates the Sudachi FTS5 tokenizer.
///
/// Supports the following tokenizer arguments:
/// - `tokenize='sudachi_tokenizer'` - normalized form (default, better recall)
/// - `tokenize='sudachi_tokenizer surface'` - use original surface form
#[unsafe(no_mangle)]
pub extern "C" fn fts5_create_sudachi_tokenizer(
    _p_context: *mut c_void,
    az_arg: *const *const c_uchar,
    n_arg: c_int,
    fts5_tokenizer: *mut *mut Fts5Tokenizer,
) -> c_int {
    // Panic boundary required: load_tokenizer allocates ~70MB and invokes sudachi.rs
    // internals that may panic (OOM, corrupt dictionary, internal assertions).
    // Any panic crossing this extern "C" boundary is undefined behaviour.
    crate::common::ffi_panic_boundary(|| {
        let use_surface_form = parse_tokenizer_args(az_arg, n_arg);
        let tokenizer = load_tokenizer(use_surface_form).map_err(|_| SQLITE_INTERNAL)?;
        let boxed = Box::new(Fts5Tokenizer { tokenizer });
        unsafe {
            *fts5_tokenizer = Box::into_raw(boxed);
        }
        Ok(())
    })
}

/// Parses tokenizer arguments to check for "surface" option.
///
/// Returns true if "surface" argument is found, false otherwise.
fn parse_tokenizer_args(az_arg: *const *const c_uchar, n_arg: c_int) -> bool {
    if az_arg.is_null() || n_arg <= 0 {
        return false;
    }

    for i in 0..n_arg as usize {
        let arg_ptr = unsafe { *az_arg.add(i) };
        if arg_ptr.is_null() {
            continue;
        }

        // Convert C string to Rust &str
        let c_str = unsafe { std::ffi::CStr::from_ptr(arg_ptr as *const c_char) };
        if let Ok(arg) = c_str.to_str()
            && arg.eq_ignore_ascii_case("surface")
        {
            return true;
        }
    }

    false
}

#[unsafe(no_mangle)]
pub extern "C" fn fts5_delete_sudachi_tokenizer(fts5_tokenizer: *mut Fts5Tokenizer) {
    // Panic boundary required: drop runs sudachi.rs destructors which may panic.
    crate::common::ffi_panic_boundary(|| {
        let _ = unsafe { Box::from_raw(fts5_tokenizer) };
        Ok(())
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn fts5_destroy_module(_module: *mut c_void) {
    // no-op
}
