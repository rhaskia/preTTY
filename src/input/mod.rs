use std::rc::Rc;

use dioxus::events::{ModifiersInteraction, PointerInteraction};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::{KeyboardData, MouseData};
use serde::Deserialize;
use crate::renderer::terminal::CellSize;

pub struct InputManager {
    key_mode: KeyMode,
    mouse_mode: MouseMode,
}

pub enum KeyMode {
    Kitty,
    Legacy,
}

pub enum MouseMode {
    SGR,
    RVXT,
    Normal,
    Unknown,
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
    pub fn new() -> InputManager {
        InputManager {
            key_mode: KeyMode::Legacy,
            mouse_mode: MouseMode::SGR,
        }
    }

    pub fn sgr_mouse(
        &mut self,
        mouse_info: Rc<MouseData>,
        x: usize,
        y: usize,
        is_press: bool,
    ) -> String {
        let trail = if is_press { "M" } else { "m" };
        let button = mouse_info.trigger_button().unwrap_or(MouseButton::Unknown);
        let mods = mouse_info.modifiers();
        println!("{:?}", mods.bits());

        let code = match button {
            MouseButton::Primary => 0,
            MouseButton::Secondary => 1,
            MouseButton::Auxiliary => 2,
            MouseButton::Fourth => 3, // Both of these do not have a proper number rn
            MouseButton::Fifth => 3,  // which is probably fixable but idk what they r
            MouseButton::Unknown => 3,
        };
        println!("{:?}", format!("\x1b[<{code};{x};{y}{trail}"));
        let (x, y) = (x + 1, y + 1);

        format!("\x1b[<{code};{x};{y}{trail}")
    }

    pub fn handle_mouse(
        &mut self,
        mouse_info: Rc<MouseData>,
        x: usize,
        y: usize,
        is_press: bool,
    ) -> String {
        match self.mouse_mode {
            MouseMode::SGR => self.sgr_mouse(mouse_info, x, y, is_press),
            MouseMode::RVXT => todo!(),
            MouseMode::Normal => format!(""),
            MouseMode::Unknown => String::new(),
        }
    }

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
        let modifiers = keyboard_data.modifiers();
        let ctrl = modifiers.ctrl();
        let alt = modifiers.alt();

        use dioxus::events::Key::*;
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
