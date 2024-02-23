use crate::input::{Input, InputManager};
use super::{cell::CellSpan};
use crate::terminal::Terminal;

use dioxus::prelude::*;
use dioxus_desktop::{use_window};
use dioxus_signals::{use_signal};

use std::time::Duration;

pub struct FontInfo {
    pub size: u32,
    pub family: String
}

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(cx: Scope) -> Element {
    let terminal = use_signal(cx, || Terminal::setup().unwrap());
    let input = use_signal(cx, || InputManager::new());
    let window = use_window(cx);
    let js = use_eval(cx);
    let font_size = use_state(cx, || 14);
    let font = use_state(cx, || "JetBrainsMono Nerd Font");

    // let glyph_size = js(r#"
    //     let size = await dioxus.recv();
    //     let width = textSize.getTextWidth({text: 'M', fontSize: size, fontName: "JetBrainsMono Nerd Font"});
    //     dioxus.send(width);
    //     "#)
    // .unwrap();
    //
    // glyph_size.send(font_size.to_string().into()).unwrap();

    let key_press = js(r#"
        window.addEventListener('keydown', function(event) {
            let key_info = {"key": event.key,
                            "ctrl": event.ctrlKey,
                            "alt": event.altKey,
                            "meta": event.metaKey,
                            "shift": event.shiftKey,
            };
            dioxus.send(key_info);
        });
    "#).unwrap();

    let current = use_future(cx, (), move |terminal| async move {
        loop {
            let value = key_press.recv().await.unwrap();
            println!("{value:?}");
        }
    });

    let handle_input = move |input: Input| match input {
        Input::String(text) => terminal.write().write_str(text),
        Input::Control(c) => match c.as_str() {
            "c" => terminal.write().write_str("\x03".to_string()),
            _ => {}
        },
        _ => {}
    };

    // Reads from the terminal and sends actions into the Terminal object
    use_future(cx, (), move |_| async move {
        loop {
            terminal.write().read_all_actions();
            tokio::time::sleep(Duration::from_nanos(100)).await;
        }
    });

    //    let future = use_future(cx, (), |_| async move { println!("Receieved glyph size"); glyph_size.recv().await.unwrap() });

    cx.render(rsx! {
        div{
            script {
                src: "/js/textsize.js"
            }
            terminal().get_cells().into_iter().map(|l| rsx!{
                pre {
                    l.iter().map(|cell| rsx!(CellSpan { cell: cell.clone()}))
                }
            })
        }
    })
}
