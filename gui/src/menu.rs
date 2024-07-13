mod settings;
mod keybinding;
use keybinding::Keybinds;
use dioxus::prelude::*;
use crate::{KEYBINDS, CONFIG};
use serde::{Serialize, Deserialize};
use dioxus_form::Form;

#[component]
pub fn Menu(active: bool) -> Element {
    // Temporary config
    let mut config = use_signal(|| CONFIG.cloned());
    let keybinds = use_signal(|| KEYBINDS().clone());

    rsx! {
        div {
            class: "menu",
            display: if active { "flex" } else { "none" },
            width: "100%",
            display: "flex",
            flex_direction: "column",
            id: "menu",
            div {
                class: "menucontent",
                div { 
                  id: "menuheader", 
                  class: "menuheader",
                  h2 { "Settings" }, 
                }
                Form { value: config  }
                Keybinds { keybinds }
            }
            div {
                height: "20px",
                width: "100%",
                class: "savebar",
                button {
                    onclick: move |_| {
                        *CONFIG.write() = config();
                        *KEYBINDS.write() = keybinds();
                        config::save_keybinds(keybinds().clone());
                        // Save to file
                    },
                    "Save Config"
                }
                button {
                    // TODO open config folder
                    "Open Config Folder"
                }
                button {
                    onclick: move |_| config.set(CONFIG()),
                    "Discard All"
                }
            }
        }
    }
}
