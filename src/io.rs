use std::fs::File;
use std::os::raw::c_int;
use crate::matrix::Matrix;
use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use crate::{open_file, get_cols, get_rows};
use std::ptr::null;
use std::io::{Error, ErrorKind};

pub struct IO {
}

impl IO {
    pub fn new() -> Self{
        IO{}
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
            unsafe{
                let opened_rows = get_rows(c_path);
                let opened_cols = get_cols(c_path);
                Ok(Matrix::from_memory(opened_matrix, opened_rows, opened_cols))
            }
        }
    }
    pub fn close() -> std::io::Result<()> {
        unimplemented!()
    }
    pub fn save() -> std::io::Result<()> {
        unimplemented!()
    }
    pub fn save_as() -> std::io::Result<()> {
        unimplemented!()
    }

}
