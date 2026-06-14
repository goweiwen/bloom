use dioxus::prelude::*;

use crate::NavBar;
use crate::bannerfont::WritingDirection;
use crate::components::{WidgetButton, WidgetSlider};
use crate::state::Settings as SettingsState;

#[component]
pub fn Settings() -> Element {
    let settings = use_context::<SettingsState>();

    let banner_direction_label = match (settings.banner_direction)() {
        WritingDirection::RightToLeft => "Right-to-Left",
        WritingDirection::LeftToRight => "Left-to-Right",
    };

    let volume_percent = ((settings.volume)() * 100.0).round() as i32;

    rsx! {
        NavBar {}
        main { id: "app",
            section { class: "settings",
                WidgetButton {
                    onclick: move |_| {
                        let mut dir = settings.banner_direction;
                        dir.set(if dir() == WritingDirection::RightToLeft {
                            WritingDirection::LeftToRight
                        } else {
                            WritingDirection::RightToLeft
                        });
                        settings.save();
                    },
                    "Banner Direction: {banner_direction_label}"
                }
                WidgetSlider {
                    value: (settings.volume)(),
                    oninput: move |v| {
                        let mut vol = settings.volume;
                        vol.set(v);
                    },
                    onchange: move |_| settings.save(),
                    "Volume: {volume_percent}%"
                }
            }
        }
    }
}
