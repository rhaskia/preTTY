#![feature(if_let_guard)]

// crate imports
mod app;
mod input;
mod renderer;
mod terminal;

use app::app;
use dioxus::prelude::*;

fn main()  {
    launch(app);
}
