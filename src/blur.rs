use crate::module::{Module};
use crate::matrix::Matrix;

pub struct Blur;

impl Module for Blur{
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self:Sized{
        println!("Contrast");
        Matrix::new()
    }
}