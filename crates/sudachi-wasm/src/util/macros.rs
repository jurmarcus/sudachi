/*
 *  Copyright (c) 2021-2024 Works Applications Co., Ltd.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

//! Cross-platform debug logging macros.
//!
//! These macros provide unified debug output that works on both native and WASM targets:
//! - Native: Uses stdout (println!, write!, writeln!)
//! - WASM: Uses browser console via log::info!

/// Print to stdout (native) or log::info! (WASM).
///
/// # Example
/// ```ignore
/// debug_println!("Processing {} morphemes", count);
/// ```
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {{
        #[cfg(not(target_arch = "wasm32"))]
        println!($($arg)*);
        #[cfg(target_arch = "wasm32")]
        log::info!($($arg)*);
    }};
}

/// Write to a writer (native) or log::info! (WASM).
///
/// On native targets, this writes to the provided writer and propagates errors with `?`.
/// On WASM targets, this logs via log::info! (ignoring the writer).
///
/// # Example
/// ```ignore
/// let mut out = std::io::stdout();
/// debug_write!(out, "Node {}: cost={}", idx, cost);
/// ```
#[macro_export]
macro_rules! debug_write {
    ($out:expr, $($arg:tt)*) => {{
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::io::Write;
            write!($out, $($arg)*)?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = &$out; // Suppress unused variable warning
            log::info!($($arg)*);
        }
    }};
}

/// Write a line to a writer (native) or log::info! (WASM).
///
/// On native targets, this writes to the provided writer with a newline.
/// On WASM targets, this logs via log::info! (which adds its own newline).
///
/// # Example
/// ```ignore
/// let mut out = std::io::stdout();
/// debug_writeln!(out, "Analysis complete");
/// debug_writeln!(out); // Just a newline on native
/// ```
#[macro_export]
macro_rules! debug_writeln {
    ($out:expr) => {{
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::io::Write;
            writeln!($out)?;
        }
        // WASM: no-op for empty writeln (log::info already adds newlines)
    }};
    ($out:expr, $($arg:tt)*) => {{
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::io::Write;
            writeln!($out, $($arg)*)?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = &$out; // Suppress unused variable warning
            log::info!($($arg)*);
        }
    }};
}
