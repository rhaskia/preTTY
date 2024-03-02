pub mod cell;
mod cursor;
pub mod pty;
pub mod screen;
mod state;

use screen::TerminalRenderer;
use state::TerminalState;
use termwiz::escape::csi::DecPrivateMode;
use self::{cell::Cell, cursor::TerminalCursor};

use termwiz::escape::{
    csi::{
        Cursor, Edit, EraseInDisplay, EraseInLine,
        Mode::{ResetDecPrivateMode, SetDecPrivateMode},
        CSI,
    },
    Action, ControlCode, OperatingSystemCommand,
};

/// Main terminal controller
/// Holds a lot of sub-objects
pub struct Terminal {
    pub rows: u16,
    pub cols: u16,

    pub renderer: TerminalRenderer,
    pub state: TerminalState,
    pub cursor: TerminalCursor,

    pub title: String,
}

impl Terminal {
    pub fn setup() -> anyhow::Result<Terminal> {
        Ok(Terminal {
            rows: 24,
            cols: 80,
            renderer: TerminalRenderer::new(),
            state: TerminalState::new(),
            cursor: TerminalCursor::new(),
            title: "Terminal".into(),
        })
    }

    // pub fn handle_actions(&mut self, actions: &mut dioxus::prelude::Write<Vec<Action>, SyncStorage>) {
    //     while let Some(action) = actions.pop() {
    //         self.handle_action(action);
    //     }
    // }

    // pub fn write_str(&mut self, s: String) {
    //     self.pty.writer.write_all(s.as_bytes());
    // }

    // Resizes how big the terminal thinks it is
    // mostly useful for rendering tui applications
    // pub fn resize(&mut self, md: &MountedData) {
    //     println!("w");
    //     println!("{:?}", md.get_raw_element().unwrap().downcast_ref::<web_sys::Element>());
    //
    //     // let screen_width = size.width.max(1);
    //     // let screen_height = size.height.max(1);
    //     //
    //     // self.rows = (screen_height as f32 / glyph_size.1) as u16;
    //     // self.cols = (screen_width as f32 / glyph_size.0) as u16;
    //     //
    //     // //println!("{}, {}, {:?}", self.rows, self.cols, glyph_size);
    //     //
    //     // self.pty
    //     //     .pair
    //     //     .master
    //     //     .resize(PtySize {
    //     //         rows: self.rows,
    //     //         cols: self.cols,
    //     //         pixel_width: glyph_size.0.round() as u16,
    //     //         pixel_height: glyph_size.1.round() as u16,
    //     //     })
    //     //     .unwrap();
    // }

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
            Position { line, col } => self.cursor.set(col.as_one_based() - 1, line.as_one_based() - 1),
            CursorStyle(style) => self.cursor.set_style(style),
            _ => println!("{:?}", cursor),
        }
    }

    /// Backspaces at the terminal cursor position
    pub fn backspace(&mut self) {
        self.cursor.x -= 1;
        self.renderer.mut_screen(self.state.alt_screen).cells[self.cursor.y].remove(self.cursor.x);
    }

    pub fn new_line(&mut self) {
        self.cursor.shift_down(1);
        self.cursor.set_x(0)
    }

    /// Pushes a cell onto the current screen
    pub fn print(&mut self, text: char) {
        let attr = self.renderer.attr.clone();

        // shells don't automatically do wrapping for applications
        // weird as hell
        if self.cursor.x >= self.cols.into() {
            self.cursor.x = 0;
            self.cursor.y += 1;
        }

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
        //println!("{:?}, {:?}", action, self.cursor);
        match action {
            Action::Print(s) => self.print(s),
            Action::PrintString(s) => self.print_str(s),

            Action::Control(control) => match control {
                ControlCode::LineFeed => self.new_line(),
                // Don't do anything for carriage return
                // would be nice to but it breaks cursor movement
                ControlCode::CarriageReturn => self.cursor.set_x(0),
                ControlCode::Backspace => self.backspace(),
                _ => println!("ControlCode({:?})", control),
            },

            Action::CSI(csi) => match csi {
                CSI::Sgr(sgr) => self.renderer.handle_sgr(sgr),
                CSI::Mode(mode) => match mode {
                    SetDecPrivateMode(pmode) => self.set_dec_private_mode(pmode, true),
                    ResetDecPrivateMode(pmode) => self.set_dec_private_mode(pmode, false),
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

    pub fn set_dec_private_mode(&mut self, pmode: DecPrivateMode, active: bool) {
        self.state.save_dec_private_mode(pmode, active);

        // let code = match mode {
        //     DecPrivateMode::Code(c) => c,
        //     DecPrivateMode::Unspecified(_) => return,
        // };

        // use termwiz::escape::csi::DecPrivateModeCode::*;
        // match code {
        //     EnableAlternateScreen | ClearAndEnableAlternateScreen => {
        //         cursor.save_alt(active);
        //         cursor.set(0, 0);
        //     },
        //     _ => println!("Code {:?}, set to {}", code, active),
        // }
    }

    pub fn erase_in_display(&mut self, edit: EraseInDisplay) {
        let screen = self.renderer.mut_screen(self.state.alt_screen);

        match edit {
            EraseInDisplay::EraseToEndOfDisplay => {}
            EraseInDisplay::EraseToStartOfDisplay => {}
            EraseInDisplay::EraseDisplay => {
                screen.cells = Vec::new();
                self.cursor.set(0, 0);
            }
            EraseInDisplay::EraseScrollback => {}
        }
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

    pub fn erase_characters(&mut self, n: u32) {
        let screen = self.renderer.mut_screen(self.state.alt_screen);
        let end = (self.cursor.x + n as usize)
                  .min(screen.cells[self.cursor.y].len() - 1);

        for x in self.cursor.x..end {
            screen.cells[self.cursor.y][x] = Cell::default();
        }
    }

    pub fn handle_edit(&mut self, edit: Edit) {
        match edit {
            Edit::EraseInLine(e) => self.erase_in_line(e),
            Edit::EraseInDisplay(e) => self.erase_in_display(e),
            Edit::EraseCharacter(n) => self.erase_characters(n),
            _ => println!("Edit {:?}", edit),
        }
    }
}
