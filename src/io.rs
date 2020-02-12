use std::fs::File;
use std::os::raw::c_int;
use crate::matrix::Matrix;
use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use crate::image_editor::{open_file, get_cols, get_rows, save_file, ImageEditor, get_type};
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
                let img_type = get_type(c_path);
                println!("img_type: {}", img_type);
                Ok(Matrix::from_memory(opened_matrix, opened_rows, opened_cols, img_type))
            }
        }
    }
    fn call_c_save(image: &Matrix, c_path: *mut i8) -> std::io::Result<()>{
        unsafe{
            match save_file(image.to_memory(), c_path, image.height as i32,
                            image.width as i32, image.image_type)
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

#[cfg(test)]
mod tests {
    use sha1::{Sha1, Digest};
    use std::{fs, io};
    use crate::ImageEditor;

    #[test]
    fn io_basic()
    {
        let mut image_editor = ImageEditor::new();
        let path = "tests/images/test.jpg";
        let path2 = "tests/images/test2.jpg";
        image_editor.open_image(path);
        image_editor.save_image_as(path2);

        let mut file = match fs::File::open(path)
        {
            Ok(f) => f,
            Err(_) => panic!("Cannot find test files")
        };
        let mut file2 = match fs::File::open(path2)
        {
            Ok(f) => f,
            Err(_) => panic!("Cannot find test files")
        };

        let mut hasher = Sha1::new();
        let mut n = match io::copy(&mut file, &mut hasher){
            Ok(val) => val,
            Err(E) => panic!("Cannot copy to hasher - {}", E)
        };

        let hash = hasher.result();

        let mut hasher2 = Sha1::new();
        n = match io::copy(&mut file2, &mut hasher2){
            Ok(val) => val,
            Err(E) => panic!("Cannot copy to hasher - {}", E)
        };

        let hash2 = hasher2.result();
        assert_eq!(hash == hash2, true);
    }
}