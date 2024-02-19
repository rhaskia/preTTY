use dioxus_desktop::tao::event::{KeyEvent};
use dioxus_desktop::tao::event::ElementState;

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

    pub fn parse_key(&mut self, key: &KeyEvent) -> Input {
        println!("{key:?}");
        // Handling modifiers
        match key {
            // Key::Named(k) => match k {
            //     NamedKey::Control => self.control = key.state.is_pressed(),
            //     _ => {}
            // },
            _ => {}
        }
        
        // // Don't sent anything on key up
        if key.state == ElementState::Pressed { return Input::None; }

        match key.logical_key {
            // Key::Named(k) => match k {
            //     NamedKey::Escape => Input::str("\u{1b}"),
            //     NamedKey::Delete => Input::str("\u{7f}"),
            //     NamedKey::Backspace => Input::str("\u{8}"),
            //     NamedKey::Enter => Input::str("\r\n"),
            //     NamedKey::Space => Input::str(" "),
            //     NamedKey::Tab => Input::str("\t"),
            //
            //     NamedKey::ArrowRight => Input::str("\x1b[C"),
            //     NamedKey::ArrowLeft => Input::str("\x1b[D"),
            //     NamedKey::ArrowUp => Input::str("\x1b[A"),
            //     NamedKey::ArrowDown => Input::str("\x1b[B"),
            //
            //     _ => Input::None,
            // },
            // Key::Character(char) if !self.control => Input::String(char.to_string()),
            // Key::Character(char) if self.control => Input::Control(char.to_string()),

            _ => Input::None,
        }
    }
}
