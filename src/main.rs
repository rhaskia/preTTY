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

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TestApp() -> Element {
    let mut terminal = use_signal(|| Terminal::setup().unwrap());

    // Reads from the terminal and sends actions into the Terminal object
    use_future(move || async move {
        loop {
            terminal.write().read_all_actions();
            tokio::time::sleep(tokio::time::Duration::from_nanos(1000)).await;
        }
    });

    rsx! {
        div{
            button {
                onclick: move |_| terminal.write().read_all_actions(),
                "load"
            }
            for l in terminal.read().get_cells() {
                pre {
                    for cell in l {
                        CellSpan { cell: cell.clone() }
                    }
                }
            }
        }
    }
}

fn main()  {
    launch(App);
}
