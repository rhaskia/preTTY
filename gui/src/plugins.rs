use std::collections::HashMap;
use std::path::Path;

use config::Plugin;
use dioxus::prelude::*;

use crate::menu::confirm::Confirm;

pub const PLUGIN_CONFIG: GlobalSignal<config::PluginConfig> = Signal::global(config::plugin_config);

#[component]
pub fn PluginManager() -> Element {
    let installed_plugins = config::installed_plugins();

    use_future(move || async {
        wait_for_next_render().await;
        let installed_plugins = config::installed_plugins();
        for (name, plugin) in &installed_plugins {
            if PLUGIN_CONFIG.read().disabled_plugins.contains(name) {
                continue;
            }
            for js in config::get_plugin_js(&plugin, name) {
                eval(&js);
            }
        }
    });

    rsx! {
        for (name, plugin) in installed_plugins {
            if !PLUGIN_CONFIG.read().disabled_plugins.contains(&plugin.name) {
                for css in config::get_plugin_css(&plugin, &name) {
                    style { "{css}" }
                }
            }
        }
    }
}

pub fn disable(name: &str) {
    PLUGIN_CONFIG
        .write()
        .disabled_plugins
        .push(name.to_string());
}

pub fn enable(name: &str) { PLUGIN_CONFIG.write().disabled_plugins.retain(|e| e != name) }

#[component]
pub fn PluginsMenu(hidden: bool) -> Element {
    let available_plugins = use_signal(config::available_plugins);
    let installed_plugins = use_signal(config::installed_plugins);
    let mut current = use_signal(|| false);
    let mut selected_plugin = use_signal(|| None);
    let plugins = || {
        if current() {
            available_plugins
        } else {
            installed_plugins
        }
    };

    rsx! {
        div {
            class: "plugins",
            display: if hidden { "none" },
            hidden,
            div {
                class: "pluginsideview",
                div {
                    class: "switchplugins",
                    button {
                        onclick: move |_| { current.set(false); },
                        "Installed"
                    }
                    button {
                        onclick: move |_| { current.set(true); },
                        "Available"
                    }
                }
                for (name, plugin) in plugins()() {
                    button {
                        class: "pluginside",
                        onclick: move |_| selected_plugin.set(Some(plugin.clone())),
                        "{plugin.name}"
                    }
                }
            }

            div {
                class: "pluginview",
                match selected_plugin() {
                    Some(plugin) => rsx!{
                        PluginView {
                            plugin: plugin.clone(),
                            installed: config::is_plugin_installed(plugin.name.clone()),
                        }
                    },
                    None => rsx!{}
                }
            }
        }
    }
}

#[component]
pub fn PluginView(plugin: Plugin, installed: bool) -> Element {
    let p = plugin.clone();
    let readme = use_resource(move || {
        let p = p.clone();
        async move { config::get_plugin_readme(p).await }
    });

    rsx! {
        h2 {
            margin_bottom: "2px",
            "{plugin.name}"
        }
        p { "{plugin.desc}" }
        div {
            margin: "10px",
            class: "pluginviewbuttons",
            if installed {
                button {
                    "Uninstall"
                }
                Confirm {}
                if PLUGIN_CONFIG.read().disabled_plugins.contains(&plugin.name) {
                    button {
                        onclick: move |_| crate::plugins::enable(&plugin.name),
                        "Enable"
                    }
                } else {
                    button {
                        onclick: move |_| crate::plugins::disable(&plugin.name),
                        "Disable"
                    }
                }
            }
        }
        div { class: "pluginsep" }
        match &*readme.read_unchecked() {
            Some(Ok(text)) => rsx! {
                p {
                    dangerous_inner_html: markdown::to_html(text),
                }
            },
            Some(Err(_)) => rsx! { p { "Failed loading plugin information." } },
            None => rsx! { p { "Loading plugin information..." } },
        }
    }
}
