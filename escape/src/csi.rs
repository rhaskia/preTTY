use crate::sgr::Sgr;

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

#[derive(Debug, PartialEq, Clone)]
pub enum CsiParam {
    Integer(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Cursor {
    Left(u32),
    Down(u32),
    Right(u32),
    Up(u32),
    PrecedingLine(u32),
    NextLine(u32),
    Position { line: u32, col: u32 },
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
