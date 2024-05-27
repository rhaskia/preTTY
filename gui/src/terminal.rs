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
use log::info;
use serde::Deserialize;
use pretty_term::pty::PseudoTerminalSystem;
use pretty_term::Terminal;
use crate::split::Tab;
use super::InputManager;

#[derive(Default, Deserialize, Clone)]
pub struct CellSize {
    pub width: f32,
    pub height: f32,
}

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(tab: Tab, pty_system: Signal<PseudoTerminalSystem>) -> Element {
    let mut input = use_signal(InputManager::new);
    let mut terminal = use_signal(|| Terminal::setup_no_window().unwrap());
    let mut debug = use_signal(|| false);
    let cursor_pos = use_memo(move || terminal.read().cursor_pos());

    // Pseudoterminal Stuff
    let (tx, rx) = async_channel::unbounded();
    let mut rx = use_signal(|| rx);
    let mut pty = use_signal(|| pty_system.write().spawn_new(tx).unwrap());

    // Shift this into a config signal
    let font_size = use_signal(|| 14);

    // Cell Size Reader
    let mut size_style = use_signal(|| String::new());
    let cell_size = use_resource(move || async move {
        wait_for_next_render().await;

        let mut glyph_size = eval(include_str!("../../js/textsizeloader.js"));

        glyph_size.send(font_size.to_string().into()).unwrap();
        let size = serde_json::from_value::<CellSize>(glyph_size.recv().await.unwrap()).unwrap();
        size_style.set(format!(
            "--cell-width: {}px; --cell-height: {}px",
            size.width, size.height
        ));
        size
    });

    // Window Resize Event
    on_resize(format!("split-{}", tab.index), move |size| {
        let DOMRectReadOnly { width, height, .. } = size.content_rect;
        if let Some(cell) = &*cell_size.read() {
            let (rows, cols) = pty.write().resize(width, height, cell.width, cell.height);
            terminal.write().resize(rows, cols);
        }
    });

    // Any Keyboard Events
    let key_press = move |e: Event<KeyboardData>| async move {
        if e.key() == Key::F1 {
            debug.set(!debug());
        }
        let key = input.write().handle_key(e.data);
        pty.write().write(key);
    };

    // ANSI code handler
    use_future(move || async move {
        loop {
            let action = rx.write().recv().await;
            match action {
                Ok(ref a) => terminal.write().handle_actions(a.clone()),
                Err(err) => {}
            }
        }
    });

    let cell_click = EventHandler::new(move |e: (Event<MouseData>, usize, usize, bool)| {
        let (mouse, x, y, is_press) = e;
        info!("Click Event @ {x}:{y}, type: {mouse:?}");
        if let Some(size) = cell_size.read().clone() {
            pty.write()
                .write(input.write().handle_mouse(mouse.data, x, y, is_press));
        }
    });

    rsx! {
        div {
            style: "{size_style.read()}",
            class: "terminal-split",
            class: if terminal.read().state.alt_screen { "alt-screen" },
            id: "split-{tab.index}",
            key: "split-{tab.index}",
            autofocus: true,
            tabindex: tab.index.to_string(),

            onkeydown: key_press,

            if terminal.read().state.alt_screen {
                CellGrid { terminal, cell_click }
            } else {
                CommandsSlice { terminal, cell_click }
            }

            if terminal.read().state.show_cursor {
                Cursor {
                    cursor_pos,
                    index: tab.index,
                }
            }
        }

        if debug() {
            TerminalDebug { terminal }
        }
    }
}
