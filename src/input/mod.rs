use dioxus_desktop::tao::event::*;
use dioxus_desktop::tao::keyboard::Key;
use crate::terminal::screen::{Cell, CellAttributes};

pub struct InputManager {
    pub alt: bool,
    pub control: bool,
    pub super_key: bool,
    pub shift: bool, // not acut
}

pub enum Input {
    String(String),
    Control(String),
    None
}

impl Input {
    pub fn str(s: &str) -> Input {
        Input::String(s.to_string())
    }
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            super_key: false,
            shift: false,
            alt: false,
            control: false,
        }
    }

    pub fn parse_raw_key(&mut self, key: RawKeyEvent) {

    }

    pub fn parse_key(&mut self, key: &KeyEvent) -> Input {
        println!("input key {key:?}");

        // Handling modifiers
        match key.logical_key {
            Key::Control => self.control = key.state == ElementState::Pressed,
            _ => {}
        }
        
        // // Don't sent anything on key up
        if key.state == ElementState::Released { return Input::None; }

        match key.logical_key {
            Key::Escape => Input::str("\u{1b}"),
            Key::Delete => Input::str("\u{7f}"),
            Key::Backspace => Input::str("\u{8}"),
            Key::Enter => Input::str("\r\n"),
            Key::Space => Input::str(" "),
            Key::Tab => Input::str("\t"),
            
            Key::ArrowRight => Input::str("\x1b[C"),
            Key::ArrowLeft => Input::str("\x1b[D"),
            Key::ArrowUp => Input::str("\x1b[A"),
            Key::ArrowDown => Input::str("\x1b[B"),
            
            Key::Character(char) if !self.control => Input::String(char.to_string()),
            Key::Character(char) if self.control => Input::Control(char.to_string()),

            _ => {
                println!("{key:?}");
                Input::None
            },
        }
    }
}
