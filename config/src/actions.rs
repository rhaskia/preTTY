use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum TerminalAction {
    NewTab(String),
    CloseTab,
    CloseTabSpecific(i64),
    Write(String),
    Quit,
}
