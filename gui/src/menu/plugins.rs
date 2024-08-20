use config::available_plugins;
use dioxus::prelude::*;
use config::Plugin;

#[component]
pub fn PluginsMenu() -> Element {
    let available_plugins = config::available_plugins();
    let installed_plugins = config::installed_plugins();
    let current = use_signal(|| true);
    let plugins = use_memo(|| if current() { available_plugins } else { installed_plugins });
    let selected_plugin = use_signal(|| None);

    rsx!{
        "{available_plugins:?}"
        select {
            class: "switchplugins",
            option {
                "Installed"
            }
            option {
                "Available"
            }
        }

        div {
            "pluginsideview"
            for plugin in plugins {
                "{plugin.name}"
            }
        }

        match selected_plugin.read() {
            Some(plugin) => PluginView { plugins[selected_plugin()] },
            None => {}
        }
    }
}

#[component]
pub fn PluginView(plugin: Plugin) -> Element {
    let readme = config::get_plugin_desc(plugin.clone());

    rsx! {
        div {
            "pluginview",
            h1 { "{plugin.name}" }
            p { "{readme:?}" }
        }
    }
}
