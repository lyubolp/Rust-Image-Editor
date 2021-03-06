use crate::matrix::Matrix;
use crate::module::Module;
use crate::io::IO;
use crate::blur::Blur;
use crate::brightness_contrast::Brightness;
use crate::brightness_contrast::Contrast;
use crate::canny::Canny;
use crate::crop::Crop;
use crate::laplace::Laplace;
use crate::shapes::DrawShape;

extern crate libc;

use libc::c_int;
use std::os::raw::{c_char, c_double};

pub struct ImageEditor {
    modules: Vec<(String, Box<dyn Module>)>,
    pub current_image: Matrix,
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
    pub fn draw_circle(image: *const c_int, rows: c_int, cols: c_int, image_type: c_int,
                       center_row: c_int, center_col: c_int, radius: c_int, color: c_int, thickness: c_int) -> *mut c_int;
    pub fn draw_rectangle(image: *const c_int, rows: c_int, cols: c_int, image_type: c_int,
                          first_row: c_int, first_col: c_int, second_row: c_int, second_col: c_int,
                          color: c_int, thickness: c_int) -> *mut c_int;
    pub fn draw_line(image: *const c_int, rows: c_int, cols: c_int, image_type: c_int,
                     first_row: c_int, first_col: c_int, second_row: c_int, second_col: c_int,
                     color: c_int, thickness: c_int) -> *mut c_int;

    pub fn crop_c(image: *const c_int, original_rows: c_int, original_cols: c_int, image_type: c_int,
                  row_start: c_int, col_start: c_int, new_rows: c_int, new_cols: c_int) -> *mut c_int;
    pub fn laplace_c(image: *const c_int, rows: c_int, cols: c_int, image_type: c_int,
                     kernel_size: c_int) -> *mut c_int;

    pub fn blur_c(image: *const c_int, rows: c_int, cols: c_int, image_type: c_int, kernel_size: c_int) -> *mut c_int;
}


impl ImageEditor {
    pub fn new() -> Self {
        ImageEditor {
            modules: vec![(String::from("Contrast"), Box::new(Contrast)),
                          (String::from("Blur"), Box::new(Blur)),
                          (String::from("Brightness"), Box::new(Brightness)),
                          (String::from("Canny"), Box::new(Canny)),
                          (String::from("Crop"), Box::new(Crop)),
                          (String::from("Laplace"), Box::new(Laplace)),
                          (String::from("Draw shape"), Box::new(DrawShape))
            ],
            current_image: Matrix::new(),
            io: IO::new(),
        }
    }
    //For testing purposes
    pub fn get_image(&self) -> &Matrix {
        &self.current_image
    }
    pub fn open_image(&mut self, path: &str) {
        match self.io.open(path) {
            Ok(image) => {
                self.current_image = image;
                println!("File opened successfully !");
            }
            Err(e) => println!("Unable to load image - error trace: {}", e)
        }
    }

    pub fn save_image(&mut self)
    {
        match self.io.save(&self.current_image) {
            Ok(_) => println!("File saved successfully"),
            Err(e) => println!("Unable to save image - error trace: {}", e)
        }
    }
    pub fn save_image_as(&mut self, path: &str)
    {
        match self.io.save_as(&self.current_image, path) {
            Ok(_) => println!("File saved successfully"),
            Err(e) => println!("Unable to save image - error trace: {}", e)
        }
    }

    pub fn call_module(&mut self, module_name: &str, args: &str)
    {
        for mut pair in self.modules.iter()
        {
            if *pair.0 == String::from(module_name) {
                let module = &pair.1;
                self.current_image = module.exec(&self.current_image, args);
                break;
            }
        }
    }
}
