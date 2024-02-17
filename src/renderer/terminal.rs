use crate::renderer::Palette;
use crate::terminal::screen::Cell;
use crate::terminal::Terminal;
use dioxus::prelude::*;
use dioxus_desktop::tao::event::WindowEvent;
use dioxus_desktop::tao::keyboard::ModifiersState;
use dioxus_desktop::use_window;
use dioxus_signals::*;
use std::time::Duration;
use termwiz::color::ColorSpec;
use termwiz::escape::Action;

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(cx: Scope) -> Element {
    let mut terminal = use_signal(cx, || Terminal::setup().unwrap());
    let modifiers = use_state(cx, || ModifiersState::empty());
    let window = use_window(cx);

    // window.create_wry_event_handler(|event, t| match event {
    //     dioxus_desktop::tao::event::Event::WindowEvent { event, .. } => match event {
    //         WindowEvent::Resized(size) => todo!(),
    //         WindowEvent::KeyboardInput {
    //             device_id, event, ..
    //         } => todo!(),
    //         WindowEvent::ModifiersChanged(new_modifiers) => modifiers.set(new_modifiers.clone()),
    //         _ => {}
    //     },
    //     _ => {}
    // });

    // Action Receiver
    use_future(cx, (), move |_| async move {
        terminal.write().pty.writer.write_all(b"nvim\n");

        loop {
            let recv = terminal().pty.rx.try_recv();
            match recv {
                Ok(action) => terminal.write().handle_action(action),
                Err(err) => {}
            }
            tokio::time::sleep(Duration::from_nanos(100)).await;
        }
    });

    cx.render(rsx! {
        div{
            terminal().get_cells().into_iter().map(|l| rsx!{
                pre {
                    l.iter().map(|cell| rsx!(CellSpan { cell: cell.clone()}))
                }
            })
        }
    })
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for ColorSpec {
    fn to_hex(&self) -> String {
        match self {
            ColorSpec::TrueColor(c) => c.to_string(),
            ColorSpec::Default => String::from("inherit"),
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
            class: "cell-span",
            style: "color: {cell.attr.fg.to_hex()};",
            "{cx.props.cell.char}"
        }
    })
}
