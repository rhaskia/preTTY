use crate::{input::Input, renderer::terminal::TerminalApp};
use async_channel::{Receiver, Sender};
use dioxus::prelude::*;

pub mod cell;
pub mod cursor;
pub mod header;
mod palette;
pub mod terminal;

#[component]
pub fn TerminalSplit(input: Signal<Receiver<Input>>) -> Element {
    //let (send, recv) = async_channel::unbounded();

    rsx!(TerminalApp { input: input })
}
