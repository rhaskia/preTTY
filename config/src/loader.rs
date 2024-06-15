use crate::{Config, keybindings::Keybinding, TerminalAction};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct RawConfig {
    pub start_up_command: Option<String>,
    pub keybinds: Vec<RawKeybinding>,
    pub font_size: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawKeybinding {
    pub key: String,
    pub modifiers: Vec<String>,
    pub action: TerminalAction,
}

pub fn load_config() -> Config {
    // will only fail on platforms that aren't supported anyway
    let path = dirs::config_dir().unwrap().join("prettyterm"); 
    let config_file = match std::fs::read_to_string(path.join("config.toml")) {
        Ok(s) => s,
        Err(_) => return Config::default(),
    };

    let RawConfig { start_up_command, keybinds, font_size } = toml::from_str(&config_file).unwrap();
    let keybinds = keybinds.clone().iter().map(|kb| Keybinding::from(kb.clone())).collect();

    Config { keybinds, font_size, start_up_command } 
}
