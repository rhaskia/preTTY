use termwiz::cell::{Blink, Intensity, Underline, VerticalAlign};
use termwiz::color::{ColorSpec, SrgbaTuple};
use termwiz::escape::csi::Font;
use termwiz::escape::osc::FinalTermPromptKind;
use termwiz::hyperlink::Hyperlink;

/// A Node system for dealing with terminal output
/// Unsure if it should be a syntax tree or just have splitter members in it
pub enum Node {
    Text(String),
    Bold { children: Vec<Node> },
    Dim { children: Vec<Node> },
}

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

#[derive(Clone, Debug, PartialEq)]
pub enum SemanticType {
    Output,
    Input(Until),
    Prompt(PromptKind),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Color {
    #[default]
    Default,
    Palette(u8),
    TrueColor,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CellAttributes {
    pub bg: Color,
    pub fg: Color,
    pub underline_fg: Color,
    // bit 0 = bold
    // bit 1 = dim
    // bit 2 = italic
    // bit 3 = strikethrough
    // bit 4 = overline
    // bit 5 = invert
    // bit 6 = hide
    // bit 7 = underline
    // bit 8 = double underline
    // bit 9 = wrapped
    // bit 10 = superscript
    // bit 11 = subscript
    // bit 12 = slow blink
    // bit 13 = fast_blink
    // bit 14 15 = semantic type
    attributes: u16,
    pub extra: Option<Box<ExtraAttributes>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExtraAttributes {
    font: Font,
    fg: Option<SrgbaTuple>,
    bg: Option<SrgbaTuple>,
    underline_fg: Option<ColorSpec>,
    hyperlink: Option<Hyperlink>,
}

impl ExtraAttributes {
    pub fn default() -> Self {
        ExtraAttributes {
            font: Font::Default,
            fg: None,
            bg: None,
            underline_fg: None,
            hyperlink: None,
        }
    }
}

macro_rules! bitfield {
    ($get:ident, $set:ident, $bit_position:expr) => {
        pub fn $get(&self) -> bool { self.get_bit($bit_position) }

        pub fn $set(&mut self, active: bool) { self.set_bit($bit_position, active); }
    };
}

macro_rules! set_colour {
    ($name:ident, $get:ident, $set:ident) => {
        pub fn $set(&mut self, color: ColorSpec) {
            self.$name = match color {
                ColorSpec::Default => Color::Default,
                ColorSpec::PaletteIndex(idx) => Color::Palette(idx),
                ColorSpec::TrueColor(tc) => {
                    self.get_extra().$name = Some(tc.clone());
                    Color::TrueColor
                }
            }
        }

        pub fn $get(&self) -> ColorSpec {
            match self.$name {
                Color::Default => ColorSpec::Default,
                Color::Palette(idx) => ColorSpec::PaletteIndex(idx),
                Color::TrueColor => {
                    ColorSpec::TrueColor(self.extra.clone().unwrap().$name.unwrap())
                }
            }
        }
    };
}

impl CellAttributes {
    pub fn default() -> CellAttributes {
        CellAttributes {
            bg: Color::Default,
            fg: Color::Default,
            underline_fg: Color::Default,
            attributes: 0,
            extra: None,
        }
    }

    pub fn get_bit(&self, pos: u8) -> bool { ((self.attributes >> pos) & 1) != 0 }

    fn set_bit(&mut self, pos: u8, active: bool) {
        let old = self.attributes;
        self.attributes = if active { old | (1 << pos) } else { old & !(1 << pos) };
    }

    bitfield!(bold, set_bold, 0);
    bitfield!(dim, set_dim, 1);
    bitfield!(italic, set_italic, 2);
    bitfield!(strike, set_strike, 3);
    bitfield!(overline, set_overline, 4);
    bitfield!(invert, set_invert, 5);
    bitfield!(hide, set_hide, 6);
    bitfield!(single_underline, set_single_ul, 7);
    bitfield!(double_underline, set_double_ul, 8);
    bitfield!(wrapped, set_wrapped, 9);
    bitfield!(superscript, set_super, 10);
    bitfield!(subscript, set_sub, 11);
    bitfield!(slow_blink, set_slow_blink, 12);
    bitfield!(rapid_blink, set_rapid_blink, 13);

    pub fn set_sem_type(&mut self, sem: SemanticType) {
        match sem {
            SemanticType::Output => {
                self.set_bit(14, false);
                self.set_bit(15, false);
            }
            SemanticType::Input(_) => {
                self.set_bit(14, true);
                self.set_bit(15, false);
            }
            SemanticType::Prompt(_) => {
                self.set_bit(14, false);
                self.set_bit(15, true);
            }
        }
    }

    pub fn semantic_type(&self) -> SemanticType {
        let first_bit = self.get_bit(14);
        let second_bit = self.get_bit(15);

        match (first_bit, second_bit) {
            (false, false) => SemanticType::Output,
            (true, false) => SemanticType::Input(Until::LineEnd),
            (false, true) => SemanticType::Prompt(PromptKind::Initial),
            _ => panic!("Semantic type bits not set properly"),
        }
    }

    pub fn set_font(&mut self, font: Font) {
        if self.extra.is_none() { self.extra = Some(Box::new(ExtraAttributes::default())) }
        if let Some(ref mut extra) = self.extra {
            extra.font = font;
        }
    }

    pub fn set_two(&mut self, a: u8, b: u8, va: bool, vb: bool) {
        self.set_bit(a, va);
        self.set_bit(b, vb);
    }

    pub fn set_vert_align(&mut self, align: VerticalAlign) {
        match align {
            VerticalAlign::BaseLine => self.set_two(10, 11, false, false),
            VerticalAlign::SuperScript => self.set_two(10, 11, true, false),
            VerticalAlign::SubScript => self.set_two(10, 11, false, true),
        }
    }

    pub fn set_intensity(&mut self, intensity: Intensity) {
        match intensity {
            Intensity::Normal => self.set_two(0, 1, false, false),
            Intensity::Bold => self.set_two(0, 1, true, false),
            Intensity::Half => self.set_two(0, 1, false, true),
        }
    }

    pub fn set_blink(&mut self, blink: Blink) {
        match blink {
            Blink::None => self.set_two(12, 13, false, false),
            Blink::Slow => self.set_two(12, 13, true, false),
            Blink::Rapid => self.set_two(12, 13, false, true),
        }
    }

    pub fn set_underline(&mut self, underline: Underline) {
        match underline {
            Underline::None => self.set_two(7, 8, false, false),
            Underline::Single => self.set_two(7, 8, true, false),
            _ => {
                self.set_two(7, 8, false, true);
                // TODO set others
            }
        }
    }

    set_colour!(fg, get_fg, set_fg);
    set_colour!(bg, get_bg, set_bg);

    pub fn set_underline_colour(&mut self, colour: ColorSpec) { 
            self.get_extra().underline_fg = Some(colour);
    }

    pub fn set_hyperlink(&mut self, link: Option<Hyperlink>) { 
            self.get_extra().hyperlink = link;
    }

    pub fn get_extra(&mut self) -> &mut Box<ExtraAttributes> {
        if self.extra.is_none() {
            self.extra = Some(Box::new(ExtraAttributes::default()))
        }
        self.extra.as_mut().unwrap()
    }

    pub fn hash(&self) -> String { format!("{:?}:{:?}:{}", self.fg, self.bg, self.attributes) }
}

// Change to enum to allow for box drawing etc
#[derive(Clone, Debug, PartialEq)]
#[repr(align(8))]
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

    pub fn hash(&self) -> String { format!("{}:{}", self.text, self.attr.hash()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn set_bold() {
        let mut attr = CellAttributes::default();
        attr.set_bold(true);
        assert!(attr.bold());
    }
}
