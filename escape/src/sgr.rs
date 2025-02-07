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

                    21 | 22 => attributes.push(Sgr::Intensity(Intensity::Normal)),
                    23 => attributes.push(Sgr::Italic(false)),
                    24 => attributes.push(Sgr::Underline(Underline::None)),
                    25 => attributes.push(Sgr::Blink(Blink::None)),
                    27 => attributes.push(Sgr::Inverse(false)),
                    28 => attributes.push(Sgr::Invisible(false)),
                    29 => attributes.push(Sgr::StrikeThrough(false)),

                    30..=37 => attributes.push(Sgr::Foreground(ColorSpec::PaletteIndex(n as u8 - 30))),
                    38 => attributes.push(Sgr::Foreground(match_color(&mut i))),
                    39 => attributes.push(Sgr::Foreground(ColorSpec::Default)),

                    40..=47 => attributes.push(Sgr::Background(ColorSpec::PaletteIndex(n as u8 - 40))),
                    48 => attributes.push(Sgr::Background(match_color(&mut i))),
                    49 => attributes.push(Sgr::Background(ColorSpec::Default)),

                    53 => attributes.push(Sgr::Overline(true)),
                    55 => attributes.push(Sgr::Overline(false)),

                    58 => attributes.push(Sgr::UnderlineColor(match_color(&mut i))),
                    59 => attributes.push(Sgr::UnderlineColor(ColorSpec::Default)),

                    73 => attributes.push(Sgr::VerticalAlign(VerticalAlign::SuperScript)),
                    74 => attributes.push(Sgr::VerticalAlign(VerticalAlign::SubScript)),
                    75 => attributes.push(Sgr::VerticalAlign(VerticalAlign::BaseLine)),

                    90..=97 => attributes.push(Sgr::Foreground(ColorSpec::PaletteIndex(n as u8 - 90 + 8))),
                    100..=107 => attributes.push(Sgr::Background(ColorSpec::PaletteIndex(n as u8 - 100 + 8))),
                    _ => log::info!("Unknown sgr: {parameters:?}"),
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
            ColorSpec::PaletteIndex(i) => {
                match i {
                    0..16 => format!("var(--palette-{i})"),
                    16..232 => {
                        let n = i - 16;
                        let r = ((n / 36) % 6) * 42;
                        let g = ((n / 6) % 6) * 42;
                        let b = (n % 6) * 42;
                        format!("rgb({r} {g} {b})")
                    }
                    232..=255 => format!("rgb({0}% {0}% {0}%)", ((i - 231) as f32 / 16.0) * 100.0)
                }
            }
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
