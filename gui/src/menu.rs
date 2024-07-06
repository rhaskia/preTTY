mod settings;
mod keybinding;
mod form;
use keybinding::Keybind;
use dioxus::prelude::*;
use crate::{CONFIG};
use macros::Form;
use serde::Serialize;
//use ::create_form;

#[derive(Serialize, Form)]
struct Example {
    pub amount: i64,
}

#[component]
pub fn Menu(active: bool) -> Element {
    // Temporary config
    let config = use_signal(|| CONFIG.cloned());
    let value = use_signal(|| 87);

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
                TestForm { value },

                for i in 0..config().keybinds.len() {
                    Keybind { index: i, config }
                }
            }
        }
    }
}
