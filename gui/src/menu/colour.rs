use dioxus::prelude::*;
use crate::PALETTES;

#[component]
pub fn ColourPalette() -> Element {
    rsx! {
        div {
            overflow: "visible",

            h2 { "Colour Palette" }
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
            label { "red" }
            input { r#type: "color", }
        }
    }
}
