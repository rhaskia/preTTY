#![feature(if_let_guard)]

// crate imports
mod input;
mod renderer;
mod terminal;

use dioxus::desktop::WindowBuilder;
use dioxus::prelude::*;
use manganis::mg;

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

            //Header {}
            TerminalSplit { }
        }
    }
}

fn main() {
    let cfg = dioxus::desktop::Config::new().with_default_menu_bar(false).with_background_color((0, 0, 0, 0)).with_window(WindowBuilder::new().with_title("PreTTY"));

    LaunchBuilder::new().with_cfg(cfg).launch(App);
}
