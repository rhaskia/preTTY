use dioxus::prelude::*;
use std::path::Path;

#[component]
pub fn PluginManager() -> Element {
    let installed_plugins = config::installed_plugins();

    rsx!{
        "whuh"
        for (name, plugin) in &installed_plugins {
            for js in config::get_plugin_js(&plugin) {
                script { "{js}" }
            }
            for css in config::get_plugin_css(&plugin, name) {
                style { "{css}" }
            }
        }
    }
} 