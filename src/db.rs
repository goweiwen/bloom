//! IndexedDB-backed storage for the settings singleton. This layer is pure
//! storage; mapping to/from the app's global signals lives in [`crate::state`].

use js_sys::Uint8Array;
use rexie::{ObjectStore, Rexie, TransactionMode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::bannerfont::WritingDirection;

const DB_NAME: &str = "bloom";
/// Schema version. Bump when adding/removing object stores; existing stores are
/// preserved on upgrade.
const DB_VERSION: u32 = 2;
const STORE_SETTINGS: &str = "settings";
const STORE_BANNERS: &str = "banners";
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

/// A row in the `banners` store: a banner's usage stats, keyed in-line by its
/// [binary encoding](crate::bannerfont::Banner) (a `Uint8Array`, lossless). `count`
/// is how many times it was made; `last_used` is a Unix-seconds timestamp for
/// frecency ranking.
#[derive(Serialize, Deserialize, Clone)]
pub struct BannerRow {
    #[serde(with = "banner_bytes")]
    pub bytes: Vec<u8>,
    pub count: u32,
    pub last_used: i64,
}

/// Serialize the `bytes` field as a JS `Uint8Array` (via `serialize_bytes`) rather
/// than the default array-of-numbers, so it is stored as binary and usable as an
/// in-line IndexedDB key.
mod banner_bytes {
    use serde::de::{self, Visitor};
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(bytes)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        struct BytesVisitor;
        impl<'de> Visitor<'de> for BytesVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a byte array")
            }

            fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                Ok(v.to_vec())
            }

            fn visit_byte_buf<E: de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(v)
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut bytes = Vec::new();
                while let Some(b) = seq.next_element()? {
                    bytes.push(b);
                }
                Ok(bytes)
            }
        }
        deserializer.deserialize_byte_buf(BytesVisitor)
    }
}

/// Open the database, creating any missing object stores on first run or upgrade.
pub async fn open() -> Result<Rexie> {
    let rexie = Rexie::builder(DB_NAME)
        .version(DB_VERSION)
        .add_object_store(ObjectStore::new(STORE_SETTINGS))
        .add_object_store(ObjectStore::new(STORE_BANNERS).key_path("bytes"))
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

/// Load every recorded banner, ordered by `count` descending (most-used first).
pub async fn all_banners(rexie: &Rexie) -> Result<Vec<BannerRow>> {
    let tx = rexie.transaction(&[STORE_BANNERS], TransactionMode::ReadOnly)?;
    let store = tx.store(STORE_BANNERS)?;
    let values = store.get_all(None, None).await?;
    tx.done().await?;

    let mut rows = values
        .into_iter()
        .map(serde_wasm_bindgen::from_value::<BannerRow>)
        .collect::<std::result::Result<Vec<_>, _>>()?;
    rows.sort_by_key(|x| -(x.count as isize));
    Ok(rows)
}

/// Record one use of `bytes` (a banner's binary encoding): increment its `count`
/// (starting from 1 for a new banner) and set `last_used` to `now`. The store is
/// keyed in-line on `bytes`, so repeats upsert the same row.
pub async fn record_banner(rexie: &Rexie, bytes: &[u8], now: i64) -> Result<()> {
    let tx = rexie.transaction(&[STORE_BANNERS], TransactionMode::ReadWrite)?;
    let store = tx.store(STORE_BANNERS)?;

    let key = JsValue::from(Uint8Array::from(bytes));
    let count = match store.get(key).await? {
        Some(value) => serde_wasm_bindgen::from_value::<BannerRow>(value)?.count + 1,
        None => 1,
    };

    let row = BannerRow {
        bytes: bytes.to_vec(),
        count,
        last_used: now,
    };
    store
        .put(&serde_wasm_bindgen::to_value(&row)?, None)
        .await?;
    tx.done().await?;
    Ok(())
}
