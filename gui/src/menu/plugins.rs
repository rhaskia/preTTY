use dioxus::prelude::*;
use config::Plugin;

#[component]
pub fn PluginsMenu(hidden: bool) -> Element {
    let available_plugins = use_signal(config::available_plugins);
    let installed_plugins = use_signal(config::installed_plugins);
    let mut current = use_signal(|| false);
    let mut selected_plugin = use_signal(|| None);
    let plugins = || if current() { available_plugins } else { installed_plugins };

    rsx!{
        div {
            class: "plugins",
            display: if hidden { "none" },
            hidden,
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
                    Some(plugin) => rsx!{ 
                        PluginView { 
                            plugin: plugins().read()[&plugin].clone(),
                            installed: current()
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
        async move {
            config::get_plugin_readme(p).await
        }
    });

    rsx! {
        h2 { 
            margin_bottom: "2px",
            "{plugin.name}"
        }
        p { "{plugin.desc}" }
        div { 
            margin: "10px",
            if installed {
                button {
                    "Uninstall"
                }
                button {
                    "Disable"
                }
            } else {
                button {
                    "Install"
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
