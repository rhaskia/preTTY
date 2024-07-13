use dioxus::prelude::*;
use dioxus::events::Modifiers;
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
            button {
                onclick: move |_| keybinds.push(Keybinding::default()), 
                "+"
            }
        }
    }
}

#[component] 
pub fn Keybind(keybinds: Signal<Vec<Keybinding>>, index: usize) -> Element {
    let mut recording_key = use_signal(|| false);
    let modifier_values = vec![Modifiers::ALT, Modifiers::CONTROL, Modifiers::META, Modifiers::SHIFT];
    let modifier_names = vec!["Alt", "Control", "Meta", "Shift"];
    let modifiers = modifier_values.into_iter().zip(modifier_names);

    rsx! {
        div {
            class: "keybinding",
            id: "keybinding-{index}",
            display: "flex",
            align_items: "center",

            select {
                name: "action[{index}]",
                for action in config::TerminalAction::VARIANTS {
                    option { 
                        value: "{action}",
                        selected: if *action == keybinds()[index].action.as_ref() { true },
                        "{action}"
                    }
                }
            }
            button { 
                onclick: move |e| recording_key.toggle(), 
                onkeydown: move |k| {
                    keybinds.write()[index].key = k.data().key();
                    println!("{:?}", keybinds()[index]);
                    // unfocus
                },
                class: "keybutton",
                "{keybinds()[index].key}"
            }
            select {
                multiple: true,
                display: "table-row",
                size: "1",
                id: "select-multiple",
                name: "modifiers[{index}]",
                onchange: move |e| keybinds.write()[index].modifiers = to_mod(e.data().value()),
                for (m, name) in modifiers {
                    option { 
                        value: "{m.bits()}",  
                        selected: keybinds.read()[index].modifiers.contains(m),
                        display: "table-cell", 
                        "{name}"
                    }
                }
            }
            br {}
        }
    }
}

pub fn to_mod(s: String) -> Modifiers {
    let split = s.split(',');
    let mut m = Modifiers::empty();
    for n in split {
        let parsed = n.parse::<u32>().unwrap();
        m.insert(Modifiers::from_bits(parsed).unwrap());
    }
    m
}
