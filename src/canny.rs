use crate::module::{Module};
use crate::matrix::Matrix;

pub struct Canny;

impl Module for Canny{
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self:Sized{
        println!("Contrast");
        Matrix::new()
    }
}