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
        println!("{:?}", value.modifiers);
        for modifier in value.modifiers {
            match modifier.trim().to_lowercase().as_str() {
                "control" => modifiers.insert(Modifiers::CONTROL),
                "alt" => modifiers.insert(Modifiers::ALT),
                "meta" => modifiers.insert(Modifiers::META),
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

impl From<Keybinding> for RawKeybinding {
    fn from(value: Keybinding) -> Self {
        let key = value.key.to_string();

        let mut modifiers = value.modifiers.iter_names().map(|(i, _)| i.to_string()).collect();
        
        Self {
            key,
            modifiers,
            action: value.action,
        }
    }
}

