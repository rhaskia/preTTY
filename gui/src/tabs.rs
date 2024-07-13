use dioxus::prelude::*;
use crate::input::InputManager;
use pretty_term::pty::PseudoTerminalSystem;
use crate::menu::Menu;

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub name: String,
    pub settings: bool,
    pub pty: String,
}

impl Tab {
    pub fn new(pty: String) -> Self {
        Tab {
            name: format!("terminal"),
            settings: false,
            pty,
        }
    }
}

#[component] 
pub fn Tab(tab: Tab, n: usize, current_tab: Signal<usize>) -> Element {
    rsx!{
        span { 
            class: "tab",
            onmousedown: move |e| {
                match e.trigger_button().unwrap() {
                    dioxus_elements::input_data::MouseButton::Primary => current_tab.set(n),
                    // dioxus_elements::input_data::MouseButton::Auxiliary => { 
                    //     tabs.remove(n);
                    // }
                    _ => {}
                }
            },
            style: if n == current_tab() { "--tab-colour: var(--bg1)" },
            " {tab.name} "
        }
    }
}

#[component]
pub fn Tabs(tabs: Signal<Vec<Tab>>, input: Signal<InputManager>, current_tab: Signal<usize>, pty_system: Signal<PseudoTerminalSystem>) -> Element {
    let mut menu_open = use_signal(|| false);

    eval(r#"
        window.onclick = function(e) {
            var myDropdown = document.getElementById("bardropdown");
            if (myDropdown.classList.contains('show')) {
                myDropdown.classList.remove('show');
            }
        }
     "#); 

    rsx! {
        pre {
            class: "tabs",
            display: "flex",
            font_size: "14px",
            for (n, tab) in tabs.read().iter().enumerate() {
                Tab { tab: tab.clone(), n, current_tab }
            }
            button {
                class: "barbutton",
                onclick: move |_| {
                    let id = crate::spawn_new(pty_system);
                    tabs.write().push(Tab::new(id));
                    current_tab.set(tabs.read().len() - 1);
                },
                ""
            } 
            div {
                class: "dropdown",
                button {
                    class: "barbutton",
                    onclick: move |_| {
                        menu_open.toggle();
                        eval(r#"document.getElementById("bardropdown").classList.toggle("show");"#);
                        // let index = tabs.len();
                        // tabs.write().push(Tab { name: "Settings".to_string(), index, settings: true, pty: -1 });
                        // current_tab.set(index);
                    },
                    ""
                } 
                if true {
                    div {
                        class: "bardropdown",
                        id: "bardropdown",
                        button { "Powershell" }
                        // More shells (generated likely)
                        hr {}
                        button { 
                            onclick: move |_| {
                                let index = tabs.len();
                                tabs.write().push(Tab { name: "Settings".to_string(), settings: true, pty: String::new() });
                                current_tab.set(index);
                            },  
                            "Settings" 
                        }
                        button { "Command Palette" }
                        button { "Help" }
                    }
                }
            }
        }
    }
}
