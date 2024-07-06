use config::Config;
use dioxus::prelude::*;

#[component] 
pub fn Keybind(index: usize, config: Signal<Config>) -> Element {
    rsx! {
        div {
            class: "keybinding",
            id: "keybinding-{index}",

            "{config().keybinds[index].action:?}"
        }
    }
}
