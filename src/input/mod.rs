use std::rc::Rc;

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

    pub fn handle_mod_key(&self, key: String, alt: bool, ctrl: bool) -> String {
        let mut key = key;
        let char = key.chars().next().unwrap();

        if ctrl {
            if let Some(k) = self.ctrl_key(char) {
                key = k.to_string();
            }
        }

        if alt {
            format!("\u{1b}{key}")
        } else {
            format!("{key}")
        }
    }

    pub fn handle_key(&self, keyboard_data: Rc<KeyboardData>) -> String {
        use keyboard_types::Key::*;
        let modifiers = keyboard_data.modifiers();
        let ctrl = modifiers.ctrl();
        let alt = modifiers.alt();

        match keyboard_data.key() {
            Character(char) => self.handle_mod_key(char, alt, ctrl),
            Enter => String::from("\r"),
            Tab => String::from("\t"),
            Escape => String::from("\u{1b}"),
            Delete => String::from("\u{8}"),
            Backspace => String::from("\u{7f}"),

            ArrowRight => String::from("\x1b[C"),
            ArrowLeft => String::from("\x1b[D"),
            ArrowUp => String::from("\x1b[A"),
            ArrowDown => String::from("\x1b[B"),

            _ => {
                println!("{keyboard_data:?}");
                String::new()
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
