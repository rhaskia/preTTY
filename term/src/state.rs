use std::collections::HashMap;

use log::info;
use termwiz::escape::csi::{DecPrivateMode, DecPrivateModeCode, TerminalMode, XtermKeyModifierResource};
use termwiz::escape::csi::Mode;

// TODO: bitfield? may not be nessecary
#[derive(Debug, Default)]
pub struct TerminalState {
    pub cwd: String,
    // dec private
    dec_modes: HashMap<u8, bool>,
    dec_saves: HashMap<u8, bool>,
    // terminal mode
    modes: HashMap<u8, bool>,

    pub alt_screen: bool,
    pub bracketed_paste: bool,
    pub show_cursor: bool,
}

macro_rules! inner_mode {
    ($mode: ident) => {
        match $mode {
            DecPrivateMode::Code(c) => c,
            DecPrivateMode::Unspecified(_) => return,
        };
    };
}

impl TerminalState {
    pub fn new() -> TerminalState {
        TerminalState {
            ..Default::default()
        }
    }

    pub fn dec_mode(&self, code: DecPrivateModeCode) -> bool {
        *self.dec_modes.get(&(code as u8)).unwrap_or(&false)
    }

    pub fn dec_save(&self, code: DecPrivateModeCode) -> bool {
        *self.dec_saves.get(&(code as u8)).unwrap_or(&false)
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
                self.dec_modes.insert(code as u8, active);
            }
        }
    }

    pub fn save_dec_private_mode(&mut self, mode: DecPrivateMode) {
        info!("Save Dec Mode {mode:?}");
        let code = inner_mode!(mode);
        self.dec_saves.insert(code.clone() as u8, self.dec_mode(code));
    }

    pub fn restore_dec_private_mode(&mut self, mode: DecPrivateMode) {
        info!("Restore Dec Mode {mode:?}");
        let code = inner_mode!(mode);
        self.dec_modes.insert(code.clone() as u8, self.dec_save(code));
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
