use color::Color;
use image::Image;
use matrix::CurveType;
use matrix::Matrix;
use parser::parse_file;
use std::env;
mod color;
mod draw;
mod image;
mod matrix;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut screen = Image::new(500, 500);
    let color = Color::new_color(0, 255, 0);
    let mut edges = Matrix::new(0, 0);
    let mut polygons = Matrix::new(0, 0);
    let mut transform = Matrix::new(4, 4);
    if args.len() > 1 && args[1] == "art" {
        parse_file(
            "macprowheels",
            &mut edges,
            &mut polygons,
            &mut transform,
            &mut screen,
            color,
        );
    } else {
        parse_file(
            "script",
            &mut edges,
            &mut polygons,
            &mut transform,
            &mut screen,
            color,
        );
    }
}
