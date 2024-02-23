use dioxus::prelude::*;
use crate::{renderer::terminal::TerminalApp};
use crate::renderer::header::Header;

#[component]
pub fn app() -> Element {
    rsx! {
        style { {include_str!("style.css")} }
        //Header {}
        TerminalApp {}
    }
}
