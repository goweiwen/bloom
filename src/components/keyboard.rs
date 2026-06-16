use crate::bannerfont::{Banner, Color, Layer, Optimized, Pattern};
use crate::components::{BannerView, Tooltip};
use crate::sound::Sound;

use dioxus::prelude::*;
use strum::IntoEnumIterator;

#[component]
pub fn Keyboard(banners: Signal<Vec<Banner>>) -> Element {
    let color = use_signal(|| Color::White);
    rsx! {
        div { class: "keyboard",
            Colors { color, banners }
            NewBanner { color, banners }
            CopyButton { banners }
            Patterns { color, banners }
        }
    }
}

#[component]
fn CopyButton(banners: ReadSignal<Vec<Banner>>) -> Element {
    rsx! {
        Tooltip { text: "Copy", style: "grid-column: 11 / span 2",
            button {
                class: "copy",
                style: "background-image: var(--icon-export)",
                onmousedown: move |_| Sound::Click.play(),
                onclick: move |_| {
                    let banners = banners.read();
                    let text = &Optimized(&banners).to_string();
                    if let Some(window) = web_sys::window() {
                        let _ = window.navigator().clipboard().write_text(text);
                    }
                },
            }
        }
    }
}

#[component]
fn Colors(mut color: Signal<Color>, banners: WriteSignal<Vec<Banner>>) -> Element {
    fn style(color: Color) -> String {
        let color = color.rgb();
        format!(
            "background-image: var(--icon-dye); --color: rgb({}, {}, {})",
            color.0, color.1, color.2,
        )
    }

    let mut prev_color = use_signal(&*color);

    rsx! {
        div { class: "colors",
            for c in Color::iter() {
                Tooltip { text: c.name(),
                    button {
                        class: "color",
                        style: style(c),
                        onmousedown: move |event: Event<MouseData>| {
                            Sound::SelectPattern.play();
                            let detail = event
                                .downcast::<web_sys::MouseEvent>()
                                .map_or(1, |e| e.detail());
                            if detail <= 1 {
                                prev_color.set(color());
                            }
                        },
                        onclick: move |_| color.set(c),
                        ondoubleclick: move |_| {
                            Sound::TakeResult.play();
                            let mut banners = banners.write();
                            if let Some(current) = banners.last() {
                                crate::state::record_banner(current);
                            }
                            banners.push(Banner::new(vec![Layer::new(Pattern::Base, c)]));
                            color.set(prev_color());
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn NewBanner(color: ReadSignal<Color>, banners: WriteSignal<Vec<Banner>>) -> Element {
    let color = color.read().to_owned();
    let rgb = color.rgb();
    let style = format!(
        "background-image: var(--icon-banner); --color: rgb({}, {}, {})",
        rgb.0, rgb.1, rgb.2,
    );
    rsx! {
        Tooltip {
            text: format!("New {} Banner", color.name()),
            style: "grid-column: 9 / span 2",
            button {
                class: "new-banner",
                style,
                onmousedown: move |_| Sound::TakeResult.play(),
                onclick: move |_| {
                    let mut banners = banners.write();
                    if let Some(current) = banners.last() {
                        crate::state::record_banner(current);
                    }
                    banners.push(Banner::new(vec![Layer::new(Pattern::Base, color)]));
                },
            }
        }
    }
}

#[component]
fn Patterns(color: ReadSignal<Color>, banners: WriteSignal<Vec<Banner>>) -> Element {
    let color = color.read().to_owned();
    let bg_color = bg_color_rgb(color);
    let layout = keyboard_layout();
    rsx! {
        div { class: "patterns",
            for (y, row) in layout.into_iter().enumerate() {
                for (x, (_key, pattern)) in row.into_iter().enumerate() {
                    Tooltip {
                        text: pattern.name(),
                        style: "grid-column: {x+1}; grid-row: {y+1}; --bg-color: rgb({bg_color.0}, {bg_color.1}, {bg_color.2})",
                        button {
                            class: "pattern",
                            onmousedown: move |_| Sound::SelectPattern.play(),
                            onclick: move |_| {
                                let layer = Layer::new(pattern, color);
                                let mut banners = banners.write();
                                match banners.last_mut() {
                                    Some(banner) => banner.layers.push(layer),
                                    None => banners.push(Banner::new(vec![layer])),
                                }
                            },
                            BannerView { banner: Banner::new(vec![Layer::new(pattern, color)]) }
                        }
                    }
                }
            }
        }
    }
}

fn bg_color_rgb(color: Color) -> (u8, u8, u8) {
    use Color::*;
    match color {
        White | LightGray | Yellow | Orange | Lime | LightBlue | Cyan | Pink | Magenta => {
            (71, 79, 82)
        }
        Gray | Red | Brown | Green | Blue | Purple | Black => (157, 157, 151),
    }
}

fn keyboard_layout() -> Vec<Vec<(char, Pattern)>> {
    use Pattern::*;
    vec![
        vec![
            ('q', SmallStripes),
            ('Q', Bricks),
            ('w', Border),
            ('W', CurlyBorder),
            ('e', HalfHorizontal),
            ('E', HalfHorizontalBottom),
            ('r', StripeTop),
            ('R', StripeBottom),
            ('t', StripeCenter),
            ('T', StripeMiddle),
            ('y', Gradient),
            ('Y', GradientUp),
        ],
        vec![
            ('a', TrianglesTop),
            ('A', TrianglesBottom),
            ('s', TriangleTop),
            ('S', TriangleBottom),
            ('d', StraightCross),
            ('D', Cross),
            ('f', SquareTopLeft),
            ('F', SquareTopRight),
            ('g', DiagonalLeft),
            ('G', DiagonalRight),
        ],
        vec![
            ('z', HalfVertical),
            ('Z', HalfVerticalRight),
            ('x', StripeLeft),
            ('X', StripeRight),
            ('c', StripeDownLeft),
            ('C', StripeDownRight),
            ('v', SquareBottomLeft),
            ('V', SquareBottomRight),
            ('b', DiagonalUpLeft),
            ('B', DiagonalUpRight),
        ],
        vec![
            ('1', Creeper),
            ('2', Skull),
            ('3', Flower),
            ('4', Globe),
            ('5', Piglin),
            ('6', Mojang),
            ('7', Circle),
            ('8', Rhombus),
            ('9', Flow),
            ('0', Guster),
        ],
    ]
}

#[test]
fn test_all_patterns_in_keyboard() {
    let keys: Vec<_> = keyboard_layout().into_iter().flatten().collect();
    for pattern in Pattern::iter() {
        if pattern == Pattern::Base {
            continue;
        }
        assert!(
            keys.iter().any(|&p| p.1 == pattern),
            "Pattern {:?} is not in the keyboard layout",
            pattern,
        );
    }
}
