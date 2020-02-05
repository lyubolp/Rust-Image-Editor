mod matrix;
mod module;
mod io;
mod blur;
mod brightness_contrast;
mod canny;
mod crop;
mod laplace;

extern crate libc;

use crate::matrix::Matrix;
use crate::module::Module;
use crate::io::IO;
use crate::blur::Blur;
use crate::brightness_contrast::Brightness;
use crate::brightness_contrast::Contrast;
use crate::canny::Canny;
use crate::crop::Crop;
use crate::laplace::Laplace;

use libc::c_int;
use std::os::raw::c_char;
use std::ffi::CString;

struct ImageEditor {
    modules: Vec<Box<dyn Module>>,
    current_image: Matrix,
    io: IO,
}
#[link(name = "core_cpp", kind = "static")]
extern "C" {
    pub fn open_file(path: *const c_char) -> *mut c_int;
    pub fn get_rows(path: *const c_char) -> c_int;
    pub fn get_cols(path: *const c_char) -> c_int;
    pub fn print(to_write: *const c_char);
    pub fn read() -> *mut c_char;
    pub fn clear_memory(memory: *mut c_char);
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

    fn open_image(&mut self, path: &str){
        match self.io.open(path){
            Ok(image) => self.current_image = image,
            Err(E) => println!("Unable to load image - error trace: {}", E)
        }
    }
}


fn main() {
    let mut image_editor = ImageEditor::new();
    let path = "/home/lyubo/Uni/RustProject/Rust-Image-Editor/test.jpg";
    image_editor.open_image(path);
}
