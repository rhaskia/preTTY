use std::sync::Arc;
use winit::{event::KeyEvent, window::Window};
use dioxus::prelude::*;

use crate::{input::InputManager, renderer::TextRenderer, terminal::Terminal};
use termwiz::escape::csi::{
    EraseInLine,
    DecPrivateMode, Edit,
    Mode::{ResetDecPrivateMode, SetDecPrivateMode},
};
use termwiz::escape::{Action, ControlCode, OperatingSystemCommand, CSI};

pub fn app(cx: Scope) -> Element {
    let terminal = use_signal(|| Terminal::setup()); 

    cx.render(rsx! {
        style { include_str!("style.css") }
        div {}
    })
}

pub struct App<'a> {
    renderer: TextRenderer<'a>,
    input: InputManager,
    terminal: Terminal,

    pub title: String,
}

impl App<'_> {
    pub fn setup(window: Arc<Window>) -> App<'static> {
        App {
            title: String::from("Term"),
            renderer: TextRenderer::new(window),
            input: InputManager::new(),
            terminal: Terminal::setup().unwrap(),
        }
    }

    /// Resizes the rendere and terminal
    pub fn resize_view(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize_view(new_size);
        let glyph_size = self.renderer.glyph_size();
        self.terminal.resize(new_size, glyph_size);
    }

    pub fn render(&mut self) {
        self.renderer.render();
    }

    /// Mostly a handler of Actions that the terminal gives out
    pub fn update(&mut self) {
        loop {
            let action = match self.terminal.pty.rx.try_recv() {
                Ok(a) => a,
                _ => break,
            };
            
            println!("cursor {}, {}", self.terminal.cursor.x, self.terminal.cursor.y);

            match action {
                Action::Print(s) => self.terminal.print(s),
                Action::PrintString(s) => self.terminal.print_str(s),

                Action::Control(control) => match control {
                    ControlCode::LineFeed => self.terminal.new_line(),
                    // Don't do anything for carriage return
                    // would be nice to but it breaks cursor movement
                    ControlCode::CarriageReturn => self.terminal.carriage_return(),
                    ControlCode::Backspace => self.terminal.backspace(),
                    _ => println!("ControlCode({:?})", control),
                },

                Action::CSI(csi) => match csi {
                    CSI::Sgr(sgr) => self.terminal.renderer.handle_sgr(sgr),
                    CSI::Mode(mode) => match mode {
                        SetDecPrivateMode(pmode) => self.set_dec_private_mode(pmode, true),
                        ResetDecPrivateMode(pmode) => self.set_dec_private_mode(pmode, false),
                        _ => println!("Mode({:?})", mode),
                    },
                    CSI::Cursor(cursor) => self.terminal.handle_cursor(cursor),
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

        // TODO: only render when needed
        // im sure dixous will fix this issue
        self.renderer.render_from_cells(self.terminal.get_cells());
    }

    pub fn handle_edit(&mut self, edit: Edit) {
        use EraseInLine::*;
        match edit {
            //Edit::EraseInLine(EraseToEndOfLine) => {}
            Edit::EraseInLine(e) => self.terminal.erase_in_line(e),
            _ => println!("Edit {:?}", edit),
        }
    }

    /// Switches dec private modes on or off
    /// Useful stuff like alt_screen, bracketed_paste etc
    pub fn set_dec_private_mode(&mut self, mode: DecPrivateMode, active: bool) {
        let code = match mode {
            DecPrivateMode::Code(c) => c,
            DecPrivateMode::Unspecified(_) => return,
        };

        use termwiz::escape::csi::DecPrivateModeCode::*;
        match code {
            BracketedPaste => self.terminal.state.bracketed_paste = active,
            EnableAlternateScreen => self.terminal.state.alt_screen = active,
            ClearAndEnableAlternateScreen => self.terminal.state.alt_screen = active,
            _ => println!("Code {:?}, set to {}", code, active),
        }
    }

    /// Handles what happends with keyboard inputs
    pub fn handle_input(&mut self, key: KeyEvent) {
        use crate::input::Input;

        match self.input.handle_input(key) {
            Input::String(s) => self.terminal.pty.writer.write_all(s.as_bytes()),
            Input::Control(c) => match c.as_str() {
                "c" => self.terminal.pty.writer.write_all("\x03".as_bytes()),
                _ => Ok(()),
            },
            Input::None => Ok(()),
        }
        .unwrap();
    }
}
