use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Palette = HashMap<String, String>;

pub fn to_css(pal: &Palette) -> String {
    format!("
    :root {{
        {}
    }}
    ", pal.iter()
        .map(|(key, value)| format!("--{}: {};", key, value))
        .collect::<Vec<String>>()
        .join("\n")
    )
}                 

pub fn default_pal() -> Palette {
    HashMap::from([
         (String::from("red"), String::from("#eb4034")),
         (String::from("orange"), String::from("#eb9c34")),
         (String::from("yellow"), String::from("#ebe234")),
         (String::from("green"), String::from("#5feb34")),
         (String::from("cyan"), String::from("#34e5eb")),
         (String::from("blue"), String::from("#6234eb")),
         (String::from("purple"), String::from("#ae34eb")),

         (String::from("bright_red"), String::from("#ff7f7a")),
         (String::from("bright_orange"), String::from("#ffc078")),
         (String::from("bright_yellow"), String::from("#f3ffa3")),
         (String::from("bright_green"), String::from("#a8fa9d")),
         (String::from("bright_blue"), String::from("#6f63f7")),
         (String::from("bright_cyan"), String::from("#8eeef5")),
         (String::from("bright_purple"), String::from("#e48ef5")),

         (String::from("fg0"), String::from("#ffffff")),
         (String::from("fg1"), String::from("#ededf2")),
         (String::from("fg2"), String::from("#cfcfd4")),
         (String::from("fg3"), String::from("#a5a5ad")),
         (String::from("fg4"), String::from("#94949c")),
         (String::from("bg4"), String::from("#777780")),
         (String::from("bg3"), String::from("#616169")),
         (String::from("bg2"), String::from("#47474d")),
         (String::from("bg1"), String::from("#2a2a2e")),
         (String::from("bg0"), String::from("#1b1b1f")),
    ])
}
