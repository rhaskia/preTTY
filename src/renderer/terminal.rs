use crate::input::{Input, InputManager};
use super::{cell::CellSpan};
use crate::terminal::Terminal;
use dioxus::prelude::*;
use std::time::Duration;

pub struct FontInfo {
    pub size: u32,
    pub family: String
}

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp() -> Element {
    let mut terminal = use_signal(|| Terminal::setup().unwrap());
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

    // let mut key_press = eval(r#"
    //     window.addEventListener('keydown', function(event) {
    //         let key_info = {"key": event.key,
    //                         "ctrl": event.ctrlKey,
    //                         "alt": event.altKey,
    //                         "meta": event.metaKey,
    //                         "shift": event.shiftKey,
    //         };
    //         dioxus.send(key_info);
    //     });
    // "#);
    //
    // let current = use_future(move || async move {
    //     key_press.recv().await.unwrap();
    // });
    //
    // let handle_input = move |input: Input| match input {
    //     Input::String(text) => terminal.write().write_str(text),
    //     Input::Control(c) => match c.as_str() {
    //         "c" => terminal.write().write_str("\x03".to_string()),
    //         _ => {}
    //     },
    //     _ => {}
    // };

    // Reads from the terminal and sends actions into the Terminal object
    use_future(move || async move {
        loop {
            terminal.write().read_all_actions();
            //TODO: wait until terminal can be read
            tokio::time::sleep(Duration::from_nanos(1000)).await;
        }
    });

    rsx! {
        div {
            script {
                src: "/js/textsize.js"
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
