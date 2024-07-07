use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, strum_macros::VariantNames)]
pub enum TerminalAction {
    NewTab,
    CloseTab,
    Write(String),
    Quit,
    ToggleMenu,
}
