use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, strum_macros::VariantNames, Default)]
pub enum TerminalAction {
    #[default]
    NoAction,
    NewTab,
    CloseTab,
    Write(String),
    Quit,
    ToggleMenu,
}
