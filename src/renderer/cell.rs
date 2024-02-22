use termwiz::cell::Intensity;
use termwiz::color::ColorSpec;
use dioxus::prelude::*;
use crate::terminal::screen::{Cell, CellAttributes};
use crate::renderer::palette::Palette;

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for ColorSpec {
    fn to_hex(&self) -> String {
        match self {
            ColorSpec::TrueColor(c) => c.to_string(),
            ColorSpec::Default => String::from("inherit"),
            // TODO: css implementation
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
