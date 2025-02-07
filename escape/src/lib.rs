pub mod osc;
pub mod csi;
pub mod parser;
pub mod hyperlink;
pub mod sgr;
pub mod esc;

pub use csi::CSI;
pub use esc::{Esc, EscCode};
pub use osc::OSC;
pub use vtparse::CsiParam;

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
pub struct KittyImage {

}

#[derive(Debug, PartialEq, Clone)]
pub struct Sixel {

}

#[derive(Debug, PartialEq, Clone)]
pub enum DeviceControlMode {
    TmuxEvents(u32),
    ShortDeviceControl(u32),
    Data(u8),
    Exit,
    Enter(Box<EnterDeviceControlMode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnterDeviceControlMode {
    byte: u8,
    params: Vec<i64>,
    intermediates: Vec<u8>,
    ignore_excess_intermediates: bool
}
