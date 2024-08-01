use dioxus::prelude::*;
use crate::PALETTES;

#[component]
pub fn ColourPalette() -> Element {
    let editing = use_signal(|| "default");

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

            for (name, colour) in PALETTES.read()[editing()].iter() {

            }
        }
    }
}
