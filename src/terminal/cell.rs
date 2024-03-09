use termwiz::{
    cell::{Blink, Intensity, Underline},
    color::ColorSpec, escape::osc::FinalTermPromptKind,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Until {
    LineEnd,
    SemanticMarker
}

#[derive(Clone, Debug, PartialEq)]
pub enum PromptKind {
    Output,
    Input(Until),
    Prompt(FinalTermPromptKind),
}

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

    pub prompt_kind: PromptKind,
}

pub trait CellHash {
    fn hash(&self) -> String;
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
            prompt_kind: PromptKind::Output,
        }
    }
}

// Change to enum to allow for box drawing etc
use dioxus::prelude::*;
#[derive(Clone, Debug, PartialEq,)]
pub struct Cell {
    pub text: String,
    pub attr: CellAttributes,
}

impl Cell {
    pub fn new(text: String, attr: CellAttributes) -> Cell {
        Cell { text, attr }
    }

    pub fn default() -> Cell {
        Cell {
            text: String::from(" "),
            attr: CellAttributes::default(),
        }
    }
}
