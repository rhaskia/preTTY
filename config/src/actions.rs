use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum TerminalAction {
    NewTab,
    CloseTab,
    Write(String),
    Quit,
    ToggleMenu,
}
