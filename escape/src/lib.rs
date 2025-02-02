pub mod osc;
pub mod csi;
pub mod parser;
pub mod hyperlink;
pub mod sgr;

use csi::CSI;

pub use osc::OSC;

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Print(char),
    PrintString(String),
    DeviceControl(DeviceControlMode),
    Sixel(Box<Sixel>),
    KittyImage(Box<KittyImage>),
    Esc(Esc),
    Control(ControlCode),
    CSI(CSI),
    OSC(Box<OSC>),
    XtGetTcap(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ControlCode {
    LineFeed,
    CarriageReturn,
    Backspace,
    Null,
    Bell,
    HorizontalTab,
    VerticalTab,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Esc {
    Code(EscCode),
    Unspecified { intermediate: String, control: String },
}

#[derive(Debug, PartialEq, Clone)]
pub enum EscCode {

}

#[derive(Debug, PartialEq, Clone)]
pub struct KittyImage {

}

#[derive(Debug, PartialEq, Clone)]
pub struct Sixel {

}

#[derive(Debug, PartialEq, Clone)]
pub enum DeviceControlMode {
    TmuxEvents(u32),
    ShortDeviceControl(u32),
    Data(u32),
    Exit,
    Enter(u32),
}
