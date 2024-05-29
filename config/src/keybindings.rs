//type Keybinds = std::collections::Hashmap<Key, Action>;
use serde::Deserialize;
use crate::Action;

#[derive(Deserialize, Debug)]
pub enum Key {
    Space
}

#[derive(Deserialize, Debug)]
pub struct Keybinding {
    key: String,
    modifiers: Option<Vec<String>>,
    action: String,
}
