mod bannerfont;
mod components;
mod sound;

use crate::bannerfont::{Banner, WritingDirection};
use crate::components::{Keyboard, WidgetButton, WidgetSlider, Writing};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/settings")]
    Settings {},
}

const FAVICON: Asset = asset!("/assets/favicon.png",);
const MAIN_CSS: Asset = asset!(
    "/assets/main.css",
    AssetOptions::css().with_static_head(true)
);
const FONT: Asset = asset!("/assets/fonts/Minecraftia.woff");
const APP_BG: Asset = asset!("/assets/ui/app_bg.png");
const GUI_BG: Asset = asset!("/assets/ui/gui_bg.png");
const BUTTON: Asset = asset!("/assets/ui/button.png");
const BUTTON_DISABLED: Asset = asset!("/assets/ui/button_disabled.png");
const ICON_BANNER: Asset = asset!("/assets/ui/icon_banner.png");
const ICON_DYE: Asset = asset!("/assets/ui/icon_dye.png");
const TAB_ACTIVE: Asset = asset!("/assets/ui/tab_active.png");
const TAB_INACTIVE: Asset = asset!("/assets/ui/tab_inactive.png");
const TOOLTIP: Asset = asset!("/assets/ui/tooltip.png");
const SLIDER: Asset = asset!("/assets/ui/slider.png");

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
:root {{
    --app-bg: url('{APP_BG}');
    --gui-bg: url('{GUI_BG}');
    --button-bg: url('{BUTTON}');
    --button-disabled-bg: url('{BUTTON_DISABLED}');
    --icon-banner: url('{ICON_BANNER}');
    --icon-dye: url('{ICON_DYE}');
    --tab-active: url('{TAB_ACTIVE}');
    --tab-inactive: url('{TAB_INACTIVE}');
    --tooltip: url('{TOOLTIP}');
    --slider: url('{SLIDER}');
}}
            "# }
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
    let mut volume = use_signal(|| 0.5);
    rsx! {
        NavBar {}
        main { id: "app",
            Writing { banners: writing, direction: WritingDirection::RightToLeft }
            WidgetButton { "test" }
            WidgetSlider {
                value: volume(),
                label: format!("Volume: {}%", (volume() * 100.0).round() as i32),
                oninput: move |v| volume.set(v),
            }
            Keyboard {}
        }
    }
}

/// Settings
#[component]
fn Settings() -> Element {
    rsx! {
        NavBar {}
        main { id: "app" }
    }
}

#[component]
fn NavBar() -> Element {
    let link = |route, name| {
        rsx! {
            Link { to: route, active_class: "active", {name} }
        }
    };
    rsx! {
        nav { id: "nav",
            {link(Route::Home {}, "Bloom")}
            {link(Route::Settings {}, "Settings")}
        }
        Outlet::<Route> {}
    }
}
