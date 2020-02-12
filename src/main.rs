mod image_editor;
mod matrix;
mod module;
mod io;
mod blur;
mod brightness_contrast;
mod canny;
mod crop;
mod laplace;



use crate::image_editor::ImageEditor;

fn main() {
    let mut image_editor = ImageEditor::new();
    let path = "test.jpg";
    image_editor.open_image(path);
    image_editor.save_image_as("test2.jpg")
}
