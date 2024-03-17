use async_channel::Receiver;
use dioxus::prelude::*;

use crate::{renderer::terminal::TerminalApp, terminal::pty::PseudoTerminalSystem};

pub mod header;
mod palette;
pub mod terminal;

pub trait GetClasses {
    fn get_classes(&self) -> String;
}

#[component]
pub fn TerminalSplit() -> Element {
    // Set up vector arrangement
    let mut pty_system = use_signal(|| PseudoTerminalSystem::setup());

    rsx!{
        TerminalApp { index: 0, pty_system },
        TerminalApp { index: 1, pty_system },
    }
}
