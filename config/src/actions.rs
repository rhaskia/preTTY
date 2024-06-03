use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum TerminalAction {
    NewTab,
    CloseTab,
    CloseTabSpecific(i64),
    Write(String),
    Quit,
}
