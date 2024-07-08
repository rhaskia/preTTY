#![feature(if_let_guard)]
#![feature(fn_traits)]
#![feature(error_generic_member_access)]

// crate imports
mod header;
mod input;
mod menu;
mod tabs;
mod terminal;

use config::keybindings::Keybinding;
use config::{Config, TerminalAction};
use dioxus::desktop::{use_window, WindowBuilder};
use dioxus::prelude::*;
use input::InputManager;
use menu::Menu;
use pretty_term::pty::PseudoTerminalSystem;
use tabs::Tabs;
use terminal::TerminalApp;

use crate::tabs::Tab;

pub static CONFIG: GlobalSignal<Config> = Signal::global(|| config::load_config());
pub static KEYBINDS: GlobalSignal<Vec<Keybinding>> = Signal::global(|| config::load_keybinds());

#[component]
pub fn App() -> Element {
    let input = use_signal(|| InputManager::new());
    let mut pty_system = use_signal(|| PseudoTerminalSystem::setup());
    let mut current_tab = use_signal(|| 0);
    let mut tabs = use_signal(|| vec![Tab::new(0, 0)]);

    rsx! {
        div {
            id: "app",
            class: "app",
            autofocus: true,
            tabindex: 0,

            onkeydown: move |e| match input.read().handle_keypress(&e) {
                TerminalAction::Write(s) => {
                    let tab = &tabs()[*current_tab.read()];
                    if tab.settings { return }
                    pty_system.write().ptys[tab.pty as usize].write(s);
                }
                TerminalAction::NewTab => {
                    tabs.write().push(Tab::new(current_tab + 1, pty_system.read().len()));
                    current_tab += 1;
                }
                // TODO pty removal
                TerminalAction::CloseTab => {
                    tabs.write().remove(*current_tab.read());
                    // Maybe vector of last tabs open instead of decreasing tab number
                    // Also try trigger quit if only one tab left
                    current_tab -= 1;
                }
                TerminalAction::Quit => use_window().close(),
                TerminalAction::ToggleMenu => {
                    //menu_open.toggle();
                }
                TerminalAction::NoAction => {}
                //action => info!("{:?} not yet implemented", action)
            },

            style {{ include_str!("../../css/style.css") }}
            style {{ include_str!("../../css/gruvbox.css") }}
            style {{ include_str!("../../css/palette.css") }}

            script { src: "/js/textsize.js" }
            script { src: "/js/waitfor.js" }

            div {
                display: "flex",
                flex_direction: "column",
                flex_grow: 1,
                Tabs { tabs, input, pty_system, current_tab }
                div {
                    display: "flex",
                    flex_direction: "row",
                    flex_grow: 1,
                    for tab in tabs().into_iter() {
                        if tab.settings {
                            Menu { active: tab.index == current_tab() }
                        } else {
                            TerminalApp { pty_system, input, hidden: tab.index != current_tab(), pty_no: tab.pty as usize }
                        }
                    }
                }
            }
        }
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            use log::Level::*;
            let colour = match record.level() {
                Error => 32,
                Warn => 33,
                Debug => 33,
                Info => 33,
                Trace => 35,
            };
            out.finish(format_args!(
                "\x1b[{}m[\x1b[1m{} {}]\x1b[m {}",
                colour,
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().unwrap();
    let window = WindowBuilder::new()
        .with_title("PreTTY")
        .with_transparent(true);

    let cfg = dioxus::desktop::Config::new()
        .with_window(window)
        .with_disable_context_menu(true)
        .with_background_color((0, 0, 0, 0))
        .with_menu(None);

    LaunchBuilder::new().with_cfg(cfg).launch(App);
}
