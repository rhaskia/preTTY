use crate::{Config, keybindings::Keybinding, TerminalAction};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct RawConfig {
    pub start_up_command: Option<String>,
    pub font_size: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawKeybinding {
    pub key: String,
    pub modifiers: Vec<String>,
    pub action: TerminalAction,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawKeybinds {
    pub keybinds: Vec<RawKeybinding>,
}

pub fn load_config() -> Config {
    // will only fail on platforms that aren't supported anyway
    let path = dirs::config_dir().unwrap().join("prettyterm"); 
    let config_file = match std::fs::read_to_string(path.join("config.toml")) {
        Ok(s) => s,
        Err(_) => return Config::default(),
    };

    let RawConfig { start_up_command, font_size } = toml::from_str(&config_file).unwrap();

    Config { font_size, start_up_command } 
}

pub fn load_keybinds() -> Vec<Keybinding> {
    let path = dirs::config_dir().unwrap().join("prettyterm"); 
    let keybind_file = match std::fs::read_to_string(path.join("keybinds.toml")) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let RawKeybinds { keybinds } = toml::from_str(&keybind_file).unwrap();

    let keybinds = keybinds.clone().iter().map(|kb| Keybinding::from(kb.clone())).collect();
    keybinds
}
