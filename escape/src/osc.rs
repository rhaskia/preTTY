use log::info;

#[derive(Debug, PartialEq, Clone)]
pub enum OSC {
    SetWindowTitle(String),
    SetIconNameAndWindowTitle(String),
    FinalTermSemanticPrompt(FinalTermSemanticPrompt),
    ITermProprietary(ITermProprietary),
    SystemNotification(String),
    CurrentWorkingDirectory(String),
    TextSize(String, TextSize),
    Unknown,
}

impl OSC {
    pub fn parse(data: &[&[u8]]) -> Self {
        if data.len() == 0 {
            return Self::Unknown;
        }
        info!("OSC {data:?}");

        let raw = match String::from_utf8(data[0].to_vec()) {
            Ok(tag) => tag,
            Err(_) => return Self::Unknown,
        };

        let tag = match raw.parse() {
            Ok(tag) => tag,
            Err(_) => return Self::Unknown,
        };

        info!("OSC {tag}");

        match tag {
            66 => Self::parse_text_size(&data[1..]),
            _ => Self::Unknown,
        }
    }

    // https://sw.kovidgoyal.net/kitty/text-sizing-protocol/
    fn parse_text_size(data: &[&[u8]]) -> Self {
        let params = String::from_utf8(data[0].to_vec()).unwrap_or_default();
        let text = String::from_utf8(data[1].to_vec()).unwrap_or_default();
        info!("{text}");

        let mut size = TextSize {
            scale: 1,
            width: 0,
            numerator: 0,
            denominator: 0,
            vertical_align: 0,
            horizontal_align: 0,
        };

        for param in params.split(":") {
            info!("TextSize param {param}");
            let param_value = param[2..].parse().unwrap_or_default();
            match &param[0..2] {
                "s=" => size.scale = param_value,
                "w=" => size.width = param_value,
                "n=" => size.numerator = param_value,
                "d=" => size.denominator = param_value,
                "v=" => size.vertical_align = param_value,
                "h=" => size.horizontal_align = param_value,
                _ => {}
            }
        }

        Self::TextSize(text, size)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FinalTermSemanticPrompt {
    FreshLine,
    FreshLineAndStartPrompt {},
    MarkEndOfCommandWithFreshLine {},
    StartPrompt(FinalTermPromptKind),
    EndOfPromptUntilMarker,
    EndOfPromptUntilEndOfLine,
    EndOfInput { aid: i32 },
    CommandStatus { status: i32, aid: i32 },
}

#[derive(Debug, PartialEq, Clone)]
pub enum ITermProprietary {
    SetUserVar { name: String, value: String },
    ClearScrollback,
    StealFocus,
    SetMark,
    SetProfile(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FinalTermPromptKind {
    Initial,
    RightSide,
    Continuation,
    Secondary,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSize {
    pub scale: u16,
    pub width: u16,
    pub numerator: u16,
    pub denominator: u16,
    pub vertical_align: u16,
    pub horizontal_align: u16,
}
