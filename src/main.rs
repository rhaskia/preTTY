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

#[component]
pub fn Term() -> Element {
    let mut pty_system = native_pty_system();
    let mut pair = use_signal(|| pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    }).unwrap());
    let (tx, rx) = std::sync::mpsc::channel();

    let mut terminal = use_signal_sync(|| Terminal::setup().unwrap());
    let child = pair.write().slave.spawn_command(CommandBuilder::new("fish")).ok()?;

    let font_size = use_signal(|| 14);
    let font = use_signal(|| "JetBrainsMono Nerd Font");

    use_future(move || async move {
        loop {
            terminal.write().handle_action(rx.recv().unwrap());
        }
    });
    
    // Reads from the terminal and sends actions into the Terminal object
    //use_future(move || async move {
        let mut reader = pair.write().master.try_clone_reader().unwrap();

        let j = std::thread::spawn(move || {
            let mut parser = termwiz::escape::parser::Parser::new();
            let mut buffer = [0u8; 1]; // Buffer to hold a single character

            loop {
                let read = reader.read(&mut buffer);
                
                match read {
                    Ok(_) => {
                        parser.parse(&buffer, |a| { tx.send(a); });//terminal.try_write().unwrap().handle_action(a));
                    }
                    Err(err) => {
                        eprintln!("Error reading from Read object: {}", err);
                        break;
                    }
                }
            }
        });
    //});

    rsx! {
        div {
            script {
                src: "/js/textsize.js"
            }
            for l in terminal.read().get_cells() {
                div {
                    for cell in l {
                        CellSpan { cell: cell.clone() }
                    }
                }
            }
        }
    }
}

fn main()  {
    launch(Term);
}
