#![feature(if_let_guard)]
#![feature(fn_traits)]

// crate imports
mod header;
mod input;
mod split;
mod terminal;

use std::time::SystemTime;

use dioxus::desktop::WindowBuilder;
use dioxus::prelude::*;
use input::InputManager;
use split::TerminalSplit;

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            id: "app",
            class: "app",

            style {{ include_str!("../../css/style.css") }}
            style {{ include_str!("../../css/gruvbox.css") }}
            style {{ include_str!("../../css/palette.css") }}
            link { href: "~/.config/pretty/style.css" }
            // link { href: "/css/palette.css" }
            // link { href: mg!(file("css/gruvbox.css")) }

            script { src: "/js/textsize.js" }
            script { src: "/js/waitfor.js" }

            //Header {}
            TerminalSplit { }
        }
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "\x1b[32m[\x1b[1m{} {}]\x1b[m {}",
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

    let cfg = dioxus::desktop::Config::new()
        .with_disable_context_menu(true)
        .with_background_color((0, 0, 0, 0))
        .with_window(WindowBuilder::new().with_title("PreTTY"))
        .with_menu(None);

    LaunchBuilder::new().with_cfg(cfg).launch(App);
}
