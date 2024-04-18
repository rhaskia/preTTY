use dioxus::prelude::*;
use term::pty::PseudoTerminalSystem;

use super::terminal::TerminalApp;

#[component]
pub fn TerminalSplit() -> Element {
    // Set up vector arrangement
    let pty_system = use_signal(|| PseudoTerminalSystem::setup());

    rsx! {
        TerminalApp { index: 0, pty_system },
    }
}
