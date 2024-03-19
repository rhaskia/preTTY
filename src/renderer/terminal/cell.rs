use dioxus::prelude::*;
use termwiz::cell::Intensity;
use termwiz::color::ColorSpec;

use crate::renderer::GetClasses;
use crate::terminal::cell::{Cell, CellAttributes, SemanticType};
use crate::terminal::Terminal;

pub type CellClick = (Event<MouseData>, usize, usize);

#[component]
pub fn CellGrid(terminal: Signal<Terminal>, cell_click: EventHandler<CellClick>) -> Element {
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
pub fn CellLine(terminal: Signal<Terminal>, y: usize, cell_click: EventHandler<CellClick>) -> Element {
    rsx! {
        span {
            id: "line_{y}",
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

impl GetClasses for CellAttributes {
    fn get_classes(&self) -> String {
        let intensity = match self.intensity {
            Intensity::Normal => "",
            Intensity::Bold => "cell-bold",
            Intensity::Half => "cell-dim",
        };

        let sem_type = match self.semantic_type {
            SemanticType::Output => "command-ouput",
            SemanticType::Input(_) => "command-input",
            SemanticType::Prompt(_) => "command-prompt",
        };

        let invert = if self.invert { "invert" } else { "" };

        format!("cellspan {intensity} {sem_type} {invert}")
    }
}

#[component]
pub fn CellSpan(cell: Cell, x: usize, y: usize, cell_click: EventHandler<CellClick>) -> Element {
    let fg = cell.attr.fg.to_hex("var(--fg-default)".to_string());
    let bg = cell.attr.bg.to_hex("var(--bg-default)".to_string());

    rsx! {
        span {
            class: "{cell.attr.get_classes()}",
            style: "--fg: {fg}; --bg: {bg}",
            onmouseup: move |e| cell_click.call((e, x, y)),
            key: "{x}:{y}",
            id: "{x}:{y}",
            "{cell.text}"
        }
    }
}
