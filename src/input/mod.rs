use serde::Deserialize;
use serde_json::{from_value, Value};

pub struct InputManager {
    kitty_mode: bool,
}

#[derive(Deserialize)]
pub struct Key {
    key: String,
    alt: bool,
    ctrl: bool,
    meta: bool,
    shift: bool,
}

impl InputManager {
    pub fn new() -> InputManager { InputManager { kitty_mode: false } }

    pub fn ctrl_key(&self, key: char) -> Option<char> {
        // https://sw.kovidgoyal.net/kitty/keyboard-protocol/#ctrl-mapping
        match key {
            // char magic that brings them down into the right range
            ' ' | '2' => Some('\u{0}'),
            'a'..='z' => return Some((key as u8 - 96) as char),
            //'A'..='Z'=> return [(key_char as u8 - 64) as char],
            '[' | '3' => Some('\u{27}'),
            '\\' | '4' => Some('\u{28}'),
            ']' | '5' => Some('\u{29}'),
            '^' | '6' => Some('\u{30}'),
            '/' | '7' => Some('\u{31}'),
            '0' => Some('\u{48}'),
            '1' => Some('\u{49}'),
            '?' | '8' => Some('\u{127}'),
            _ => None,
        }
    }

    pub fn handle_mod_key(&self, key: char, alt: bool, ctrl: bool) -> String {
        let mut key = key;
        if ctrl {
            if let Some(k) = self.ctrl_key(key) {
                key = k;
                println!("{key}");
            }
        }

        if alt {
            format!("\u{1b}{key}")
        } else {
            format!("{key}")
        }
    }

    pub fn handle_key(&self, js_key: Value) -> String {
        let key: Key = from_value(js_key).unwrap();

        if key.key.len() == 1 {
            let key_char = key.key.chars().next().unwrap();

            return self.handle_mod_key(key_char, key.alt, key.ctrl);
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
        }
        .to_string()
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
