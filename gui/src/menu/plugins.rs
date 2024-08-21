use dioxus::prelude::*;
use config::Plugin;

#[component]
pub fn PluginsMenu() -> Element {
    let available_plugins = use_signal(config::available_plugins);
    let installed_plugins = use_signal(config::installed_plugins);
    let mut current = use_signal(|| false);
    let mut selected_plugin = use_signal(|| None);
    let plugins = || if current() { available_plugins } else { installed_plugins };

    rsx!{
        div {
            class: "plugins",
            div {
                class: "pluginsideview",
                div {
                    class: "switchplugins",
                    button {
                        onclick: move |_| { current.set(false); selected_plugin.set(None) },
                        "Installed"
                    }
                    button {
                        onclick: move |_| { current.set(true); selected_plugin.set(None) },
                        "Available"
                    }
                }
                for (name, plugin) in plugins()() {
                    button {
                        class: "pluginside",
                        onclick: move |_| selected_plugin.set(Some(name.clone())),
                        "{plugin.name}"
                    }
                }
            }

            div {
                class: "pluginview",
                match selected_plugin() {
                    Some(plugin) => rsx!{ PluginView { plugin: plugins().read()[&plugin].clone() }},
                    None => rsx!{}
                }
            }
        }
    }
}

#[component]
pub fn PluginView(plugin: Plugin) -> Element {
    let readme = config::get_plugin_desc(plugin.clone());

    rsx! {
        h3 { "{plugin.name}" }
        p { "{plugin.desc}" }
        hr {}
        p { "{readme:?}" }
    }
}
