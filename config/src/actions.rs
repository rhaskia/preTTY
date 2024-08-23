use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, VariantNames};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, VariantNames, AsRefStr, Default)]
pub enum TerminalAction {
    #[default]
    NoAction,
    CloseTabSpecific(usize),
    Write(String),

    OpenSettings,
    OpenPluginMenu,
    ToggleCommandPalette,
    OpenDevTools,

    PasteText,
    CopyText,
    ClearBuffer,

    NewTab,
    CloseTab,
    NextTab,
    PreviousTab,
    CloseOtherTabs,

    Quit,

    ScrollUp,
    ScrollUpPage,
    ScrollDown,
    ScrollDownPage,
    ScrollToBottom,
    ScrollToTop,
}

impl TerminalAction {
    pub fn palette_usable() -> Vec<TerminalAction> {
        use TerminalAction::*;
        vec![
            OpenSettings,
            ToggleCommandPalette,
            OpenDevTools,

            PasteText,
            CopyText,
            ClearBuffer,

            NewTab,
            CloseTab,
            NextTab,
            PreviousTab,
            CloseOtherTabs,

            Quit,

            ScrollUp,
            ScrollUpPage,
            ScrollDown,
            ScrollDownPage,
            ScrollToBottom,
            ScrollToTop,
        ]
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
