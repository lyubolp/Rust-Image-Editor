use std::fs::File;
use std::os::raw::c_int;
use crate::matrix::Matrix;
use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use crate::image_editor::{open_file, get_cols, get_rows, save_file};
use std::ptr::null;
use std::io::{Error, ErrorKind};

pub struct IO {
    opened_path: String
}

impl IO {
    pub fn new() -> Self{
        IO{
            opened_path: String::from("")
        }
    }
    pub fn open(&mut self, path: &str) -> std::io::Result<Matrix> {
        let c_path = CString::new(path).expect("CString::new failed").into_raw();
        let opened_matrix = unsafe{
            open_file(c_path)
        };

        if opened_matrix.is_null(){
            Err(Error::new(ErrorKind::NotFound, "File not found"))
        }
        else{
            self.opened_path = String::from(path);
            unsafe{
                let opened_rows = get_rows(c_path);
                let opened_cols = get_cols(c_path);
                Ok(Matrix::from_memory(opened_matrix, opened_rows, opened_cols))
            }
        }
    }
    fn call_c_save(image: &Matrix, c_path: *mut i8) -> std::io::Result<()>{
        unsafe{
            match save_file(image.to_memory(), c_path, image.height as i32, image.width as i32)
            {
                1 => Ok(()),
                0 => Err(Error::new(ErrorKind::Other, "File could not be saved")),
                _ => Err(Error::new(ErrorKind::Other, "Unknown error"))
            }
        }
    }
    pub fn save(&mut self, image: &Matrix) -> std::io::Result<()> {
        let c_path = CString::new(self.opened_path.as_str()).expect("CString::new failed").into_raw();
        IO::call_c_save(image, c_path)
    }
    pub fn save_as(&mut self, image: &Matrix, path: &str) -> std::io::Result<()> {
        let c_path = CString::new(path).expect("CString::new failed").into_raw();
        IO::call_c_save(image, c_path)
    }

}
