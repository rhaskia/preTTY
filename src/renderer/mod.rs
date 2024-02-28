use crate::renderer::terminal::TerminalApp;
use dioxus::prelude::*;
use portable_pty::{native_pty_system, PtySize};

mod write_block;
pub mod cell;
pub mod header;
mod palette;
pub mod terminal;

#[component]
pub fn TerminalSplit() -> Element {
    let pty_system = native_pty_system();
    let pair = use_signal(|| pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    }).unwrap());

    rsx!(TerminalApp { pair })
}
