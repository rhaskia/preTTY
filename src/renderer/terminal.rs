pub mod cell;
pub mod commands;
pub mod cursor;

use std::rc::Rc;

use async_channel::Receiver;
use cell::{CellClick, CellGrid};
use commands::CommandsSlice;
use cursor::Cursor;
use dioxus::desktop::use_window;
use dioxus::prelude::*;
use serde::Deserialize;
use termwiz::escape::Action;

use crate::hooks::use_div_size;
use crate::terminal::pty::{PseudoTerminal, PseudoTerminalSystem};
use crate::terminal::Terminal;
use crate::InputManager;

#[derive(Default, Deserialize)]
pub struct CellSize {
    width: f32,
    height: f32,
}

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(index: usize, pty_system: Signal<PseudoTerminalSystem>) -> Element {
    let mut input = use_signal(InputManager::new);
    let mut terminal = use_signal(|| Terminal::setup().unwrap());

    let (tx, rx) = async_channel::unbounded();
    let mut rx = use_signal(|| rx);
    let mut pty = use_signal(|| pty_system.write().spawn_new(tx).unwrap());

    // Shift this into a config signal
    let font_size = use_signal(|| 14);
    let font = use_signal(|| "JetBrainsMono Nerd Font");
    let window = use_window();

    let size = use_div_size(format!("split-{index}"));

    let cell_size = use_resource(move || async move {
        let mut glyph_size = eval(
            r#"
            let size = await dioxus.recv();
            let width = getTextSize(size, "JetBrainsMono Nerd Font");
            dioxus.send(width);
            "#,
        );

        glyph_size.send(font_size.to_string().into()).unwrap();
        let CellSize { width, height } = serde_json::from_value::<CellSize>(glyph_size.recv().await.unwrap()).unwrap();
        format!("--cell-width: {width}px; --cell-height: {height}px")
    });

    let key_press = move |e: Event<KeyboardData>| async move {
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

    let overflow = use_memo(move || {
        if terminal.read().state.alt_screen {
            "hidden"
        } else {
            "auto"
        }
    });

    let cell_click = EventHandler::new(move |e: CellClick| {
        let (mouse, x, y) = e;
        pty.write().write(format!("\x1b[1006<0;{x};{y}M"));
    });

    rsx! {
        div {
            style: "{cell_size.read().clone().unwrap_or_default()}",
            class: "terminal-split",
            id: "split-{index}",
            key: "split-{index}",
            autofocus: true,
            tabindex: index.to_string(),

            onkeydown: key_press,

            "{size.value():?}"

            if terminal.read().state.alt_screen {
                CellGrid { terminal, cell_click: cell_click.clone() }
            } else {
                CommandsSlice { terminal, cell_click  }
            }

            Cursor {
                x: terminal.read().cursor.x,
                y: terminal.read().phys_cursor_y(),
                index,
            }


        }
    }
}
