use winit::keyboard::{Key, NamedKey};

pub fn key_event_to_str(key: Key) -> String {
    match key {
        Key::Named(k) => match k {
            NamedKey::Escape => String::from("\u{1b}"),
            NamedKey::Delete => String::from("\u{7f}"),
            NamedKey::Backspace => String::from("\u{8}"),
            NamedKey::Enter => String::from("\r\n"),
            NamedKey::Space => String::from(" "),

            _ => String::new(),
        },
        Key::Character(char) => char.to_string(),

        _ => String::new(),
    }
}
