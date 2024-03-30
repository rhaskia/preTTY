use dioxus::prelude::*;
use termwiz::cell::Intensity;
use termwiz::color::ColorSpec;

use crate::terminal::cell::{Cell, CellAttributes, SemanticType};
use crate::terminal::Terminal;

pub type ClickEvent = EventHandler<(Event<MouseData>, usize, usize, bool)>;

#[component]
pub fn CellGrid(terminal: Signal<Terminal>, cell_click: ClickEvent) -> Element {
    let scrollback = use_signal(|| 0);

    rsx! {
        pre {
            class: "cells",
            overflow_y: "overflow",

            for y in terminal.read().screen().scroll_range(scrollback()) {
                CellLine { terminal, y, cell_click: cell_click.clone() }
            }
        }
    }
}

#[component]
pub fn CellLine(terminal: Signal<Terminal>, y: usize, cell_click: ClickEvent) -> Element {
    rsx! {
        span {
            id: "line_{y}",
            class: "cellline",
            for (x, cell) in terminal.read().screen().line(y).iter().enumerate() {
                CellSpan { cell: cell.clone(), x, y, cell_click: cell_click.clone() }
            }
            br {}
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

#[component]
pub fn CellSpan(cell: Cell, x: usize, y: usize, cell_click: ClickEvent) -> Element {
    let fg = cell.attr.fg.to_hex("var(--fg-default)".to_string());
    let bg = cell.attr.bg.to_hex("var(--bg-default)".to_string());
    let click_up = cell_click.clone();

    rsx! {
        span {
            class: "cellspan",
            class: match cell.attr.intensity { 
                Intensity::Bold => "cell-bold",
                Intensity::Half => "cell-dim",
                _ => ""
            },
            class: match cell.attr.semantic_type {
                SemanticType::Output => "command-ouput",
                SemanticType::Input(_) => "command-input",
                SemanticType::Prompt(_) => "command-prompt",
            },
            class: if cell.attr.invert { "invert" },

            style: "--fg: {fg}; --bg: {bg}",
            key: "{x}:{y}",
            id: "{x}:{y}",

            onmousedown: move |e| cell_click.call((e, x, y, true)),
            onmouseup: move |e| click_up.call((e, x, y, false)),

            "{cell.text}"
        }
    }
}
