//! IndexedDB-backed storage for the settings singleton. This layer is pure
//! storage; mapping to/from the app's global signals lives in [`crate::state`].

use rexie::{ObjectStore, Rexie, TransactionMode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::bannerfont::WritingDirection;

const DB_NAME: &str = "bloom";
/// Schema version. Bump when adding/removing object stores; existing stores are
/// preserved on upgrade.
const DB_VERSION: u32 = 1;
const STORE_SETTINGS: &str = "settings";
/// Out-of-line key under which the single settings record is stored.
const SETTINGS_KEY: &str = "app";

pub type Result<T> = std::result::Result<T, Error>;

/// Either an IndexedDB failure or a (de)serialisation mismatch between a stored
/// row and our record types.
#[derive(Debug)]
pub enum Error {
    Db(rexie::Error),
    Serde(serde_wasm_bindgen::Error),
}

impl From<rexie::Error> for Error {
    fn from(e: rexie::Error) -> Self {
        Error::Db(e)
    }
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Error::Serde(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Db(e) => write!(f, "indexeddb: {e}"),
            Error::Serde(e) => write!(f, "deserialize: {e}"),
        }
    }
}

impl std::error::Error for Error {}

/// Persisted user settings (the `settings` singleton).
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Settings {
    pub banner_direction: WritingDirection,
    pub volume: f64,
}

/// Open the database, creating the settings store on first run or upgrade.
pub async fn open() -> Result<Rexie> {
    let rexie = Rexie::builder(DB_NAME)
        .version(DB_VERSION)
        .add_object_store(ObjectStore::new(STORE_SETTINGS))
        .build()
        .await?;
    Ok(rexie)
}

/// Load the persisted settings, or `None` on first run.
pub async fn load_settings(rexie: &Rexie) -> Result<Option<Settings>> {
    let tx = rexie.transaction(&[STORE_SETTINGS], TransactionMode::ReadOnly)?;
    let store = tx.store(STORE_SETTINGS)?;
    let value = store.get(JsValue::from_str(SETTINGS_KEY)).await?;
    tx.done().await?;
    match value {
        Some(value) => Ok(Some(serde_wasm_bindgen::from_value(value)?)),
        None => Ok(None),
    }
}

/// Persist the settings singleton.
pub async fn save_settings(rexie: &Rexie, settings: &Settings) -> Result<()> {
    let tx = rexie.transaction(&[STORE_SETTINGS], TransactionMode::ReadWrite)?;
    let store = tx.store(STORE_SETTINGS)?;
    let value = serde_wasm_bindgen::to_value(settings)?;
    store
        .put(&value, Some(&JsValue::from_str(SETTINGS_KEY)))
        .await?;
    tx.done().await?;
    Ok(())
}
