use dioxus::prelude::*;

#[component]
pub fn Tooltip(text: String, children: Element) -> Element {
    rsx! {
        div { class: "tooltip-container",
            {children}
            span { class: "tooltip", { text
                    } }
        }
    }
}
