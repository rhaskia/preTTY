use eframe::egui;
use egui::*;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::{
    io::{Read, Write},
    sync::mpsc::{Receiver, Sender},
};
use egui::{Event, Key};

use std::{
    sync::mpsc::channel,
    thread,
};

fn main() -> anyhow::Result<()> {
    // Send data to the pty by writing to the master
    let mut pty_system = native_pty_system();

    // Create a new pty
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
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
    let mut reader = pair.master.try_clone_reader()?;
    let mut writer = pair.master.take_writer()?;

    let (tx, rx) = channel();

    thread::spawn(move || {
        read_and_send_chars(reader, tx);
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 320.0]),
        ..Default::default()
    };

    eframe::run_native(
        "term",
        options,
        Box::new(|cc| Box::new(Content::new(writer, rx))),
    )
    .unwrap();

    Ok(())
}

fn read_and_send_chars(mut reader: Box<dyn Read + Send>, tx: Sender<char>) {
    let mut buffer = [0u8; 1]; // Buffer to hold a single character

    loop {
        match reader.read(&mut buffer) {
            Ok(_) => {
                let char = buffer[0] as char;
                tx.send(char).unwrap();
            }
            Err(err) => {
                eprintln!("Error reading from Read object: {}", err);
                break;
            }
        }
    }
}

struct Content {
    text: String,
    writer: Box<dyn Write + Send>,
    rx: Receiver<char>,
}

impl Content {
    fn new(writer: Box<dyn Write + Send>, rx: Receiver<char>) -> Content {
        Content {
            text: String::new(),
            writer,
            rx,
        }
    }

    fn handle_inputs(&mut self, events: Vec<Event>) {
        events.iter().for_each(|e| match e {
            Event::Key { key, modifiers, .. } => self.handle_key(&key, &modifiers),
            _ => {},
        });
    }

    fn handle_key(&mut self, key: &Key, modifiers: &Modifiers) {
        write!(self.writer, "{}", key.symbol_or_name());
        // TODO: modifiers
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("Press/Hold/Release example. Press A to test.");

            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.label(&self.text);
                });

            loop {
                match self.rx.try_recv() {
                    Ok(c) => self.text.push(c),
                    Err(_) => break,
                }
            }

            let events = ui.input(|c| c.events.clone());
            println!("{:?}", events);
            self.handle_inputs(events);
        });
    }
}
