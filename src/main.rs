#![feature(if_let_guard)]

// crate imports
mod input;
mod renderer;
mod terminal;

use dioxus::prelude::*;

use crate::renderer::cell::CellSpan;
use crate::terminal::Terminal;
use portable_pty::{CommandBuilder, native_pty_system, PtySize};

use crate::renderer::TerminalSplit;
use crate::renderer::terminal::TerminalApp;
use crate::renderer::header::Header;

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            id: "app",
            class: "app",

            style { {include_str!("style.css")} }
            //Header {}
            TerminalSplit {}
        }
    }
}

fn main()  {
    launch(App);
}
