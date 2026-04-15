/*
 *  Copyright (c) 2021 Works Applications Co., Ltd.
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

//! Error types for WASM bindings

use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

/// Structured error type for WASM bindings.
///
/// Serializes to JSON: `{ "error": "ErrorType", "details": "Human-readable message" }`
#[derive(Serialize, Debug)]
pub struct WasmError {
    pub error: String,
    pub details: String,
}

impl From<WasmError> for JsValue {
    fn from(err: WasmError) -> JsValue {
        to_value(&err).unwrap_or_else(|_| {
            JsValue::from_str(
                r#"{"error":"SerializationError","details":"Failed to serialize error"}"#,
            )
        })
    }
}
