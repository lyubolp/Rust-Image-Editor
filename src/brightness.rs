use crate::Module;
use crate::Matrix;

pub struct Brightness;

impl Module for Brightness{
    fn new() ->  Brightness{
        Brightness
    }
    fn exec(image: Matrix) -> Matrix{
        println!("Contrast");
        Matrix::new()
    }
}