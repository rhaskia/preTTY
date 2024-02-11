use std::{
    io::Read,
    io::Write,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use portable_pty::{native_pty_system, Child, CommandBuilder, PtyPair, PtySize, PtySystem};
use termwiz::escape::Action;

pub struct PseudoTerminal {
    pub reader_thread: JoinHandle<()>,
    pub pty_system: Box<dyn PtySystem + Send>,
    pub pair: PtyPair,
    pub child: Box<dyn Child + Sync + Send>,

    pub writer: Box<dyn Write + Send>,
    pub rx: Receiver<Action>,
}

impl PseudoTerminal {
    pub fn setup() -> anyhow::Result<PseudoTerminal> {
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
        let cmd = CommandBuilder::new("bash");
        let child = pair.slave.spawn_command(cmd)?;

        // Read and parse output from the pty with reader
        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let (tx, rx) = channel();

        let reader_thread = thread::spawn(move || {
            parse_terminal_ouput(reader, tx);
        });

        // Pretty much everything needs to be kept in the struct,
        // else drop gets called on the terminal, causing the
        // program to hang on windows
        Ok(PseudoTerminal {
            reader_thread,
            pty_system,
            pair,
            child,
            writer,
            rx,
        })
    }
}

/// Reads from the pseudoterminal, parses using termwiz and
/// then sends it for the app to pick up and use
fn parse_terminal_ouput(mut reader: Box<dyn Read + Send>, tx: Sender<Action>) {
    let mut buffer = [0u8; 1]; // Buffer to hold a single character
    let mut parser = termwiz::escape::parser::Parser::new();

    loop {
        match reader.read(&mut buffer) {
            Ok(_) => {
                parser.parse(&buffer, |t| {
                    tx.send(t);
                });
            }
            Err(err) => {
                eprintln!("Error reading from Read object: {}", err);
                break;
            }
        }
    }
}
