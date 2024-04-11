use std::collections::HashMap;

use termwiz::escape::csi::{DecPrivateMode, DecPrivateModeCode};

// TODO: bitfield? may not be nessecary
#[derive(Debug, Default)]
pub struct TerminalState {
    pub cwd: String,

    pub alt_screen: bool,
    pub bracketed_paste: bool,
    pub show_cursor: bool,
    codes: HashMap<u8, bool>,
}

impl TerminalState {
    pub fn new() -> TerminalState { TerminalState { ..Default::default() } }

    pub fn check(&self, code: DecPrivateModeCode) -> bool { 
        *self.codes.get(&(code as u8)).unwrap_or(&false) 
    }

    /// Switches dec private modes on or off
    /// Useful stuff like alt_screen, bracketed_paste etc
    pub fn set_dec_private_mode(&mut self, mode: DecPrivateMode, active: bool) {
        let code = match mode {
            DecPrivateMode::Code(c) => c,
            DecPrivateMode::Unspecified(_) => return,
        };

        use termwiz::escape::csi::DecPrivateModeCode::*;
        match code {
            BracketedPaste => self.bracketed_paste = active,
            EnableAlternateScreen => self.alt_screen = active,
            ClearAndEnableAlternateScreen => self.alt_screen = active,
            ShowCursor => self.show_cursor = active,
            _ => { 
                self.codes.insert(code as u8, active);
            },
        }
    }
}
