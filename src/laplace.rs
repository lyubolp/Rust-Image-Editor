use crate::module::Module;
use crate::matrix::Matrix;

pub struct Laplace;

impl Module for Laplace{
    fn new() ->  Laplace{
        Laplace
    }
    fn exec(image: Matrix) -> Matrix{
        println!("Contrast");
        Matrix::new()
    }
}