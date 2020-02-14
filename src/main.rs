mod image_editor;
mod matrix;
mod module;
mod io;
mod blur;
mod brightness_contrast;
mod canny;
mod crop;
mod laplace;
mod shapes;



use crate::image_editor::ImageEditor;

fn main() {
    let mut image_editor = ImageEditor::new();
    let path = "tests/images/test_b.png";
    image_editor.open_image(path);
    image_editor.call_module("Crop", "10:10:70:70");
    image_editor.save_image_as("tests/images/result.png");
}
