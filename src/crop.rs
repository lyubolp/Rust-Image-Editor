use crate::module::Module;
use crate::matrix::Matrix;
use crate::image_editor::crop_c;

pub struct Crop;

impl Module for Crop {
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self: Sized {
        let rows = image.rows as i32;
        let cols = image.cols as i32;
        let image_type = image.image_type;
        //args: x1:y1:x2:y2
        let args_vec: Vec<&str> = args.split(":").collect();
        let x1 = match args_vec[0].parse::<i32>() {
            Ok(T) => T,
            Err(_) => panic!("Invalid argument")
        };
        let y1 = match args_vec[1].parse::<i32>() {
            Ok(T) => T,
            Err(_) => panic!("Invalid argument")
        };
        let x2 = match args_vec[2].parse::<i32>() {
            Ok(T) => T,
            Err(_) => panic!("Invalid argument")
        };
        let y2 = match args_vec[3].parse::<i32>() {
            Ok(T) => T,
            Err(_) => panic!("Invalid argument")
        };
        let new_rows = y2 - y1;
        let new_cols = x2 - x1;
        unsafe {
            let memory = crop_c(image.to_memory(),
                              rows, cols, image.image_type,
                              y1, x1,
                              new_rows, new_cols);
            Matrix::from_memory(memory, new_rows, new_cols, image_type)
        }
    }
}