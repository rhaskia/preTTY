pub struct Palette {
    pub colors: Vec<[f32; 4]>,
}

impl Palette {
    pub fn default() -> Palette { 
        Palette {
            colors: vec![
                [0.0, 0.0, 0.0, 1.0],

                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
                [1.0, 1.0, 0.0, 1.0],

                [0.0, 0.0, 1.0, 1.0],
                [1.0, 0.0, 1.0, 1.0],
                [0.0, 1.0, 1.0, 1.0],

                [1.0, 1.0, 1.0, 1.0],
            ]
        }
    }
}
