use termwiz::escape::osc::FinalTermPromptKind;

pub struct Prompt {
    mode: FinalTermPromptKind,
    content: Vec<Vec<Cell>>,
}

pub struct Command {
    prompt: Prompt,
    input: Vec<Vec<Cell>>,
    output: Vec<Vec<Cell>>,
}
