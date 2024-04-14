use dioxus::prelude::*;
use termwiz::cell::Intensity;
use termwiz::color::ColorSpec;
use term::cell::Color;
use term::cell::{Cell, SemanticType};
use term::Terminal;

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
    let term = terminal.read();
    let line = term.screen().line(y);
    rsx! {
        span {
            id: "line_{y}",
            class: "cellline",
            class: if line.double_width() { "doublewidth" },
            class: if line.double_height() { "doubleheight" },
            class: if line.double_size() { "doublesize" },

            for (x, cell) in line.iter().enumerate() {
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
    let fg = cell.attr.get_fg().to_hex("var(--fg-default)".to_string());
    let bg = cell.attr.get_bg().to_hex("var(--bg-default)".to_string());
    let click_up = cell_click.clone();

    rsx! {
        span {
            class: "cellspan",
            // clean this ungodly mess up
            class: if cell.attr.bold() { "bold" },
            class: if cell.attr.dim() { "dim" },
            class: if cell.attr.italic() { "italic" },
            class: if cell.attr.strike() { "strikethrough" },
            class: if cell.attr.overline() { "overline" },
            class: if cell.attr.invert() { "invert" },
            class: if cell.attr.hide() { "hide" },
            class: if cell.attr.single_underline() { "underline" },
            class: if cell.attr.double_underline() { "double_underline" },
            class: if cell.attr.wrapped() { "wrapped" },
            class: if cell.attr.superscript() { "superscript" },
            class: if cell.attr.subscript() { "subscript" },
            class: if cell.attr.slow_blink() { "blink" },
            class: if cell.attr.rapid_blink() { "rapid_blink" },
            class: match cell.attr.semantic_type() {
                SemanticType::Output => "command-ouput",
                SemanticType::Input(_) => "command-input",
                SemanticType::Prompt(_) => "command-prompt",
            },

            style: "--fg: {fg}; --bg: {bg}; --x: {x}; --y: {y}",
            key: "{x}:{y}",
            id: "{x}:{y}",

            onmousedown: move |e| cell_click.call((e, x, y, true)),
            onmouseup: move |e| click_up.call((e, x, y, false)),

            "{cell.text}"
        }
    }
}
