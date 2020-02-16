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
use gtk::prelude::*;
use gio::prelude::*;
use gdk_pixbuf::Pixbuf;
use std::ffi::OsStr;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::main_quit;

fn update_view(editor: &mut ImageEditor, viewer: &gtk::Image)
{
    editor.save_image_as("/tmp/tmp1.png");
    let pixbuf = Pixbuf::new_from_file_at_scale(
        "/tmp/tmp1.png",
        950,
        520,
        true).expect("Can't get pixbuf");
    viewer.set_from_pixbuf(Some(&pixbuf));
}

fn main() {
    let mut image_editor = ImageEditor::new();
    let image_editor_rc = Rc::new(RefCell::new(image_editor));
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("builder_basics.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window_main").unwrap();

    let open_button: gtk::Button = builder.get_object("button_open").unwrap();
    let save_button: gtk::Button = builder.get_object("button_save").unwrap();
    let save_as_button: gtk::Button = builder.get_object("button_save_as").unwrap();
    let brightness_button: gtk::Button = builder.get_object("button_brightness").unwrap();
    let contrast_button: gtk::Button = builder.get_object("button_contrast").unwrap();
    let line_button: gtk::Button = builder.get_object("button_line").unwrap();
    let circle_button: gtk::Button = builder.get_object("button_circle").unwrap();
    let rectangle_button: gtk::Button = builder.get_object("button_rectangle").unwrap();
    let crop_button: gtk::Button = builder.get_object("button_crop").unwrap();
    let laplace_button: gtk::Button = builder.get_object("button_laplace").unwrap();
    let blur_button: gtk::Button = builder.get_object("button_blur").unwrap();

    let open_file_dialog: gtk::FileChooserDialog = builder.get_object("fileopendialog").unwrap();
    let save_as_file_dialog: gtk::FileChooserDialog = builder.get_object("filesavedialog").unwrap();
    let image_viewer: gtk::Image = builder.get_object("image").unwrap();

    let image_viewer_rc = Rc::new(RefCell::new(image_viewer));

    let brightness_dialog: gtk::Dialog = builder.get_object("scrollbarbrightnessdialog").unwrap();
    let contrast_dialog: gtk::Dialog = builder.get_object("scrollbarcontrastdialog").unwrap();
    let laplace_dialog: gtk::Dialog = builder.get_object("scrollbarlaplacedialog").unwrap();
    let line_dialog: gtk::Dialog = builder.get_object("linedialog").unwrap();
    let circle_dialog: gtk::Dialog = builder.get_object("circledialog").unwrap();
    let rectangle_dialog: gtk::Dialog = builder.get_object("rectangledialog").unwrap();
    let crop_dialog: gtk::Dialog = builder.get_object("cropdialog").unwrap();
    let blur_dialog: gtk::Dialog = builder.get_object("blurdialog").unwrap();

    let invalid_argument: gtk::Dialog = builder.get_object("invalid_arguments_dialog").unwrap();
    let invalid_argument_rc = Rc::new(RefCell::new(invalid_argument));

    let brightness_slider: gtk::Scale = builder.get_object("value_slider").unwrap();
    let contrast_slider: gtk::Scale = builder.get_object("value_slider_contrast").unwrap();
    let laplace_slider: gtk::Scale = builder.get_object("value_slider_laplace").unwrap();
    let blur_slider: gtk::Scale = builder.get_object("value_slider_blur").unwrap();

    let line_x1: gtk::Entry = builder.get_object("x1_l").unwrap();
    let line_y1: gtk::Entry = builder.get_object("y1_l").unwrap();
    let line_x2: gtk::Entry = builder.get_object("x2_l").unwrap();
    let line_y2: gtk::Entry = builder.get_object("y2_l").unwrap();
    let line_color: gtk::Entry = builder.get_object("color_l").unwrap();

    let rectangle_x1: gtk::Entry = builder.get_object("x1_r").unwrap();
    let rectangle_y1: gtk::Entry = builder.get_object("y1_r").unwrap();
    let rectangle_x2: gtk::Entry = builder.get_object("x2_r").unwrap();
    let rectangle_y2: gtk::Entry = builder.get_object("y2_r").unwrap();
    let rectangle_color: gtk::Entry = builder.get_object("color_r").unwrap();

    let circle_x1: gtk::Entry = builder.get_object("x1_c").unwrap();
    let circle_y1: gtk::Entry = builder.get_object("y1_c").unwrap();
    let circle_r: gtk::Entry = builder.get_object("r_c").unwrap();
    let circle_color: gtk::Entry = builder.get_object("color_c").unwrap();

    let crop_x1: gtk::Entry = builder.get_object("x1_crop").unwrap();
    let crop_y1: gtk::Entry = builder.get_object("y1_crop").unwrap();
    let crop_x2: gtk::Entry = builder.get_object("x2_crop").unwrap();
    let crop_y2: gtk::Entry = builder.get_object("y2_crop").unwrap();


    open_file_dialog.add_buttons(&[
        ("Open", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    save_as_file_dialog.add_buttons(&[
        ("Save", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    brightness_dialog.add_buttons(&[
        ("Apply", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    contrast_dialog.add_buttons(&[
        ("Apply", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    laplace_dialog.add_buttons(&[
        ("Apply", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    rectangle_dialog.add_buttons(&[
        ("Apply", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    circle_dialog.add_buttons(&[
        ("Apply", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    line_dialog.add_buttons(&[
        ("Apply", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    crop_dialog.add_buttons(&[
        ("Apply", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    blur_dialog.add_buttons(&[
        ("Apply", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    let mut image_editor_access_open_button = image_editor_rc.clone();
    let image_viewer_access_open_button = image_viewer_rc.clone();
    open_button.connect_clicked(move |_| {
        if open_file_dialog.run() == gtk::ResponseType::Ok {
            let filename = open_file_dialog.get_filename().expect("Couldn't get filename");
            image_editor_access_open_button.borrow_mut().open_image(filename.to_str().unwrap());
            update_view(&mut image_editor_access_open_button.borrow_mut(), &image_viewer_access_open_button.borrow_mut());
        }
        open_file_dialog.hide();
    });

    let mut image_editor_access_save_button = image_editor_rc.clone();
    save_button.connect_clicked(move |_| {
        image_editor_access_save_button.borrow_mut().save_image();
    });

    let mut image_editor_access_save_as_button = image_editor_rc.clone();
    save_as_button.connect_clicked(move |_| {
        if save_as_file_dialog.run() == gtk::ResponseType::Ok {
            let filename = save_as_file_dialog.get_filename().expect("Couldn't get filename");
            image_editor_access_save_as_button.borrow_mut().save_image_as(filename.to_str().unwrap());
        }
        save_as_file_dialog.hide();
    });

    let mut image_editor_access_brightness = image_editor_rc.clone();
    let image_viewer_access_brightness = image_viewer_rc.clone();
    brightness_button.connect_clicked(move |_| {
        if brightness_dialog.run() == gtk::ResponseType::Ok {
            let slider_value = brightness_slider.get_value() as i32;
            image_editor_access_brightness.borrow_mut().call_module("Brightness", &slider_value.to_string());
            update_view(&mut image_editor_access_brightness.borrow_mut(), &image_viewer_access_brightness.borrow_mut());
        }
        brightness_dialog.hide();
    });

    let mut image_editor_access_contrast = image_editor_rc.clone();
    let image_viewer_access_contrast = image_viewer_rc.clone();
    contrast_button.connect_clicked(move |_| {
        if contrast_dialog.run() == gtk::ResponseType::Ok {
            let slider_value = contrast_slider.get_value() as i32;
            image_editor_access_contrast.borrow_mut().call_module("Contrast", &slider_value.to_string());
            update_view(&mut image_editor_access_contrast.borrow_mut(), &image_viewer_access_contrast.borrow_mut());
        }
        contrast_dialog.hide();
    });

    let mut image_editor_access_laplace = image_editor_rc.clone();
    let image_viewer_access_laplace = image_viewer_rc.clone();
    laplace_button.connect_clicked(move |_| {
        if laplace_dialog.run() == gtk::ResponseType::Ok {
            let slider_value = laplace_slider.get_value() as i32;
            image_editor_access_laplace.borrow_mut().call_module("Laplace", &slider_value.to_string());
            update_view(&mut image_editor_access_laplace.borrow_mut(), &image_viewer_access_laplace.borrow_mut());
        }
        laplace_dialog.hide();
    });


    let mut image_editor_access_rectangle = image_editor_rc.clone();
    let image_viewer_access_rectangle = image_viewer_rc.clone();
    let invalid_argument_access_rectangle = invalid_argument_rc.clone();
    rectangle_button.connect_clicked(move |_| {
        let mut is_data_valid: bool = true;
        if rectangle_dialog.run() == gtk::ResponseType::Ok {
            let x1 = match rectangle_x1.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let y1 = match rectangle_y1.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let x2 = match rectangle_x2.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let y2 = match rectangle_y2.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let color = rectangle_color.get_text().unwrap().to_string();

            if color.len() == 7 {
                if color.chars().nth(0) != Some('#') {
                    is_data_valid = false;
                }
                let mut iter = color.chars();
                iter.next();
                for current_char in iter {
                    if (current_char > 'F' || current_char < 'A') && (current_char < '0' || current_char > '9')
                    {
                        is_data_valid = false;
                    }
                }
            } else {
                is_data_valid = false;
            }
            if is_data_valid {
                let mut arg = String::from("r:");
                arg.push_str(color.as_str());
                arg.push_str(":-1:");
                arg.push_str(&x1.to_string());
                arg.push_str(":");
                arg.push_str(&y1.to_string());
                arg.push_str(":");
                arg.push_str(&x2.to_string());
                arg.push_str(":");
                arg.push_str(&y2.to_string());
                image_editor_access_rectangle.borrow_mut().call_module("Draw shape", &arg);
                update_view(&mut image_editor_access_rectangle.borrow_mut(), &image_viewer_access_rectangle.borrow_mut());
            } else {
                invalid_argument_access_rectangle.borrow_mut().run();
                invalid_argument_access_rectangle.borrow_mut().hide();
                rectangle_dialog.hide();
            }
        }
        rectangle_dialog.hide();
    });

    let mut image_editor_access_circle = image_editor_rc.clone();
    let image_viewer_access_circle = image_viewer_rc.clone();
    let invalid_argument_access_circle = invalid_argument_rc.clone();
    circle_button.connect_clicked(move |_| {
        let mut is_data_valid: bool = true;
        if circle_dialog.run() == gtk::ResponseType::Ok {
            let x1 = match circle_x1.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let y1 = match circle_y1.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let radius = match circle_r.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let color = circle_color.get_text().unwrap().to_string();

            if color.len() == 7 {
                if color.chars().nth(0) != Some('#') {
                    is_data_valid = false;
                }
                let mut iter = color.chars();
                iter.next();
                for current_char in iter {
                    if (current_char > 'F' || current_char < 'A') && (current_char < '0' || current_char > '9')
                    {
                        is_data_valid = false;
                    }
                }
            } else {
                is_data_valid = false;
            }
            if is_data_valid {
                let mut arg = String::from("c:");
                arg.push_str(color.as_str());
                arg.push_str(":-1:");
                arg.push_str(&x1.to_string());
                arg.push_str(":");
                arg.push_str(&y1.to_string());
                arg.push_str(":");
                arg.push_str(&radius.to_string());
                image_editor_access_circle.borrow_mut().call_module("Draw shape", &arg);
                update_view(&mut image_editor_access_circle.borrow_mut(), &image_viewer_access_circle.borrow_mut());
            } else {
                invalid_argument_access_circle.borrow_mut().run();
                invalid_argument_access_circle.borrow_mut().hide();
                circle_dialog.hide();
            }
        }
        circle_dialog.hide();
    });

    let mut image_editor_access_line = image_editor_rc.clone();
    let image_viewer_access_line = image_viewer_rc.clone();
    let invalid_argument_access_line = invalid_argument_rc.clone();
    line_button.connect_clicked(move |_| {
        let mut is_data_valid: bool = true;
        if line_dialog.run() == gtk::ResponseType::Ok {
            let x1 = match line_x1.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let y1 = match line_y1.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let x2 = match line_x2.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let y2 = match line_y2.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let color = line_color.get_text().unwrap().to_string();

            if color.len() == 7 {
                if color.chars().nth(0) != Some('#') {
                    is_data_valid = false;
                }
                let mut iter = color.chars();
                iter.next();
                for current_char in iter {
                    if (current_char > 'F' || current_char < 'A') && (current_char < '0' || current_char > '9')
                    {
                        is_data_valid = false;
                    }
                }
            } else {
                is_data_valid = false;
            }
            if is_data_valid {
                let mut arg = String::from("l:");
                arg.push_str(color.as_str());
                arg.push_str(":10:");
                arg.push_str(&x1.to_string());
                arg.push_str(":");
                arg.push_str(&y1.to_string());
                arg.push_str(":");
                arg.push_str(&x2.to_string());
                arg.push_str(":");
                arg.push_str(&y2.to_string());
                image_editor_access_line.borrow_mut().call_module("Draw shape", &arg);
                update_view(&mut image_editor_access_line.borrow_mut(), &image_viewer_access_line.borrow_mut());
            } else {
                invalid_argument_access_line.borrow_mut().run();
                invalid_argument_access_line.borrow_mut().hide();
                line_dialog.hide();
            }
        }
        line_dialog.hide();
    });

    let mut image_editor_access_crop = image_editor_rc.clone();
    let image_viewer_access_crop = image_viewer_rc.clone();
    let invalid_argument_access_crop = invalid_argument_rc.clone();
    crop_button.connect_clicked(move |_| {
        let mut is_data_valid: bool = true;
        if crop_dialog.run() == gtk::ResponseType::Ok {
            let x1 = match crop_x1.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let y1 = match crop_y1.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let x2 = match crop_x2.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };
            let y2 = match crop_y2.get_text().unwrap().parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    is_data_valid = false;
                    0
                }
            };

            if is_data_valid {
                let mut arg = String::from(&x1.to_string());
                arg.push_str(":");
                arg.push_str(&y1.to_string());
                arg.push_str(":");
                arg.push_str(&x2.to_string());
                arg.push_str(":");
                arg.push_str(&y2.to_string());

                image_editor_access_crop.borrow_mut().call_module("Crop", &arg);
                update_view(&mut image_editor_access_crop.borrow_mut(), &image_viewer_access_crop.borrow_mut());
            } else {
                invalid_argument_access_crop.borrow_mut().run();
                invalid_argument_access_crop.borrow_mut().hide();
                crop_dialog.hide();
            }
        }
        crop_dialog.hide();
    });

    let mut image_editor_access_blur = image_editor_rc.clone();
    let image_viewer_access_blur = image_viewer_rc.clone();
    blur_button.connect_clicked(move |_| {
        if blur_dialog.run() == gtk::ResponseType::Ok {
            let slider_value = blur_slider.get_value() as i32;
            update_view(&mut image_editor_access_blur.borrow_mut(), &image_viewer_access_blur.borrow_mut());
            image_editor_access_blur.borrow_mut().call_module("Blur", &slider_value.to_string());
        }

        blur_dialog.hide();
    });

    window.connect_delete_event(move |_, _| {
        main_quit();
        Inhibit(false)
    });
    window.show_all();
    gtk::main();
}
