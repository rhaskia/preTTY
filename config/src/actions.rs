use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TerminalAction {
    NewTab,
    CloseTab,
    Write(String),
    Quit,
    ToggleMenu,
}
