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
use pretty_term::Terminal;
use log::info;
use std::{thread, pin::Pin};
use crate::{TABS, PTY_SYSTEM, INPUT};
use dioxus_document::{Eval, Evaluator, eval};
use pretty_term::pty::{PseudoTerminalSystemInner, PseudoTerminal, AsyncReader};
use tokio::time;
use escape::Action;
use tokio::io::AsyncReadExt;
use std::pin::pin;

#[derive(Default, Deserialize, Clone)]
pub struct CellSize {
    pub width: f32,
    pub height: f32,
}

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(pty: String, hidden: bool, index: usize) -> Element {
    let mut terminal = use_signal(|| Terminal::setup_no_window().unwrap());
    let debug = use_signal(|| false);
    let cursor_pos = use_memo(move || terminal.read().cursor_pos());
    let pty = use_signal(|| pty);
    let mut cell_size = use_signal(|| CellSize { width: 8.0, height: 14.0 });

    use_effect(move || {
        INPUT.write().set_kitty_state(terminal.read().kitty_state());
    });

    use_effect(move || {
        TABS.write()[index].name = terminal.read().title.clone();
    });

    // Cell Size Reader
    let mut size_style = use_signal(|| String::new());
    use_future(move || async move {
        let mut glyph_size = eval(include_str!("../../js/textsizeloader.js"));

        glyph_size.send((CONFIG.read().font_size)).unwrap();
        if let Ok(glyph_size) = glyph_size.recv().await {
            let size = serde_json::from_value::<CellSize>(glyph_size).unwrap();
            size_style.set(format!(
                "--cell-width: {}px; --cell-height: {}px",
                size.width, size.height
            ));
            cell_size.set(size);
        }
    });

    // Window Resize Event
    on_resize(format!("split-{}", pty), move |size| {
        let DOMRectReadOnly { width, height, .. } = size.content_rect;
        let (rows, cols) = PTY_SYSTEM.write().get(&pty()).resize(width, height, cell_size.read().width, cell_size.read().height);
        info!("Resize Event, {rows}:{cols}");
        terminal.write().resize(rows, cols);
    });

    // Terminal reading and parsing
    // Need to move to another file
    use_future(move || async move {
        let mut reader = PTY_SYSTEM.write().get(&pty()).reader();
        let mut buffer = [0u8; 1024]; // Buffer to hold a single character
        let mut parser = escape::parser::Parser::new();

        loop {
            let res = reader.read(&mut buffer).await;
            match res {
                Ok(0) => {},
                Ok(n) => {
                    let actions = parser.parse_as_vec(&buffer[..n]);
                    eval(&format!("
                        document.getElementById('split-{pty}').dispatchEvent(new Event(\"scrollCheck\"));
                    "));
                    terminal.write().handle_actions(actions.clone());
                    eval(&format!("
                        document.getElementById('split-{pty}').dispatchEvent(new Event(\"termUpdate\"));
                    "));
                },
                Err(err) => log::error!("{err}"),
            };
        }
    });

    // Terminal Auto Scroll
    use_future(move || async move {
        //wait_for_next_render().await;

        eval(&format!("
            function scrollToBottom() {{
                const termWindow = document.getElementById('split-{pty}'); 
                let n = termWindow.children.length;
                // Do not scroll if there is no scroll, as it bugs out
                if (termWindow.scrollHeight == termWindow.offsetHeight) {{
                    return;
                }}
                termWindow.children[n - 1].scrollIntoView(false);
                termWindow.autoScrolled = true;
            }}

            scrollToBottom();

            const termWindow = document.getElementById('split-{pty}'); 
            termWindow.autoScroll = true;
            termWindow.addEventListener('termUpdate', () => {{
                if (termWindow.autoScroll) {{ scrollToBottom(); }}
                termWindow.autoScrolled = true;
            }});
            
            termWindow.addEventListener('scrollCheck', () => {{
                termWindow.autoScroll = Math.abs(termWindow.scrollHeight - termWindow.scrollTop - termWindow.clientHeight) < 50;
                console.log(termWindow.scrollTop);
            }})
        "))
    });

    rsx! {
        div {
            style: "{size_style.read()}",
            class: "terminal-split",
            class: if terminal.read().state.alt_screen { "alt-screen" },
            id: "split-{pty}",
            key: "split-{pty}",
            hidden,

            if terminal.read().state.alt_screen {
                CellGrid { terminal }
            } else {
                CommandsSlice { terminal }
            }

            if terminal.read().state.show_cursor {
                Cursor {
                    cursor_pos,
                    index: pty,
                }
            }
        }

        if debug() {
            TerminalDebug { terminal }
        }
    }
}
