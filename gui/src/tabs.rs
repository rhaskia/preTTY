use dioxus::prelude::*;
use crate::{CURRENT_TAB, TABS};
use config::TerminalAction;
use crate::handle_action;
use crate::dioxus_elements::input_data::MouseButton;
use dioxus_document::{Eval, Evaluator, eval};

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub name: String,
    pub tab_type: TabType,
    pub pty: String,
}

#[derive(Clone, PartialEq)]
pub enum TabType {
    Menu,
    Terminal,
    PluginMenu,
}

impl Tab {
    pub fn new(pty: String) -> Self {
        Tab {
            name: format!("terminal"),
            tab_type: TabType::Terminal,
            pty,
        }
    }
}

#[component] 
pub fn TabButton(tab: Tab, n: usize) -> Element {
    rsx!{
        span { 
            class: "tab",
            onmousedown: move |e| {
                match e.trigger_button().unwrap() {
                    MouseButton::Primary => *CURRENT_TAB.write() = n,
                    MouseButton::Auxiliary => handle_action(TerminalAction::CloseTabSpecific(n)),
                    _ => {}
                }
            },
            style: if n == CURRENT_TAB() { "--tab-colour: var(--bg1)" },
            div {
                class: "tabtext",
                " {tab.name} "
            }
        }
    }
}

#[component]
pub fn Tabs() -> Element {
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
            for (n, tab) in TABS.read().iter().enumerate() {
                TabButton { tab: tab.clone(), n }
            }
            button {
                class: "barbutton",
                onclick: move |_| handle_action(TerminalAction::NewTab),
                ""
            } 
            div {
                class: "dropdown",
                button {
                    class: "barbutton",
                    onclick: move |_| { 
                        eval(r#"document.getElementById("bardropdown").classList.toggle("show");"#);
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
                            onclick: move |_| handle_action(TerminalAction::OpenSettings),  
                            "Settings" 
                        }
                        button { 
                            onclick: move |_| *crate::COMMAND_PALETTE.write() = true,
                            "Command Palette"
                        }
                        button {
                            onclick: move |_| handle_action(TerminalAction::OpenPluginMenu),
                            "Plugins Menu"
                        }
                        button { "Help" }
                    }
                }
            }
        }
    }
}
