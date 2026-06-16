use dioxus::prelude::*;

use crate::NavBar;
use crate::bannerfont::Banner;
use crate::components::BannerView;
use crate::db;

#[component]
pub fn Data() -> Element {
    let banners = use_resource(|| async {
        let Ok(db) = db::open().await else {
            return Vec::new();
        };
        db::all_banners(&db).await.unwrap_or_default()
    });

    rsx! {
        NavBar {}
        main { id: "app",
            section { class: "data",
                match &*banners.read() {
                    None => rsx! {
                        p { class: "data-empty", "Loading…" }
                    },
                    Some(rows) if rows.is_empty() => rsx! {
                        p { class: "data-empty", "No banners recorded yet." }
                    },
                    Some(rows) => rsx! {
                        for row in rows.iter() {
                            if let Ok(banner) = row.code.parse::<Banner>() {
                                div { class: "data-row",
                                    BannerView { banner }
                                    span { class: "data-code", "{row.code}" }
                                    span { class: "data-count", "×{row.count}" }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
