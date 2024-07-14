use dioxus::prelude::*;

#[component] 
pub fn CommandPalette() -> Element { 
    rsx! {
        div {
            class: "commandpalette",
            div {
                class: "commandsearch",
            }
        }
    }
}
