use std::ops::Range;

#[derive(Debug)]
pub struct CommandSlicer {
    commands: Vec<CommandSlice>
}

#[derive(Debug)]
pub struct CommandSlice {
    prompt: Range<usize>,
    input: Range<usize>,
    output: Range<usize>,
}

impl CommandSlicer {
    pub fn new() -> Self {
        CommandSlicer { commands: Vec::new() }
    }
} 
