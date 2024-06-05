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
pub fn TerminalSplit(tabs: Signal<Vec<Tab>>, input: Signal<InputManager>, pty_system: Signal<PseudoTerminalSystem>) -> Element {

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            flex_grow: 1,
            pre {
                class: "tabs",
                display: "flex",
                font_size: "14px",
                for tab in tabs.read().iter() {
                    span { 
                        class: "tab",
                        " {tab.name} "
                    }
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                flex_grow: 1,
                for tab in tabs.read().iter() {
                    TerminalApp { tab: tab.clone(), pty_system, input }
                }
            }
        }
    }
}
