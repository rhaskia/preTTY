use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub categories: Vec<String>,
    pub git_repo: String,
}

pub fn available_plugins() -> Result<HashMap<String, String>, String> {
    let mut plugins = HashMap::new();

    Ok(plugins)
}

pub fn installed_plugins() -> Result<HashMap<String, String>, String> {
    let mut plugins = HashMap::new();

    Ok(plugins)
}

pub fn get_plugin_desc(plugin: Plugin) -> Result<String, String> {
    Ok(String::from("Not yet implemented"))
}
