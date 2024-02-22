use crate::renderer::palette::Palette;
use crate::terminal::screen::{Cell, CellAttributes};
use dioxus::prelude::*;
use termwiz::cell::Intensity;
use termwiz::color::ColorSpec;

pub trait ToHex {
    fn to_hex(&self, def: String) -> String;
}

impl ToHex for ColorSpec {
    fn to_hex(&self, def: String) -> String {
        match self {
            ColorSpec::TrueColor(c) => c.to_string(),
            ColorSpec::Default => def,
            ColorSpec::PaletteIndex(i) => format!("var(--pallete-{i})"),
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
    let fg = cell.attr.fg.to_hex(String::from("var(--fg-default)"));
    let bg = cell.attr.fg.to_hex(String::from("var(--bg-default)"));

    cx.render(rsx! {
        span {
            class: "{cell.attr.get_classes()}",
            style: "--fg: {fg}; --bg: {bg};",
            "{cx.props.cell.char}"
        }
    })
}
