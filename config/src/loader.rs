use crate::Config;

pub fn load_config() -> Config {
    // will only fail on platforms that aren't supported anyway
    let path = dirs::config_dir().unwrap().join("prettyterm"); 
    let config_file = match std::fs::read_to_string(path.join("config.toml")) {
        Ok(s) => s,
        Err(_) => return Config::default(),
    };

    let config = toml::from_str(&config_file).unwrap();
    println!("{config:?}");

    config
}
