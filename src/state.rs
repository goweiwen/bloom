use dioxus::prelude::*;

use crate::bannerfont::WritingDirection;
use crate::db;

/// App-wide settings, shared through context. Holds one signal per field so
/// reads subscribe the consuming component and writes notify it.
#[derive(Clone, Copy)]
pub struct Settings {
    /// Render banner rows right-to-left (first banner rightmost).
    pub banner_direction: Signal<WritingDirection>,
    /// Sound-effect volume, 0.0–1.0.
    pub volume: Signal<f64>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            banner_direction: Signal::new(WritingDirection::default()),
            volume: Signal::new(0.5),
        }
    }

    /// Load persisted settings into the signals, fire-and-forget. A first run
    /// (no stored record) leaves the defaults in place.
    pub fn load(mut self) {
        spawn(async move {
            let Ok(db) = db::open().await else {
                return;
            };
            if let Ok(Some(settings)) = db::load_settings(&db).await {
                self.banner_direction.set(settings.banner_direction);
                self.volume.set(settings.volume);
            }
        });
    }

    /// Persist the current settings to IndexedDB, fire-and-forget.
    pub fn save(self) {
        let settings = db::Settings {
            banner_direction: (self.banner_direction)(),
            volume: (self.volume)(),
        };
        spawn(async move {
            if let Ok(db) = db::open().await {
                let _ = db::save_settings(&db, &settings).await;
            }
        });
    }
}
