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

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {}
    }

    pub fn handle_key(&self, js_key: Value) -> String {
        let key: Key = from_value(js_key).unwrap();
        
        if key.key.len() == 1 {
            let key_char = key.key.chars().next().unwrap();

            if key.ctrl {
                match key_char {
                    // char magic that brings them down into the right range
                    'a'..='z' => return ((key_char as u8 - 96) as char).to_string(),
                    'A'..='Z' => return ((key_char as u8 - 64) as char).to_string(),
                    '[' => return "\u{27}".to_string(),
                    '\\' => return "\u{28}".to_string(),
                    '}' => return "\u{29}".to_string(),
                    '^' => return "\u{30}".to_string(),
                    ' ' => return "\u{31}".to_string(),
                    _ => {}

                }
            } else {
                return key.key;
            };
        }

        // https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values
        match key.key.as_str() {
            "Escape" => "\u{1b}",
            "Delete" => "\u{7f}",
            "Backspace" => "\u{8}",
            "Enter" => "\r",
            "Tab" => "\t",

            "ArrowRight" => "\x1b[C",
            "ArrowLeft" => "\x1b[D",
            "ArrowUp" => "\x1b[A",
            "ArrowDown" => "\x1b[B",

            _ => {
                println!("{:?}", key.key);
                ""
            }
        }.to_string()
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
