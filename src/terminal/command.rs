use std::ops::{Range, RangeFrom};

#[derive(Debug)]
pub struct CommandSlicer {
    commands: Vec<CommandSlice>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CommandSlice {
    prompt: Position,
    input: Option<Position>,
    output: Option<Position>,
    end: Option<Position>,
    status: CommandStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandStatus {
    Success,
    Error,
    ShellCommandMisuse,
    CannotExecute,
    NotFound,
    FatalError(i32),
    None,
}

impl CommandStatus {
    pub fn from_int(n: i32) -> Self {
        match n {
            0 => CommandStatus::Success,
            1 => CommandStatus::Error,
            2 => CommandStatus::ShellCommandMisuse,
            126 => CommandStatus::CannotExecute,
            127 => CommandStatus::NotFound,
            128..=255 => CommandStatus::FatalError(n - 128),
            _ => CommandStatus::None,
        }
    }
}

impl CommandSlicer {
    pub fn new() -> Self { CommandSlicer { commands: vec![CommandSlice::new(0, 0)] } }

    pub fn get(&self) -> &Vec<CommandSlice> { &self.commands }

    pub fn start_new(&mut self, x: usize, y: usize) {
        if let Some(command) = self.commands.last_mut() {
            command.end = Some(Position { x, y });
        }
        self.commands.push(CommandSlice::new(x, y));
        println!("STARTED PROMPT");
    }

    pub fn start_input(&mut self, x: usize, y: usize) {
        self.commands.last_mut().unwrap().input = Some(Position { x, y });
        println!("STARTED INPUT");
    }

    pub fn start_output(&mut self, x: usize, y: usize) {
        self.commands.last_mut().unwrap().output = Some(Position { x, y });
        println!("STARTED OUPUT");
    }

    pub fn set_status(&mut self, status: i32) { self.commands.last_mut().unwrap().status = CommandStatus::from_int(status); }
}

impl CommandSlice {
    pub fn new(x: usize, y: usize) -> Self { CommandSlice { prompt: Position { x, y }, input: None, output: None, end: None, status: CommandStatus::None } }

    pub fn get_status(&self) -> CommandStatus { self.status }

    pub fn range(&self, end: usize) -> Range<usize> {
        match self.end {
            Some(end) => self.prompt.y..end.y,
            None => self.prompt.y..end,
        }
    }

    pub fn finished(&self) -> bool { self.end.is_some() }
}
