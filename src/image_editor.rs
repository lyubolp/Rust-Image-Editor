use crate::matrix::Matrix;
use crate::module::Module;
use crate::io::IO;
use crate::blur::Blur;
use crate::brightness_contrast::Brightness;
use crate::brightness_contrast::Contrast;
use crate::canny::Canny;
use crate::crop::Crop;
use crate::laplace::Laplace;

extern crate libc;

use libc::{c_int};
use std::os::raw::{c_char};
use std::ffi::CString;

pub struct ImageEditor {
    modules: Vec<Box<dyn Module>>,
    current_image: Matrix,
    io: IO,
}
#[link(name = "core_cpp", kind = "static")]
extern "C" {
    pub fn open_file(path: *const c_char) -> *mut c_int;
    pub fn get_rows(path: *const c_char) -> c_int;
    pub fn get_cols(path: *const c_char) -> c_int;
    pub fn save_file(memory: *const c_int, path: *const c_char, rows: c_int, cols: c_int) -> c_int;
    pub fn clear_memory(memory: *mut c_char);
}

impl ImageEditor {
    pub fn new() -> Self {
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

    pub fn open_image(&mut self, path: &str){
        match self.io.open(path){
            Ok(image) => {
                self.current_image = image;
                println!("File opened successfully !");
            },
            Err(E) => println!("Unable to load image - error trace: {}", E)
        }
    }

    pub fn save_image(&mut self)
    {
        match self.io.save(&self.current_image){
            Ok(_) => println!("File saved successfully"),
            Err(E) => println!("Unable to save image - error trace: {}", E)
        }
    }
    pub fn save_image_as(&mut self, path: &str)
    {
        match self.io.save_as(&self.current_image, path){
            Ok(_) => println!("File saved successfully"),
            Err(E) => println!("Unable to save image - error trace: {}", E)
        }
    }
}
