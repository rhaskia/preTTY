use serde::Deserialize;
use serde_json::{from_value, Value};

pub struct InputManager {}

#[derive(Deserialize)]
pub struct Key {
    key: String,
    alt: bool,
    ctrl: bool,
    meta: bool,
    shift: bool,
}

#[derive(Debug)]
pub enum Input {
    String(String),
    Control(String),
    None,
}

impl Input {
    pub fn str(s: &str) -> Input {
        Input::String(s.to_string())
    }
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {}
    }

    pub fn handle_key(&self, js_key: Value) -> Input {
        let key: Key = from_value(js_key).unwrap();
        
        if key.key.len() == 1 {
            return if key.ctrl {
                Input::Control(key.key)
            } else {
                Input::String(key.key)
            };
        }

        // https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values
        match key.key.as_str() {
            "Escape" => Input::str("\u{1b}"),
            "Delete" => Input::str("\u{7f}"),
            "Backspace" => Input::str("\u{8}"),
            "Enter" => Input::str("\r"),
            "Tab" => Input::str("\t"),

            "ArrowRight" => Input::str("\x1b[C"),
            "ArrowLeft" => Input::str("\x1b[D"),
            "ArrowUp" => Input::str("\x1b[A"),
            "ArrowDown" => Input::str("\x1b[B"),

            _ => {
                println!("{:?}", key.key);
                Input::None
            }
        }
    }
}

use dioxus::prelude::*;

pub fn use_js_input() -> UseEval {
    eval(
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
    )
}
