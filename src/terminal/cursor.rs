use termwiz::escape::csi::CursorStyle;

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

    pub fn set(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    pub fn shift_down(&mut self, amount: u32) {
        self.y += amount;
    }

    pub fn shift_right(&mut self, amount: u32) {
        self.x += amount
    }

    pub fn shift_up(&mut self, amount: u32) {
        self.y = self.y.checked_sub(amount).unwrap();
    }

    pub fn shift_left(&mut self, amount: u32) {
        self.x = self.x.checked_sub(amount).unwrap();
    }

    pub fn set_style(&mut self, style: CursorStyle) {
        self.style = style;
    }
}
