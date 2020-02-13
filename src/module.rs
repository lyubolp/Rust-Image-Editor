use crate::matrix::Matrix;

pub trait Module{
    fn exec(&self, image: &Matrix, args: &str) -> Matrix;
}

