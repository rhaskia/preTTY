use dioxus::prelude::*;
use crate::input::InputManager;
use pretty_term::pty::PseudoTerminalSystem;
use super::terminal::TerminalApp;

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub index: usize,
    pub name: String,
}

impl Tab {
    pub fn new(idx: usize) -> Self {
        Tab {
            index: idx,
            name: format!("terminal {idx}")
        }
    }
}

#[component]
pub fn TerminalSplit(tabs: Signal<Vec<Tab>>, input: Signal<InputManager>, current_pty: Signal<usize>, pty_system: Signal<PseudoTerminalSystem>, menu_open: Signal<bool>) -> Element {
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
                        onclick: move |_| current_pty.set(n),
                        " {tab.name} "
                    }
                }
                button {
                    class: "barbutton",
                    align_self: "flex-end",
                    margin_right: "14px",
                    margin_left: "auto",
                    onclick: move |_| menu_open.toggle(),
                    " "
                } 
            }
            div {
                display: "flex",
                flex_direction: "row",
                flex_grow: 1,
                TerminalApp { tab: tabs.get(*current_pty.read()).unwrap().clone(), pty_system, input }
            }
        }
    }
}
