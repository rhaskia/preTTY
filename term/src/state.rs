use std::collections::HashMap;

use log::info;
use num_traits::cast::ToPrimitive;
use escape::csi::{
    DecPrivateMode, Keyboard, KittyKeyboardMode, Mode, TerminalMode, XtermKeyModifierResource
};
use escape::DeviceControlMode;

// TODO: bitfield? may not be nessecary
#[derive(Debug, Default)]
pub struct TerminalState {
    pub cwd: String,
    // dec private
    pub dec_modes: HashMap<u16, bool>,
    dec_saves: HashMap<u16, bool>,
    // terminal mode
    pub modes: HashMap<u16, bool>,

    pub alt_screen: bool,
    pub bracketed_paste: bool,
    pub show_cursor: bool,
    pub alt_keypad: bool,
    pub kitty_state: u16,
}

impl TerminalState {
    pub fn new() -> TerminalState {
        TerminalState {
            ..Default::default()
        }
    }

    pub fn dec_mode(&self, code: u16) -> bool {
        *self
            .dec_modes
            .get(&code)
            .unwrap_or(&false)
    }

    pub fn dec_save(&self, code: u16) -> bool {
        *self
            .dec_saves
            .get(&code)
            .unwrap_or(&false)
    }

    pub fn device_control(&mut self, device_command: DeviceControlMode) {
        match device_command {
            DeviceControlMode::Enter(_) => todo!(),
            DeviceControlMode::Exit => todo!(),
            DeviceControlMode::Data(_) => todo!(),
            DeviceControlMode::ShortDeviceControl(_) => todo!(),
            DeviceControlMode::TmuxEvents(_) => todo!(),
            // _ => info!("Device Command {:?}", device_command),
        }
    }

    pub fn handle_state(&mut self, mode: Mode) {
        use escape::csi::Mode::*;
        match mode {
            SetDecPrivateMode(pmode) => self.set_dec_private_mode(pmode, true),
            ResetDecPrivateMode(pmode) => self.set_dec_private_mode(pmode, false),
            SaveDecPrivateMode(pmode) => self.save_dec_private_mode(pmode),
            RestoreDecPrivateMode(pmode) => self.restore_dec_private_mode(pmode),
            QueryDecPrivateMode(pmode) => info!("Query Mode {:?}", pmode),

            SetMode(mode) => self.set_mode(mode, true),
            ResetMode(mode) => self.set_mode(mode, false),
            XtermKeyMode { resource, value } => self.set_key_mode(resource, value),
            QueryMode(mode) => info!("Query Mode {:?}", mode),
        }
    }

    /// Switches dec private modes on or off
    /// Useful stuff like alt_screen, bracketed_paste etc
    pub fn set_dec_private_mode(&mut self, mode: u16, active: bool) {
        // https://docs.rs/termwiz/latest/termwiz/escape/csi/enum.DecPrivateModeCode.html
        match mode {
            2004 => self.bracketed_paste = active,
            47 => self.alt_screen = active,
            1049 => self.alt_screen = active,
            25 => self.show_cursor = active,
            _ => {
                self.dec_modes.insert(mode, active);
            }
        }
    }

    pub fn save_dec_private_mode(&mut self, mode: u16) {
        self.dec_saves
            .insert(mode, self.dec_mode(mode));
    }

    pub fn restore_dec_private_mode(&mut self, mode: u16) {
        self.dec_modes
            .insert(mode, self.dec_save(mode));
    }

    /// Handles Terminal Modes
    pub fn set_mode(&mut self, mode: TerminalMode, active: bool) {
        info!("Set Mode {mode:?} {active}");
    }

    /// Handles XtermKeyModes
    pub fn set_key_mode(&mut self, mode: XtermKeyModifierResource, value: Option<i64>) {
        info!("Set Key Mode {mode:?}, {value:?}");
    }

    pub fn handle_kitty_keyboard(&mut self, command: Keyboard) {
        info!("Kitty Keyboard Mode set {command:?}");
        match command {
            Keyboard::SetKittyState { flags, mode } => match mode {
                KittyKeyboardMode::AssignAll => self.kitty_state = flags,
                KittyKeyboardMode::SetSpecified => self.kitty_state |= flags, // bitwise or over the bits to set
                KittyKeyboardMode::ClearSpecified => self.kitty_state &= !flags, //bitwise and over a mask of the bits to keep
            },
            Keyboard::PushKittyState { flags, mode } => self.kitty_state = flags,
            Keyboard::PopKittyState(state) => self.kitty_state = 0,
            Keyboard::QueryKittySupport => {} // TODO write CSI ? 0 u (increase as support),
            Keyboard::ReportKittyState(state) => info!("Pseudoterminal reported kitty state {state:?}"),
        }
    }
}
