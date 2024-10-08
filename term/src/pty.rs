use std::collections::HashMap;
use std::io::{Read, Write};
use std::ops::Deref;

use async_channel::Sender;
use portable_pty::{
    native_pty_system, Child, CommandBuilder, MasterPty, PtyPair, PtySize, PtySystem,
};
use rand::Rng;
use termwiz::escape::Action;
use tokio::runtime::Runtime;

pub struct PseudoTerminalSystem {
    pub pty_system: Box<dyn PtySystem + Send>,
    pub ptys: HashMap<String, PseudoTerminal>, // Hashmap?
}

pub struct PseudoTerminal {
    pub pair: PtyPair,
    pub child: Box<dyn Child + Sync + Send>,
    pub writer: Box<dyn Write + Send>,
}

impl PseudoTerminalSystem {
    /// Creates a new PseudoTerminal object.
    pub fn setup() -> PseudoTerminalSystem {
        PseudoTerminalSystem {
            pty_system: native_pty_system(),
            ptys: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize { self.ptys.len() }

    /// Requires a sender to pull data out of it
    pub fn spawn_new(&mut self, mut startup_command: Option<String>) -> anyhow::Result<String> {
        // Create a new pty
        let pair = self.pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        // Spawn a shell into the pty
        if let Some(ref c) = startup_command {
            if c.is_empty() {
                startup_command = None;
            }
        }

        let shell = startup_command.unwrap_or(Self::default_shell());
        log::info!("Opening shell {:?}", shell);

        let cmd = CommandBuilder::new(shell);
        let child = pair.slave.spawn_command(cmd)?;

        // Read and parse output from the pty with reader
        let master = &pair.master;
        let writer = master.take_writer().unwrap();

        // Pretty much everything needs to be kept in the struct,
        // else drop gets called on the terminal, causing the
        // program to hang on windows
        let id = generate_id();
        self.ptys.insert(
            id.clone(),
            PseudoTerminal {
                pair,
                child,
                writer,
            },
        );

        Ok(id)
    }

    /// Default shell as per ENV vars or whatever is default for the platform
    pub fn default_shell() -> String {
        if cfg!(windows) {
            String::from("powershell.exe") // TODO: proper windows implementation
        } else {
            match std::env::var("SHELL") {
                Ok(shell) => shell,
                Err(_) => String::from("bash"), /* apple should implement SHELL but if they don't too bad */
            }
        }
    }

    pub fn sleep_pty() {}

    pub fn kill_pty(index: usize) {}

    pub fn get(&mut self, pty: &String) -> &mut PseudoTerminal { self.ptys.get_mut(pty).unwrap() }
}

fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let mut id_bytes: [u8; 16] = [0; 16];
    rng.fill(&mut id_bytes);
    return id_bytes
        .iter()
        .map(|b| format!("{:x?}", b))
        .collect::<Vec<String>>()
        .join("");
}

impl Deref for PseudoTerminalSystem {
    type Target = HashMap<String, PseudoTerminal>;

    fn deref(&self) -> &Self::Target { &self.ptys }
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
            Ok(n) => rt.block_on(async {
                tx.send(parser.parse_as_vec(&buffer[..n])).await;
            }),
            Err(err) => {
                eprintln!("Error reading from Read object: {}", err);
                break;
            }
        }
    }
}
