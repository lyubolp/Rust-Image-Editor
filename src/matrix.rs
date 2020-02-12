use std::os::raw::c_int;

pub struct Matrix {
    pub values: Vec<i32>,
    pub width: u32,
    pub height: u32,
    pub image_type: i32
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            values: vec![],
            width: 0,
            height: 0,
            image_type: 0
        }
    }

    pub unsafe fn from_memory(memory: *mut i32, rows: c_int, cols: c_int, img_type: c_int) -> Self{
        Matrix{
            values: Vec::from_raw_parts(memory, (rows * cols) as usize, (rows * cols) as usize),
            width: cols as u32,
            height: rows as u32,
            image_type: img_type as i32
        }
    }

    pub unsafe fn to_memory(&self) -> *const i32{
        self.values.as_ptr()
    }
}
