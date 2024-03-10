use std::ops::Range;

#[derive(Debug)]
pub struct CommandSlicer {
    commands: Vec<CommandSlice>,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct CommandSlice {
    prompt: Position,
    input: Option<Position>,
    output: Option<Position>,
    end: Option<Position>,
}

impl CommandSlicer {
    pub fn new() -> Self {
        CommandSlicer {
            commands: Vec::new(),
        }
    }

    pub fn get(&self) -> &Vec<CommandSlice> {
        &self.commands
    }

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
}

impl CommandSlice {
    pub fn new(x: usize, y: usize) -> Self {
        CommandSlice {
            prompt: Position { x, y },
            input: None,
            output: None,
            end: None,
        }
    }

    pub fn lines_range(&self) -> Option<Range<usize>> {
        match self.end {
            Some(end) => Some(self.prompt.y..end.y),
            None => None,
        }
    }

    pub fn finished(&self) -> bool {
        self.end.is_some()
    }
}
