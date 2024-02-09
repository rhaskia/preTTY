use std::sync::Arc;
use winit::{event::KeyEvent, window::Window};

use termwiz::escape::csi::Sgr;
use termwiz::escape::{Action, ControlCode, CSI};

use crate::render::WGPUColor;
use crate::{input::InputManager, render::TextRenderer, terminal::Terminal};

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
            match self.terminal.rx.try_recv() {
                Ok(action) => match action {
                    Action::Print(s) => self.renderer.push_text(s.to_string()),
                    Action::PrintString(s) => self.renderer.push_text(s),
                    Action::Control(control) => match control {
                        ControlCode::LineFeed => self.renderer.push_text("\n".to_string()),
                        ControlCode::CarriageReturn => self.renderer.push_text("\r".to_string()),
                        _ => println!("{:?}", control),
                    },
                    Action::CSI(csi) => match csi {
                        CSI::Sgr(sgr) => match sgr {
                            Sgr::Foreground(f) => self.renderer.color = f.to_vec(),
                            Sgr::Reset => self.renderer.color = [1.0; 4],
                            _ => println!("{:?}", sgr),
                        },
                        _ => println!("{:?}", csi),
                    },
                    _ => println!("{:?}", action),
                },
                _ => return,
            }
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        self.terminal
            .writer
            .write_all(self.input.key_to_str(key).as_bytes());
    }
}
