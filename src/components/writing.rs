use crate::bannerfont::{Banner, WritingDirection};
use crate::components::{BannerView, Tooltip};

use dioxus::prelude::*;

#[component]
pub fn Writing(banners: Vec<Banner>, direction: WritingDirection) -> Element {
    let direction = direction.class();
    rsx! {
        div { class: "writing {direction}",
            for banner in banners {
                Tooltip { text: "Hello",
                    BannerView { banner: banner.clone() }
                }
            }
        }
    }
}
