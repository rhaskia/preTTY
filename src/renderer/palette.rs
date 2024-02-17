pub struct Palette {
    pub colors: Vec<String>,
}

impl Palette {
    pub fn default() -> Palette { 
        Palette {
            colors: vec![
                // Black
                "#000000".to_string(),

                // Red, Green, Yellow
                "#ff0000".to_string(),
                "#00ff00".to_string(),
                "#ffff00".to_string(),

                // Blue, Cyan, Magenta
                "#0000ff".to_string(),
                "#00ffff".to_string(),
                "#ff00ff".to_string(),

                // White
                "#dddddd".to_string(),

                // Bright Colours
                // Black
                "#999999".to_string(),

                // Red, Green, Yellow
                "#ff0000".to_string(),
                "#00ff00".to_string(),
                "#ffff00".to_string(),

                // Blue, Cyan, Magenta
                "#0000ff".to_string(),
                "#00ffff".to_string(),
                "#ff00ff".to_string(),

                // White
                "#ffffff".to_string(),
            ]
        }
    }
}
