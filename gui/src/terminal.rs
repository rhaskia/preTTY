pub mod cell;
pub mod commands;
pub mod cursor;
pub mod debug;

use cell::CellGrid;
use commands::CommandsSlice;
use cursor::Cursor;
use debug::TerminalDebug;
use dioxus::prelude::*;
use pretty_hooks::{on_resize, DOMRectReadOnly};
use serde::Deserialize;
use crate::CONFIG;
use crate::tabs::Tab;
use pretty_term::pty::PseudoTerminalSystem;
use pretty_term::Terminal;
use super::InputManager;
use log::info;

#[derive(Default, Deserialize, Clone)]
pub struct CellSize {
    pub width: f32,
    pub height: f32,
}

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(tab: usize, pty_system: Signal<PseudoTerminalSystem>, input: Signal<InputManager>, hidden: bool) -> Element {
    let mut terminal = use_signal(|| Terminal::setup_no_window().unwrap());
    let debug = use_signal(|| false);
    let cursor_pos = use_memo(move || terminal.read().cursor_pos());

    // Pseudoterminal Stuff
    let (tx, rx) = async_channel::unbounded();
    let mut rx = use_signal(|| rx);
    let pty = use_signal(|| pty_system.write().spawn_new(tx).unwrap());

    // Cell Size Reader
    let mut size_style = use_signal(|| String::new());
    let cell_size = use_resource(move || async move {
        wait_for_next_render().await;

        let mut glyph_size = eval(include_str!("../../js/textsizeloader.js"));

        glyph_size.send((CONFIG.read().font_size).into()).unwrap();
        let size = serde_json::from_value::<CellSize>(glyph_size.recv().await.unwrap()).unwrap();
        size_style.set(format!(
            "--cell-width: {}px; --cell-height: {}px",
            size.width, size.height
        ));
        size
    });

    // Window Resize Event
    on_resize(format!("split-{}", tab), move |size| {
        let DOMRectReadOnly { width, height, .. } = size.content_rect;
        if let Some(cell) = &*cell_size.read() {
            let (rows, cols) = pty_system.write().ptys[*pty.read()].resize(width, height, cell.width, cell.height);
            info!("Resize Event, {rows}:{cols}");
            terminal.write().resize(rows, cols);
        }
    });

    // ANSI code handler
    use_future(move || async move {
        loop {
            if let Ok(a) = rx.write().recv().await {
                terminal.write().handle_actions(a.clone());
            }
        }
    });

    rsx! {
        div {
            style: "{size_style.read()}",
            class: "terminal-split",
            class: if terminal.read().state.alt_screen { "alt-screen" },
            id: "split-{tab}",
            key: "split-{tab}",
            hidden,

            if terminal.read().state.alt_screen {
                CellGrid { terminal }
            } else {
                CommandsSlice { terminal }
            }

            if terminal.read().state.show_cursor {
                Cursor {
                    cursor_pos,
                    index: tab,
                }
            }
        }

        if debug() {
            TerminalDebug { terminal }
        }
    }
}
