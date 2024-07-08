// type Keybinds = std::collections::Hashmap<Key, Action>;
use dioxus::events::{Key, Modifiers};
use serde::{Deserialize, Serialize};
use crate::loader::RawKeybinding;
use crate::TerminalAction;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Keybinding {
    pub key: Key,
    pub modifiers: Modifiers,
    pub action: TerminalAction,
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

        let mut modifiers = Modifiers::empty();
        for modifier in value.modifiers {
            match modifier.trim().to_lowercase().as_str() {
                "ctrl" => modifiers.insert(Modifiers::CONTROL),
                "alt" => modifiers.insert(Modifiers::ALT),
                "super" => modifiers.insert(Modifiers::SUPER),
                "shift" => modifiers.insert(Modifiers::SHIFT),
                // TODO: more?
                _ => { } // TODO: error
            }
        }

        Self {
            key: key.key,
            modifiers,
            action: value.action,
        }
    }
}

