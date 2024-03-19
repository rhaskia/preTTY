pub mod cell;
pub mod command;
mod cursor;
pub mod pty;
pub mod screen;
mod state;

use std::any::Any;
use std::collections::{HashMap, VecDeque};

use cell::{Cell, PromptKind, SemanticType, Until};
use cursor::TerminalCursor;
use notify_rust::Notification;
use screen::{Screen, TerminalRenderer};
use state::TerminalState;
use termwiz::escape::csi::Mode::{ResetDecPrivateMode, SetDecPrivateMode};
use termwiz::escape::csi::{Cursor, Device, Edit, EraseInDisplay, EraseInLine, Keyboard, CSI};
use termwiz::escape::osc::{FinalTermSemanticPrompt, ITermProprietary};
use termwiz::escape::{Action, ControlCode, OperatingSystemCommand};

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
    pub user_vars: HashMap<String, String>,

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
            user_vars: HashMap::new(),
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
            Position { line, col } => self.cursor.set(col.as_one_based() - 1, line.as_one_based() - 1),
            CursorStyle(style) => self.cursor.set_style(style),
            _ => println!("Cursor {cursor:?}"),
        }
    }

    /// Backspaces at the terminal cursor position
    pub fn backspace(&mut self) {
        self.cursor.x -= 1;
        // self.renderer.mut_screen(self.state.alt_screen).cells[self.cursor.y][self.cursor.x] = Cell::default();
        // self.renderer.mut_screen(self.state.alt_screen).cells[self.cursor.y].remove(self.cursor.x);
    }

    pub fn new_line(&mut self) {
        self.cursor.shift_down(1);
        if self.cursor.y == self.rows as usize {
            self.cursor.y = self.rows as usize - 1;
            self.mut_screen().scrollback();
        }
    }

    pub fn phys_cursor_y(&self) -> usize { self.screen().phys_line(self.cursor.y) }

    /// Pushes a cell onto the current screen
    pub fn print(&mut self, char: char) {
        let attr = self.renderer.attr.clone();

        // shells don't automatically do wrapping for applications
        // weird as hell
        if self.cursor.x >= self.cols.into() {
            self.cursor.shift_down(1);
            self.cursor.set_x(0);
        }

        self.renderer.mut_screen(self.state.alt_screen).push(Cell::new(char, attr), self.cursor.x, self.cursor.y);

        self.cursor.x += 1;
    }

    pub fn print_str(&mut self, text: String) {
        for char in text.chars() {
            self.print(char);
        }
    }

    pub fn handle_actions(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.handle_action(action);
        }
    }

    pub fn handle_action(&mut self, action: Action) {
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
                CSI::Device(device) => self.handle_device(device),
                CSI::Keyboard(keyboard) => self.handle_kitty_keyboard(keyboard),
                _ => println!("CSI({:?})", csi),
            },

            Action::OperatingSystemCommand(command) => self.handle_os_command(command),

            _ => println!("{:?}", action),
        }
    }

    pub fn handle_kitty_keyboard(&mut self, command: Keyboard) {}

    pub fn handle_device(&mut self, device_command: Box<Device>) {}

    pub fn handle_os_command(&mut self, command: Box<OperatingSystemCommand>) {
        use OperatingSystemCommand::*;
        match *command {
            SetWindowTitle(title) => self.title = title,
            SetIconNameAndWindowTitle(title) => self.title = title,
            FinalTermSemanticPrompt(ftsprompt) => self.handle_fts_prompt(ftsprompt),
            ITermProprietary(iterm_command) => self.handle_iterm(iterm_command),
            SystemNotification(notif) => Self::notify_window(notif),

            _ => println!("OperatingSystemCommand({:?})", command),
        };
    }

    /// Handling of all Iterm-based commands
    pub fn handle_iterm(&mut self, command: ITermProprietary) {
        match command {
            ITermProprietary::SetUserVar { name, value } => {
                self.user_vars.insert(name, value);
            }

            _ => println!("Iterm {command:?}"),
        }
    }

    // TODO: Replace this shitty thing with a more explicit system
    // Ideally there would be a Command object, that has prompt, input and output fields
    pub fn handle_fts_prompt(&mut self, prompt: FinalTermSemanticPrompt) {
        use FinalTermSemanticPrompt::*;

        let phys_y = self.screen().phys_line(self.cursor.y);

        match prompt {
            FreshLine => self.fresh_line(),
            FreshLineAndStartPrompt { aid, cl } => {
                self.fresh_line();
                self.start_command();

                if let Some(a) = aid {
                    println!("AID {a}");
                }
                if let Some(c) = cl {
                    println!("FINALCLICK {c}");
                }
            }
            MarkEndOfCommandWithFreshLine { aid, cl } => {
                self.fresh_line();
                self.start_command();

                if let Some(a) = aid {
                    println!("AID {a}");
                }
                if let Some(c) = cl {
                    println!("FINALCLICK {c}");
                }
            }
            StartPrompt(prompt_kind) => self.renderer.attr.semantic_type = SemanticType::Prompt(PromptKind::from(prompt_kind)),
            MarkEndOfPromptAndStartOfInputUntilNextMarker => self.start_input(Until::SemanticMarker),
            MarkEndOfPromptAndStartOfInputUntilEndOfLine => self.start_input(Until::LineEnd),
            MarkEndOfInputAndStartOfOutput { aid } => {
                self.renderer.attr.semantic_type = SemanticType::Output;
                self.commands.start_output(self.cursor.x, self.screen().phys_line(self.cursor.y));

                if let Some(a) = aid {
                    println!("AID {a}");
                }
            }
            CommandStatus { status, aid } => self.commands.set_status(status),
        }
    }

    pub fn fresh_line(&mut self) {
        if self.cursor.x == 0 {
            return;
        }
        self.new_line();
        self.cursor.x = 0;
    }

    pub fn start_command(&mut self) {
        self.renderer.attr.semantic_type = SemanticType::Prompt(PromptKind::Initial);
        self.commands.start_new(self.cursor.x, self.screen().phys_line(self.cursor.y));
    }

    pub fn start_input(&mut self, until: Until) {
        self.renderer.attr.semantic_type = SemanticType::Input(until);
        self.commands.start_input(self.cursor.x, self.screen().phys_line(self.cursor.y));
        // TODO: Do some state management. maybe some sort of custom editor?
    }

    pub fn notify_window(notif: String) { Notification::new().summary("Term").body(&notif).icon("firefox").show().unwrap(); }

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
                if screen.len() > self.cursor.y {
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
                // self.cursor.set_x(0);
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

    pub fn screen(&self) -> &Screen { self.renderer.get_screen(self.state.alt_screen) }

    pub fn mut_screen(&mut self) -> &mut Screen { self.renderer.mut_screen(self.state.alt_screen) }
}
