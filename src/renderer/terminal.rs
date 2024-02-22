use crate::input::{Input, InputManager};
use crate::renderer::palette::Palette;
use crate::terminal::screen::{Cell, CellAttributes};
use crate::terminal::Terminal;
use dioxus::html::object;
use dioxus::prelude::*;
use dioxus_desktop::tao::event::DeviceEvent;
use dioxus_desktop::tao::{
    event::{Event, WindowEvent},
    keyboard::ModifiersState,
};
use dioxus_desktop::{use_window, use_wry_event_handler};
use dioxus_signals::{use_signal, Signal};
use std::rc::Rc;
use std::time::Duration;
use termwiz::cell::Intensity;
use termwiz::color::ColorSpec;

// TODO: split this up for the use of multiple ptys per terminal
#[component]
pub fn TerminalApp(cx: Scope) -> Element {
    let terminal = use_signal(cx, || Terminal::setup().unwrap());
    let input = use_signal(cx, || InputManager::new());
    let window = use_window(cx);
    let js = use_eval(cx);
    let mut font_size = use_state(cx, || 14);

    let mut glyph_size = js(r#"
        let size = await dioxus.recv();
        let width = textSize.getTextWidth({text: 'M', fontSize: size, fontName: 'JetBrainsMono Nerd Font'});
        dioxus.send(width);
        "#)
    .unwrap();

    glyph_size.send(font_size.to_string().into()).unwrap();

    let handle_input = move |input: Input| match input {
        Input::String(text) => terminal.write().write_str(text),
        Input::Control(c) => match c.as_str() {
            "c" => terminal.write().write_str("\x03".to_string()),
            _ => {}
        },
        _ => {}
    };

    // Window event listener
    // Might need to move it up a component to make way for multiple terminals

    // Reads from the terminal and sends actions into the Terminal object
    use_future(cx, (), move |_| async move {
        loop {
            terminal.write().read_all_actions();
            tokio::time::sleep(Duration::from_nanos(100)).await;
        }
    });

    let future = use_future(cx, (), |_| async move { println!("Receieved glyph size"); glyph_size.recv().await.unwrap() });

    cx.render(rsx! {
        div{
            "{future.value():?}"
            script {
                include_str!("../../js/textsize.js")
            }
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

pub trait GetClasses {
    fn get_classes(&self) -> String;
}

impl GetClasses for CellAttributes {
    fn get_classes(&self) -> String {
        let intensity = match self.intensity {
            Intensity::Normal => "",
            Intensity::Bold => "cell-bold",
            Intensity::Half => "cell-dim",
        };

        format!("cellspan {intensity}")
    }
}

#[component]
pub fn CellSpan(cx: Scope<CellProps>) -> Element {
    let cell = &cx.props.cell;

    cx.render(rsx! {
        span {
            class: "{cell.attr.get_classes()}",
            style: "color: {cell.attr.fg.to_hex()}; background-color: {cell.attr.bg.to_hex()}",
            "{cx.props.cell.char}"
        }
    })
}
