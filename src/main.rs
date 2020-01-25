mod matrix;
mod module;
mod io;
mod contrast;
mod blur;
mod brightness;
mod canny;
mod crop;
mod laplace;

use crate::matrix::Matrix;
use crate::module::Module;
use crate::io::IO;
use crate::contrast::Contrast;
use crate::blur::Blur;
use crate::brightness::Brightness;
use crate::canny::Canny;
use crate::crop::Crop;
use crate::laplace::Laplace;

struct ImageEditor {
    modules: Vec<Box<dyn Module>>,
    current_image: Matrix,
    io: IO,
}

impl ImageEditor {
    fn new() -> Self {
        ImageEditor {
            modules: vec![Box::new(Contrast::new()),
                          Box::new(Blur::new()),
                          Box::new(Brightness::new()),
                          Box::new(Canny::new()),
                          Box::new(Crop::new()),
                          Box::new(Laplace::new())
            ],
            current_image: Matrix::new(),
            io: IO::new(),
        }
    }
}

fn main(){
    //io::IO::test_c();

}
