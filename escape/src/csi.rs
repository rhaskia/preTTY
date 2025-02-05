use crate::sgr::Sgr;
pub use vtparse::CsiParam;

#[derive(Debug, PartialEq, Clone)]
pub enum CSI {
    Sgr(Sgr),
    Mode(Mode),
    Cursor(Cursor),
    Edit(Edit),
    Keyboard(Keyboard),
    Device(Box<Device>),
    Mouse,
    Window(Box<Window>),
    SelectCharacterPath(String, String),
    Unspecified(Box<Unspecified>),
}

impl CSI {
    pub fn parse(params: &[CsiParam], parameters_truncated: bool, control: u8) -> Vec<Self> {
        match control {
            b'm' => Sgr::parse(params, parameters_truncated).into_iter().map(|sgr| CSI::Sgr(sgr)).collect(),
            b'A'..=b'H' => {
                let param = if params.len() == 0 { 1 } else { params[0].as_integer().unwrap_or(1) as u32 };

                let cursor = match control {
                    b'A' => Cursor::Up(param),
                    b'B' => Cursor::Down(param),
                    b'C' => Cursor::Right(param),
                    b'D' => Cursor::Left(param),
                    b'E' => Cursor::NextLine(param),
                    b'F' => Cursor::PrecedingLine(param),
                    b'G' => Cursor::HorizontalPosition(param),
                    b'H' => Cursor::Position { 
                        line: param, 
                        col: params.get(2).unwrap_or(&CsiParam::Integer(1)).as_integer().unwrap_or(1) as u32
                    },
                    _ => Cursor::Up(0),
                };
                vec![CSI::Cursor(cursor)]
            }
            _ => vec![CSI::Unspecified(Box::new(Unspecified { 
                params: params.to_vec(),
                parameters_truncated,
                control: control as char
            }))]
        }
    } 
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum CsiParam {
//     Integer(u32),
// }
//
#[derive(Debug, PartialEq, Clone)]
pub enum Cursor {
    Left(u32),
    Down(u32),
    Right(u32),
    Up(u32),
    PrecedingLine(u32),
    NextLine(u32),
    Position { line: u32, col: u32 },
    HorizontalPosition(u32),
    CursorStyle(CursorStyle),
} 

#[derive(Debug, PartialEq, Clone)]
pub enum Device {

} 

#[derive(Debug, PartialEq, Clone)]
pub enum Edit {
    EraseInLine(EraseInLine),
    EraseInDisplay(EraseInDisplay),
    EraseCharacter(u32),
} 

#[derive(Debug, PartialEq, Clone)]
pub enum EraseInDisplay {
    EraseDisplay,
} 

#[derive(Debug, PartialEq, Clone)]
pub enum EraseInLine {
    EraseLine,
    EraseToEnd,
    EraseToStart,
} 

#[derive(Debug, PartialEq, Clone)]
pub struct Unspecified {
    pub control: char,
    pub params: Vec<CsiParam>,
    pub parameters_truncated: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DecPrivateMode {
    Code(usize),
    Unspecified(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyboard {
    SetKittyState { flags: u16, mode: KittyKeyboardMode },
    PushKittyState { flags: u16, mode: KittyKeyboardMode },
    PopKittyState(u16),
    QueryKittySupport,
    ReportKittyState(u16),
} 

#[derive(Debug, PartialEq, Clone)]
pub enum KittyKeyboardMode {
    AssignAll,
    SetSpecified,
    ClearSpecified,
} 

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    SetDecPrivateMode(u16),
    ResetDecPrivateMode(u16),
    SaveDecPrivateMode(u16),
    RestoreDecPrivateMode(u16),
    QueryDecPrivateMode(u16),

    SetMode(TerminalMode),
    ResetMode(TerminalMode),
    XtermKeyMode { resource: XtermKeyModifierResource, value: Option<i64> } ,
    QueryMode(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TerminalMode {}

#[derive(Debug, PartialEq, Clone)]
pub enum XtermKeyModifierResource {}

#[derive(Debug, PartialEq, Clone)]
pub enum CursorStyle {
    Default
}

#[derive(Debug, PartialEq, Clone)]
pub enum Window {}
