use serde::{Deserialize, Serialize};

use crate::keybindings::Keybinding;
use crate::{Config, TerminalAction};

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
    let path = dirs::config_dir().unwrap().join("prettyterm/config");
    let config_file = match std::fs::read_to_string(path.join("config.toml")) {
        Ok(s) => s,
        Err(_) => return Config::default(),
    };

    let config = toml::from_str(&config_file).unwrap();

    config
}

pub fn load_keybinds() -> Vec<Keybinding> {
    let path = dirs::config_dir().unwrap().join("prettyterm/config");
    let keybind_file = match std::fs::read_to_string(path.join("keybinds.toml")) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let RawKeybinds { keybinds } = toml::from_str(&keybind_file).unwrap();

    let keybinds = keybinds
        .clone()
        .iter()
        .map(|kb| Keybinding::from(kb.clone()))
        .collect();
    keybinds
}

pub fn save_keybinds(keybinds: Vec<Keybinding>) {
    let raw = keybinds
        .into_iter()
        .map(|k| RawKeybinding::from(k))
        .collect::<Vec<RawKeybinding>>();

    let wrapper = RawKeybinds { keybinds: raw };
    let path = dirs::config_dir().unwrap().join("prettyterm");
    let file = toml::to_string(&wrapper).unwrap();
    confy::store("prettyterm", Some("keybinds"), wrapper).unwrap();
    println!("Saved to {:?}", confy::get_configuration_file_path("prettyterm", Some("keybinds")).unwrap());
}
