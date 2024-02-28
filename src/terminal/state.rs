use termwiz::escape::csi::DecPrivateMode;

#[derive(Debug)]
pub struct TerminalState {
    pub alt_screen: bool,
    pub bracketed_paste: bool,
}

impl TerminalState {
    pub fn new() -> TerminalState {
        TerminalState {
            alt_screen: false,
            bracketed_paste: false,
        }
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
            _ => println!("Code {:?}, set to {}", code, active),
        }
    }
}
