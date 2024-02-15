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


        // Pretty much everything needs to be kept in the struct,
        // else drop gets called on the terminal, causing the
        // program to hang on windows
        Ok(PseudoTerminal {
            pty_system,
            pair,
            child,
        })
    }
}
