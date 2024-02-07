use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::{
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};
use termwiz::escape::Action;
use std::sync::mpsc::Sender;

pub struct Terminal {
    reader_thread: JoinHandle<()>,
    pub writer: Box<dyn Write + Send>,
    pub rx: Receiver<Action>,
}

impl Terminal {
    pub fn setup() -> anyhow::Result<Terminal> {
        // Send data to the pty by writing to the master
        let pty_system = native_pty_system();

        // Create a new pty
        let pair = pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            // TODO: set this to an actual size
            // Not all systems support pixel_width, pixel_height,
            // but it is good practice to set it to something
            // that matches the size of the selected font.  That
            // is more complex than can be shown here in this
            // brief example though!
            pixel_width: 0,
            pixel_height: 0,
        })?;

        // Spawn a shell into the pty
        let cmd = CommandBuilder::new("bash");
        let _child = pair.slave.spawn_command(cmd)?;


        // Read and parse output from the pty with reader
        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let (tx, rx) = channel();

        let reader_thread = thread::spawn(move || {
            read_and_send_chars(reader, tx);
        });

        Ok(Terminal {
            reader_thread,
            writer,
            rx,
        })
    }
}

fn read_and_send_chars(mut reader: Box<dyn Read + Send>, tx: Sender<Action>) {
    let mut buffer = [0u8; 1]; // Buffer to hold a single character
    let mut parser = termwiz::escape::parser::Parser::new();

    loop {
        match reader.read(&mut buffer) {
            Ok(_) => {
                let _char = buffer[0];
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
