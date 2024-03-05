use dioxus::prelude::*;

#[component]
pub fn Cursor(x: usize, y: usize) -> Element {
    rsx! {
        div {
            class: "cursor",
            left: "calc({x} * var(--cell-width))",
            top: "calc({y} * var(--cell-height))",
            height: "var(--cell-height)",
            width: "var(--cell-width)",
        }
    }
}
