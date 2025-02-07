use std::ops::{DerefMut, Deref};
use escape::Action;
use async_channel::{Sender, RecvError};
use std::io::Read;
use tokio::{runtime::Runtime, task};
use rand::Rng;
use tokio::io::AsyncRead;

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
    async fn write(&mut self, input: String) {}
    fn reader(&mut self) -> impl AsyncReader + Sized;
}

pub trait AsyncReader {
    async fn read(&mut self, _: &mut [u8]) -> Result<usize, RecvError>;
}

#[cfg(not(target_family = "wasm"))]
pub fn setup_pseudoterminal() -> PseudoTerminalSystem<desktop::PtySystemDesktop> {
    PseudoTerminalSystem { inner: desktop::PtySystemDesktop::setup() }
}

#[cfg(target_family = "wasm")]
pub fn setup_pseudoterminal() -> PseudoTerminalSystem<custom::CustomPtySystem> {
    PseudoTerminalSystem { inner: custom::CustomPtySystem::setup() }
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
