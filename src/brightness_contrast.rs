use crate::module::Module;
use crate::matrix::Matrix;

pub struct Brightness;

pub struct Contrast;

extern crate libc;

use libc::{c_int, c_double};
use crate::image_editor::{change_brightness, change_contrast};

impl Module for Brightness {
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self: Sized {
        let rows = image.rows as i32;
        let cols = image.cols as i32;
        let image_type = image.image_type;
        let brightness = match args.parse::<i32>()
        {
            Ok(val) => val,
            Err(e) => {
                println!("Argument invalid, setting it to 0");
                0
            }
        };
        unsafe {
            let memory = change_brightness(image.to_memory(), rows, cols, image.image_type, brightness);
            Matrix::from_memory(memory, rows, cols, image_type)
        }
    }
}

impl Module for Contrast {
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self: Sized {
        let rows = image.rows as i32;
        let cols = image.cols as i32;
        let image_type = image.image_type;
        unsafe {
            let contrast = match args.parse::<f64>()
            {
                Ok(val) => val,
                Err(e) => {
                    println!("Argument invalid, setting it to 0");
                    0f64
                }
            };
            let memory = change_contrast(image.to_memory(), rows, cols, image.image_type, contrast);
            Matrix::from_memory(memory, rows, cols, image_type)
        }
    }
}