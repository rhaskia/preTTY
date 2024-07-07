use dioxus::prelude::*;
use config::keybindings::Keybinding;
use crate::KEYBINDS;
use strum::VariantNames;

#[component] 
pub fn Keybinds(keybinds: Signal<Vec<Keybinding>>) -> Element {
    rsx! {
        div {
            class: "keybindings",
            h3 { "Keybinds" }

            for index in 0..keybinds().len() {
                Keybind { keybinds, index }
            }
        }
    }
}

#[component] 
pub fn Keybind(keybinds: Signal<Vec<Keybinding>>, index: usize) -> Element {
    let mut recording_key = use_signal(|| false);
    rsx! {
        div {
            class: "keybinding",
            id: "keybinding-{index}",

            select {
                for action in config::TerminalAction::VARIANTS {
                    option { value: "{action}", "{action}" }
                }
            }
            button { 
                onclick: move |e| recording_key.toggle(), 
                onkeydown: move |k| {
                    keybinds.write()[index].key = k.data().key();
                    println!("{:?}", keybinds()[index]);
                    // unfocus
                },
                "{keybinds()[index].key}"
            }
            br {}
        }
    }
}
