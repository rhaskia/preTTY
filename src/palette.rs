pub struct Palette {
    pub colors: Vec<[f32; 4]>,
}

impl Palette {
    pub fn default() -> Palette { 
        Palette {
            colors: vec![
                // Black
                [0.0, 0.0, 0.0, 1.0],

                // Red, Yellow, Green
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
                [1.0, 1.0, 0.0, 1.0],

                // Blue, Cyan, Magenta
                [0.0, 0.0, 1.0, 1.0],
                [1.0, 0.0, 1.0, 1.0],
                [0.0, 1.0, 1.0, 1.0],

                // White
                [1.0, 1.0, 1.0, 1.0],

                // Bright Colours
                // Black
                [0.5, 0.5, 0.5, 1.0],

                // Red, Yellow, Green
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
                [1.0, 1.0, 0.0, 1.0],

                // Blue, Cyan, Magenta
                [0.0, 0.0, 1.0, 1.0],
                [1.0, 0.0, 1.0, 1.0],
                [0.0, 1.0, 1.0, 1.0],

                // White
                [1.0, 1.0, 1.0, 1.0],
            ]
        }
    }
}
