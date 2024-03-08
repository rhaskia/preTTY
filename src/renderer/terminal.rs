use super::cell::CellSpan;
use super::cursor::Cursor;

use crate::input::{use_js_input, Key};
use crate::input::{Input, InputManager};
use crate::terminal::{pty::PseudoTerminal, Terminal};

use async_channel::Receiver;
use dioxus::desktop::use_window;
use dioxus::prelude::*;
use serde::Deserialize;
use tokio::runtime::Runtime;

#[derive(Default, Deserialize)]
pub struct CellSize {
    width: f32,
    height: f32,
}

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(input: Signal<Receiver<Input>>) -> Element {
    let (tx, rx) = async_channel::unbounded();
    let mut rx = use_signal(|| rx);
    let mut terminal = use_signal(|| Terminal::setup().unwrap());
    let mut pty = use_signal(|| PseudoTerminal::setup(tx).unwrap());

    let font_size = use_signal(|| 14);
    let mut cell_size = use_signal_sync(|| CellSize::default());
    let font = use_signal(|| "JetBrainsMono Nerd Font");
    let window = use_window();

    let mut glyph_size = eval(
        r#"
        let size = await dioxus.recv();
        let width = getTextSize(size, "JetBrainsMono Nerd Font");
        dioxus.send(width);
        "#,
    );

    use_future(move || async move {
        let w = serde_json::from_value(glyph_size.recv().await.unwrap()).unwrap();
        cell_size.set(w);
    });

    glyph_size.send(font_size.to_string().into()).unwrap();

    // Key Input Writer
    use_future(move || async move {
        loop {
            let key = input.write().recv().await.unwrap();
            println!("{key:?}");
            pty.write().write_key_input(key);
        }
    });

    // ANSI code handler
    use_future(move || async move {
        loop {
            let action = rx.write().recv().await;
            match action {
                Ok(ref a) => terminal.write().handle_action(a.clone()),
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

    rsx! {
        div {
            style: "--cell-width: {cell_size.read().width}px; --cell-height: {cell_size.read().height}px",
            class: "terminal-split",
            script { src: "/js/textsize.js" }

            if terminal.read().state.alt_screen {
                div {
                    class: "cells",
                    overflow_y: overflow,

                    // Cells
                    for l in terminal.read().renderer.screen.cells {
                        pre {
                            for cell in l {
                                CellSpan { cell: cell.clone() }
                            }
                        }
                    }
                }
            } else {
                for command in terminal.read().renderer.screen.commands {

                }
            }

            Cursor {
                x: terminal.read().cursor.x,
                y: terminal.read().cursor.y,
            }
        }
    }
}
