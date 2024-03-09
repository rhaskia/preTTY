use crate::terminal::{
    cell::{Cell, CellAttributes},
    Terminal,
};
use dioxus::prelude::*;
use termwiz::{cell::Intensity, color::ColorSpec};

#[component]
pub fn CellGrid(terminal: Signal<Terminal>) -> Element {
    let scrollback = use_signal(|| 0);

    rsx! {
        pre {
            class: "cells",
            overflow_y: "overflow",

            // Cells
            for y in terminal.read().screen().scroll_range(scrollback()) {
                for (x, cell) in terminal.read().screen().line(y).iter().enumerate() {
                    CellSpan { cell: cell.clone(), x, y }
                }
                br {}
            }
        }
    }
}

pub trait ToHex {
    fn to_hex(&self, def: String) -> String;
}

impl ToHex for ColorSpec {
    fn to_hex(&self, def: String) -> String {
        match self {
            ColorSpec::TrueColor(c) => c.to_string(),
            ColorSpec::Default => def,
            ColorSpec::PaletteIndex(i) => format!("var(--palette-{i})"),
        }
    }
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
pub fn CellSpan(cell: Cell, x: usize, y: usize) -> Element {
    let fg = cell.attr.fg.to_hex(String::from("var(--fg-default)"));
    let bg = cell.attr.bg.to_hex(String::from("var(--bg-default)"));

    rsx! {
        span {
            class: "{cell.attr.get_classes()}",
            style: "--fg: {fg}; --bg: {bg}; --len: {cell.text.len()}",
            key: "{x}:{y}",
            "{cell.text}"
        }
    }
}
