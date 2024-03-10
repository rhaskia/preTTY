use dioxus::prelude::*;

use crate::terminal::Terminal;

#[component]
pub fn CommandsSlice(terminal: Signal<Terminal>) -> Element {
    to_owned![terminal];

    rsx! {
        for command in terminal.read().commands.get() {
            pre {
                "{command:?}"
            }
        }
    }
}
