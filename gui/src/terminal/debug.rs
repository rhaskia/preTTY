use std::any::Any;

use dioxus::prelude::*;
use num_traits::cast::FromPrimitive;
use term::Terminal;
use termwiz::escape::csi::DecPrivateModeCode;

#[component]
pub fn TerminalDebug(terminal: Signal<Terminal>) -> Element {
    rsx! {
        div {
            id: "terminal-debug",
            class: "terminal-debug",
            table {
                tr {
                    th { "Mode" }
                    th { "Value" }
                }
                tr {
                    td { "Alt Screen" }
                    td { "{terminal.read().state.alt_screen}" }
                }
                tr {
                    td { "Show Cursor" }
                    td { "{terminal.read().state.show_cursor}" }
                }
                tr {
                    td { "Bracketed Paste" }
                    td { "{terminal.read().state.bracketed_paste}" }
                }
                tr {
                    td { "Commands" }
                    td { "{terminal.read().commands.len()}" }
                }
                for (key, value) in &terminal.read().state.dec_modes {
                    tr {
                        td { "{as_dec(key):?}", }
                        td { "{value}" }
                    }
                }
                for (key, value) in &terminal.read().state.modes {
                    tr {
                        td { "{key:?}" }
                        td { "{value}" }
                    }
                }
            }

        }
    }
}

fn as_dec(n: &u16) -> DecPrivateModeCode { DecPrivateModeCode::from_u16(*n).unwrap() }
