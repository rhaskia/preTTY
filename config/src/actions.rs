use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Action {
    NewTab(String),
    CloseTab,
    CloseTabSpecific(i64),
    Quit,
}
