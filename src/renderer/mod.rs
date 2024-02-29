use crate::renderer::terminal::TerminalApp;
use dioxus::prelude::*;


pub mod cell;
pub mod header;
mod palette;
pub mod terminal;

#[component]
pub fn TerminalSplit() -> Element {
    rsx!(TerminalApp {})
}
