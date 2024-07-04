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
}

impl Tab {
    pub fn new(idx: usize) -> Self {
        Tab {
            index: idx,
            name: format!("terminal {idx}"),
            settings: false,
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
                        background: if n == current_pty() {"var(--bg1)"},
                        " {n} "
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
                for tab in tabs().into_iter() {
                        TerminalApp { pty_system, input, hidden: index != current_pty(), tab: index },
                    // if tab.settings {
                    // } else {
                    //     Menu {}
                    // }
                }
            }
        }
    }
}
