pub mod cell;
pub mod command;
pub mod cursor;
pub mod line;
pub mod screen;
pub mod state;
pub mod window;
pub mod pty;

use std::collections::HashMap;

use cell::{Cell, PromptKind, SemanticType, Until};
use cursor::TerminalCursor;
use line::Line;
use log::info;
use screen::{Screen, TerminalRenderer};
use state::TerminalState;
use escape::csi::{CsiParam, Cursor, Device, Edit, EraseInDisplay, EraseInLine, Unspecified, CSI};
use escape::osc::{FinalTermSemanticPrompt, ITermProprietary};
use escape::{Action, ControlCode, Esc, KittyImage, OSC, Sixel};
use window::WindowHandler;

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
    pub window: Box<dyn WindowHandler>,
    pub marks: Vec<(usize, usize)>,

    pub title_stack: Vec<String>,
    pub title: String,
}

impl Terminal {
    // TODO: pty box
    // Creates a Terminal Object with a Window Handler
    pub fn setup<T: WindowHandler + 'static>(window: Box<T>) -> anyhow::Result<Terminal> {
        Ok(Terminal {
            rows: 24,
            cols: 80,
            renderer: TerminalRenderer::new(24, 80),
            state: TerminalState::new(),
            cursor: TerminalCursor::new(),
            commands: CommandSlicer::new(),
            user_vars: HashMap::new(),
            window,
            marks: Vec::new(),
            title_stack: Vec::new(),
            title: "PreTTY".into(),
        })
    }

    /// Creates a Terminal without a Window Handler
    /// Useful for if window control is unnessecary or impossible on a platform
    pub fn setup_no_window() -> anyhow::Result<Terminal> { Self::setup(Box::new(())) }

    /// Handles ANSI codes
    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Print(s) => self.print(s),
            Action::PrintString(s) => self.print_str(s),
            Action::Control(control) => self.handle_control(control),
            Action::CSI(csi) => self.handle_csi(csi),
            Action::OSC(command) => self.handle_os_command(command),
            Action::DeviceControl(control) => self.state.device_control(control),
            Action::Esc(code) => self.handle_esc(code),
            Action::Sixel(sixel) => self.handle_sixel(sixel),
            Action::XtGetTcap(terminfo) => info!("TERMINFO {terminfo:?}"),
            Action::KittyImage(image) => self.kitty_image(image),
        }
    }

    /// Handles many ANSI codes
    pub fn handle_actions(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.handle_action(action);
        }
    }

    /// Immutable reference to the current screen object
    pub fn screen(&self) -> &Screen { self.renderer.get_screen(self.state.alt_screen) }

    /// Mutable reference to the current screen object
    pub fn mut_screen(&mut self) -> &mut Screen { self.renderer.mut_screen(self.state.alt_screen) }

    /// The physical position of the cursor
    /// Needed for accessing cells from a screen object with scrollback
    pub fn cursor_pos(&self) -> (usize, usize) { (self.cursor.x, self.phys_cursor_y()) }

    pub fn current_line(&mut self) -> &mut Line {
        let line_index = self.phys_cursor_y();
        self.mut_screen().mut_line(line_index)
    }

    /// The physical position of the cursor line
    pub fn phys_cursor_y(&self) -> usize { self.screen().phys_line(self.cursor.y) }

    /// Sets how large the terminal believes it is
    /// Only needed if you need TUI apps to work
    pub fn resize(&mut self, rows: u16, cols: u16) {
        self.rows = rows.into();
        self.cols = cols.into()
    }

    fn handle_control(&mut self, control_code: ControlCode) {
        match control_code {
            ControlCode::LineFeed => self.new_line(),
            ControlCode::CarriageReturn => self.cursor.set_x(0),
            ControlCode::Backspace => self.backspace(),
            ControlCode::Null => info!("Read NULL char"),
            ControlCode::Bell => self.window.bell(),
            ControlCode::HorizontalTab => self.print_str("    ".to_string()),
            _ => info!("Unimplemented: {control_code:?}"),
        }
    }

    fn handle_csi(&mut self, csi: CSI) {
        match csi {
            CSI::Sgr(sgr) => self.renderer.handle_sgr(sgr),
            CSI::Mode(mode) => self.state.handle_state(mode),
            CSI::Cursor(cursor) => self.handle_cursor(cursor),
            CSI::Edit(edit) => self.handle_edit(edit),
            CSI::Device(device) => self.handle_device(device),
            CSI::Keyboard(keyboard) => self.state.handle_kitty_keyboard(keyboard),
            CSI::Mouse => {} // These are input only
            CSI::Window(command) => self.window.csi_window(command),
            // ECMA-48 SCP (not secure contain protect)
            // pretty sure this is RTL / LTR text, which the webview should implement
            CSI::SelectCharacterPath(_, _) => {}
            CSI::Unspecified(bytes) => self.handle_csi_unspecified(bytes),
        }
    }

    fn handle_csi_unspecified(&mut self, unspecified: Box<Unspecified>) {
        if unspecified.control != 't' || unspecified.parameters_truncated {
            info!("Unknown CSI {unspecified:?}");
            return;
        }

        if unspecified.params == [CsiParam::Integer(23)] { // Pop title
            match self.title_stack.pop() {
                Some(e) => self.title = e,
                None => {},
            }
        } else if unspecified.params == [CsiParam::Integer(22)] { // Push title
            self.title_stack.push(self.title.clone());
        } else {
            info!("Unknown CSI {unspecified:?}");
        }
    }

    fn kitty_image(&mut self, _image: Box<KittyImage>) {
        todo!("Kitty Image");
    }

    /// Handles any Esc codes
    fn handle_esc(&mut self, esc: Esc) {
        use escape::Esc::Code;

        let code = match esc {
            Code(c) => c,
            Esc::Unspecified {
                intermediate,
                control,
            } => {
                info!("ESC Unknown {:?} {:?}", intermediate, control);
                return;
            }
        };

        use escape::EscCode::*;
        match code {
            DecDoubleWidthLine => self.current_line().set_width(true),
            DecDoubleHeightTopHalfLine => self.current_line().set_double(true),
            // TODO: something else needed
            DecDoubleHeightBottomHalfLine => self.current_line().set_double(true),
            DecNormalKeyPad => self.state.alt_keypad = false,
            DecApplicationKeyPad => self.state.alt_keypad = true,
            AsciiCharacterSetG0 => {}
            _ => info!("ESC {:?}", code),
        }
    }

    /// "Renders" a sixel image
    /// Really just stores it in a state for the webview to render
    fn handle_sixel(&mut self, sixel: Box<Sixel>) { info!("Sixel Image {sixel:?}") }

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
                .set(col - 1, line - 1),
            CursorStyle(style) => self.cursor.set_style(style),
            _ => info!("Cursor {cursor:?}"),
        }
    }

    /// Backspaces at the terminal cursor position
    fn backspace(&mut self) { self.cursor.x -= 1; }

    // Performs a new line at the terminal cursor position
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

    // Prints each char of a string into the screen
    fn print_str(&mut self, text: String) {
        for char in text.chars() {
            self.print(char);
        }
    }

    fn handle_device(&mut self, device_command: Box<Device>) {
        info!("Device Command {device_command:?}")
    }

    // Operating System Commands
    // Usually for things like notifications and window control
    // TODO: config to toggle these as they may be unwanted
    fn handle_os_command(&mut self, command: Box<OSC>) {
        use OSC::*;
        match *command {
            SetWindowTitle(title) => self.title = title,
            SetIconNameAndWindowTitle(title) => self.title = title,
            FinalTermSemanticPrompt(ftsprompt) => self.handle_fts_prompt(ftsprompt),
            ITermProprietary(iterm_command) => self.handle_iterm(iterm_command),
            SystemNotification(notif) => self.window.send_notification(notif),
            CurrentWorkingDirectory(cwd) => self.state.cwd = cwd,
            _ => info!("OperatingSystemCommand({:?})", command),
        };
    }

    /// Handling of all Iterm-based commands
    fn handle_iterm(&mut self, command: ITermProprietary) {
        use ITermProprietary::*;
        match command {
            SetUserVar { name, value } => {
                self.user_vars.insert(name, value);
            }
            ClearScrollback => self.mut_screen().clear_scrollback(),
            StealFocus => self.window.steal_focus(),
            SetMark => self.set_mark(),
            SetProfile(profile) => self.set_profile(profile),
            _ => info!("ITERM2 {command:?}"),
        }
    }

    /// Basically vim marks system, aka bookmark for cursor positions
    fn set_mark(&mut self) { self.marks.push(self.cursor_pos()); }

    /// ITerm2 Profiles
    fn set_profile(&mut self, profile: String) { info!("Set Iterm2 Profile {profile}") }

    pub fn kitty_state(&self) -> u16 { self.state.kitty_state }
}

