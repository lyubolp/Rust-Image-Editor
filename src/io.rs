use std::fs::File;
use std::os::raw::c_int;

//extern "C"{
//    fn test(a: c_int, b: c_int) -> c_int;
//}
pub struct IO {
    pub opened_file: Option<File>,
    path: String,
}

impl IO {
    pub fn new() -> Self{
        IO{
            opened_file: None,
            path: String::from("")
        }
    }
    pub fn open(&mut self, path: &str) -> std::io::Result<()> {
        unimplemented!()
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

//    pub fn test_c()
//    {
//        let res = unsafe { test(3, 5)};
//        println!("{}", res);
//    }
}
