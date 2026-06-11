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
const FAVICON: Asset = asset!("/assets/favicon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
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
