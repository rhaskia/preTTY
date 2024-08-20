pub mod colour;
pub mod plugins;
mod keybinding;
pub mod palette;
mod settings;
use colour::ColourPalette;
use dioxus::prelude::*;
use dioxus_form::Form;
use keybinding::Keybinds;

use crate::{CONFIG, KEYBINDS, PALETTES};

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
                ColourPalette { }
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
                        config::save_palettes(PALETTES());
                        // Save to file
                    },
                    "Save Config"
                }
                button {
                    // TODO open config folder
                    onclick: |_| open_file_explorer(),
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

pub fn open_file_explorer() {
    let directory = config::dir();
    let command = if cfg!(target_os = "windows") {
        "explorer"
    } else {
        if cfg!(target_os = "macos") {
            "open"
        } else {
            "xdg-open"
        }
    };
    std::process::Command::new(command)
        .arg(directory) // <- Specify the directory you'd like to open.
        .spawn()
        .unwrap();
}
