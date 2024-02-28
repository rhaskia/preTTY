use crate::renderer::terminal::TerminalApp;
use dioxus::prelude::*;
use portable_pty::{native_pty_system, PtySize};

mod write_block;
pub mod cell;
pub mod header;
mod palette;
pub mod terminal;

#[component]
pub fn TerminalSplit() -> Element {
    rsx!(TerminalApp { })
}
