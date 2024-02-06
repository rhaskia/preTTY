use winit::keyboard::{Key, NamedKey};
use termwiz::escape::parser::Parser;

pub fn key_event_to_str(key: Key) -> String {
    match key {
        Key::Named(k) => match k {
            NamedKey::Escape => String::from("\u{1b}"),
            NamedKey::Delete => String::from("\u{7f}"),
            NamedKey::Backspace => String::from("\u{8}"),
            NamedKey::Enter => String::from("\r\n"),
            NamedKey::Space => String::from(" "),
            NamedKey::Tab => String::from("\t"),

            _ => String::new(),
        },
        Key::Character(char) => char.to_string(),

        _ => String::new(),
    }
}

