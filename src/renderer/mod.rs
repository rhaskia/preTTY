use crate::{renderer::terminal::TerminalApp, input::Input};
use async_channel::{Sender, Receiver};
use dioxus::prelude::*;

pub mod cell;
pub mod header;
mod palette;
pub mod terminal;

#[component]
pub fn TerminalSplit(input: Signal<Receiver<Input>>) -> Element {
    //let (send, recv) = async_channel::unbounded();

    rsx!(TerminalApp { input: input })
}
