use dioxus::prelude::*;
use termwiz::cell::Intensity;
use termwiz::color::ColorSpec;

use crate::renderer::GetClasses;
use crate::terminal::cell::{Cell, CellAttributes, SemanticType};
use crate::terminal::Terminal;

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
    fn to_hex(&self) -> String;
}

impl ToHex for ColorSpec {
    fn to_hex(&self) -> String {
        match self {
            ColorSpec::TrueColor(c) => c.to_string(),
            ColorSpec::Default => "inherit".to_string(),
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

        format!("cellspan {intensity} {sem_type}")
    }
}

#[component]
pub fn CellSpan(cell: Cell, x: usize, y: usize) -> Element {
    let fg = cell.attr.fg.to_hex();
    let bg = cell.attr.bg.to_hex();

    rsx! {
        span {
            class: "{cell.attr.get_classes()}",
            style: "--fg: {fg}; --bg: {bg}",
            key: "{x}:{y}",
            id: "{x}:{y}",
            "{cell.text}"
        }
    }
}
