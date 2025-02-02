#[derive(Debug, PartialEq, Clone)]
pub enum Sgr {
    Reset,
    Foreground(ColorSpec),
    Background(ColorSpec),
    UnderlineColor(ColorSpec),
    Blink(Blink),
    Intensity(Intensity),
    Italic(bool),
    StrikeThrough(bool),
    Inverse(bool),
    Invisible(bool),
    Overline(bool),
    Underline(Underline),
    VerticalAlign(VerticalAlign),
    Font(Font),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Blink {
    None,
    Slow, 
    Rapid
} 

#[derive(Debug, PartialEq, Clone)]
pub enum Intensity {
    Normal,
    Bold,
    Half
}

#[derive(Debug, PartialEq, Clone)]
pub enum Underline {
    None,
    Single,
    Double
}

#[derive(Debug, PartialEq, Clone)]
pub enum VerticalAlign {
    BaseLine,
    SuperScript,
    SubScript
}

#[derive(Debug, PartialEq, Clone)]
pub enum ColorSpec {
    Default,
    PaletteIndex(u8),
    TrueColor(SrgbaTuple),
}

impl ColorSpec {
    pub fn to_hex(&self, def: String) -> String {
        match self {
            ColorSpec::TrueColor(c) => c.to_string(),
            ColorSpec::Default => def,
            ColorSpec::PaletteIndex(i) => format!("var(--palette-{i})"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SrgbaTuple {
    r: u8,
    g: u8,
    b: u8
}

impl SrgbaTuple {
    pub fn to_string(&self) -> String {
        format!("#{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Font {
    Default
}
