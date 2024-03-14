use termwiz::cell::{Blink, Intensity, Underline};
use termwiz::color::ColorSpec;
use termwiz::escape::osc::FinalTermPromptKind;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Until {
    LineEnd,
    SemanticMarker,
}

impl PromptKind {
    pub fn from(prompt_kind: FinalTermPromptKind) -> Self {
        match prompt_kind {
            FinalTermPromptKind::Initial => Self::Initial,
            FinalTermPromptKind::RightSide => Self::RightSide,
            FinalTermPromptKind::Continuation => Self::Continuation,
            FinalTermPromptKind::Secondary => Self::Secondary,
        }
    }
}

/// Copy of FinalTermPromptKind from termwiz with copy
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum PromptKind {
    /// A normal left side primary prompt
    Initial,
    /// A right-aligned prompt
    RightSide,
    /// A continuation prompt for an input that can be edited
    Continuation,
    /// A continuation prompt where the input cannot be edited
    Secondary,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SemanticType {
    Output,
    Input(Until),
    Prompt(PromptKind),
}

#[derive(Clone, Debug, PartialEq, Copy)]
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

    pub semantic_type: SemanticType,
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
            semantic_type: SemanticType::Output,
        }
    }
}

// Change to enum to allow for box drawing etc
use dioxus::prelude::*;
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Cell {
    pub text: char,
    pub attr: CellAttributes,
}

impl Cell {
    pub fn new(text: char, attr: CellAttributes) -> Cell { Cell { text, attr } }

    pub fn default() -> Cell {
        Cell {
            text: ' ',
            attr: CellAttributes::default(),
        }
    }
}
