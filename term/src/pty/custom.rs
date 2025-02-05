use super::{PseudoTerminalSystemInner, PseudoTerminal};
use std::collections::HashMap;
use std::io::{Read, Error};
use async_channel::{Sender, Receiver, unbounded};
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

    fn reader(&mut self) -> Box<dyn AsyncRead + Send> {
        let (tx, rx) = unbounded();
        self.writers.push(tx);
        Box::new(Reader { reader: rx, excess: Vec::new() })
    } 

    async fn write(&mut self, input: String) {
        for tx in self.writers.iter() {
            tx.send(input.clone()).await;
        }
    }
}

pub struct Reader {
    reader: Receiver<String>,
    excess: Vec<u8>
}

impl AsyncRead for Reader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        log::info!("why isnt this fucking called");
        match pin!(self.reader.recv()).poll(cx) {
            Poll::Ready(res) => {
                log::info!("woah {res:?}");
                match res {
                    Ok(input) => {
                        buf.put_slice(input.as_bytes());
                        Poll::Ready(Ok(()))
                    }
                    Err(err) => Poll::Ready(Err(Error::new(std::io::ErrorKind::Other, err))),
                }
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
