use super::{PseudoTerminalSystemInner, PseudoTerminal};
use std::collections::HashMap;
use std::io::Read;

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

}

impl CustomPty {
    pub fn new() -> Self {
        Self {}
    }
}

impl PseudoTerminal for CustomPty {
    fn resize(&mut self, screen_width: f32,
        screen_height: f32,
        cell_width: f32,
        cell_height: f32) -> (u16, u16) { 
        (1, 1)
    }

    fn reader(&mut self) -> Box<dyn Read + Send> {
        Box::new(Reader { h: true})
    } 
}

pub struct Reader {
    h: bool,
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.h { self.h = false; return Ok(0); }
        for i in 0..10 {
            buf[i] = 65;
        }
        Ok(9)
    }
}
