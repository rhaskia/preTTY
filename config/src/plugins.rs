use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub desc: String,
    pub version: String,
    pub categories: Vec<String>,
    pub git_repo: String,
    pub js_files: Vec<String>,
    pub css_files: Vec<String>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PluginConfig {
    pub disabled_plugins: Vec<String>,
    allow_js: bool,
}

impl PluginConfig {
    pub fn disable_plugin(&mut self, name: &str) {
        println!("{name}");
        self.disabled_plugins.push(name.to_string());
        println!("{:?}", self.disabled_plugins);
    }

    pub fn enable_plugin(&mut self, name: &str) {
        self.disabled_plugins.retain(|e| e != name)
    }
}

pub fn available_plugins() -> HashMap<String, Plugin> {
    let plugins = HashMap::new();

    plugins
}

pub fn plugin_config() -> PluginConfig {
    let dir = crate::dir().join("plugins");
    let config_str = std::fs::read_to_string(dir.join("plugins.toml")).unwrap_or_default();
    toml::from_str(&config_str).unwrap_or_default()
}

pub fn installed_plugins() -> HashMap<String, Plugin> {
    let mut plugins = HashMap::new();
    let dir = crate::dir().join("plugins");
    let mut read_dir = std::fs::read_dir(dir.clone()).unwrap();
    
    while let Some(Ok(entry)) = read_dir.next() {
        if entry.file_type().unwrap().is_dir() {
            let dir = entry.path();
            let plugin_file = std::fs::read_to_string(dir.join("plugin.toml")).unwrap_or_default();
            let plugin = toml::from_str(&plugin_file).unwrap_or_default();
            let name = entry.file_name().into_string().unwrap();
            plugins.insert(name, plugin);
        }
    }

    plugins
}

pub async fn get_plugin_readme(plugin: Plugin) -> Result<String, anyhow::Error> {
    let url = format!("https://raw.githubusercontent.com/{}/main/README.md", plugin.git_repo);
    let client = Client::new();
    let response = client.get(url).send().await.unwrap();

    let readme = response.text().await;
    Ok(readme?)
}

pub fn get_plugin_js(plugin: &Plugin, path: &str) -> Vec<String> {
    let dir = crate::dir().join("plugins").join(path);
    let mut js_files = Vec::new();
    for file in &plugin.js_files {
        let string = std::fs::read_to_string(dir.join(file)).unwrap();
        js_files.push(string);
    }
    js_files
}

pub fn get_plugin_css(plugin: &Plugin, path: &str) -> Vec<String> {
    let dir = crate::dir().join("plugins").join(path);
    let mut css_files = Vec::new();
    for file in &plugin.css_files {
        let string = std::fs::read_to_string(dir.join(file)).unwrap();
        css_files.push(string);
    }
    css_files
}

pub fn get_user_css() -> String {
    let path = crate::dir().join("user.css");
    // default will return an empty string
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn download_plugin(plugin: &Plugin) {
    let dir = crate::dir().join("plugins").join(plugin.name.clone());
    let url = gix::prepare_clone(plugin.git_repo.clone(), dir).unwrap();
}

pub fn is_plugin_installed(plugin: String) -> bool {
    crate::dir().join("plugins").join(plugin).exists()
}
