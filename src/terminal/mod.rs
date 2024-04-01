pub mod cell;
pub mod command;
mod cursor;
mod line;
pub mod pty;
pub mod screen;
mod state;

use std::collections::HashMap;

use cell::{Cell, PromptKind, SemanticType, Until};
use cursor::TerminalCursor;
use line::Line;

use notify_rust::Notification;
use screen::{Screen, TerminalRenderer};
use state::TerminalState;

use termwiz::escape::csi::Mode::{ResetDecPrivateMode, SetDecPrivateMode};
use termwiz::escape::csi::{Cursor, Device, Edit, EraseInDisplay, EraseInLine, Keyboard, CSI};
use termwiz::escape::osc::{FinalTermSemanticPrompt, ITermProprietary};
use termwiz::escape::{
    Action, ControlCode, DeviceControlMode, Esc, KittyImage, OperatingSystemCommand, Sixel,
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
    pub user_vars: HashMap<String, String>,

    pub title: String,
}

impl Terminal {
    // TODO: pty box
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

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Print(s) => self.print(s),
            Action::PrintString(s) => self.print_str(s),
            Action::Control(control) => self.handle_control(control),
            Action::CSI(csi) => self.handle_csi(csi),
            Action::OperatingSystemCommand(command) => self.handle_os_command(command),
            Action::DeviceControl(control) => self.device_control(control),
            Action::Esc(code) => self.handle_esc(code),
            Action::Sixel(sixel) => self.handle_sixel(sixel),
            Action::XtGetTcap(terminfo) => println!("TERMINFO {terminfo:?}"),
            Action::KittyImage(image) => self.kitty_image(image),
        }
    }

    pub fn handle_actions(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.handle_action(action);
        }
    }

    pub fn screen(&self) -> &Screen { self.renderer.get_screen(self.state.alt_screen) }

    pub fn mut_screen(&mut self) -> &mut Screen { self.renderer.mut_screen(self.state.alt_screen) }

    pub fn cursor_pos(&self) -> (usize, usize) { (self.cursor.x, self.phys_cursor_y()) }

    pub fn current_line(&mut self) -> &mut Line { 
        let line_index = self.phys_cursor_y();
        self.mut_screen().mut_line(line_index)
    }

    pub fn phys_cursor_y(&self) -> usize { self.screen().phys_line(self.cursor.y) }

    pub fn resize(&mut self, rows: u16, cols: u16) {
        self.rows = rows.into();
        self.cols = cols.into()
    }

    fn handle_control(&mut self, control_code: ControlCode) {
        match control_code {
            ControlCode::LineFeed => self.new_line(),
            ControlCode::CarriageReturn => self.cursor.set_x(0),
            ControlCode::Backspace => self.backspace(),
            _ => println!("ControlCode({:?})", control_code),
        }
    }

    fn handle_csi(&mut self, csi: CSI) {
        match csi {
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
        }
    }

    fn device_control(&mut self, device_command: DeviceControlMode) {
        match device_command {
            _ => println!("{:?}", device_command),
        }
    }

    fn kitty_image(&mut self, image: Box<KittyImage>) { println!("Kitty Image") }

    /// Handles any Esc codes
    fn handle_esc(&mut self, esc: Esc) {
        use termwiz::escape::Esc::Code;

        let code = match esc {
            Code(c) => c,
            Esc::Unspecified { intermediate, control } => {
                println!("ESC {:?}", esc);
                return;
            }
        };

        use termwiz::escape::EscCode::*;
        match code {
            DecDoubleWidthLine => self.current_line().set_width(true),
            DecDoubleHeightTopHalfLine => self.current_line().set_double(true),
            // TODO: something else needed
            DecDoubleHeightBottomHalfLine => self.current_line().set_double(true),
            _ => println!("ESC {:?}", code),
        }
    }

    /// "Renders" a sixel image
    /// Really just stores it in a state for the webview to render
    fn handle_sixel(&mut self, sixel: Box<Sixel>) { println!("Sixel Image") }

    /// Handles cursor movements, etc
    // Really need to move this to the cursor object
    fn handle_cursor(&mut self, cursor: Cursor) {
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
    fn backspace(&mut self) {
        self.cursor.x -= 1;
        // self.renderer.mut_screen(self.state.alt_screen).cells[self.cursor.y][self.cursor.x] = Cell::default();
        // self.renderer.mut_screen(self.state.alt_screen).cells[self.cursor.y].remove(self.cursor.x);
    }

    fn new_line(&mut self) {
        self.cursor.shift_down(1);
        if self.cursor.y == self.rows as usize {
            self.cursor.y = self.rows as usize - 1;
            self.mut_screen().scrollback();
        }
    }

    /// Pushes a cell onto the current screen
    fn print(&mut self, char: char) {
        let attr = self.renderer.attr.clone();

        // shells don't automatically do wrapping for applications
        // weird as hell
        if self.cursor.x >= self.cols.into() {
            self.cursor.shift_down(1);
            self.cursor.set_x(0);
        }

        self.renderer.mut_screen(self.state.alt_screen).push(
            Cell::new(char, attr),
            self.cursor.x,
            self.cursor.y,
        );

        self.cursor.x += 1;
    }

    fn print_str(&mut self, text: String) {
        for char in text.chars() {
            self.print(char);
        }
    }

    fn handle_kitty_keyboard(&mut self, command: Keyboard) {}

    fn handle_device(&mut self, device_command: Box<Device>) {}

    fn handle_os_command(&mut self, command: Box<OperatingSystemCommand>) {
        use OperatingSystemCommand::*;
        match *command {
            SetWindowTitle(title) => self.title = title,
            SetIconNameAndWindowTitle(title) => self.title = title,
            FinalTermSemanticPrompt(ftsprompt) => self.handle_fts_prompt(ftsprompt),
            ITermProprietary(iterm_command) => self.handle_iterm(iterm_command),
            SystemNotification(notif) => Self::notify_window(notif),
            CurrentWorkingDirectory(cwd) => self.state.cwd = cwd,
            _ => println!("OperatingSystemCommand({:?})", command),
        };
    }

    /// Handling of all Iterm-based commands
    fn handle_iterm(&mut self, command: ITermProprietary) {
        match command {
            ITermProprietary::SetUserVar { name, value } => {
                self.user_vars.insert(name, value);
            }
            _ => println!("Iterm {command:?}"),
        }
    }

    // TODO: Replace this with a more explicit system
    // Ideally there would be a Command object, that has prompt, input and output fields
    fn handle_fts_prompt(&mut self, prompt: FinalTermSemanticPrompt) {
        use FinalTermSemanticPrompt::*;

        match prompt {
            FreshLine => self.fresh_line(),
            FreshLineAndStartPrompt { .. } | MarkEndOfCommandWithFreshLine { .. } => {
                self.fresh_line();
                self.start_command();
            }
            StartPrompt(prompt_kind) => {
                self.renderer.attr.semantic_type =
                    SemanticType::Prompt(PromptKind::from(prompt_kind))
            }
            // why are these so long :sob:
            MarkEndOfPromptAndStartOfInputUntilNextMarker => {
                self.start_input(Until::SemanticMarker)
            }
            MarkEndOfPromptAndStartOfInputUntilEndOfLine => self.start_input(Until::LineEnd),
            MarkEndOfInputAndStartOfOutput { aid } => {
                self.renderer.attr.semantic_type = SemanticType::Output;
                self.commands
                    .start_output(self.cursor.x, self.screen().phys_line(self.cursor.y));
            }
            CommandStatus { status, aid: _ } => self.commands.set_status(status),
        }
    }

    /// Creates a 'Fresh Line' as described by the FinalTermSemanticPrompt protocol
    fn fresh_line(&mut self) {
        if self.cursor.x == 0 {
            return;
        }
        self.new_line();
        self.cursor.x = 0;
    }

    fn start_command(&mut self) {
        self.renderer.attr.semantic_type = SemanticType::Prompt(PromptKind::Initial);
        self.commands
            .start_new(self.cursor.x, self.screen().phys_line(self.cursor.y));
    }

    fn start_input(&mut self, until: Until) {
        self.renderer.attr.semantic_type = SemanticType::Input(until);
        self.commands
            .start_input(self.cursor.x, self.screen().phys_line(self.cursor.y));
        // TODO: Do some state management. maybe some sort of custom editor?
    }

    fn notify_window(notif: String) {
        Notification::new()
            .summary("Term")
            .body(&notif)
            .icon("firefox")
            .show()
            .unwrap();
    }

    fn erase_in_display(&mut self, edit: EraseInDisplay) {
        let screen = self.renderer.mut_screen(self.state.alt_screen);

        match edit {
            EraseInDisplay::EraseDisplay => {
                screen.erase_all();
                self.cursor.set(0, 0);
            }
            _ => println!("Erase {edit:?}"),
        }
    }

    fn erase_in_line(&mut self, edit: EraseInLine) {
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

    fn erase_characters(&mut self, n: u32) {
        let screen = self.renderer.mut_screen(self.state.alt_screen);
        let end = (self.cursor.x + n as usize).min(screen.mut_line(self.cursor.y).len());

        for x in self.cursor.x..end {
            screen.mut_line(self.cursor.y)[x] = Cell::default();
        }
    }

    fn handle_edit(&mut self, edit: Edit) {
        match edit {
            Edit::EraseInLine(e) => self.erase_in_line(e),
            Edit::EraseInDisplay(e) => self.erase_in_display(e),
            Edit::EraseCharacter(n) => self.erase_characters(n),
            _ => println!("Edit {:?}", edit),
        }
    }
}

#[cfg(test)]
mod tests {
    use termwiz::escape::csi::DecPrivateModeCode::EnableAlternateScreen;
    use termwiz::escape::csi::{DecPrivateMode, Mode};

    use super::*;

    #[test]
    pub fn alt_screen() {
        let mut terminal = Terminal::setup().unwrap();

        terminal.handle_action(Action::CSI(CSI::Mode(Mode::SetDecPrivateMode(
            DecPrivateMode::Code(EnableAlternateScreen),
        ))));
        assert!(terminal.state.alt_screen)
    }

    #[test]
    pub fn disable_alt_screen() {
        let mut terminal = Terminal::setup().unwrap();

        terminal.handle_action(Action::CSI(CSI::Mode(Mode::ResetDecPrivateMode(
            DecPrivateMode::Code(EnableAlternateScreen),
        ))));
        assert_eq!(terminal.state.alt_screen, false)
    }

    #[test]
    pub fn clear_line() {
        // No clue how I'm gonna do this
    }
}
