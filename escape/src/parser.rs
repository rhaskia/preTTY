use crate::{Action, ControlCode, DeviceControlMode, EnterDeviceControlMode};
use crate::esc::Esc;
use crate::csi::CSI;
use crate::osc::OSC;
use vtparse::{VTActor, VTParser, CsiParam};

pub struct Parser {
    input: Vec<u8>,
    actions: Vec<Action>,
    vtparser: VTParser
}

impl Parser {
    pub fn new() -> Self {
        Self { input: Vec::new(), actions: Vec::new(), vtparser: VTParser::new() }
    }

    pub fn parse<F: FnMut(Action)>(&mut self, input: &[u8], mut callback: F) {
        let mut actor = Actor { callback: &mut callback };
        self.vtparser.parse(input, &mut actor);
    }

    pub fn parse_as_vec(&mut self, input: &[u8]) -> Vec<Action> {
        let mut actions = Vec::new();
        self.parse(input, |action| actions.push(action));
        actions
    }
}

struct Actor<'a, F: FnMut(Action) + 'a> {
    callback: &'a mut F
}

impl<F: FnMut(Action)> Actor<'_, F> {
    fn callback(&mut self, a: Action) {
        (self.callback)(a)
    }
}

impl<F: FnMut(Action)> VTActor for Actor<'_, F> {
    fn print(&mut self, b: char) {
        self.callback(Action::Print(b));
    }

    fn execute_c0_or_c1(&mut self, b: u8) {
        match b as char {
            '\u{7}' => self.callback(Action::Control(ControlCode::Bell)),
            '\u{8}' => self.callback(Action::Control(ControlCode::Backspace)),
            '\u{9}' => self.callback(Action::Control(ControlCode::HorizontalTab)),
            '\n' => self.callback(Action::Control(ControlCode::LineFeed)),
            '\r' => self.callback(Action::Control(ControlCode::CarriageReturn)),
            _ => {}
        }
    }

    fn dcs_hook(&mut self, byte: u8, params: &[i64], intermediates: &[u8], ignore_excess_intermediates: bool) {
        // TODO: handle tmux, sixel, etc
        self.callback(Action::DeviceControl(DeviceControlMode::Enter(
            Box::new(
                EnterDeviceControlMode {
                    byte,
                    params: params.to_vec(),
                    intermediates: intermediates.to_vec(),
                    ignore_excess_intermediates
                }
            )
        )));
    }

    fn dcs_put(&mut self, data: u8) {
        // TODO: handle tmux, sixel, etc
        self.callback(Action::DeviceControl(DeviceControlMode::Data(data)))
    }

    fn dcs_unhook(&mut self) {
        self.callback(Action::DeviceControl(DeviceControlMode::Exit))
    }

    fn esc_dispatch(&mut self, params: &[i64], intermediates: &[u8], ignore_excess_intermediates: bool, control: u8) {
        let esc = Esc::parse(intermediates.get(0).copied(), control);
        self.callback(Action::Esc(esc));
    }

    fn csi_dispatch(&mut self, params: &[CsiParam], parameters_truncated: bool, byte: u8) {
        let csi = CSI::parse(params, parameters_truncated, byte);
        for action in csi {
            self.callback(Action::CSI(action))
        }
    }

    fn osc_dispatch(&mut self, osc: &[&[u8]]) {
        let osc = OSC::parse(osc);
        self.callback(Action::OSC(Box::new(osc)));
    }

    fn apc_dispatch(&mut self, data: Vec<u8>) {
        // TODO: parse kitty image
        log::info!("Kitty Image tried to render");
    }
}
