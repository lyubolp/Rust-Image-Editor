pub struct Matrix {
    pub values: Vec<(u8, u8, u8)>,
    pub width: u32,
    pub height: u32,
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            values: vec![],
            width: 0,
            height: 0,
        }
    }
}
