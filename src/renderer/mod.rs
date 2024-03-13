use crate::{input::Input, renderer::terminal::TerminalApp};
use async_channel::Receiver;
use dioxus::prelude::*;

pub mod header;
mod palette;
pub mod terminal;

pub trait GetClasses {
    fn get_classes(&self) -> String;
}

#[component]
pub fn TerminalSplit(input: Signal<Receiver<Input>>) -> Element {
    //let (send, recv) = async_channel::unbounded();

    rsx!(TerminalApp { input: input })
}
