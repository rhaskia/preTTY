use termwiz::{
    cell::{Blink, Intensity, Underline},
    color::ColorSpec,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CellAttributes {
    pub bg: ColorSpec,
    pub fg: ColorSpec,
    // TODO bitmap these
    pub underline: Underline,
    pub intensity: Intensity,
    pub italic: bool,
    pub strikethrough: bool,
    pub blink: Blink,
    pub underline_fg: ColorSpec,
}

impl CellAttributes {
    pub fn default() -> CellAttributes {
        CellAttributes {
            bg: ColorSpec::Default,
            fg: ColorSpec::Default,
            underline: Underline::None,
            intensity: Intensity::Normal,
            italic: false,
            strikethrough: false,
            blink: Blink::None,
            underline_fg: ColorSpec::Default,
        }
    }
}

// Change to enum to allow for box drawing etc
use dioxus::prelude::*;
#[derive(Clone, Debug, PartialEq, Props)]
pub struct Cell {
    pub char: char,
    pub attr: CellAttributes,
}

impl Cell {
    pub fn new(char: char, attr: CellAttributes) -> Cell {
        Cell { char, attr }
    }

    pub fn default() -> Cell {
        Cell {
            char: ' ',
            attr: CellAttributes::default(),
        }
    }

    pub fn new_line() -> Cell {
        Cell {
            char: '\n',
            attr: CellAttributes::default(),
        }
    }
}
