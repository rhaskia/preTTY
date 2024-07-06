mod settings;
mod keybinding;
use keybinding::Keybind;
use dioxus::prelude::*;
use crate::CONFIG;

#[component]
pub fn Menu(active: bool) -> Element {
    // Temporary config
    let config = use_signal(|| CONFIG.cloned());

    rsx! {
        div {
            class: "menu",
            display: if active { "block" } else { "none" },
            id: "menu",
            div {
                class: "menucontent",
                div { 
                  id: "menuheader", 
                  class: "menuheader",
                  h2 { "Settings" }, 
                }
                div { "Font Size" input { r#type: "number", value: config().font_size } }

                for i in 0..config().keybinds.len() {
                    Keybind { index: i, config }
                }
            }
        }
    }
}
