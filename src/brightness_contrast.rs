use crate::module::Module;
use crate::matrix::Matrix;

pub struct Brightness;
pub struct Contrast;

impl Module for Brightness{
    fn new() ->  Brightness{
        Brightness
    }
    fn exec(image: Matrix) -> Matrix{
        println!("Contrast");
        Matrix::new()
    }
}

impl Module for Contrast{
    fn new() ->  Contrast{
        Contrast
    }
    fn exec(image: Matrix) -> Matrix{
        println!("Contrast");
        Matrix::new()
    }
}