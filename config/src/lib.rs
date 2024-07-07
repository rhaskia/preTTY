use serde::{Deserialize, Serialize};
pub mod keybindings;
mod loader;
mod actions;
pub use actions::TerminalAction;
use keybindings::Keybinding;
pub use loader::load_config;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Config {
    pub start_up_command: Option<String>,
    pub keybinds: Vec<Keybinding>,
    pub font_size: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self { start_up_command: None, keybinds: Default::default(), font_size: 14 }
    }

}
