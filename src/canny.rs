use crate::module::Module;
use crate::matrix::Matrix;

pub struct Canny;

impl Module for Canny{
    fn new() ->  Canny{
        Canny
    }
    fn exec(image: Matrix) -> Matrix{
        println!("Contrast");
        Matrix::new()
    }
}