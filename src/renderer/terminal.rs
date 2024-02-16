use dioxus::prelude::*;
use crate::terminal::Terminal;
use termwiz::escape::Action;

// #[derive(PartialEq, Props, Clone)]
// struct TerminalProps {
//     data: u8,
// }

pub fn TerminalApp(cx: Scope) -> Element{
    let mut reciever = use_coroutine(cx, |mut rx: UnboundedReceiver<Action>| async move {
        println!("{:?}", rx.try_next());
    });
    let sender = |a| { reciever.send(a) };  

    let t = Terminal::setup(reciever).unwrap();
    let mut terminal = use_state(cx, || t);

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
