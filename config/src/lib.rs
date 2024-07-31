use serde::{Deserialize, Serialize};
mod actions;
pub mod keybindings;
pub mod colour_pal;
mod loader;
pub use actions::TerminalAction;
use keybindings::Keybinding;
pub use loader::*;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Config {
    pub default_cwd: String,
    pub start_up_command: String,
    pub term: String,
    pub palette: String,
    pub show_tabs: bool,
    pub font_size: u64,
    pub max_scrollback: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font_size: 14,
            max_scrollback: 1000,
            default_cwd: String::from("~"),
            start_up_command: String::new(),
            term: String::from("xterm-256color"),
            show_tabs: true,
            palette: String::from("default")
        }
    }
}
