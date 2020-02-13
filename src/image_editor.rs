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
use std::os::raw::{c_char, c_double};

pub struct ImageEditor {
    modules: Vec<(String, Box<dyn Module>)>,
    current_image: Matrix,
    io: IO,
}
#[link(name = "core_cpp", kind = "static")]
extern "C" {
    pub fn open_file(path: *const c_char) -> *mut c_int;
    pub fn get_rows(path: *const c_char) -> c_int;
    pub fn get_cols(path: *const c_char) -> c_int;
    pub fn get_type(path: *const c_char) -> c_int;
    pub fn save_file(memory: *const c_int, path: *const c_char, rows: c_int, cols: c_int, image_type: c_int) -> c_int;
    pub fn clear_memory(memory: *mut c_char);
    pub fn change_brightness(image: *const c_int, rows: c_int, cols: c_int, image_type: c_int, brightness: c_int) -> *mut c_int;
    pub fn change_contrast(image: *const c_int, rows: c_int, cols: c_int, image_type: c_int, contrast: c_double) -> *mut c_int;

}

impl ImageEditor {
    pub fn new() -> Self {
        ImageEditor {
            modules: vec![(String::from("Contrast"), Box::new(Contrast::new())),
                          (String::from("Blur"), Box::new(Blur::new())),
                          (String::from("Brightness"), Box::new(Brightness::new())),
                          (String::from("Canny"), Box::new(Canny::new())),
                          (String::from("Crop"), Box::new(Crop::new())),
                          (String::from("Laplace"), Box::new(Laplace::new()))
            ],
            current_image: Matrix::new(),
            io: IO::new(),
        }
    }
    //For testing purposes
    pub fn get_image(&self) -> &Matrix{
        &self.current_image
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

    pub fn call_module(&mut self, module_name: &str, args: &str)
    {

    }
}
