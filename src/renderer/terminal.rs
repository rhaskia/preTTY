use crate::terminal::screen::Cell;
use crate::terminal::Terminal;
use dioxus::prelude::*;
use dioxus_signals::*;
use std::time::Duration;
use termwiz::escape::Action;
use termwiz::color::ColorSpec;
use crate::renderer::Palette;

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(cx: Scope) -> Element {
    let mut terminal = use_signal(cx, || Terminal::setup().unwrap());

    // Action Receiver
    use_future(cx, (), move |_| async move {
        terminal.write().pty.writer.write_all(b"nvim\n");

        loop {
            let recv = terminal().pty.rx.try_recv();
            match recv {
                Ok(action) => terminal.write().handle_action(action),
                Err(err) => {}
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });

    cx.render(rsx! {
        terminal().get_cells().into_iter().map(|l| rsx!{
            pre {
                l.iter().map(|cell| rsx!(CellSpan { cell: cell.clone()}))
            }
        })
    })
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for ColorSpec {
    fn to_hex(&self) -> String {
        match self {
            ColorSpec::TrueColor(c) => c.to_rgba_string(),
            ColorSpec::Default => String::from("#ffffff"),
            // TODO: better implementation
            ColorSpec::PaletteIndex(i) => Palette::default().colors[*i as usize].clone(),
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct CellProps {
    pub cell: Cell,
}

#[component]
pub fn CellSpan(cx: Scope<CellProps>) -> Element {
    let cell = &cx.props.cell;

    cx.render(rsx! {
        span {
            style {
                color: "{cell.attr.fg.to_hex()}"
            }
            "{cx.props.cell.char}"
        }
    })
}


