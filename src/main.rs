#![feature(if_let_guard)]

// crate imports
mod app;
mod input;
mod renderer;
mod terminal;

use app::App;
use dioxus::prelude::*;

use crate::renderer::cell::CellSpan;
use crate::terminal::Terminal;

fn main()  {
    launch(App);
}
