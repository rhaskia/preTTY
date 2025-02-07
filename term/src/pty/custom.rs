use super::{PseudoTerminalSystemInner, PseudoTerminal, AsyncReader};
use std::collections::HashMap;
use std::io::{Read, Error};
use async_channel::{Sender, Receiver, unbounded, RecvError};
use std::task::Poll;
use tokio::io::AsyncRead;
use std::pin::pin;
use std::future::Future;

pub struct CustomPtySystem {
    ptys: HashMap<String, CustomPty>,
}

impl PseudoTerminalSystemInner for CustomPtySystem {
    fn setup() -> Self {
        CustomPtySystem { ptys: HashMap::new() }
    }

    fn spawn_new(&mut self, mut startup_command: Option<String>) -> anyhow::Result<String> {
        let id = super::generate_id(); 

        self.ptys.insert(id.clone(), CustomPty::new());

        Ok(id)
    }

    fn get(&mut self, id: &str) -> &mut CustomPty {
        self.ptys.get_mut(id).unwrap()
    }
}

pub struct CustomPty {
    writers: Vec<Sender<String>>,
    command_builder: String,
}

impl CustomPty {
    pub fn new() -> Self {
        Self { writers: Vec::new(), command_builder: String::new() }
    }

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
        (rows, cols)
    }

    async fn match_command(&mut self) {
        match self.command_builder.as_str() {
            "help" => self.send(String::from("\r\nAvailable commands include: colors")).await,
            "colors" => {
                let fg = (30..38).map(|n| format!("\u{1b}[{n}m test")).collect::<Vec<String>>().join(" ");
                let bg = (40..48).map(|n| format!("\u{1b}[{n}m test")).collect::<Vec<String>>().join("\u{1b}[m");
                let c256 = (0..256).map(|n| format!("\u{1b}[48;5;{n}m.")).collect::<Vec<String>>().join("");
                self.send(format!("{fg}\r\n{bg}\r\n{c256}\u{1b}[0m\r\n> ")).await;
                self.send(String::from("")).await;
            }
            command => self.send(format!("\r\nCommand {command} is unknown")).await,
        }

        self.send(String::from("\u{1b}[0m\r\n> ")).await;
        self.command_builder = String::new();
    }

    async fn manage_char(&mut self, c: char) {
        match c {
            '\u{0D}' => self.match_command().await,
            a => { 
                self.command_builder.push(c);
                self.send(format!("{a}")).await;
            }
        }
    }

    async fn send(&mut self, output: String) {
        for tx in self.writers.iter() {
            tx.send(output.clone()).await;
        }
    }
}

impl PseudoTerminal for CustomPty {
    fn resize(&mut self, screen_width: f32,
        screen_height: f32,
        cell_width: f32,
        cell_height: f32) -> (u16, u16) { 
        (1, 1)
    }

    fn reader(&mut self) -> Reader {
        let (tx, rx) = unbounded();
        self.writers.push(tx);
        Reader { rx, excess: Vec::new(), initialized: false }
    } 

    async fn write(&mut self, input: String) {
        log::info!("writing {input}");
        for c in input.chars() {
            self.manage_char(c).await;
        }
    }
}

pub struct Reader {
    rx: Receiver<String>,
    excess: Vec<u8>,
    initialized: bool,
}

impl AsyncReader for Reader {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, RecvError> {
        if !self.initialized {
            buf[0] = b'>';
            buf[1] = b' ';
            self.initialized = true;
            return Ok(2);
        }

        let mut i = 0;
        if !self.excess.is_empty() {
            while let Some(c) = self.excess.get(0) { 
                buf[i] = *c;
                self.excess.remove(0);
                i += 1;
                if buf.len() == i {
                    return Ok(i);
                }
            }
        }

        let res = self.rx.recv().await?;
        let len = res.len();
        let mut bytes = res.as_bytes();
        for j in 0..bytes.len() {
            buf[i] = bytes[j];
            i += 1;
            if buf.len() == i {
                self.excess.append(&mut bytes[j..].to_vec());
                return Ok(i);
            }
        }
        Ok(len)
    }
}
