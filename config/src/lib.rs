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
    pub default_cwd: String,
    pub start_up_command: String,
    pub term: String,
    pub show_tabs: bool,
    pub font_size: u64,
    pub max_scrollback: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self { font_size: 14, max_scrollback: 1000, ..Default::default() }
    }
}
