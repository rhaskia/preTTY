use super::cell::CellSpan;
use crate::input::{Input, InputManager};
use crate::terminal::screen::TerminalRenderer;
use crate::terminal::Terminal;
use dioxus::prelude::*;
use portable_pty::{PtySystem, PtyPair, CommandBuilder};
use std::time::Duration;

pub struct FontInfo {
    pub size: u32,
    pub family: String,
}

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(pair: Signal<PtyPair>) -> Element {
    let mut terminal = use_signal_sync(|| Terminal::setup().unwrap());
    let mut screen = use_signal(|| TerminalRenderer::new());
    let mut actions = use_signal_sync(|| Vec::new());
    
    let child = pair.write().slave.spawn_command(CommandBuilder::new("bash")).ok()?;

    let input = use_signal(|| InputManager::new());
    let font_size = use_signal(|| 14);
    let font = use_signal(|| "JetBrainsMono Nerd Font");

    // let mut glyph_size = eval(r#"
    //     let size = await dioxus.recv();
    //     let width = textSize.getTextWidth({text: 'M', fontSize: size, fontName: "JetBrainsMono Nerd Font"});
    //     dioxus.send(width);
    //     "#);
    //
    // glyph_size.send(font_size.to_string().into()).unwrap();
    //
    // let future = use_future(move || async move { println!("Receieved glyph size"); glyph_size.recv().await.unwrap() });

    let mut key_press = eval(
        r#"
        console.log("adding key listener");
        window.addEventListener('keydown', function(event) {
            let key_info = {"key": event.key,
                            "ctrl": event.ctrlKey,
                            "alt": event.altKey,
                            "meta": event.metaKey,
                            "shift": event.shiftKey,
            };
            dioxus.send(key_info);
        });
        //await dioxus.recv();
    "#,
    );
    
    // Writer future
    use_future(move || async move {
        let mut writer = pair.write().master.take_writer().unwrap();
        
        loop {
            let key = key_press.recv().await.unwrap();
            println!("{actions:?}");

            match input.read().handle_key(key) {
                Input::String(text) => writer.write_all(text.as_bytes()).unwrap(),
                Input::Control(c) => match c.as_str() {
                    "c" => writer.write_all(b"\x03").unwrap(),
                    _ => {}
                },
                _ => {}
            }
        }
    });

    // Reads from the terminal and sends actions into the Terminal object
    use_future(move || async move {
        let mut reader = pair.write().master.try_clone_reader().unwrap();

        std::thread::spawn(move || {
            let mut parser = termwiz::escape::parser::Parser::new();
            let mut buffer = [0u8; 1]; // Buffer to hold a single character

            loop {
                let read = reader.read(&mut buffer);
                
                match read {
                    Ok(_) => {
                        parser.parse(&buffer, |a| actions.push(a)); // terminal.write().handle_action(a));
                    }
                    Err(err) => {
                        eprintln!("Error reading from Read object: {}", err);
                        break;
                    }
                }
            }
        });
    });

    rsx! {
        div {
            script {
                src: "/js/textsize.js"
            }
            pre {
                "hi"
            }
            // pre {
            //     "{actions.read():?}"
            // }
            // for l in terminal.read().get_cells() {
            //     pre {
            //         for cell in l {
            //             CellSpan { cell: cell.clone() }
            //         }
            //     }
            // }
        }
    }
}
