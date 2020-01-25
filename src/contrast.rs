use crate::Module;
use crate::Matrix;

pub struct Contrast;

impl Module for Contrast{
    fn new() ->  Contrast{
        Contrast
    }
    fn exec(image: Matrix) -> Matrix{
        println!("Contrast");
        Matrix::new()
    }
}