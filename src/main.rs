mod bannerfont;
mod components;
use crate::bannerfont::{Banner, WritingDirection};
use crate::components::Writing;
use dioxus::prelude::*;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}
const FAVICON: Asset = asset!("/assets/favicon.png",);
const MAIN_CSS: Asset = asset!(
    "/assets/main.css",
    AssetOptions::css().with_static_head(true)
);
const FONT: Asset = asset!("/assets/fonts/Minecraftia.woff");
const APP_BG: Asset = asset!("/assets/ui/app_bg.png");
const GUI_BG: Asset = asset!("/assets/ui/gui_bg.png");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        style { dangerous_inner_html: r#"
@font-face {{ font-family: 'Minecraftia'; font-style: normal; font-weight: 400; src: url('{FONT}') format('woff'); }}
body {{ background-image: url('{APP_BG}'); }}
#app {{ border-image-source: url('{GUI_BG}'); }}
            "# }
        div { id: "app", Router::<Route> {} }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    let writing = vec![
        Banner::try_from_code("b10").unwrap(),
        Banner::try_from_code("b10ss2").unwrap(),
        Banner::try_from_code("b10ss2bri10").unwrap(),
        Banner::try_from_code("b10ss2bri10cbo2").unwrap(),
        Banner::try_from_code("b10ss2bri10cbo2bo15").unwrap(),
    ];
    rsx! {
        Writing { banners: writing, direction: WritingDirection::RightToLeft }
    }
}
