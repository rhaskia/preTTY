mod settings;
mod keybinding;
mod form;
use keybinding::Keybinds;
use dioxus::prelude::*;
use crate::{KEYBINDS, CONFIG};
use serde::Serialize;
use form::Form;
//use ::create_form;

#[derive(Serialize, PartialEq, Clone)]
struct Example {
    pub amount: i64,
    pub name: String,
    pub keybinds: Vec<i64>,
    pub nested: Example2,
}

#[derive(Serialize, PartialEq, Clone)]
struct Example2 {
    pub amount: bool,
    pub name: String,
}

#[component]
pub fn Menu(active: bool) -> Element {
    // Temporary config
    let config = use_signal(|| CONFIG.cloned());
    let keybinds = use_signal(|| KEYBINDS().clone());
    let value = use_signal(|| Example {amount:56,name:"hello".to_string(),keybinds:vec![2,3,4,56], nested: Example2 { amount: false, name: "john".to_string() } });

    rsx! {
        div {
            class: "menu",
            display: if active { "block" } else { "none" },
            id: "menu",
            div { 
              id: "menuheader", 
              class: "menuheader",
              h2 { "Settings" }, 
            }
            div {
                class: "menucontent",
                Form { value: config }
                Keybinds { keybinds }
            }
            div {
                height: "20px",
                color: "red",
                width: "100%",
            }
        }
    }
}
