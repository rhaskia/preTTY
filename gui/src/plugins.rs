use dioxus::prelude::*;
use std::path::Path;

#[component]
pub fn PluginManager() -> Element {
    let installed_plugins = config::installed_plugins();

    for plugin in &installed_plugins {
        
    }

    rsx!{
        "{installed_plugins:?}"
    }
} 

pub fn get_plugin_js(plugin_path: &Path) {

} 
