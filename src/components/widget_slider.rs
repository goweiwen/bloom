use dioxus::prelude::*;

use crate::sound::Sound;

#[component]
pub fn WidgetSlider(
    value: f64,
    #[props(default = 0.0)] min: f64,
    #[props(default = 1.0)] max: f64,
    #[props(default = 0.01)] step: f64,
    oninput: Option<EventHandler<f64>>,
    onchange: Option<EventHandler<f64>>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "widget-slider",
            input {
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value}",
                oninput: move |e| {
                    if let (Some(oninput), Ok(v)) = (oninput, e.value().parse::<f64>()) {
                        oninput.call(v);
                    }
                },
                onchange: move |e| {
                    Sound::Click.play();
                    if let (Some(onchange), Ok(v)) = (onchange, e.value().parse::<f64>()) {
                        onchange.call(v);
                    }
                },
            }
            label { {children} }
        }
    }
}
