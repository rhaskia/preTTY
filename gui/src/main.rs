#![feature(if_let_guard)]
#![feature(fn_traits)]

// crate imports
mod header;
mod input;
mod tabs;
mod terminal;

use config::Config;
use dioxus::desktop::{WindowBuilder, use_wry_event_handler, tao::{event::{Event, KeyEvent}, window::Window}, use_window};
use dioxus::prelude::*;
use input::InputManager;
use tabs::TerminalSplit;
use config::TerminalAction;
use dioxus::desktop::tao::keyboard::ModifiersState;
use pretty_term::pty::PseudoTerminalSystem;
use log::info;
use crate::tabs::Tab;

pub static CONFIG: GlobalSignal<Config> = Signal::global(|| config::load_config());

#[component]
pub fn App() -> Element {
    let mut input = use_signal(|| InputManager::new());
    let mut pty_system = use_signal(|| PseudoTerminalSystem::setup());
    let mut current_pty = use_signal(|| 0);
    let mut tabs = use_signal(|| vec![Tab::new(0)]);

    rsx! {
        div {
            id: "app",
            class: "app",
            autofocus: true,
            tabindex: 0,

            onkeydown: move |e| match input.read().handle_keypress(&e) {
                TerminalAction::Write(s) => pty_system.write().ptys[*current_pty.read()].write(s),
                TerminalAction::NewTab => {
                    tabs.write().push(Tab::new(90));
                    current_pty += 1;
                }
                // TODO pty removal
                TerminalAction::CloseTab => { 
                    tabs.write().remove(*current_pty.read());
                    // Maybe vector of last tabs open instead of decreasing tab number
                    // Also try trigger quit if only one tab left
                    current_pty -= 1;
                }
                TerminalAction::Quit => use_window().close(),
                action => info!("{:?} not yet implemented", action)
            },

            style {{ include_str!("../../css/style.css") }}
            style {{ include_str!("../../css/gruvbox.css") }}
            style {{ include_str!("../../css/palette.css") }}

            script { src: "/js/textsize.js" }
            script { src: "/js/waitfor.js" }

            TerminalSplit { tabs, input, pty_system }
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
                _ => 1,
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
