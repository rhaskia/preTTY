#![feature(if_let_guard)]
#![feature(fn_traits)]

// crate imports
mod input;
mod renderer;
mod terminal;
mod hooks;

use dioxus::desktop::WindowBuilder;
use dioxus::prelude::*;
use tracing_subscriber::EnvFilter;

use crate::input::InputManager;
use crate::renderer::TerminalSplit;

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            id: "app",
            class: "app",

            style {{ include_str!("../css/style.css") }}
            style {{ include_str!("../css/gruvbox.css") }}
            style {{ include_str!("../css/palette.css") }}
            // link { href: "/css/style.css" }
            // link { href: "/css/palette.css" }
            // link { href: mg!(file("css/gruvbox.css")) }

            script { src: "/js/textsize.js" }
            script { src: "/js/waitfor.js" }

            //Header {}
            TerminalSplit { }
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .init();

    let cfg = dioxus::desktop::Config::new()
        .with_disable_context_menu(true)
        .with_background_color((0, 0, 0, 0))
        .with_window(WindowBuilder::new().with_title("PreTTY"))
        .with_menu(None);

    LaunchBuilder::new().with_cfg(cfg).launch(App);
}
