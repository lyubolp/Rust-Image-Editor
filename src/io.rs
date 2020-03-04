use crate::matrix::Matrix;
use std::ffi::{CString};
use crate::image_editor::{open_file, get_cols, get_rows, save_file, get_type};
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
                Ok(Matrix::from_memory(opened_matrix, opened_rows, opened_cols, img_type))
            }
        }
    }
    fn call_c_save(image: &Matrix, c_path: *mut i8) -> std::io::Result<()>{
        unsafe{
            match save_file(image.to_memory(), c_path, image.rows as i32,
                            image.cols as i32, image.image_type)
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
    use crate::ImageEditor;

    #[test]
    fn io_basic()
    {
        let mut image_editor = ImageEditor::new();
        let path = "tests/images/test_b.png";
        let path2 = "tests/images/test_a.jpg";

        image_editor.open_image(path);
        assert_eq!(image_editor.get_image().rows, 100);
        assert_eq!(image_editor.get_image().cols, 100);

        image_editor.open_image(path2);
        assert_eq!(image_editor.get_image().rows, 1200);
        assert_eq!(image_editor.get_image().cols, 1800);

    }
}