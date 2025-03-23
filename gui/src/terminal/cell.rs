use dioxus::prelude::*;
use escape::sgr::ColorSpec;
use pretty_term::cell::CellAttributes;
use pretty_term::Terminal;

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
    let mut line = term.screen().line(y).unwrap().iter();
    let mut last_attr = CellAttributes::default();
    let mut last_size = None;
    let mut open = false;
    let mut size_open = false;
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

        if let Some(ref extra) = &cell.attr.extra {
            if extra.text_size != last_size {
                if size_open {
                    rendered.push_str("</size>");
                    size_open = false;
                }

                if let Some(ref size) = &extra.text_size {
                    let fraction = if size.numerator > 0 && size.denominator > 0 {
                        size.numerator as f32 / size.denominator as f32
                    } else {
                        1.0
                    };

                    let v_align = match size.vertical_align {
                        0 => "top",
                        1 => "bottom",
                        _ => "text-top",
                    };

                    rendered.push_str(&format!(
                        "<size style=\"--s: {}; --f: {}; vertical-align: {}; --h: {}; ",
                        size.scale, fraction, v_align, size.horizontal_align
                    ));

                    if size.width != 0 {
                        rendered.push_str(&format!("--w: {}", size.width))
                    }

                    rendered.push_str("\">");

                    size_open = true;
                }
            }


            last_size = extra.text_size.clone();
        } else {
            if last_size.is_some() {
                rendered.push_str("</size>");
                size_open = false;
            }
            last_size = None;
        }

        // TODO: macro for colours?
        // FG Differences
        if cell.attr.get_fg() != last_attr.get_fg() || cell.attr.get_bg() != last_attr.get_bg() {
            let fg = cell.attr.get_fg().to_hex("var(--fg-default)".to_string());
            let bg = cell.attr.get_bg().to_hex("var(--bg-default)".to_string());
            if open {
                rendered.push_str("</span>");
            }
            rendered.push_str(&format!(
                "<span class= \"cellspan\" style=\"--fg: {fg}; --bg: {bg};\">"
            ));
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
