use dioxus::prelude::*;
use term::cell::{Cell, CellAttributes, Color, SemanticType};
use term::Terminal;
use termwiz::cell::Intensity;
use termwiz::color::ColorSpec;

pub type ClickEvent = EventHandler<(Event<MouseData>, usize, usize, bool)>;

#[component]
pub fn CellGrid(terminal: Signal<Terminal>) -> Element {
    let scrollback = use_signal(|| 0);

    rsx! {
        pre {
            class: "cells",
            overflow_y: "overflow",

            for y in terminal.read().screen().scroll_range(scrollback()) {
                CellLine { terminal, y }
            }
        }
    }
}

#[component]
pub fn CellLine(terminal: Signal<Terminal>, y: usize) -> Element {
    let term = terminal.read();
    let mut line = term.screen().line(y).iter();
    let mut last_attr = CellAttributes::default();
    let mut open = false;
    let mut rendered = String::new();

    while let Some(cell) = line.next() {
        // Every bit in attributes, associated with a certain tag
        // Multibit attributes are ignored
        for i in 0..13 {
            let last = last_attr.get_bit(i);
            let current = cell.attr.get_bit(i);
            let tag = get_tag(i);

            match (last, current) {
                (true, false) => rendered.push_str(&format!("</{tag}>")),
                (false, true) => rendered.push_str(&format!("<{tag}>")),
                _ => {}
            }
        }

        // TODO: macro for colours?
        // FG Differences
        if cell.attr.get_fg() != last_attr.get_fg() || cell.attr.get_bg() != last_attr.get_bg() {
            let fg = cell.attr.get_fg().to_hex("var(--fg-default)".to_string());
            let bg = cell.attr.get_bg().to_hex("var(--bg-default)".to_string());
            if open {
                rendered.push_str("</fg>");
            }
            rendered.push_str(&format!("<span style=\"color: {fg}; background: {bg}\">"));
            open = true;
        }

        rendered.push(cell.text);
        last_attr = cell.attr.clone();
    }

    rsx! {
        div {
            font_size: "14px",
            id: "line_{y}",
            dangerous_inner_html: rendered,
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

pub fn get_tag(tag: u8) -> String {
    String::from(match tag {
        0 => "strong",
        1 => "dim",
        2 => "em",
        3 => "strike",
        4 => "overline",
        5 => "invert",
        6 => "hide",
        7 => "underline",
        8 => "doubleunderline",
        9 => "wrapped",
        10 => "super",
        11 => "sub",
        12 => "blink",
        13 => "rapidblink",
        _ => "",
    })
}
