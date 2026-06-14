use dioxus::prelude::*;

use crate::sound::Sound;

#[component]
pub fn WidgetButton(onclick: Option<EventHandler<MouseEvent>>, children: Element) -> Element {
    rsx! {
        button {
            class: "widget-button",
            onmousedown: move |_| Sound::Click.play(),
            onclick: move |e| {
                if let Some(onclick) = onclick {
                    onclick.call(e);
                }
            },
            {children}
        }
    }
}
