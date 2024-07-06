mod settings;
mod keybinding;
mod form;
use keybinding::Keybind;
use dioxus::prelude::*;
use crate::CONFIG;
use serde::Serialize;
//use ::create_form;

#[derive(Serialize)]
struct Example {
    pub amount: i64,
    pub name: String,
    pub keybinds: Vec<i64>,
    pub nested: Example2,
}

#[derive(Serialize)]
struct Example2 {
    pub amount: i64,
    pub name: String,
}

#[component]
pub fn Menu(active: bool) -> Element {
    // Temporary config
    let config = use_signal(|| CONFIG.cloned());
    let value = use_signal(|| Example {amount:56,name:"hello".to_string(),keybinds:vec![2,3,4,56], nested: Example2 { amount: 23, name: "john".to_string() } });

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
                //div { "Font Size" input { r#type: "number", value: config().font_size } }
                form { 
                    oninput: |i| println!("{i:?}"),
                    dangerous_inner_html: { form::create_form(value).ok()? } 
                }

                // for i in 0..config().keybinds.len() {
                //     Keybind { index: i, config }
                // }
            }
        }
    }
}
