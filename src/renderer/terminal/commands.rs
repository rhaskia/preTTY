use dioxus::prelude::*;

use crate::renderer::GetClasses;
use crate::terminal::Terminal;
use crate::terminal::command::{CommandSlice, CommandStatus};
use super::cell::CellSpan;

impl GetClasses for CommandSlice {
    fn get_classes(&self) -> String {
        let status = match self.get_status() {
            CommandStatus::Success => "command-success",
            CommandStatus::Error => "command-error",
            CommandStatus::ShellCommandMisuse => "command-misuse",
            CommandStatus::CannotExecute => "command-no-exec",
            CommandStatus::NotFound => "command-not-found",
            CommandStatus::FatalError(_) => "command-fatal",
            CommandStatus::None => "",
        };

        format!("command {status}")
    }
}

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
            class: command.get_classes(),

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
