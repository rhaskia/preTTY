use dioxus::prelude::*;
use crate::PALETTES;

#[component]
pub fn ColourPalette() -> Element {
    rsx! {
        div {
            overflow: "visible",

            h3 { "Colour Palette" }
            select {
                for (name, palette) in PALETTES.read().iter() {
                    option {
                        "{name}"
                    }
                }
            }
            button {
                "Create New"
            }
            label { "red" } input { r#type: "color", }
            label { "orange" } input { r#type: "color", }
            label { "yellow" } input { r#type: "color", }
            label { "green" } input { r#type: "color", }
            label { "cyan" } input { r#type: "color", }
            label { "purple" } input { r#type: "color", }
        }
    }
}
