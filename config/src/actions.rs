use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum TerminalAction {
    NewTab,
    CloseTab,
    Write(String),
    Quit,
    ToggleMenu,
}
