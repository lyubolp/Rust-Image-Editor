use crate::module::{Module};
use crate::matrix::Matrix;

pub struct Laplace;

impl Module for Laplace{
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self:Sized{
        println!("Contrast");
        Matrix::new()
    }
}