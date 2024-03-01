#![feature(if_let_guard)]

// crate imports
mod input;
mod renderer;
mod terminal;

use dioxus::prelude::*;
use crate::renderer::TerminalSplit;

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            id: "app",
            class: "app",

            style { {include_str!("style.css")} }
            style { {include_str!("palette.css")} }
            //Header {}
            TerminalSplit {}
        }
    }
}

fn main()  {
    launch(App);
}
