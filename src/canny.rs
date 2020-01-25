use crate::Module;
use crate::Matrix;

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