use portable_pty::PtySize;
use winit::dpi::PhysicalSize;

mod pty;
pub mod screen;
use pty::PseudoTerminal;
use screen::{AltScreen, Screen, TerminalRenderer};

use self::screen::{Cell, CellAttributes};

// windows build hangs if these fields aren't stored
pub struct Terminal {
    pub rows: u16,
    pub cols: u16,

    pub renderer: TerminalRenderer,
    pub state: TerminalState,
    pub pty: PseudoTerminal,
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
            self.renderer.alt_screen.screen.concat()
        } else {
            self.renderer.screen.cells.clone()
        }
    }

    pub fn print(&mut self, text: char) {
        if self.state.alt_screen {
            // Draw to alt screen
        } else {
            self.renderer.screen.push(Cell::new(text, self.renderer.attr.clone()))
        }
    }

    pub fn print_str(&mut self, text: String) {
        println!("String {}", text);
    }

    pub fn setup() -> anyhow::Result<Terminal> {
        Ok(Terminal {
            pty: PseudoTerminal::setup()?,
            rows: 0,
            cols: 0,
            renderer: TerminalRenderer::new((0, 0)),
            state: TerminalState::new(),
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
