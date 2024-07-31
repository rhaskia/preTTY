use dioxus::prelude::*;

pub fn ColourPalette() -> Element {
    rsx! {
        h2 { "Colour Palette" }
        label { "red" }
        input { r#type: "color", }
    }
}