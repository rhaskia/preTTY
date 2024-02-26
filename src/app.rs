use dioxus::prelude::*;
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
