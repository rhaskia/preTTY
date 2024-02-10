use portable_pty::PtySize;
use termwiz::escape::csi::Cursor;
use winit::dpi::PhysicalSize;

mod pty;
mod cursor;
pub mod screen;
use pty::PseudoTerminal;
use screen::{Screen, TerminalRenderer};

use self::{cursor::TerminalCursor, screen::{Cell, CellAttributes}};

// windows build hangs if these fields aren't stored
pub struct Terminal {
    pub rows: u16,
    pub cols: u16,

    pub renderer: TerminalRenderer,
    pub state: TerminalState,
    pub pty: PseudoTerminal,
    pub cursor: TerminalCursor,
}

impl Terminal {
    // Resizes how big the terminal thinks it is
    // mostly useful for rendering tui applications
    pub fn resize(&mut self, size: PhysicalSize<u32>, glyph_size: (f32, f32)) {
        let screen_width = size.width.max(1);
        let screen_height = size.height.max(1);

        self.rows = (screen_height as f32 / glyph_size.1) as u16;
        self.cols = (screen_width as f32 / glyph_size.0) as u16 + 14;

        println!("{}, {}, {:?}", self.rows, self.cols, glyph_size);

        self.pty.pair.master.resize(PtySize {
            rows: self.rows,
            cols: self.cols,
            pixel_width: glyph_size.0.round() as u16,
            pixel_height: glyph_size.1.round() as u16,
        });
    }

    pub fn get_cells(&self) -> Vec<Cell> {
        if self.state.alt_screen {
            self.renderer.alt_screen.cells.clone()
        } else {
            self.renderer.screen.cells.clone()
        }
    }

    pub fn handle_cursor(&mut self, cursor: Cursor) {
        use Cursor::*;
        match cursor {
            Left(amount) => self.cursor.shift_left(amount),
            Down(amount) | NextLine(amount) => self.cursor.shift_down(amount),
            Right(amount) => self.cursor.shift_right(amount),
            Up(amount) | PrecedingLine(amount) => self.cursor.shift_right(amount),
            Position { line, col } => self.cursor.set(line.as_zero_based(), col.as_zero_based()),
            CursorStyle(style) => self.cursor.set_style(style),
            _ => println!("{:?}", cursor)
        }
    }

    pub fn backspace(&mut self) {
        self.renderer.get_screen(self.state.alt_screen).cells.pop();
        // TODO pop at cursor position
    }

    pub fn print(&mut self, text: char) {
        let attr = self.renderer.attr.clone();

        self.renderer
            .get_screen(self.state.alt_screen)
            .push(Cell::new(text, attr))
    }

    // I don't believe this ever happens
    pub fn print_str(&mut self, text: String) {
        println!("String {}", text);
    }

    pub fn setup() -> anyhow::Result<Terminal> {
        Ok(Terminal {
            pty: PseudoTerminal::setup()?,
            rows: 0,
            cols: 0,
            renderer: TerminalRenderer::new(),
            state: TerminalState::new(),
            cursor: TerminalCursor::new(),
        })
    }
}

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
}
