use crate::bannerfont::{Color, Layer, Pattern};
use crate::components::BannerLayer;

use crate::bannerfont::Color;
use dioxus::prelude::*;
#[component]
pub fn Keyboard() -> Element {
    rsx! {
        div { class: "keyboard", Colors {} }
    }
}

#[component]
fn Colors() -> Element {
    fn style(i: u8) -> String {
        let color = Color::try_from(i).unwrap();
        let color = color.rgb();
        format!("--color: rgb({}, {}, {})", color.0, color.1, color.2)
    }
    rsx! {
        for i in (0..16) {
            button { class: "color", style: style(i) }
        }
    }
}
