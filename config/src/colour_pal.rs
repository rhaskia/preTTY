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
         (String::from("yellow"), String::from("#ebe234")),
         (String::from("green"), String::from("#5feb34")),
         (String::from("cyan"), String::from("#34e5eb")),
         (String::from("blue"), String::from("#4C13F6")),
         (String::from("purple"), String::from("#E666FF")),

         (String::from("bright_red"), String::from("#ff7f7a")),
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

pub fn default_pal_hc() -> Palette {
    HashMap::from([
         (String::from("red"), String::from("#ff0000")),
         (String::from("yellow"), String::from("#ffff00")),
         (String::from("green"), String::from("#00ff00")),
         (String::from("cyan"), String::from("#00ffff")),
         (String::from("blue"), String::from("#0000ff")),
         (String::from("purple"), String::from("#ff00ff")),

         (String::from("bright_red"), String::from("#ff7f7a")),
         (String::from("bright_yellow"), String::from("#f3ffa3")),
         (String::from("bright_green"), String::from("#a8fa9d")),
         (String::from("bright_blue"), String::from("#6f63f7")),
         (String::from("bright_cyan"), String::from("#8eeef5")),
         (String::from("bright_purple"), String::from("#e48ef5")),

         (String::from("fg0"), String::from("#ffffff")),
         (String::from("fg1"), String::from("#ffffff")),
         (String::from("fg2"), String::from("#ffffff")),
         (String::from("fg3"), String::from("#ffffff")),
         (String::from("fg4"), String::from("#ffffff")),

         (String::from("bg4"), String::from("#1b1b1f")),
         (String::from("bg3"), String::from("#1b1b1f")),
         (String::from("bg2"), String::from("#1b1b1f")),
         (String::from("bg1"), String::from("#1b1b1f")),
         (String::from("bg0"), String::from("#1b1b1f")),
    ])
}

pub fn pal_groups() -> Vec<Vec<&'static str>> {
    vec![
        vec!["red", "yellow", "green", "cyan", "blue", "purple"],
        vec!["bright_red", "bright_yellow", "bright_green", "bright_cyan", "bright_blue", "bright_purple"],
        vec!["fg0", "fg1", "fg2", "fg3", "fg4"],
        vec!["bg0", "bg1", "bg2", "bg3", "bg4"],
    ]
}