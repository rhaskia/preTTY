#![feature(if_let_guard)]

// crate imports
mod input;
mod renderer;
mod terminal;

use dioxus::prelude::*;
use crate::renderer::TerminalSplit;

#[component]
pub fn App() -> Element {
    let input = use_signal(|| InputManager::new());

    // Keyboard input
    let mut js_input = eval(
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

    use_future(move || async move {
        let key: serde_json::Value = js_input.recv().await.unwrap();
        match input.read().handle_key(key) {

        }
    });

    rsx! {
        div {
            id: "app",
            class: "app",

            style { {include_str!("style.css")} }
            style { {include_str!("palette.css")} }
            //Header {}
            TerminalSplit {}
        }
    }
}

fn main()  {
    launch(App);
}
