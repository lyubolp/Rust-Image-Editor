use crate::Module;
use crate::Matrix;

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