use std::sync::mpsc::Receiver;

use joinery::JoinableIterator;
use portable_pty::PtySize;
use winit::dpi::PhysicalSize;

mod cursor;
mod pty;
mod state;
pub mod screen;

use pty::PseudoTerminal;
use screen::TerminalRenderer;
use state::TerminalState;

use termwiz::escape::csi::{
    DecPrivateMode, Edit, EraseInLine,Cursor,
    Mode::{ResetDecPrivateMode, SetDecPrivateMode},
};
use termwiz::escape::{Action, ControlCode, OperatingSystemCommand, CSI};
use self::{
    cursor::TerminalCursor,
    screen::{Cell, CellAttributes},
};

use dioxus::prelude::Coroutine;

/// Main terminal controller
/// Holds a lot of sub-objects
pub struct Terminal {
    pub rows: u16,
    pub cols: u16,

    pub renderer: TerminalRenderer,
    pub state: TerminalState,
    pub pty: PseudoTerminal,
    pub cursor: TerminalCursor,

    pub title: String,
}

impl Terminal {
    // Resizes how big the terminal thinks it is
    // mostly useful for rendering tui applications
    pub fn resize(&mut self, size: PhysicalSize<u32>, glyph_size: (f32, f32)) {
        let screen_width = size.width.max(1);
        let screen_height = size.height.max(1);

        self.rows = (screen_height as f32 / glyph_size.1) as u16;
        self.cols = (screen_width as f32 / glyph_size.0) as u16;

        //println!("{}, {}, {:?}", self.rows, self.cols, glyph_size);

        self.pty
            .pair
            .master
            .resize(PtySize {
                rows: self.rows,
                cols: self.cols,
                pixel_width: glyph_size.0.round() as u16,
                pixel_height: glyph_size.1.round() as u16,
            })
            .unwrap();
    }

    /// Gets all cells the renderer should be showing
    pub fn get_cells(&self) -> &Vec<Vec<Cell>> {
        &self.renderer.get_screen(self.state.alt_screen).cells
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

        self.renderer.mut_screen(self.state.alt_screen).cells[self.cursor.y].remove(self.cursor.x);
    }

    pub fn carriage_return(&mut self) {
        self.cursor.set_x(0)
    }

    pub fn new_line(&mut self) {
        self.cursor.shift_down(1);
        self.cursor.set_x(0)
    }

    pub fn erase_in_line(&mut self, edit: EraseInLine) {
        let screen = self.renderer.mut_screen(self.state.alt_screen);

        match edit {
            EraseInLine::EraseToEndOfLine => {
                if screen.cells.len() > self.cursor.y {
                    screen.cells[self.cursor.y].drain(self.cursor.x..);
                }
            }
            EraseInLine::EraseToStartOfLine => {
                screen.cells[self.cursor.y].truncate(self.cursor.x);
                self.cursor.set_x(0);
            }
            EraseInLine::EraseLine => {
                screen.cells.remove(self.cursor.y);
                self.cursor.set_x(0);
            }
        }
    }

    /// Pushes a cell onto the current screen
    pub fn print(&mut self, text: char) {
        let attr = self.renderer.attr.clone();

        self.renderer.mut_screen(self.state.alt_screen).push(
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

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Print(s) => self.print(s),
            Action::PrintString(s) => self.print_str(s),

            Action::Control(control) => match control {
                ControlCode::LineFeed => self.new_line(),
                // Don't do anything for carriage return
                // would be nice to but it breaks cursor movement
                ControlCode::CarriageReturn => self.carriage_return(),
                ControlCode::Backspace => self.backspace(),
                _ => println!("ControlCode({:?})", control),
            },

            Action::CSI(csi) => match csi {
                CSI::Sgr(sgr) => self.renderer.handle_sgr(sgr),
                CSI::Mode(mode) => match mode {
                    SetDecPrivateMode(pmode) => self.state.set_dec_private_mode(pmode, true),
                    ResetDecPrivateMode(pmode) => self.state.set_dec_private_mode(pmode, false),
                    _ => println!("Mode({:?})", mode),
                },
                CSI::Cursor(cursor) => self.handle_cursor(cursor),
                CSI::Edit(edit) => self.handle_edit(edit),
                _ => println!("CSI({:?})", csi),
            },

            Action::OperatingSystemCommand(command) => match *command {
                OperatingSystemCommand::SetIconNameAndWindowTitle(title) => self.title = title,
                _ => println!("OperatingSystemCommand({:?})", command),
            },
            _ => println!("{:?}", action),
        }
    }

    pub fn handle_edit(&mut self, edit: Edit) {
        use EraseInLine::*;
        match edit {
            //Edit::EraseInLine(EraseToEndOfLine) => {}
            Edit::EraseInLine(e) => self.erase_in_line(e),
            _ => println!("Edit {:?}", edit),
        }
    }

    pub fn setup() -> anyhow::Result<Terminal> {
        Ok(Terminal {
            pty: PseudoTerminal::setup()?,
            rows: 0,
            cols: 0,
            renderer: TerminalRenderer::new(),
            state: TerminalState::new(),
            cursor: TerminalCursor::new(),
            title: "Terminal".into(),
        })
    }
}