// Prompt Management
impl Terminal {
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
            StartPrompt(prompt_kind) => self
                .renderer
                .attr
                .set_sem_type(SemanticType::Prompt(PromptKind::from(prompt_kind))),
            EndOfPromptUntilMarker => {
                self.start_input(Until::SemanticMarker)
            }
            EndOfPromptUntilEndOfLine => self.start_input(Until::LineEnd),
            EndOfInput { aid: _ } => {
                self.renderer.attr.set_sem_type(SemanticType::Output);
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
        self.renderer
            .attr
            .set_sem_type(SemanticType::Prompt(PromptKind::Initial));
        self.commands
            .start_new(self.cursor.x, self.screen().phys_line(self.cursor.y));
    }

    fn start_input(&mut self, until: Until) {
        self.renderer.attr.set_sem_type(SemanticType::Input(until));
        self.commands
            .start_input(self.cursor.x, self.screen().phys_line(self.cursor.y));
        // TODO: Do some state management. maybe some sort of custom editor?
    }
}

// Erase functions
impl Terminal {
    fn empty_cell(&self) -> Cell {
        Cell { text: ' ', attr: self.renderer.attr.clone() }
    }

    fn erase_in_display(&mut self, edit: EraseInDisplay) {
        let empty = self.empty_cell();
        let screen = self.renderer.mut_screen(self.state.alt_screen);

        match edit {
            EraseInDisplay::EraseDisplay => {
                //screen.ensure_lines(self.rows as usize);
                for i in 0..screen.len() {
                    let cols = self.cols as usize; 
                    let line = screen.mut_line(i);
                    *line = Line::repeat(empty.clone(), cols);
                }
            }
            _ => info!("Erase {edit:?}"),
        }
    }

