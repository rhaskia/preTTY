use termwiz::escape::csi::CursorStyle;

/// Cursor object to store cursor position and style
/// Allows for storing, restoring etc with positions as well
pub struct TerminalCursor {
    pub x: u32,
    pub y: u32,

    pub saved_x: u32,
    pub saved_y: u32,

    pub style: CursorStyle,
}

impl TerminalCursor {
    pub fn new() -> TerminalCursor {
        TerminalCursor {
            x: 0,
            y: 0,
            saved_x: 0,
            saved_y: 0,
            style: CursorStyle::Default,
        }
    }

    /// Sets the cursor position
    pub fn set(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x
    }

    pub fn set_y(&mut self, y: u32) {
        self.y = y
    }

    /// Shifts the cursor down
    pub fn shift_down(&mut self, amount: u32) {
        self.y += amount;
    }

    /// Shifts the cursor right
    pub fn shift_right(&mut self, amount: u32) {
        self.x += amount
    }

    /// Shifts the cursor up
    pub fn shift_up(&mut self, amount: u32) {
        self.y = self.y.checked_sub(amount).unwrap();
    }

    /// Shifts the cursor left
    pub fn shift_left(&mut self, amount: u32) {
        self.x = self.x.checked_sub(amount).unwrap();
    }

    pub fn set_style(&mut self, style: CursorStyle) {
        self.style = style;
    }
}
