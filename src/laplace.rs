use crate::module::{Module};
use crate::matrix::Matrix;

pub struct Laplace;

impl Module for Laplace{
    fn new() ->  Laplace{
        Laplace
    }
    fn exec(image: Matrix, args: &str) -> Matrix where Self:Sized{
        println!("Contrast");
        Matrix::new()
    }
}