use crate::bannerfont::{Banner, WritingDirection};
use crate::components::{BannerView, Tooltip};

use dioxus::prelude::*;

#[component]
pub fn Writing(
    banners: Vec<Banner>,
    direction: WritingDirection,
    #[props(default)] onreorder: Option<EventHandler<(usize, usize)>>,
) -> Element {
    let rtl = matches!(direction, WritingDirection::RightToLeft);
    let direction = direction.class();
    let reorderable = onreorder.is_some();
    let mut dragged = use_signal(|| Option::<usize>::None);

    rsx! {
        // Drop handling lives on the container so the gaps between banners (and the
        // space before the first / after the last) are valid drop targets too.
        div {
            class: "writing {direction}",
            ondragover: move |event: Event<DragData>| {
                if reorderable && let Some(event) = event.downcast::<web_sys::DragEvent>() {
                    event.prevent_default();
                }
            },
            ondrop: move |event: Event<DragData>| {
                if let (Some(onreorder), Some(from)) = (onreorder, dragged.take())
                    && let Some(insert_pos) = drop_index(&event, rtl)
                {
                    let to = if from < insert_pos { insert_pos - 1 } else { insert_pos };
                    if from != to {
                        onreorder.call((from, to));
                    }
                }
            },
            for (index , banner) in banners.into_iter().enumerate() {
                div {
                    class: if reorderable { "writing-item draggable" } else { "writing-item" },
                    draggable: reorderable,
                    ondragstart: move |_| dragged.set(Some(index)),
                    ondragend: move |_| dragged.set(None),
                    Tooltip { text: "Hello",
                        BannerView { banner }
                    }
                }
            }
        }
    }
}

/// The index at which a dropped banner should be inserted, found by counting the
/// items the cursor has passed: an item's midpoint left of the cursor for `ltr`
/// (right of it for `rtl`, where the row runs the other way).
fn drop_index(event: &Event<DragData>, rtl: bool) -> Option<usize> {
    use wasm_bindgen::JsCast;

    let event = event.downcast::<web_sys::DragEvent>()?;
    let cursor_x = f64::from(event.client_x());
    let target = event.target()?.dyn_into::<web_sys::Element>().ok()?;
    let container = target.closest(".writing").ok()??;
    let items = container.query_selector_all(".writing-item").ok()?;

    let insert_index = (0..items.length())
        .filter_map(|i| items.item(i)?.dyn_into::<web_sys::Element>().ok())
        .filter(|item| {
            let rect = item.get_bounding_client_rect();
            let midpoint = rect.left() + rect.width() / 2.0;
            if rtl {
                midpoint > cursor_x
            } else {
                midpoint < cursor_x
            }
        })
        .count();
    Some(insert_index)
}
