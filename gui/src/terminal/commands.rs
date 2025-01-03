use dioxus::prelude::*;
use pretty_term::command::{CommandSlice, CommandStatus};
use pretty_term::Terminal;
use log::info;
use super::cell::CellLine;

#[component]
pub fn CommandsSlice(terminal: Signal<Terminal>) -> Element {
    to_owned![terminal];

    rsx! {
        for command in terminal.read().commands.get() {
            Command { command: *command, terminal }
            hr { class: "command-sep" }
        }
    }
}

#[component]
pub fn RightClickCommand() -> Element {
    rsx! {
        div {
            "hi"
        }
    }
}

#[component]
pub fn Command(
    command: CommandSlice,
    terminal: Signal<Terminal>,
) -> Element {
    let mut hovering = use_signal(|| false);
    let status_class = match command.get_status() {
        CommandStatus::Success => "command-success",
        CommandStatus::Error => "command-error",
        CommandStatus::ShellCommandMisuse => "command-misuse",
        CommandStatus::CannotExecute => "command-no-exec",
        CommandStatus::NotFound => "command-not-found",
        CommandStatus::FatalError(_) => "command-fatal",
        CommandStatus::None => "",
    };

    rsx! {
        div {
            class: "command {status_class}",
            onmouseover: move |_| hovering.set(true),
            onmouseleave: move  |_| hovering.set(false),

            pre {
                for y in command.range(terminal.read().screen().scrollback_len()) {
                    CellLine { terminal, y }
                }
            }

            div {
                class: "command-bar",
                button {
                    class: "command-button copy",
                    onclick: |_| info!("Copy not yet implemented"),
                    hidden: !command.finished() || !hovering(),
                    ""
                }
            }
        }
    }
}
