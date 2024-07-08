use serde::{Deserialize, Serialize};
use strum_macros::{VariantNames, AsRefStr};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, VariantNames, AsRefStr, Default)]
pub enum TerminalAction {
    #[default]
    NoAction,
    NewTab,
    CloseTab,
    Write(String),
    Quit,
    ToggleMenu,
}
