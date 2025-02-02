use std::ops::{DerefMut, Deref};
use escape::Action;
use async_channel::Sender;
use std::io::Read;
use tokio::{runtime::Runtime, task};
use rand::Rng;

#[cfg(not(target_family = "wasm"))]
pub mod desktop;
pub mod custom;

pub struct PseudoTerminalSystem<T: PseudoTerminalSystemInner> {
    inner: T
}

impl<T: PseudoTerminalSystemInner> Deref for PseudoTerminalSystem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: PseudoTerminalSystemInner> DerefMut for PseudoTerminalSystem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub trait PseudoTerminalSystemInner {
    fn setup() -> Self;
    fn len(&self) -> usize { 0 }
    fn spawn_new(&mut self, command: Option<String>) -> anyhow::Result<String>;
    fn get(&mut self, id: &str) -> &mut impl PseudoTerminal;
}

pub trait PseudoTerminal {
    fn resize(&mut self, screen_width: f32,
        screen_height: f32,
        cell_width: f32,
        cell_height: f32) -> (u16, u16) { (1, 1) }
    fn write(&mut self, input: String) {}
    fn reader(&mut self) -> Box<dyn Read + Send>;
}

#[cfg(not(target_family = "wasm"))]
pub fn setup_pseudoterminal() -> PseudoTerminalSystem<desktop::PtySystemDesktop> {
    PseudoTerminalSystem { inner: desktop::PtySystemDesktop::setup() }
}

#[cfg(target_family = "wasm")]
pub fn setup_pseudoterminal() -> PseudoTerminalSystem<custom::CustomPtySystem> {
    PseudoTerminalSystem { inner: custom::CustomPtySystem::setup() }
}

pub async fn parse_terminal_output(tx: Sender<Vec<Action>>, mut reader: Box<dyn Read + Send>) {
    let mut buffer = [0u8; 1024]; // Buffer to hold a single character

    let mut parser = escape::parser::Parser::new();

    task::spawn_blocking(move || {
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {}
                Ok(n) => {
                    let res = parser.parse_as_vec(&buffer[..n]);
                    log::info!("{res:?}");
                    tx.send(res);
                }
                Err(err) => {
                    eprintln!("Error reading from Read object: {}", err);
                    break;
                }
            }
        }
    }).await;
}

pub fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let mut id_bytes: [u8; 16] = [0; 16];
    rng.fill(&mut id_bytes);
    return id_bytes
        .iter()
        .map(|b| format!("{:x?}", b))
        .collect::<Vec<String>>()
        .join("");
}
