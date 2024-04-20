use std::collections::HashMap;

use log::info;
use num_traits::cast::ToPrimitive;
use termwiz::escape::csi::{
    DecPrivateMode, DecPrivateModeCode, Mode, TerminalMode, XtermKeyModifierResource,
};
use termwiz::escape::DeviceControlMode;

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
}

macro_rules! inner_mode {
    ($mode: ident) => {
        match $mode {
            DecPrivateMode::Code(c) => c,
            DecPrivateMode::Unspecified(_) => return,
        }
    };
}

impl TerminalState {
    pub fn new() -> TerminalState {
        TerminalState {
            ..Default::default()
        }
    }

    pub fn dec_mode(&self, code: DecPrivateModeCode) -> bool {
        *self
            .dec_modes
            .get(&(code.to_u16().unwrap()))
            .unwrap_or(&false)
    }

    pub fn dec_save(&self, code: DecPrivateModeCode) -> bool {
        *self
            .dec_saves
            .get(&(code.to_u16().unwrap()))
            .unwrap_or(&false)
    }

    pub fn device_control(&mut self, device_command: DeviceControlMode) {
        match device_command {
            DeviceControlMode::Enter(mode) => todo!(),
            DeviceControlMode::Exit => todo!(),
            DeviceControlMode::Data(_) => todo!(),
            DeviceControlMode::ShortDeviceControl(_) => todo!(),
            DeviceControlMode::TmuxEvents(_) => todo!(),
            // _ => info!("Device Command {:?}", device_command),
        }
    }

    pub fn handle_state(&mut self, mode: Mode) {
        use termwiz::escape::csi::Mode::*;
        match mode {
            SetDecPrivateMode(pmode) => self.set_dec_private_mode(pmode, true),
            ResetDecPrivateMode(pmode) => self.set_dec_private_mode(pmode, false),
            SaveDecPrivateMode(pmode) => self.save_dec_private_mode(pmode),
            RestoreDecPrivateMode(pmode) => self.restore_dec_private_mode(pmode),
            QueryDecPrivateMode(pmode) => todo!(),

            SetMode(mode) => self.set_mode(mode, true),
            ResetMode(mode) => self.set_mode(mode, false),
            XtermKeyMode { resource, value } => self.set_key_mode(resource, value),
            QueryMode(mode) => todo!(),
        }
    }

    /// Switches dec private modes on or off
    /// Useful stuff like alt_screen, bracketed_paste etc
    pub fn set_dec_private_mode(&mut self, mode: DecPrivateMode, active: bool) {
        info!("Set Dec Mode {mode:?} {active}");
        let code = inner_mode!(mode);

        use termwiz::escape::csi::DecPrivateModeCode::*;
        match code {
            BracketedPaste => self.bracketed_paste = active,
            EnableAlternateScreen => self.alt_screen = active,
            ClearAndEnableAlternateScreen => self.alt_screen = active,
            ShowCursor => self.show_cursor = active,
            _ => {
                self.dec_modes.insert(code.to_u16().unwrap(), active);
            }
        }
    }

    pub fn save_dec_private_mode(&mut self, mode: DecPrivateMode) {
        info!("Save Dec Mode {mode:?}");
        let code = inner_mode!(mode);
        self.dec_saves
            .insert(code.to_u16().unwrap(), self.dec_mode(code));
    }

    pub fn restore_dec_private_mode(&mut self, mode: DecPrivateMode) {
        info!("Restore Dec Mode {mode:?}");
        let code = inner_mode!(mode);
        self.dec_modes
            .insert(code.to_u16().unwrap(), self.dec_save(code));
    }

    /// Handles Terminal Modes
    pub fn set_mode(&mut self, mode: TerminalMode, active: bool) {
        info!("Set Mode {mode:?} {active}");
    }

    /// Handles XtermKeyModes
    pub fn set_key_mode(&mut self, mode: XtermKeyModifierResource, value: Option<i64>) {
        info!("Set Key Mode {mode:?}, {value:?}");
    }
}
