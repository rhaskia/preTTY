use vtparse::CsiParam;

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

impl Sgr {
    pub fn parse(parameters: &[CsiParam], parameters_truncated: bool) -> Vec<Self> {
        let mut attributes = Vec::new();

        log::info!("{parameters:?}");

        if parameters.len() == 0 {
            return vec![Sgr::Reset];
        }

        let mut i = 0;

        let mut match_color = |i: &mut usize| {
            *i += 2;
            match parameters[*i].as_integer().unwrap() {
                5 => {
                    *i += 2;
                    let n = parameters[*i].as_integer().unwrap() as u8;
                    ColorSpec::PaletteIndex(n)
                }
                2 => {
                    *i += 2;
                    let r = parameters[*i].as_integer().unwrap() as u8;
                    let g = parameters[*i + 2].as_integer().unwrap() as u8;
                    let b = parameters[*i + 4].as_integer().unwrap() as u8;
                    *i += 4;
                    ColorSpec::TrueColor(SrgbaTuple{r, g ,b})
                }
                _ => ColorSpec::Default,
            } 
        };
    
        while i < parameters.len() {
            match parameters[i] {
                CsiParam::Integer(n) => match n {
                    0 => attributes.push(Sgr::Reset),
                    1 => attributes.push(Sgr::Intensity(Intensity::Bold)),
                    2 => attributes.push(Sgr::Intensity(Intensity::Half)),
                    3 => attributes.push(Sgr::Italic(true)),
                    4 => attributes.push(Sgr::Underline(Underline::Single)),
                    5 => attributes.push(Sgr::Blink(Blink::Slow)),
                    6 => attributes.push(Sgr::Blink(Blink::Rapid)),
                    7 => attributes.push(Sgr::Inverse(true)),
                    8 => attributes.push(Sgr::Invisible(true)),
                    9 => attributes.push(Sgr::StrikeThrough(true)),
                    30..=37 => attributes.push(Sgr::Foreground(ColorSpec::PaletteIndex(n as u8 - 30))),
                    38 => attributes.push(Sgr::Foreground(match_color(&mut i))),
                    40..=47 => attributes.push(Sgr::Background(ColorSpec::PaletteIndex(n as u8 - 30))),
                    48 => attributes.push(Sgr::Background(match_color(&mut i))),
                    _ => {}
                }
                _ => {}
            }
            i += 1;
        }

        attributes
    }
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
