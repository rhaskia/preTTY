mod settings;
mod keybinding;
mod form;
use keybinding::Keybind;
use dioxus::prelude::*;
use crate::{CONFIG};
use serde::Serialize;
//use ::create_form;

#[derive(Serialize)]
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
                { form::create_form(config); }

                for i in 0..config().keybinds.len() {
                    Keybind { index: i, config }
                }
            }
        }
    }
}
