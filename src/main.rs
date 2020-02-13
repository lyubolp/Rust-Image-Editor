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
    image_editor.call_module("Draw shape", "c:#000000:-1:50:10:10");
    image_editor.save_image_as("tests/images/test_a2.png");
}
