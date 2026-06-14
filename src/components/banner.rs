use dioxus::prelude::*;

use crate::bannerfont::{Banner, Layer};

const ATLAS: Asset = asset!("/assets/textures/banner_atlas.webp");
const ATLAS_COLS: u32 = 43;
const ATLAS_ROWS: u32 = 16;
const TILE_W: u32 = 20;
const TILE_H: u32 = 40;

/// An atlas tile, as the unscaled pixel offset of its top-left corner into
/// `atlas.png`.
pub struct Sprite {
    pub x: u32,
    pub y: u32,
}

impl From<Layer> for Sprite {
    fn from(layer: Layer) -> Self {
        Sprite {
            x: layer.pattern as u8 as u32 * TILE_W,
            y: layer.color as u8 as u32 * TILE_H,
        }
    }
}

#[component]
pub fn BannerView(banner: Banner) -> Element {
    fn style(layers: &[Layer]) -> String {
        let bg_w = ATLAS_COLS * 100;
        let bg_h = ATLAS_ROWS * 100;

        let (images, positions): (Vec<_>, Vec<_>) = layers
            .iter()
            .rev()
            .map(|&layer| {
                let sprite = Sprite::from(layer);
                let col = (sprite.x / TILE_W) as f64;
                let px = col * 100.0 / (ATLAS_COLS - 1) as f64;
                let row = (sprite.y / TILE_H) as f64;
                let py = row * 100.0 / (ATLAS_ROWS - 1) as f64;
                (format!("url({ATLAS})"), format!("{px}% {py}%"))
            })
            .unzip();
        let sizes = vec![format!("{bg_w}% {bg_h}%"); layers.len()];

        format!(
            "background-image: {}; background-position: {}; background-size: {};",
            images.join(", "),
            positions.join(", "),
            sizes.join(", "),
        )
    }

    rsx! {
        div { class: "banner", style: style(&banner.layers) }
    }
}
