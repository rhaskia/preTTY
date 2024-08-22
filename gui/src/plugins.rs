use dioxus::prelude::*;
use std::path::Path;

#[component]
pub fn PluginManager() -> Element {
    let installed_plugins = config::installed_plugins();

    use_future(move || async {
        wait_for_next_render().await;
        let installed_plugins = config::installed_plugins();
        for (name, plugin) in &installed_plugins {
            for js in config::get_plugin_js(&plugin, name) {
                eval(&js);
            }
        }
    });

    rsx!{
        for (name, plugin) in &installed_plugins {
            for css in config::get_plugin_css(&plugin, name) {
                style { "{css}" }
            }
        }
    }
} 
