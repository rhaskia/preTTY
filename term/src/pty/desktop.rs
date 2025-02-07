use std::collections::HashMap;
use std::io::{Read, Write};
use std::ops::Deref;
use super::generate_id;
use std::task::Poll;
use std::pin::{pin, Pin};
use std::future::Future;
use async_channel::{Sender, Receiver, Recv, RecvError};
use portable_pty::{
    native_pty_system, Child, CommandBuilder, MasterPty, PtyPair, PtySize, PtySystem,
};
use escape::Action;
use tokio::runtime::Runtime;
use tokio::task::{JoinHandle, spawn_blocking};
use super::{PseudoTerminalSystemInner, PseudoTerminal, PseudoTerminalSystem, AsyncReader};
use tokio::io::AsyncRead;

pub struct PtySystemDesktop {
    pub pty_system: Box<dyn PtySystem + Send>,
    pub ptys: HashMap<String, PtyDesktop>,
}

pub struct PtyDesktop {
    pub pair: PtyPair,
    pub child: Box<dyn Child + Sync + Send>,
    pub writer: Box<dyn Write + Send>,
}

impl PseudoTerminalSystemInner for PtySystemDesktop {
    /// Creates a new PseudoTerminal object.
    fn setup() -> Self {
        PtySystemDesktop {
            pty_system: native_pty_system(),
            ptys: HashMap::new(),
        }
    }

    fn len(&self) -> usize { self.ptys.len() }

    /// Requires a sender to pull data out of it
    fn spawn_new(&mut self, mut startup_command: Option<String>) -> anyhow::Result<String> {
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
            PtyDesktop {
                pair,
                child,
                writer,
            },
        );

        Ok(id)
    }


    fn get(&mut self, pty: &str) -> &mut PtyDesktop { self.ptys.get_mut(pty).unwrap() }
}

impl PtySystemDesktop {
    /// Default shell as per ENV vars or whatever is default for the platform
    fn default_shell() -> String {
        if cfg!(windows) {
            String::from("powershell.exe") // TODO: proper windows implementation
        } else {
            match std::env::var("SHELL") {
                Ok(shell) => shell,
                Err(_) => String::from("bash"), /* apple should implement SHELL but if they don't too bad */
            }
        }
    }

    fn sleep_pty() {}

    fn kill_pty(index: usize) {}
}


impl Deref for PtySystemDesktop {
    type Target = HashMap<String, PtyDesktop>;

    fn deref(&self) -> &Self::Target { &self.ptys }
}

impl PseudoTerminal for PtyDesktop {
    // Resizes how big the terminal thinks it is
    fn resize(
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
    async fn write(&mut self, input: String) { 
        log::info!("writing {input}");
        self.writer.write_all(input.as_bytes()).unwrap() 
    }

    fn reader(&mut self) -> Reader {
        let mut inner = self.pair.master.try_clone_reader().unwrap();
        let (tx, rx) = async_channel::unbounded();
        let mut handle = spawn_blocking(move || { 
            let mut buffer = [0; 1024];
            loop {
                let n = inner.read(&mut buffer).unwrap();
                tx.send_blocking(buffer[..n].to_vec());
            }
        });

        Reader { handle, rx }
    }
}

pub struct Reader {
    handle: JoinHandle<()>,
    rx: Receiver<Vec<u8>>,
}

impl AsyncReader for Reader {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, RecvError> {
        let res = self.rx.recv().await?;
        let len = res.len();
        for i in 0..len {
            buf[i] = res[i];
        }
        Ok(len)
    }
}
