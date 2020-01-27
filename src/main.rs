mod matrix;
mod module;
mod io;
mod contrast;
mod blur;
mod brightness;
mod canny;
mod crop;
mod laplace;

extern crate libc;

use crate::matrix::Matrix;
use crate::module::Module;
use crate::io::IO;
use crate::contrast::Contrast;
use crate::blur::Blur;
use crate::brightness::Brightness;
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


fn main() {
    #[link(name = "core_cpp", kind = "static")]
    extern "C" {
        pub fn open_file(path: *const c_char) -> *mut c_int;
        pub fn print(to_write: *const c_char);
        pub fn read() -> *mut c_char;
        pub fn clear_memory(memory: *mut c_char);
    }

    unsafe {
        let opened_matrix = open_file(CString::new("/home/lyubo/Uni/RustProject/Rust-Image-Editor/test.jpg").unwrap().as_ptr());
        //let result = Vec::from_raw_parts(opened_matrix, 2160000, 2160010);
        //println!("{:?}", result);
    };

}
