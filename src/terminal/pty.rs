use async_channel::Sender;
use portable_pty::{native_pty_system, Child, CommandBuilder, PtyPair, PtySize, PtySystem};
use std::{
    io::{Read, Write},
    thread::{self, JoinHandle},
};
use termwiz::escape::Action;
use tokio::runtime::Runtime;

use crate::input::Input;

pub struct PseudoTerminal {
    pub pty_system: Box<dyn PtySystem + Send>,
    pub pair: PtyPair,
    pub child: Box<dyn Child + Sync + Send>,
    pub writer: Box<dyn Write + Send>,
    pub reader_thread: JoinHandle<()>,
}

impl PseudoTerminal {
    pub fn setup(tx: Sender<Vec<Action>>) -> anyhow::Result<PseudoTerminal> {
        // Send data to the pty by writing to the master
        let pty_system = native_pty_system();

        // Create a new pty
        let pair = pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        // Spawn a shell into the pty
        let cmd = CommandBuilder::new(Self::default_shell());
        let child = pair.slave.spawn_command(cmd)?;

        // Read and parse output from the pty with reader
        let master = &pair.master;
        let writer = master.take_writer().unwrap();
        let reader = master.try_clone_reader().unwrap();

        let reader_thread = thread::spawn(move || {
            parse_terminal_output(tx, reader);
        });

        // Pretty much everything needs to be kept in the struct,
        // else drop gets called on the terminal, causing the
        // program to hang on windows
        Ok(PseudoTerminal {
            pty_system,
            pair,
            child,
            writer,
            reader_thread,
        })
    }

    // Resizes how big the terminal thinks it is
    // mostly useful for rendering tui applications
    pub fn resize(&mut self, screen_width: u32, screen_height: u32, cell_width: f32, cell_height: f32) {
        self.pair
            .master
            .resize(PtySize {
                rows: (screen_height as f32 / cell_height) as u16,
                cols: (screen_width as f32 / cell_width) as u16,
                pixel_width: cell_width.round() as u16,
                pixel_height: cell_height.round() as u16,
            })
            .unwrap();
    }

    pub fn default_shell() -> String {
        if cfg!(windows) {
            String::from("cmd") // TODO: proper windows implementation
        } else {
            match std::env::var("SHELL") {
                Ok(shell) => shell,
                Err(_) => String::from("bash"), // apple should implement SHELL but if they don't too bad
            }
        }
    }

    pub fn write_key_input(&mut self, input: Input) {
        match input {
            Input::String(text) => self.writer.write_all(text.as_bytes()).unwrap(),
            Input::Control(c) => match c.as_str() {
                "c" => self.writer.write_all(b"\x03").unwrap(),
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn parse_terminal_output(tx: Sender<Vec<Action>>, mut reader: Box<dyn Read + Send>) {
    let mut buffer = [0u8; 1024]; // Buffer to hold a single character

    let mut parser = termwiz::escape::parser::Parser::new();
    let rt = Runtime::new().unwrap();

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => {}
            Ok(n) => {
                let mut actions = Vec::new();
                let mut i = 0;

                while i < n {
                    match parser.parse_first(&buffer[i..n]) {
                        Some((action, size)) => {
                            actions.push(action);
                            i += size;
                        }
                        None => break,
                    }
                }

                rt.block_on(async { tx.send(actions.clone()).await });
            }
            Err(err) => {
                eprintln!("Error reading from Read object: {}", err);
                break;
            }
        }
    }
}
