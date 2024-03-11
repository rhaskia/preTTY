pub mod cell;
pub mod command;
mod cursor;
pub mod pty;
pub mod screen;
mod state;

use std::collections::VecDeque;

use cell::{Cell, PromptKind, SemanticType, Until};
use cursor::TerminalCursor;
use screen::Screen;
use std::any::Any;

use notify_rust::Notification;
use screen::TerminalRenderer;
use state::TerminalState;

use termwiz::escape::osc::FinalTermSemanticPrompt;
use termwiz::escape::{
    csi::{
        Cursor, Edit, EraseInDisplay, EraseInLine,
        Mode::{ResetDecPrivateMode, SetDecPrivateMode},
        CSI,
    },
    Action, ControlCode, OperatingSystemCommand,
};

use self::command::CommandSlicer;

/// Main terminal controller
/// Holds a lot of sub-objects
pub struct Terminal {
    pub rows: u16,
    pub cols: u16,

    pub renderer: TerminalRenderer,
    pub state: TerminalState,
    pub cursor: TerminalCursor,
    pub commands: CommandSlicer,

    pub title: String,
}

impl Terminal {
    pub fn setup() -> anyhow::Result<Terminal> {
        Ok(Terminal {
            rows: 24,
            cols: 80,
            renderer: TerminalRenderer::new(24, 80),
            state: TerminalState::new(),
            cursor: TerminalCursor::new(),
            title: "Terminal".into(),
            commands: CommandSlicer::new(),
        })
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
            _ => println!("Cursor {cursor:?}"),
        }
    }

    /// Backspaces at the terminal cursor position
    pub fn backspace(&mut self) {
        self.cursor.x -= 1;
        //self.renderer.mut_screen(self.state.alt_screen).cells[self.cursor.y][self.cursor.x] = Cell::default();
        //self.renderer.mut_screen(self.state.alt_screen).cells[self.cursor.y].remove(self.cursor.x);
    }

    pub fn new_line(&mut self) {
        self.cursor.shift_down(1);
        if self.cursor.y == self.rows as usize {
            self.cursor.y = self.rows as usize - 1;
            self.mut_screen().scrollback();
        }
    }

    /// Pushes a cell onto the current screen
    pub fn print(&mut self, char: char) {
        let attr = self.renderer.attr.clone();

        //shells don't automatically do wrapping for applications
        //weird as hell
        if self.cursor.x >= self.cols.into() {
            self.cursor.shift_down(1);
            self.cursor.set_x(0);
        }

        self.renderer.mut_screen(self.state.alt_screen).push(
            Cell::new(char.to_string(), attr),
            self.cursor.x,
            self.cursor.y,
        );

        self.cursor.x += 1;
    }

    pub fn print_str(&mut self, text: String) {
        for char in text.chars() {
            self.print(char);
        }
    }

    pub fn handle_actions(&mut self, actions: Vec<Action>) {
        for action in actions { self.handle_action(action); }
    }

    pub fn handle_action(&mut self, action: Action) {
        println!("{action:?}, {}", self.cursor.y);
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
                self.renderer.attr.semantic_type = SemanticType::Prompt(PromptKind::Initial);
                self.commands.start_new(self.cursor.x, self.cursor.y);

                if let Some(a) = aid {
                    println!("AID {a}");
                }
                if let Some(c) = cl {
                    println!("FINALCLICK {c}");
                }
            }
            MarkEndOfCommandWithFreshLine { aid, cl } => {
                self.fresh_line();
                self.renderer.attr.semantic_type = SemanticType::Prompt(PromptKind::Initial);
                self.commands.start_new(self.cursor.x, self.cursor.y);

                if let Some(a) = aid {
                    println!("AID {a}");
                }
                if let Some(c) = cl {
                    println!("FINALCLICK {c}");
                }
            }
            StartPrompt(prompt_kind) => {
                self.renderer.attr.semantic_type =
                    SemanticType::Prompt(PromptKind::from(prompt_kind))
            }
            MarkEndOfPromptAndStartOfInputUntilNextMarker => self.start_input(Until::SemanticMarker),
            MarkEndOfPromptAndStartOfInputUntilEndOfLine => self.start_input(Until::LineEnd),
            MarkEndOfInputAndStartOfOutput { aid } => {
                self.renderer.attr.semantic_type = SemanticType::Output;
                self.commands.start_output(self.cursor.x, self.cursor.y);

                if let Some(a) = aid {
                    println!("AID {a}");
                }
            }
            CommandStatus { status, aid } => println!("COMMAND ENDED {status}, aid {aid:?}"),
        }
    }

    pub fn fresh_line(&mut self) {
        if self.cursor.x == 0 {
            return;
        }
        self.new_line();
        self.cursor.x = 0;
    }

    pub fn start_input(&mut self, until: Until) {
        self.renderer.attr.semantic_type = SemanticType::Input(until);
        self.commands.start_input(self.cursor.x, self.cursor.y);
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
            EraseInDisplay::EraseDisplay => {
                screen.erase_all();
                self.cursor.set(0, 0);
            }
            _ => println!("Erase {edit:?}"),
        }
    }

    pub fn erase_in_line(&mut self, edit: EraseInLine) {
        let screen = self.renderer.mut_screen(self.state.alt_screen);

        match edit {
            EraseInLine::EraseToEndOfLine => {
                if screen.visible_len() > self.cursor.y {
                    for x in self.cursor.x..screen.line(self.cursor.y).len() {
                        screen.push(Cell::default(), x, self.cursor.y);
                    }
                }
            }
            EraseInLine::EraseToStartOfLine => {
                // may go out of bounds. idk
                for x in 0..self.cursor.x {
                    screen.push(Cell::default(), x, self.cursor.y);
                }
            }
            EraseInLine::EraseLine => {
                screen.set_line(self.cursor.y, Vec::new());
                //self.cursor.set_x(0);
            }
        }
    }

    pub fn erase_characters(&mut self, n: u32) {
        let screen = self.renderer.mut_screen(self.state.alt_screen);
        let end = (self.cursor.x + n as usize).min(screen.mut_line(self.cursor.y).len());

        for x in self.cursor.x..end {
            screen.mut_line(self.cursor.y)[x] = Cell::default();
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

    pub fn screen(&self) -> &Screen {
        self.renderer.get_screen(self.state.alt_screen)
    }

    pub fn mut_screen(&mut self) -> &mut Screen {
        self.renderer.mut_screen(self.state.alt_screen)
    }
}
