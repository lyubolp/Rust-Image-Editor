use crate::Module;
use crate::Matrix;

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