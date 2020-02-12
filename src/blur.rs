use crate::module::Module;
use crate::matrix::Matrix;

pub struct Blur;

impl Module for Blur{
    fn new() ->  Blur{
        Blur
    }
    fn exec(image: Matrix) -> Matrix{
        println!("Contrast");
        Matrix::new()
    }
}