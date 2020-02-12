use crate::module::Module;
use crate::matrix::Matrix;

pub struct Crop;

impl Module for Crop{
    fn new() ->  Crop{
        Crop
    }
    fn exec(image: Matrix) -> Matrix{
        println!("Contrast");
        Matrix::new()
    }
}