use dioxus::prelude::*;
use num_traits::cast::FromPrimitive;
use term::Terminal;
use termwiz::escape::csi::DecPrivateModeCode;
use std::any::Any;

#[component]
pub fn TerminalDebug(terminal: Signal<Terminal>) -> Element {
    rsx!{
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
                    td { "Bracketed Paste" }
                    td { "{terminal.read().state.bracketed_paste}" }
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

fn as_dec(n: &u16) -> DecPrivateModeCode {
    DecPrivateModeCode::from_u16(*n).unwrap()
}
