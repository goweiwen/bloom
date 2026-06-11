use crate::bannerfont::{Banner, WritingDirection};
use crate::components::{BannerView, Tooltip};

use dioxus::prelude::*;

#[component]
pub fn Writing(banners: Vec<Banner>, direction: Option<WritingDirection>) -> Element {
    let direction = direction.unwrap_or_default().class();
    rsx! {
        div { class: "writing {direction}",
            for banner in banners {
                BannerView { banner: banner.clone() }
            }
        }
    }
}
