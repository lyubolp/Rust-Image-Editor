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
    let paths = ["tests/images/test_b.png",
        "tests/images/test_a.jpg"];
    
    image_editor.open_image(paths[1]);
    image_editor.call_module("Laplace", "3");
    image_editor.save_image_as("tests/images/result.png");
}
