use dioxus::prelude::*;
use crate::terminal::Terminal;

// #[derive(PartialEq, Props, Clone)]
// struct TerminalProps {
//     data: u8,
// }

pub fn TerminalApp(cx: Scope) -> Element{
    let t = Terminal::setup().unwrap();

    let mut terminal = use_state(cx, || t);

    // Read and parse output from the pty with reader
    let master = &terminal.pty.pair.master;
    let reader = master.try_clone_reader().unwrap();
    let writer = master.take_writer().unwrap();

    use_future(cx, (), |_| async move {
        let mut buffer = [0u8; 1]; // Buffer to hold a single character
        let mut parser = termwiz::escape::parser::Parser::new();

        loop {
            match reader.read(&mut buffer) {
                Ok(_) => {
                    parser.parse(&buffer, |t| terminal.handle_action(t));
                }
                Err(err) => {
                    eprintln!("Error reading from Read object: {}", err);
                    break;
                }
            }
        }
    });

    cx.render(rsx! {
        terminal.get().get_cells().iter().map(|l| rsx!{ l.iter().map(|cell|
            rsx!{
                div {
                    "{cell.char}"
                }
            })
        })
    })
}
