use ImageEditor;

mod tests {
    use sha1::{Sha1, Digest};
    use std::{fs, io};

    #[test]
    fn io_basic()
    {
        let mut image_editor = ImageEditor::new();
        let path = "test.jpg";
        image_editor.open_image(path);
        image_editor.save_image_as("test2.jpg");

        let mut file = match fs::File::open("test.jpg")
        {
            Ok(f) => f,
            Err(_) => panic!("Cannot find test files")
        };
        let mut file2 = match fs::File::open("test2.jpg")
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

        n = match io::copy(&mut file2, &mut hasher){
            Ok(val) => val,
            Err(E) => panic!("Cannot copy to hasher - {}", E)
        };

        let hash2 = hasher.result();


        assert_eq!(hash, hash2);
    }
}