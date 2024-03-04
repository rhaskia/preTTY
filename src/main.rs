#![feature(if_let_guard)]

// crate imports
mod input;
mod renderer;
mod terminal;

use crate::input::InputManager;
use crate::renderer::TerminalSplit;
use async_channel::Receiver;
use dioxus::prelude::*;
use input::Input;

#[component]
pub fn App() -> Element {
    let mut input = use_signal(|| InputManager::new());
    let (input_send, input_recv) = async_channel::unbounded();
    let mut input_send = use_signal(|| input_send);
    let mut input_recv = use_signal(|| input_recv);

    // Keyboard input
    let mut js_input = eval(
        r#"
            window.addEventListener('keydown', function(event) {
                let key_info = {"key": event.key,
                                "ctrl": event.ctrlKey,
                                "alt": event.altKey,
                                "meta": event.metaKey,
                                "shift": event.shiftKey,
                };
                console.log(key_info);
                dioxus.send(key_info);
            });
            //await dioxus.recv();
        "#,
    );

    use_future(move || async move {
        loop {
            let raw_key = js_input.recv().await.unwrap();
            let key = input.write().handle_key(raw_key);
            input_send.write().send(key).await;
        }
    });

    rsx! {
        div {
            id: "app",
            class: "app",

            style { {include_str!("style.css")} }
            style { {include_str!("palette.css")} }
            //Header {}
            TerminalSplit { input: input_recv }
        }
    }
}

fn main() {
    launch(App);
}
