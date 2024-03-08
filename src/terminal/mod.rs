pub mod cell;
mod cursor;
pub mod pty;
pub mod screen;
mod state;

use self::cell::{PromptKind, Until};
use self::{cell::Cell, cursor::TerminalCursor};
use dioxus::desktop::use_window;
use notify_rust::Notification;
use screen::TerminalRenderer;
use state::TerminalState;
use termwiz::escape::csi::DecPrivateMode;

use termwiz::escape::osc::{FinalTermPromptKind, FinalTermSemanticPrompt};
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
            Position { line, col } => self
                .cursor
                .set(col.as_one_based() - 1, line.as_one_based() - 1),
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
    pub fn print(&mut self, char: char) {
        let attr = self.renderer.attr.clone();

        // shells don't automatically do wrapping for applications
        // weird as hell
        if self.cursor.x >= self.cols.into() {
            self.cursor.x = 0;
            self.cursor.y += 1;
        }

        self.renderer.mut_screen(self.state.alt_screen).push(
            Cell::new(char.to_string(), attr),
            self.cursor.x,
            self.cursor.y,
        );

        self.cursor.x += 1;
    }

    // I don't believe this ever happens
    pub fn print_str(&mut self, text: String) {
        let attr = self.renderer.attr.clone();
        let len = text.len();

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

        self.cursor.x += len;
    }

    pub fn handle_action(&mut self, action: Action) {
        //println!("{:?}, {:?}", action, self.cursor);
        match action {
            Action::Print(s) => self.print(s),
            Action::PrintString(s) => self.print_str(s),

            Action::Control(control) => match control {
                ControlCode::LineFeed => self.new_line(),
                ControlCode::CarriageReturn => self.cursor.set_x(0),
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
                OperatingSystemCommand::FinalTermSemanticPrompt(ftsprompt) => {
                    self.handle_fts_prompt(ftsprompt)
                }
                //OperatingSystemCommand::SystemNotification(notif) => Self::notify_window(notif),
                _ => println!("OperatingSystemCommand({:?})", command),
            },
            _ => println!("{:?}", action),
        }
    }

    // TODO: Replace this shitty thing with a more explicit system
    // Ideally there would be a Command object, that has prompt, input and output fields
    pub fn handle_fts_prompt(&mut self, prompt: FinalTermSemanticPrompt) {
        use FinalTermSemanticPrompt::*;
        match prompt {
            FreshLine => self.fresh_line(),
            FreshLineAndStartPrompt { aid, cl } => {
                self.fresh_line();
                self.renderer.attr.prompt_kind = PromptKind::Prompt(FinalTermPromptKind::Initial);

                if let Some(a) = aid { println!("AID {a}"); }
                if let Some(c) = cl { println!("FINALCLICK {c}"); }
            }
            MarkEndOfCommandWithFreshLine { aid, cl } => {
                self.fresh_line();
                self.renderer.attr.prompt_kind = PromptKind::Prompt(FinalTermPromptKind::Initial);

                if let Some(a) = aid { println!("AID {a}"); }
                if let Some(c) = cl { println!("FINALCLICK {c}"); }
            }
            StartPrompt(prompt_kind) => self.renderer.attr.prompt_kind = PromptKind::Prompt(prompt_kind),
            MarkEndOfPromptAndStartOfInputUntilNextMarker => {
                self.start_input(Until::SemanticMarker)
            }
            MarkEndOfPromptAndStartOfInputUntilEndOfLine => self.start_input(Until::LineEnd),
            MarkEndOfInputAndStartOfOutput { aid } => {
                self.renderer.attr.prompt_kind = PromptKind::Output;

                if let Some(a) = aid { println!("AID {a}"); }
            }
            CommandStatus { status, aid } => println!("COMMAND ENDED {status}, aid {aid:?}"),
        }
    }

    pub fn fresh_line(&mut self) {
        if self.cursor.x == 0 {
            return;
        }
        self.cursor.set_x(0);
        self.cursor.y += 1;
    }

    pub fn start_input(&mut self, until: Until) {
        self.renderer.attr.prompt_kind = PromptKind::Input(until)
        // TODO: Do some state management. maybe some sort of custom editor?
    }

    pub fn notify_window(notif: String) {
        Notification::new()
            .summary("Term")
            .body(&notif)
            .icon("firefox")
            .show()
            .unwrap();
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
                    for i in self.cursor.x..screen.cells[self.cursor.y].len() {
                        screen.cells[self.cursor.y][i] = Cell::default();
                    }
                }
            }
            EraseInLine::EraseToStartOfLine => {
                // may go out of bounds. idk
                for i in 0..self.cursor.x {
                    screen.cells[self.cursor.y][i] = Cell::default();
                }
            }
            EraseInLine::EraseLine => {
                screen.cells[self.cursor.y] = Vec::new();
                //self.cursor.set_x(0);
            }
        }
    }

    pub fn erase_characters(&mut self, n: u32) {
        let screen = self.renderer.mut_screen(self.state.alt_screen);
        let end = (self.cursor.x + n as usize).min(screen.cells[self.cursor.y].len() - 1);

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
