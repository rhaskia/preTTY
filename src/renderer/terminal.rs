use super::cell::CellSpan;
use std::sync::mpsc::channel;
use super::write_block::*;
use crate::input::{Input, InputManager};
use crate::terminal::screen::TerminalRenderer;
use crate::terminal::{Terminal, pty::PseudoTerminal};
use dioxus::prelude::*;
use portable_pty::{PtySystem, PtyPair, CommandBuilder};
use std::time::Duration;

pub struct FontInfo {
    pub size: u32,
    pub family: String,
}

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp() -> Element {
    let (tx, rx) = async_channel::unbounded();
    let mut rx = use_signal(|| rx);
    let mut terminal = use_signal(|| Terminal::setup().unwrap());
    let mut pty = use_signal(|| PseudoTerminal::setup(tx).unwrap());

    let input = use_signal(|| InputManager::new());
    let font_size = use_signal(|| 14);
    let font = use_signal(|| "JetBrainsMono Nerd Font");

    // let mut glyph_size = eval(r#"
    //     let size = await dioxus.recv();
    //     let width = textSize.getTextWidth({text: 'M', fontSize: size, fontName: "JetBrainsMono Nerd Font"});
    //     dioxus.send(width);
    //     "#);
    //
    // glyph_size.send(font_size.to_string().into()).unwrap();
    //
    // let future = use_future(move || async move { println!("Receieved glyph size"); glyph_size.recv().await.unwrap() });

    let mut key_press = eval(
        r#"
        console.log("adding key listener");
        window.addEventListener('keydown', function(event) {
            let key_info = {"key": event.key,
                            "ctrl": event.ctrlKey,
                            "alt": event.altKey,
                            "meta": event.metaKey,
                            "shift": event.shiftKey,
            };
            dioxus.send(key_info);
        });
        //await dioxus.recv();
    "#,
    );
     
    // Writer future
    use_future(move || async move {
        loop {
            let key = key_press.recv().await.unwrap();

            match input.read().handle_key(key) {
                Input::String(text) => pty.write().writer.write_all(text.as_bytes()).unwrap(),
                Input::Control(c) => match c.as_str() {
                    "c" => pty.write().writer.write_all(b"\x03").unwrap(),
                    _ => {}
                },
                _ => {}
            }
        }
    });

    use_future(move || async move {
        loop {
            println!("hello");
            let action = rx.write().recv().await;
            println!("happned");
            terminal.write().handle_action(action.unwrap());
        }
    });
    
    rsx! {
        div {
            script {
                src: "/js/textsize.js"
            }
            pre {
                "hi"
            }
            for l in terminal.read().get_cells() {
                pre {
                    for cell in l {
                        CellSpan { cell: cell.clone() }
                    }
                }
            }
        }
    }
}
