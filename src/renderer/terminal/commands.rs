use dioxus::prelude::*;

use crate::terminal::Terminal;
use crate::terminal::command::CommandSlice;
use super::cell::CellSpan;

#[component]
pub fn CommandsSlice(terminal: Signal<Terminal>) -> Element {
    to_owned![terminal];

    rsx! {
        for command in terminal.read().commands.get() {
            Command { command: *command, terminal }
        }
    }
}

#[component]
pub fn Command(command: CommandSlice, terminal: Signal<Terminal>) -> Element {
    rsx! {
        div {
            class: "command-slice",

            pre {
                for y in command.range(terminal.read().screen().len()) {
                    for (x, cell) in terminal.read().screen().line(y).iter().enumerate() {
                        CellSpan { cell: *cell, x, y } 
                    }
                    br {}
                }
            }
        }
    }
}
