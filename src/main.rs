mod bannerfont;
mod components;
mod db;
mod sound;
mod state;

use crate::bannerfont::Banner;
use crate::components::{Data, Keyboard, Settings, Writing};
use crate::state::Settings as SettingsState;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/data")]
    Data {},
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
const ICON_IMPORT: Asset = asset!("/assets/ui/icon_import.png");
const ICON_EXPORT: Asset = asset!("/assets/ui/icon_export.png");
const TAB_ACTIVE: Asset = asset!("/assets/ui/tab_active.png");
const TAB_INACTIVE: Asset = asset!("/assets/ui/tab_inactive.png");
const TOOLTIP: Asset = asset!("/assets/ui/tooltip.png");
const SLIDER: Asset = asset!("/assets/ui/slider.png");

fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    let settings = use_context_provider(SettingsState::new);
    use_hook(move || settings.load());
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
    --icon-import: url('{ICON_IMPORT}');
    --icon-export: url('{ICON_EXPORT}');
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
    let settings = use_context::<SettingsState>();
    let mut banners = use_signal(Vec::<Banner>::new);
    rsx! {
        NavBar {}
        main { id: "app",
            Writing {
                banners: banners(),
                direction: (settings.banner_direction)(),
                onreorder: move |(from, to)| {
                    let mut banners = banners.write();
                    let banner = banners.remove(from);
                    banners.insert(to, banner);
                },
            }
            Keyboard { banners }
        }
    }
}

#[component]
pub(crate) fn NavBar() -> Element {
    let link = |route, name| {
        rsx! {
            Link { to: route, active_class: "active", {name} }
        }
    };
    rsx! {
        nav { id: "nav",
            {link(Route::Home {}, "Bloom")}
            {link(Route::Data {}, "Data")}
            {link(Route::Settings {}, "Settings")}
        }
        Outlet::<Route> {}
    }
}
