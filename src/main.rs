#![feature(if_let_guard)]

// crate imports
mod app;
mod input;
mod renderer;
mod terminal;

use std::ops::Add;
use app::App;
use dioxus::prelude::*;

use crate::renderer::cell::CellSpan;
use crate::terminal::Terminal;
use portable_pty::{CommandBuilder, native_pty_system, PtySize};

fn main()  {
    launch(App);
}
