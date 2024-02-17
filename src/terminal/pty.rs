use std::{
    io::Read,
    io::Write,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use portable_pty::{native_pty_system, Child, CommandBuilder, PtyPair, PtySize, PtySystem};
use termwiz::escape::Action;

pub struct PseudoTerminal {
    pub pty_system: Box<dyn PtySystem + Send>,
    pub pair: PtyPair,
    pub child: Box<dyn Child + Sync + Send>,
    pub reader_thread: JoinHandle<()>,
    pub rx: Receiver<Action>,
    pub writer: Box<dyn Write + Send>
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
        let master = &pair.master;
        let writer = master.take_writer().unwrap();
        let reader = master.try_clone_reader().unwrap();

        let (tx, rx) = channel();

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
            rx,
        })
    }
}

pub fn parse_terminal_output(tx: Sender<Action>, mut reader: Box<dyn Read + Send>) {
        let mut buffer = [0u8; 1]; // Buffer to hold a single character
        let mut parser = termwiz::escape::parser::Parser::new();

        loop {
            match reader.read(&mut buffer) {
                Ok(_) => {
                    parser.parse(&buffer, |t| {tx.send(t);});
                }
                Err(err) => {
                    eprintln!("Error reading from Read object: {}", err);
                    break;
                }
            }
        }
}
