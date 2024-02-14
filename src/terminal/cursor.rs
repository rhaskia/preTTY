use termwiz::escape::csi::CursorStyle;

/// Cursor object to store cursor position and style
/// Allows for storing, restoring etc with positions as well
pub struct TerminalCursor {
    pub x: usize,
    pub y: usize,


    pub alt_x: usize,
    pub alt_y: usize,

    pub saved_x: usize,
    pub saved_y: usize,

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
            alt_x: 0,
            alt_y: 0,
        }
    }

    /// Sets the cursor position
    pub fn set(&mut self, x: u32, y: u32) {
        self.x = x as usize;
        self.y = y as usize;
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x as usize
    }

    pub fn set_y(&mut self, y: u32) {
        self.y = y as usize
    }

    /// Shifts the cursor down
    pub fn shift_down(&mut self, amount: u32) {
        self.y += amount as usize;
    }

    /// Shifts the cursor right
    pub fn shift_right(&mut self, amount: u32) {
        self.x += amount as usize
    }

    /// Shifts the cursor up
    pub fn shift_up(&mut self, amount: u32) {
        self.y = self.y.checked_sub(amount as usize).unwrap();
    }

    /// Shifts the cursor left
    pub fn shift_left(&mut self, amount: u32) {
        self.x = self.x.checked_sub(amount as usize).unwrap();
    }

    pub fn set_style(&mut self, style: CursorStyle) {
        self.style = style;
    }
}
