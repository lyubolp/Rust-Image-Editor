use crate::matrix::Matrix;

pub trait Module{
    fn new() -> Self where Self:Sized;
    fn exec(image: Matrix, args: &str) -> Matrix where Self:Sized;
}

