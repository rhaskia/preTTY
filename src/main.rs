#![feature(if_let_guard)]

// crate imports
mod app;
mod input;
mod renderer;
mod terminal;

use app::app;
use dioxus_desktop::{Config, WindowBuilder};

fn main()  {
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            //.with_background_color((0, 0, 0, 0))
            //.with_disable_context_menu(true)
            //.with_window(WindowBuilder::new().with_decorations(false)),
    );
}
