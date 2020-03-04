use crate::module::Module;
use crate::matrix::Matrix;
use crate::image_editor::{draw_circle, draw_line, draw_rectangle};

pub struct DrawShape;

impl Module for DrawShape {
    fn exec(&self, image: &Matrix, args: &str) -> Matrix where Self: Sized {
        println!("1");
        /*
        shape:color_hex:thickness:first_point_col:first_point_row:second_point_col:second_point_row

        shape - {c - (circle), r (rectangle), l (line)}
        color_hex - #RRGGBB
        first_point_row ->
        */
        let rows = image.rows as i32;
        let cols = image.cols as i32;
        let image_type = image.image_type;


        let args_vec: Vec<&str> = args.split(":").collect();
        if args_vec.len() < 6 {
            panic!("Invalid arguments");
            //return image;
        }
        let shape = args_vec[0];
        let color = {

            (i32::from_str_radix(&args_vec[1][1..3], 16).unwrap() * 256 * 256) +
                i32::from_str_radix(&args_vec[1][3..5], 16).unwrap() * 256 +
                i32::from_str_radix(&args_vec[1][5..7], 16).unwrap()
        };
        let thickness = match args_vec[2].parse::<i32>(){
            Ok(val) => val,
            Err(_) => panic!("Invalid argument")
        };
        let first_row = match args_vec[3].parse::<i32>(){
            Ok(val) => val,
            Err(_) => panic!("Invalid argument")
        };
        let first_column = match args_vec[4].parse::<i32>(){
            Ok(val) => val,
            Err(_) => panic!("Invalid argument")
        };

        if shape == "r" || shape == "l" {
            let second_row = match args_vec[5].parse::<i32>(){
                Ok(val) => val,
                Err(_) => panic!("Invalid argument")
            };
            let second_col = match args_vec[6].parse::<i32>(){
                Ok(t) => t,
                Err(_) => panic!("Invalid argument")
            };

            if shape == "r" {
                unsafe {
                    let memory = draw_rectangle(image.to_memory(),
                                           rows, cols, image.image_type,
                                           first_row, first_column,
                                           second_row, second_col, color, thickness);
                    Matrix::from_memory(memory, rows, cols, image_type)
                }
            } else if shape == "l" {
                println!("2");
                unsafe {
                    let memory = draw_line(image.to_memory(),
                                             rows, cols, image.image_type,
                                             first_row, first_column,
                                             second_row, second_col, color, thickness);
                    println!("3");
                    Matrix::from_memory(memory, rows, cols, image_type)
                }
            }
            else{
                Matrix::new()
            }
        } else {
            let radius = match args_vec[5].parse::<i32>(){
                Ok(val) => val,
                Err(_) => panic!("Invalid argument")
            };
            unsafe {
                let memory = draw_circle(image.to_memory(),
                                         rows, cols, image.image_type,
                                         first_row, first_column,
                                         radius, color, thickness);
                Matrix::from_memory(memory, rows, cols, image_type)
            }
        }
    }
}