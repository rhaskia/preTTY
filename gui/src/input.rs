use std::rc::Rc;
use config::keybindings::Keybinding;
use config::TerminalAction;
use dioxus::events::{Modifiers, ModifiersInteraction, PointerInteraction};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::{Event, KeyboardData, MouseData, Readable};
use log::*;
use crate::KEYBINDS;

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
        let _mods = mouse_info.modifiers();

        let code = match button {
            MouseButton::Primary => 0,
            MouseButton::Secondary => 2,
            MouseButton::Auxiliary => 1,
            MouseButton::Fourth => 3, // Both of these do not have a proper number rn
            MouseButton::Fifth => 3,  // which is probably fixable but idk what they r
            MouseButton::Unknown => 3,
        };
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

    pub fn match_key(&self, keyboard_data: &Event<KeyboardData>) -> String {
        let modifiers = keyboard_data.modifiers();
        let ctrl = modifiers.ctrl();
        let alt = modifiers.alt();

        if self.key_mode == KeyMode::Kitty { return self.kitty_key(keyboard_data); }

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
                info!("Unused Key: {keyboard_data:?}");
                String::new()
            }
        }
    }

    pub fn kitty_key(&self, keyboard_data: &Event<KeyboardData>) -> String {
        let modifier = keyboard_data.modifiers();
    }

    pub fn kitty_modifiers(&self, mods: Modifiers) -> u8 {
        let result = 0;
        set_nth_bit(result, 1, mods.alt());

        result
    }

    fn set_nth_bit(mut num: u8, n: usize, value: bool) -> u8 {
        let mask = 1 << n;
        num &= !mask; // Clear the nth bit
        num |= (value as u32) << n; // Set the nth bit based on the boolean value
        num
    }

    pub fn handle_keypress(&self, key_data: &Event<KeyboardData>) -> TerminalAction {
        for keybind in KEYBINDS.read().iter() {
            if keybind.modifiers == key_data.modifiers() && keybind.key == key_data.key() {
                return keybind.action.clone();
            }
        }

        TerminalAction::Write(self.match_key(key_data))
    }
}
