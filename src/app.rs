use std::sync::Arc;
use winit::{event::KeyEvent, window::Window};

use crate::{input::InputManager, renderer::TextRenderer, terminal::Terminal};
use termwiz::escape::csi::{
    DecPrivateMode,
    Mode::{ResetDecPrivateMode, SetDecPrivateMode},
};
use termwiz::escape::{Action, ControlCode, CSI};

pub struct App<'a> {
    renderer: TextRenderer<'a>,
    input: InputManager,
    terminal: Terminal,
}

impl App<'_> {
    pub fn setup(window: Arc<Window>) -> App<'static> {
        App {
            renderer: TextRenderer::new(window),
            input: InputManager::new(),
            terminal: Terminal::setup().unwrap(),
        }
    }

    pub fn resize_view(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize_view(new_size);
        let glyph_size = self.renderer.glyph_size();
        self.terminal.resize(new_size, glyph_size);
    }

    pub fn render(&mut self) {
        self.renderer.render();
    }

    pub fn update(&mut self) {
        loop {
            match self.terminal.pty.rx.try_recv() {
                Ok(action) => match action {
                    Action::Print(s) => self.terminal.print(s),
                    Action::PrintString(s) => self.terminal.print_str(s),
                    Action::Control(control) => match control {
                        ControlCode::LineFeed => self.terminal.print('\n'),
                        ControlCode::CarriageReturn => self.terminal.print('\r'),
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
                        _ => println!("CSI({:?})", csi),
                    },
                    _ => println!("{:?}", action),
                },
                _ => break,
            }
        }

        self.renderer.render_from_cells(self.terminal.get_cells());
    }

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

    pub fn handle_input(&mut self, key: KeyEvent) {
        self.terminal
            .pty
            .writer
            .write_all(self.input.key_to_str(key).as_bytes());
    }
}
