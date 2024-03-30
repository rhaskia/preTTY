use std::io::{Read, Write};
use std::thread::{self, JoinHandle};

use async_channel::Sender;
use portable_pty::{native_pty_system, Child, CommandBuilder, PtyPair, PtySize, PtySystem};
use termwiz::escape::Action;
use tokio::runtime::Runtime;

pub struct PseudoTerminalSystem {
    pub pty_system: Box<dyn PtySystem + Send>,
}

pub struct PseudoTerminal {
    pub pair: PtyPair,
    pub child: Box<dyn Child + Sync + Send>,
    pub writer: Box<dyn Write + Send>,
    pub reader_thread: JoinHandle<()>,
}

impl PseudoTerminalSystem {
    /// Creates a new PseudoTerminal object.
    pub fn setup() -> PseudoTerminalSystem {
        PseudoTerminalSystem {
            pty_system: native_pty_system(),
        }
    }

    /// Requires a sender to pull data out of it
    pub fn spawn_new(&mut self, tx: Sender<Vec<Action>>) -> anyhow::Result<PseudoTerminal> {
        // Create a new pty
        let pair = self.pty_system.openpty(PtySize {
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
            pair,
            child,
            writer,
            reader_thread,
        })
    }

    /// Default shell as per ENV vars or whatever is default for the platform
    pub fn default_shell() -> String {
        if cfg!(windows) {
            String::from("pwsh.exe") // TODO: proper windows implementation
        } else {
            match std::env::var("SHELL") {
                Ok(shell) => shell,
                Err(_) => String::from("bash"), /* apple should implement SHELL but if they don't too bad */
            }
        }
    }
}

impl PseudoTerminal {
    // Resizes how big the terminal thinks it is
    pub fn resize(
        &mut self,
        screen_width: f32,
        screen_height: f32,
        cell_width: f32,
        cell_height: f32,
    ) -> (u16, u16) {
        let (rows, cols) = (
            (screen_height / cell_height) as u16,
            (screen_width / cell_width) as u16,
        );
        self.pair
            .master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: cell_width.round() as u16,
                pixel_height: cell_height.round() as u16,
            })
            .unwrap();
        (rows, cols)
    }

    /// Writes input directly into the pty
    pub fn write(&mut self, input: String) { self.writer.write_all(input.as_bytes()).unwrap() }
}

pub fn parse_terminal_output(tx: Sender<Vec<Action>>, mut reader: Box<dyn Read + Send>) {
    let mut buffer = [0u8; 1024]; // Buffer to hold a single character

    let mut parser = termwiz::escape::parser::Parser::new();
    let rt = Runtime::new().unwrap();

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => {}
            Ok(n) => {
                let actions = parser.parse_as_vec(&buffer[..n]);
                rt.block_on(async { tx.send(actions.clone()).await })
                    .unwrap();
            }
            Err(err) => {
                eprintln!("Error reading from Read object: {}", err);
                break;
            }
        }
    }
}
