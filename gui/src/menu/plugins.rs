use dioxus::prelude::*;

#[component]
pub fn PluginsMenu() -> Element {
    let available_plugins = config::available_plugins();

    rsx!{
        "{available_plugins:?}"
    }
}
