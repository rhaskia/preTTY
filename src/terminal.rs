use glyph_brush::ab_glyph::PxScale;
use portable_pty::{native_pty_system, Child, CommandBuilder, PtyPair, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::mpsc::Sender;
use std::{
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};
use termwiz::escape::Action;
use winit::dpi::PhysicalSize;

pub struct Terminal {
    reader_thread: JoinHandle<()>,
    pty_system: Box<dyn PtySystem + Send>,
    pair: PtyPair,
    child: Box<dyn Child + Sync + Send>,

    pub writer: Box<dyn Write + Send>,
    pub rx: Receiver<Action>,

    pub rows: u16,
    pub cols: u16,
}

impl Terminal {
    // Resizes how big the terminal thinks it is
    // mostly useful for rendering tui applications
    pub fn resize(&mut self, size: PhysicalSize<u32>, glyph_size: (f32, f32)) {
        let screen_width = size.width.max(1);
        let screen_height = size.height.max(1);

        self.rows = screen_width as u16 / (glyph_size.0.round() as u16);
        self.cols = screen_height as u16 / (glyph_size.1.round() as u16);

        self.pair.master.resize(PtySize {
            rows: self.rows,
            cols: self.cols,
            pixel_width: glyph_size.0.round() as u16,
            pixel_height: glyph_size.1.round() as u16,
        });
    }

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
        let child = pair.slave.spawn_command(cmd)?;

        // Read and parse output from the pty with reader
        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let (tx, rx) = channel();

        let reader_thread = thread::spawn(move || {
            read_and_send_chars(reader, tx);
        });

        // Pretty much everything needs to be kept in the struct,
        // else drop gets called on the terminal, causing the
        // program to hang on windows
        Ok(Terminal {
            reader_thread,
            writer,
            rx,
            pty_system,
            pair,
            child,
            rows: 24,
            cols: 24,
        })
    }
}

// thread to read from terminal output
// really need to rename to match the fact that it
// no longer sends chars, but Actions
fn read_and_send_chars(mut reader: Box<dyn Read + Send>, tx: Sender<Action>) {
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
