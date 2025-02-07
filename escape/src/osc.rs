#[derive(Debug, PartialEq, Clone)]
pub enum OSC {
    SetWindowTitle(String),
    SetIconNameAndWindowTitle(String),
    FinalTermSemanticPrompt(FinalTermSemanticPrompt),
    ITermProprietary(ITermProprietary),
    SystemNotification(String),
    CurrentWorkingDirectory(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FinalTermSemanticPrompt {
    FreshLine,
    FreshLineAndStartPrompt { },
    MarkEndOfCommandWithFreshLine { },
    StartPrompt(FinalTermPromptKind),
    EndOfPromptUntilMarker,
    EndOfPromptUntilEndOfLine,
    EndOfInput { aid: i32 },
    CommandStatus { status: i32, aid: i32 } 
} 

#[derive(Debug, PartialEq, Clone)]
pub enum ITermProprietary {
    SetUserVar { name: String, value: String },
    ClearScrollback,
    StealFocus,
    SetMark,
    SetProfile(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FinalTermPromptKind {
    Initial,
    RightSide,
    Continuation,
    Secondary
}
