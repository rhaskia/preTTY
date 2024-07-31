use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Palette {
    red: String,
    orange: String,
    yellow: String,
    green: String,
    cyan: String,
    blue: String,
    purple: String,

    bright_red: String,
    bright_orange: String,
    bright_yellow: String,
    bright_green: String,
    bright_cyan: String,
    bright_blue: String,
    bright_purple: String,

    /// fg 0 is the most extreme, and fg4 should be closer to the background
    fg0: String, // corresponds with ansi colour code for white
    fg1: String,
    fg2: String, // corresponds with ansi colour code for bright white (actually grey) 
    fg3: String,
    fg4: String,

    /// bg 0 is the most extreme, and bg4 should be closer to the foreground
    bg0: String, // corresponds with ansi colour code for black
    bg1: String,
    bg2: String, // corresponds with ansi colour code for bright black 
    bg3: String,
    bg4: String,
}

// TODO not so messy
impl Palette {
    pub fn to_css(&self) -> String {
        format!("
        :root {{
            --red: {};
            --orange: {};
            --yellow: {};
            --green: {};
            --cyan: {};
            --blue: {};
            --purple: {};
            --bright_red: {};
            --bright_orange: {};
            --bright_yellow: {};
            --bright_green: {};
            --bright_cyan: {};
            --bright_blue: {};
            --bright_purple: {};
            --fg0: {};
            --fg1: {};
            --fg2: {};
            --fg3: {};
            --fg4: {};
            --bg0: {};
            --bg1: {};
            --bg2: {};
            --bg3: {};
            --bg4: {};
        }}
        ", self.red, self.orange, self.yellow, self.green, self.cyan, self.blue, self.purple,
        self.bright_red, self.bright_orange, self.bright_yellow, self.bright_green, self.bright_cyan, self.bright_blue, self.bright_purple,
        self.fg0, self.fg1, self.fg2, self.fg3, self.fg4, self.bg0, self.bg1, self.bg2, self.bg3, self.bg4)          
    }                 
}                     

impl Default for Palette {
    fn default() -> Self {
        Self {
            red: String::from("#eb4034"),
            orange: String::from("#eb9c34"),
            yellow: String::from("#ebe234"),
            green: String::from("#5feb34"),
            cyan: String::from("#34e5eb"),
            blue: String::from("#6234eb"),
            purple: String::from("#ae34eb"),

            bright_red: String::from("#ff7f7a"),
            bright_orange: String::from("#ffc078"),
            bright_yellow: String::from("#f3ffa3"),
            bright_green: String::from("#a8fa9d"),
            bright_blue: String::from("#6f63f7"),
            bright_cyan: String::from("#8eeef5"),
            bright_purple: String::from("#e48ef5"),

            fg0: String::from("#ffffff"),
            fg1: String::from("#ededf2"),
            fg2: String::from("#cfcfd4"),
            fg3: String::from("#a5a5ad"),
            fg4: String::from("#94949c"),
            bg4: String::from("#777780"),
            bg3: String::from("#616169"),
            bg2: String::from("#47474d"),
            bg1: String::from("#2a2a2e"),
            bg0: String::from("#1b1b1f"),
        }
    }
}
