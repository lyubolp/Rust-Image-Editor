use crate::module::{Module};
use crate::matrix::Matrix;

pub struct Crop;

impl Module for Crop{
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self:Sized{
        println!("Contrast");
        Matrix::new()
    }
}