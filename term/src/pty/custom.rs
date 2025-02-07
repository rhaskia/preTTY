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
    writers: Vec<Sender<String>>
}

impl CustomPty {
    pub fn new() -> Self {
        Self { writers: Vec::new() }
    }
}

impl PseudoTerminal for CustomPty {
    fn resize(&mut self, screen_width: f32,
        screen_height: f32,
        cell_width: f32,
        cell_height: f32) -> (u16, u16) { 
        (1, 1)
    }

    fn reader(&mut self) -> Box<impl AsyncReader + Send> {
        let (tx, rx) = unbounded();
        self.writers.push(tx);
        Box::new(Reader { rx, excess: Vec::new() })
    } 

    async fn write(&mut self, input: String) {
        for tx in self.writers.iter() {
            tx.send(input.clone()).await;
        }
    }
}

pub struct Reader {
    rx: Receiver<String>,
    excess: Vec<u8>
}

impl AsyncReader for Reader {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, RecvError> {
        let res = self.rx.recv().await?;
        let len = res.len();
        buf.copy_from_slice(res.as_bytes());
        Ok(len)
    }
}
