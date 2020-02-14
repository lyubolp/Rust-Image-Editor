use crate::module::{Module};
use crate::matrix::Matrix;
use crate::image_editor::{laplace_c};

pub struct Laplace;

impl Module for Laplace{
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self:Sized{
        let rows = image.rows as i32;
        let cols = image.cols as i32;
        let image_type = image.image_type;
        let kernel_size: i32 = match args.parse::<i32>(){
            Ok(T) => T,
            Err(E) => panic!("Invalid argument format")
        };

        unsafe{
            let memory = laplace_c(image.to_memory(), rows, cols, image_type, kernel_size);
            Matrix::from_memory(memory, rows, cols, image_type)
        }
    }
}