    fn erase_in_line(&mut self, edit: EraseInLine) {
        let screen = self.renderer.mut_screen(self.state.alt_screen);
        let start = self.cursor.x;
        let y = self.cursor.y;
        let empty = self.empty_cell();

        match edit {
            EraseInLine::EraseToEnd => {
                let line = self.mut_screen().mut_line(y);

                for x in start..line.len() {
                    line[x] = empty.clone();
                }
            }
            EraseInLine::EraseToStart => {
                let line = self.mut_screen().mut_line(y);

                for x in start..line.len() {
                    line[x] = empty.clone();
                }
            }
            EraseInLine::EraseLine => {
                *self.mut_screen().mut_line(y) = Line::default();
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
            _ => info!("EDIT {edit:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use escape::csi::DecPrivateModeCode::EnableAlternateScreen;
    use escape::csi::{DecPrivateMode, Mode};

    use super::*;

    #[test]
    pub fn alt_screen() {
        let mut terminal = Terminal::setup_no_window().unwrap();

        terminal.handle_action(Action::CSI(CSI::Mode(Mode::SetDecPrivateMode(
            DecPrivateMode::Code(EnableAlternateScreen),
        ))));
        assert!(terminal.state.alt_screen)
    }

    #[test]
    pub fn disable_alt_screen() {
        let mut terminal = Terminal::setup_no_window().unwrap();

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
