/*
 * IndexedDB dictionary caching for sudachi-wasm
 *
 * Caches decompressed dictionary bytes to avoid re-downloading on page reload.
 * Uses the URL as cache key.
 */

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    IdbDatabase, IdbFactory, IdbObjectStore, IdbOpenDbRequest, IdbRequest,
    IdbTransactionMode, IdbVersionChangeEvent,
};

const DB_NAME: &str = "sudachi-wasm-cache";
const DB_VERSION: u32 = 1;
const STORE_NAME: &str = "dictionaries";

/// Get IndexedDB from global scope (works in both Window and Worker contexts)
fn get_indexed_db() -> Result<IdbFactory, JsValue> {
    let global = js_sys::global();
    let idb_value = js_sys::Reflect::get(&global, &JsValue::from_str("indexedDB"))
        .map_err(|_| JsValue::from_str("Failed to access indexedDB"))?;

    if idb_value.is_undefined() || idb_value.is_null() {
        return Err(JsValue::from_str("IndexedDB not available"));
    }

    idb_value
        .dyn_into::<IdbFactory>()
        .map_err(|_| JsValue::from_str("indexedDB is not an IdbFactory"))
}

/// Opens the cache database, creating it if needed
async fn open_cache_db() -> Result<IdbDatabase, JsValue> {
    // Use global indexedDB (works in both Window and Worker contexts)
    let idb = get_indexed_db()?;

    let request: IdbOpenDbRequest = idb.open_with_u32(DB_NAME, DB_VERSION)?;

    // Set up upgrade handler to create object store
    let on_upgrade = Closure::once(move |event: IdbVersionChangeEvent| {
        let target = event.target().expect("Event should have target");
        let request: IdbOpenDbRequest = target.dyn_into().expect("Target should be IdbOpenDbRequest");
        let db: IdbDatabase = request.result().expect("Request should have result").dyn_into().expect("Result should be IdbDatabase");

        // Create object store if it doesn't exist
        if !db.object_store_names().contains(STORE_NAME) {
            db.create_object_store(STORE_NAME)
                .expect("Failed to create object store");
        }
    });
    request.set_onupgradeneeded(Some(on_upgrade.as_ref().unchecked_ref()));
    on_upgrade.forget(); // Prevent closure from being dropped

    // Wait for the request to complete
    let result = idb_request_to_future(&request).await?;
    result.dyn_into().map_err(|_| JsValue::from_str("Failed to cast result to IdbDatabase"))
}

/// Convert an IdbRequest to a Future
async fn idb_request_to_future(request: &IdbRequest) -> Result<JsValue, JsValue> {
    let (tx, rx) = futures_channel::oneshot::channel::<Result<JsValue, JsValue>>();

    let on_success = {
        let tx = std::cell::RefCell::new(Some(tx));
        Closure::once(move |_event: web_sys::Event| {
            if let Some(tx) = tx.borrow_mut().take() {
                let target: web_sys::EventTarget = _event.target().unwrap();
                let request: IdbRequest = target.dyn_into().unwrap();
                let _ = tx.send(Ok(request.result().unwrap_or(JsValue::UNDEFINED)));
            }
        })
    };

    let on_error = {
        Closure::once(move |event: web_sys::Event| {
            log::error!("IndexedDB error: {:?}", event);
        })
    };

    request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
    request.set_onerror(Some(on_error.as_ref().unchecked_ref()));

    on_success.forget();
    on_error.forget();

    rx.await.map_err(|_| JsValue::from_str("Channel closed"))?
}

/// Get cached dictionary bytes by URL key
pub async fn get_cached_dict(key: &str) -> Option<Vec<u8>> {
    let db = open_cache_db().await.ok()?;

    let tx = db
        .transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readonly)
        .ok()?;

    let store: IdbObjectStore = tx.object_store(STORE_NAME).ok()?;

    let request = store.get(&JsValue::from_str(key)).ok()?;
    let result = idb_request_to_future(&request).await.ok()?;

    if result.is_undefined() || result.is_null() {
        return None;
    }

    // Convert Uint8Array back to Vec<u8>
    let array: Uint8Array = result.dyn_into().ok()?;
    Some(array.to_vec())
}

/// Cache dictionary bytes with URL as key
pub async fn cache_dict(key: &str, data: &[u8]) -> Result<(), JsValue> {
    let db = open_cache_db().await?;

    let tx = db.transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readwrite)?;

    let store: IdbObjectStore = tx.object_store(STORE_NAME)?;

    // Convert Vec<u8> to Uint8Array
    let array = Uint8Array::from(data);

    let request = store.put_with_key(&array, &JsValue::from_str(key))?;
    idb_request_to_future(&request).await?;

    log::info!("Cached dictionary: {} ({} bytes)", key, data.len());
    Ok(())
}

/// Clear all cached dictionaries
#[wasm_bindgen]
pub async fn clear_dictionary_cache() -> Result<(), JsValue> {
    let db = open_cache_db().await?;

    let tx = db.transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readwrite)?;

    let store: IdbObjectStore = tx.object_store(STORE_NAME)?;

    let request = store.clear()?;
    idb_request_to_future(&request).await?;

    log::info!("Cleared dictionary cache");
    Ok(())
}
