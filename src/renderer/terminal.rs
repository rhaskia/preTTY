use super::cell::CellSpan;
use crate::input::use_js_input;
use crate::input::{Input, InputManager};
use crate::terminal::{pty::PseudoTerminal, Terminal};
use dioxus::prelude::*;
use tokio::runtime::Runtime;

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

    let font_size = use_signal(|| 14);
    let mut font_width = use_signal(|| 0);
    let font = use_signal(|| "JetBrainsMono Nerd Font");

    let mut glyph_size = eval(
        r#"
        let size = await dioxus.recv();
        let width = textSize.getTextWidth({text: 'M', fontSize: size, fontName: "JetBrainsMono Nerd Font"});
        dioxus.send(width);
        "#,
    );

    use_future(move || async move {
        font_width.set(serde_json::from_value(glyph_size.recv().await.unwrap()).unwrap());
    });

    use_effect(move || {
        glyph_size.send(font_size.to_string().into()).unwrap();
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
