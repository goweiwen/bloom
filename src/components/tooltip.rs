use dioxus::prelude::*;

#[component]
pub fn Tooltip(text: String, children: Element) -> Element {
    let mut mouse = use_signal(|| (0.0, 0.0));
    rsx! {
        div {
            class: "tooltip-container",
            style: "--mouse-x: {mouse().0}px; --mouse-y: {mouse().1}px;",
            onmousemove: move | evt | {
                let coords = evt.element_coordinates();
                mouse.set((coords.x, coords.y));
            },
            {children}
            span { class: "tooltip", {text} }
        }
    }
}
