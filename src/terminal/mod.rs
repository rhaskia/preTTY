use portable_pty::PtySize;
use termwiz::escape::csi::Cursor;
use winit::dpi::PhysicalSize;
use joinery::JoinableIterator;

mod cursor;
mod pty;
pub mod screen;
use pty::PseudoTerminal;
use screen::TerminalRenderer;

use self::{
    cursor::TerminalCursor,
    screen::{Cell, CellAttributes},
};

/// Main terminal controller
/// Holds a lot of sub-objects
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
        }).unwrap();
    }

    /// Gets all cells the renderer should be showing
    pub fn get_cells(&mut self) -> Vec<Cell> {
        self.renderer
            .get_screen(self.state.alt_screen)
            .cells
            .clone()
            .into_iter()
            .flatten()
            .collect()
    }

    /// Handles cursor movements, etc
    // Really need to move this to the cursor object
    pub fn handle_cursor(&mut self, cursor: Cursor) {
        use Cursor::*;
        match cursor {
            Left(amount) => self.cursor.shift_left(amount),
            Down(amount) | NextLine(amount) => self.cursor.shift_down(amount),
            Right(amount) => self.cursor.shift_right(amount),
            Up(amount) | PrecedingLine(amount) => self.cursor.shift_right(amount),
            Position { line, col } => self.cursor.set(line.as_zero_based(), col.as_zero_based()),
            CursorStyle(style) => self.cursor.set_style(style),
            _ => println!("{:?}", cursor),
        }
    }

    /// Backspaces at the terminal cursor position
    pub fn backspace(&mut self) {
        self.cursor.x -= 1;

        self.renderer.get_screen(self.state.alt_screen).cells[self.cursor.y].remove(self.cursor.x);
    }

    pub fn new_line(&mut self) {
        // self.renderer.get_screen(self.state.alt_screen).push(
        //     Cell::new('\n', CellAttributes::default()),
        //     self.cursor.x,
        //     self.cursor.y,
        // );

        self.cursor.shift_down(1);
        self.cursor.set_x(0)
    }

    /// Pushes a cell onto the current screen
    pub fn print(&mut self, text: char) {
        let attr = self.renderer.attr.clone();

        self.renderer.get_screen(self.state.alt_screen).push(
            Cell::new(text, attr),
            self.cursor.x,
            self.cursor.y,
        );

        self.cursor.x += 1;
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
