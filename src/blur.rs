use crate::module::{Module};
use crate::matrix::Matrix;
use crate::image_editor::blur_c;

pub struct Blur;

impl Module for Blur{
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self: Sized {
        let rows = image.rows as i32;
        let cols = image.cols as i32;
        let image_type = image.image_type;
        unsafe {
            let value = match args.parse::<i32>()
            {
                Ok(val) => val,
                Err(e) => {
                    println!("Argument invalid, setting it to 1");
                    1
                }
            };
            let memory = blur_c(image.to_memory(), rows, cols, image.image_type, value);
            Matrix::from_memory(memory, rows, cols, image_type)
        }
    }
}