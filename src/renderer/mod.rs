use async_channel::Receiver;
use dioxus::prelude::*;

use crate::renderer::terminal::TerminalApp;

pub mod header;
mod palette;
pub mod terminal;

pub trait GetClasses {
    fn get_classes(&self) -> String;
}

#[component]
pub fn TerminalSplit(input: Signal<Receiver<String>>) -> Element {
    // let (send, recv) = async_channel::unbounded();

    rsx!(TerminalApp { input })
}
