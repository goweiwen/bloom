use dioxus::prelude::*;

#[component]
pub fn Tooltip(text: String, style: Option<String>, children: Element) -> Element {
    let mut mouse = use_signal(|| (0.0, 0.0));
    let style = style.unwrap_or_default();
    rsx! {
        div {
            class: "tooltip-container",
            style: "--mouse-x: {mouse().0}px; --mouse-y: {mouse().1}px; {style}",
            onmousemove: move | evt | {
                let coords = evt.element_coordinates();
                mouse.set((coords.x, coords.y));
            },
            {children}
            span { class: "tooltip", {text} }
        }
    }
}
