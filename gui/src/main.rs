#![feature(if_let_guard)]
#![feature(fn_traits)]

// crate imports
mod header;
mod input;
mod menu;
mod tabs;
mod terminal;

use async_channel::Receiver;
use config::keybindings::Keybinding;
use config::{Config, TerminalAction};
use dioxus::desktop::{use_window, WindowBuilder};
use dioxus::prelude::*;
use input::InputManager;
use menu::Menu;
use pretty_term::pty::PseudoTerminalSystem;
use tabs::Tabs;
use menu::palette::CommandPalette;
use terminal::TerminalApp;

use crate::tabs::Tab;

pub static CONFIG: GlobalSignal<Config> = Signal::global(|| config::load_config());
pub static KEYBINDS: GlobalSignal<Vec<Keybinding>> = Signal::global(|| config::load_keybinds());
pub static CURRENT_TAB: GlobalSignal<usize> = Signal::global(|| 0);
pub static TABS: GlobalSignal<Vec<Tab>> = Signal::global(|| vec![Tab::new(spawn_new())]);
pub static PTY_SYSTEM: GlobalSignal<PseudoTerminalSystem> = Signal::global(|| PseudoTerminalSystem::setup());
pub static COMMAND_PALETTE: GlobalSignal<bool> = Signal::global(|| false);

pub fn spawn_new() -> String {
    let mut command = None;
    if CONFIG.read().start_up_command.is_empty() {
        command = Some(CONFIG.read().start_up_command.clone());
    }
    PTY_SYSTEM.write().spawn_new(command).unwrap()
}

pub fn handle_action(action: TerminalAction) {
    match action {
        TerminalAction::Write(s) => {
            let tab = &TABS()[*CURRENT_TAB.read()];
            if tab.settings { return }
            PTY_SYSTEM.write().get(&tab.pty).write(s);
        }
        TerminalAction::NewTab => {
            let id = spawn_new();
            TABS.write().push(Tab::new(id));
            *CURRENT_TAB.write() = TABS.read().len() - 1;
        }
        // TODO pty removal
        TerminalAction::CloseTab => {
            TABS.write().remove(*CURRENT_TAB.read());
            if CURRENT_TAB() != 0 { *CURRENT_TAB.write() -= 1; }
            if TABS.read().len() == 0 { use_window().close(); }
        }
        TerminalAction::CloseTabSpecific(n) => {
            TABS.write().remove(n);
            if n <= CURRENT_TAB() { *CURRENT_TAB.write() -= 1; }
            if TABS.read().len() == 0 { use_window().close(); }
        }
        TerminalAction::Quit => use_window().close(),
        TerminalAction::ToggleMenu => {
            let index = TABS.len();
            TABS.write().push(Tab { name: "Settings".to_string(), settings: true, pty: String::new() });
            *CURRENT_TAB.write() = index;
        }
        TerminalAction::NoAction => {}
    }
}

#[component]
pub fn App() -> Element {
    let input = use_signal(|| InputManager::new());

    rsx! {
        style {{ include_str!("../../css/style.css") }}
        style {{ include_str!("../../css/gruvbox.css") }}
        style {{ include_str!("../../css/palette.css") }}

        div {
            id: "app",
            class: "app",
            autofocus: true,
            tabindex: 0,

            onkeydown: move |e| if !COMMAND_PALETTE() {
                handle_action(input.read().handle_keypress(&e)); 
            },

            script { src: "/js/textsize.js" }
            script { src: "/js/waitfor.js" }

            if CONFIG.read().show_tabs { Tabs { input } }
            if COMMAND_PALETTE() { CommandPalette {} }

            div {
                display: "flex",
                flex_grow: 1,
                for (i, tab) in TABS().into_iter().enumerate() {
                    if tab.settings {
                        Menu { active: i == CURRENT_TAB() }
                    } else {
                        TerminalApp { input, hidden: i != CURRENT_TAB(), pty: tab.pty, index: i }
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
