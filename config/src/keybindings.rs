// type Keybinds = std::collections::Hashmap<Key, Action>;
use dioxus::events::{Key, Modifiers};
use serde::Deserialize;
use toml::Table;

use crate::loader::RawKeybinding;
use crate::TerminalAction;

#[derive(Deserialize, Debug, Clone)]
pub struct Keybinding {
    pub key: Key,
    pub modifiers: Modifiers,
    pub action: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeyWrapper {
    key: Key,
}

impl From<RawKeybinding> for Keybinding {
    fn from(value: RawKeybinding) -> Self {
        let key =
            toml::from_str::<KeyWrapper>(&format!("key=\"{}\"", value.key)).unwrap_or(KeyWrapper {
                key: Key::Character(value.key),
            });


        println!("{key:?}");
        Self {
            key: key.key,
            modifiers: Modifiers::ALT,
            action: value.action,
        }
    }
}
