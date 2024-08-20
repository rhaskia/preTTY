use config::available_plugins;
use dioxus::prelude::*;
use config::Plugin;

#[component]
pub fn PluginsMenu() -> Element {
    let available_plugins = use_signal(|| config::available_plugins());
    let installed_plugins = use_signal(|| config::installed_plugins());
    let mut current = use_signal(|| true);
    let plugins = use_memo(move || if current() { available_plugins } else { installed_plugins });
    let selected_plugin = use_signal(|| Some(String::from("plugin")));

    rsx!{

        div {
            class: "plugins",
            "{plugins:?}"
            div {
                class: "pluginsideview",
                div {
                    class: "switchplugins",
                    button {
                        onclick: move |_| current.set(false),
                        "Installed"
                    }
                    button {
                        onclick: move |_| current.set(true),
                        "Available"
                    }
                }
                if let Ok(plugins) = plugins.read()() {
                    for (name, plugin) in plugins.iter() {
                        "{plugin.name}"
                    }
                }
            }

            div {
                class: "pluginview"
            }
        }


        // match selected_plugin() {
        //     Some(plugin) => rsx!{ PluginView { plugin: plugins[selected_plugin()] }},
        //     None => rsx!{}
        // }
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
