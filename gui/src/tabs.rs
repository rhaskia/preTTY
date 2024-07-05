use dioxus::prelude::*;
use crate::input::InputManager;
use pretty_term::pty::PseudoTerminalSystem;
use super::terminal::TerminalApp;
use crate::menu::Menu;

#[derive(Clone, PartialEq)]
pub struct Tab {
    index: usize, 
    name: String,
    settings: bool,
    pty: isize,
}

impl Tab {
    pub fn new(idx: usize, pty: usize) -> Self {
        Tab {
            index: idx,
            name: format!("terminal {idx}"),
            settings: false,
            pty: pty as isize,
        }
    }
}

#[component]
pub fn TerminalSplit(tabs: Signal<Vec<Tab>>, input: Signal<InputManager>, current_tab: Signal<usize>, pty_system: Signal<PseudoTerminalSystem>) -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            flex_grow: 1,
            pre {
                class: "tabs",
                display: "flex",
                font_size: "14px",
                for (n, tab) in tabs.read().iter().enumerate() {
                    span { 
                        class: "tab",
                        onmousedown: move |e| {
                            match e.trigger_button().unwrap() {
                                dioxus_elements::input_data::MouseButton::Primary => current_tab.set(n),
                                // dioxus_elements::input_data::MouseButton::Auxiliary => { 
                                //     tabs.remove(n);
                                // }
                                _ => {}
                            }
                        },
                        background: if n == current_tab() {"var(--bg1)"},
                        " {tab.name} "
                    }
                }
                button {
                    class: "barbutton",
                    onclick: move |_| {
                        let index = tabs.len();
                        tabs.write().push(Tab::new(index, pty_system.read().len()));
                        current_tab.set(index);
                    },
                    ""
                } 
                div {
                    class: "dropdown",
                    button {
                        class: "barbutton",
                        onclick: move |_| {
                            menu_open.toggle();
                            // let index = tabs.len();
                            // tabs.write().push(Tab { name: "Settings".to_string(), index, settings: true, pty: -1 });
                            // current_tab.set(index);
                        },
                        ""
                    } 
                    if menu_open() {
                        div {
                            class: "bardropdown",
                            button { "Powershell" }
                            // More shells (generated likely)
                            hr {}
                            button { "Settings" }
                            button { "Command Palette" }
                            button { "Help" }
                        }
                    }
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                flex_grow: 1,
                for tab in tabs().into_iter() {
                    if tab.settings {
                        Menu { active: tab.index == current_tab() }
                    } else {
                        TerminalApp { pty_system, input, hidden: tab.index != current_tab(), pty_no: tab.pty as usize }
                    }
                }
            }
        }
    }
}
