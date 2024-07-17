use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::{AsRefStr, VariantNames};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, VariantNames, AsRefStr, Default)]
pub enum TerminalAction {
    #[default]
    NoAction,
    NewTab,
    CloseTab,
    CloseTabSpecific(usize),
    Write(String),
    OpenSettings,
    ToggleCommandPalette,
    Quit,
}

impl TerminalAction {
    pub fn palette_usable() -> Vec<TerminalAction> {
        use TerminalAction::*;
        vec![NewTab, CloseTab, Quit, OpenSettings]
    }

    // Produces a human readable version of the variant name
    pub fn readable(&self) -> String { insert_spaces(self.as_ref()) }
}

impl Display for TerminalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.readable())
    }
}

fn insert_spaces(text: &str) -> String {
    let mut new_text = String::new();
    let mut prev_lower = false;
    let mut first = true;

    for c in text.chars() {
        if c.is_uppercase() && !first {
            new_text.push(' ');
        } else {
            prev_lower = c.is_lowercase();
        }
        new_text.push(c);
        first = false;
    }

    new_text
}
