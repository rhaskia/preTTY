use dioxus::prelude::*;
use term::pty::PseudoTerminalSystem;

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
pub fn TerminalSplit(tabs: bool) -> Element {
    // Set up vector arrangement
    let pty_system = use_signal(|| PseudoTerminalSystem::setup());
    let tabs = use_signal(|| vec![Tab::new(0)]);

    rsx! {
        div {
            for tab in tabs.read().iter() {
                span { "{tab.name}" }
            }
        }
        for tab in tabs.read().iter() {
            TerminalApp { tab: tab.clone(), pty_system }
        }
    }
}
