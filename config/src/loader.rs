use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::colour_pal::{Palette, default_pal};
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
        Err(err) => {
            return Config::default()
        },
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
    confy::store("prettyterm", Some("keybinds"), wrapper).unwrap();
    println!("Saved to {:?}", confy::get_configuration_file_path("prettyterm", Some("keybinds")).unwrap());
}

pub fn save_config(config: Config) {
    confy::store("prettyterm", Some("config"), config).unwrap();
    println!("Saved to {:?}", confy::get_configuration_file_path("prettyterm", Some("config")).unwrap());
}

pub fn load_palette(name: &str) -> Palette {
    let path = confy::get_configuration_file_path("prettyterm", Some("palettes")).unwrap();
    let pal_file = std::fs::read_to_string(path.join(name.to_owned() +".toml")).unwrap_or_default();

    toml::from_str(&pal_file).unwrap_or_default()
}

pub fn load_palettes() -> HashMap<String, Palette> {
    let path = dirs::config_dir().unwrap().join("prettyterm/palettes");
    std::fs::create_dir_all(&path).ok();
    let read = std::fs::read_dir(path);
    let mut palettes = HashMap::new();
    palettes.insert("default".to_string(), default_pal());

    for file_maybe in read.unwrap() {
        if let Ok(file) = file_maybe {
            let path = file.path();
            let file_string = std::fs::read_to_string(&path).unwrap();
            let mut palette = toml::from_str(&file_string).unwrap_or_default();
            let name = path.file_stem().unwrap().to_str().unwrap();
            fill_out_pal(&mut palette);
            palettes.insert(name.to_string(), palette); 
        }
    }
    
    palettes
}

pub fn fill_out_pal(pal: &mut Palette) {
    for (key, colour) in default_pal() {
        pal.entry(key).or_insert_with(|| colour);
    }
}

pub fn save_palettes(palettes: HashMap<String, Palette>) {
    let path = dirs::config_dir().unwrap().join("prettyterm/palettes");

    for (key, palette) in palettes {
        let pal_str =  serialize_pal_ordered(palette);
        std::fs::write(path.join(key + ".toml"), pal_str).unwrap();
    }
}

pub fn serialize_pal_ordered(pal: Palette) -> String {
    let mut toml = String::new();
    let order = crate::colour_pal::pal_groups().into_iter().flatten();

    for key in order {
        let val = &pal.get(key);
        if let Some(colour) = val {
            toml.push_str(&format!("{key} = \"{colour}\" \n"));
        }
    }

    toml
}
