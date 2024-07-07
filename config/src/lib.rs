use serde::{Deserialize, Serialize};
pub mod keybindings;
mod loader;
mod actions;
pub use actions::TerminalAction;
use keybindings::Keybinding;
pub use loader::{load_config, load_keybinds};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Config {
    pub start_up_command: Option<String>,
    pub font_size: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self { start_up_command: None, font_size: 14 }
    }
}
