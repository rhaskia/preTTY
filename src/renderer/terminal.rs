use super::cell::CellSpan;
use crate::input::{use_js_input, Key};
use crate::input::{Input, InputManager};
use crate::terminal::{pty::PseudoTerminal, Terminal};
use async_channel::Receiver;
use dioxus::desktop::use_window;
use dioxus::prelude::*;
use tokio::runtime::Runtime;

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(input: Signal<Receiver<Input>>) -> Element {
    let (tx, rx) = async_channel::unbounded();
    let mut rx = use_signal(|| rx);
    let mut terminal = use_signal(|| Terminal::setup().unwrap());
    let mut pty = use_signal(|| PseudoTerminal::setup(tx).unwrap());

    let font_size = use_signal(|| 14);
    let mut font_width = use_signal_sync(|| 0.0f64);
    let font = use_signal(|| "JetBrainsMono Nerd Font");
    let window = use_window();

    window.webview.evaluate_script_with_callback(
        r#"
        textSize.getTextWidth({text: 'M', fontSize: 14, fontName: "JetBrainsMono Nerd Font"})
        "#,
        move |width| match width.parse() {
            Ok(w) => font_width.clone().set(w),
            Err(_) => {}
        },
    );

    // use_future(move || async move {
    //     font_width.set(serde_json::from_value(glyph_size.recv().await.unwrap()).unwrap());
    // });

    // use_effect(move || {
    //     glyph_size.send(font_size.to_string().into()).unwrap();
    // });

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
            overflow_y: overflow,
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
