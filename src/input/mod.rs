use winit::event::KeyEvent;
use winit::keyboard::{Key, NamedKey};

pub struct InputManager {
    pub alt: bool,
    pub control: bool,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            alt: false,
            control: true,
        }
    }

    // TODO: option string
    pub fn key_to_str(&mut self, key: KeyEvent) -> String {
        match key.logical_key {
            Key::Named(k) => match k {
                NamedKey::Control => self.alt = key.state.is_pressed(),
                _ => {}
            },
            _ => {}
        }

        if key.state.is_pressed() { return String::new(); }

        match key.logical_key {
            Key::Named(k) => match k {
                NamedKey::Escape => String::from("\u{1b}"),
                NamedKey::Delete => String::from("\u{7f}"),
                NamedKey::Backspace => String::from("\u{8}"),
                NamedKey::Enter => String::from("\r\n"),
                NamedKey::Space => String::from(" "),
                NamedKey::Tab => String::from("\t"),

                NamedKey::ArrowRight => String::from("\x1b[C"),
                NamedKey::ArrowLeft => String::from("\x1b[D"),
                NamedKey::ArrowUp => String::from("\x1b[A"),
                NamedKey::ArrowDown => String::from("\x1b[B"),

                _ => String::new(),
            },
            Key::Character(char) => char.to_string(),

            _ => String::new(),
        }
    }
}
