use crate::color::Color;
use crate::image::Image;
use crate::matrix::CurveType;
use crate::matrix::Matrix;
use std::f32;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process::Command;

///Goes through the file named filename and performs all of the actions listed in that file.
///The file follows the following format:
///     Every command is a single character that takes up a line
///     Any command that requires arguments must have those arguments in the second line.
///     The commands are as follows:
///
///     push: push a copy of the curent top of the coordinate system stack to the stack
///
///     pop: remove the current top of the coordinate system stack
///
///     All the shape commands work as follows:
///        1) Add the shape to a temporary matrix
///        2) Multiply that matrix by the current top of the coordinate system stack
///        3) Draw the shape to the screen
///        4) Clear the temporary matrix
///
///         sphere: add a sphere to the POLYGON matrix -
///                 takes 4 arguemnts (cx, cy, cz, r)
///         torus: add a torus to the POLYGON matrix -
///                takes 5 arguemnts (cx, cy, cz, r1, r2)
///         box: add a rectangular prism to the POLYGON matrix -
///              takes 6 arguemnts (x, y, z, width, height, depth)
///         clear: clears the edge and POLYGON matrices
///
///         circle: add a circle to the edge matrix -
///                 takes 4 arguments (cx, cy, cz, r)
///         hermite: add a hermite curve to the edge matrix -
///                  takes 8 arguments (x0, y0, x1, y1, rx0, ry0, rx1, ry1)
///         bezier: add a bezier curve to the edge matrix -
///                 takes 8 arguments (x0, y0, x1, y1, x2, y2, x3, y3)
///         line: add a line to the edge matrix -
///               takes 6 arguemnts (x0, y0, z0, x1, y1, z1)
///         ident: set the transform matrix to the identity matrix -
///         scale: create a scale matrix,
///                then multiply the transform matrix by the scale matrix -
///                takes 3 arguments (sx, sy, sz)
///         move: create a translation matrix,
///               then multiply the transform matrix by the translation matrix -
///               takes 3 arguments (tx, ty, tz)
///         rotate: create a rotation matrix,
///                 then multiply the transform matrix by the rotation matrix -
///                 takes 2 arguments (axis, theta) axis should be x y or z
///         apply: apply the current transformation matrix to the edge and
///                POLYGON matrices
///         display: clear the screen, then
///                  draw the lines of the edge and POLYGON matrices to the screen
///                  display the screen
///         save: clear the screen, then
///               draw the lines of the edge and POLYGON matrices to the screen
///               save the screen to a file -
///               takes 1 argument (file name)
///         quit: end parsing
///
///See the file script for an example of the file format
///
///IMPORTANT MATH NOTE:
///the trig functions int math.h use radian measure, but us normal
///humans use degrees, so the file will contain degrees for rotations,
///be sure to convert those degrees to radians (M_PI is the constant
///for PI)
pub fn parse_file(
    fname: &str,
    cstack: &mut Vec<Matrix>,
    points: &mut Matrix,
    polygons: &mut Matrix,
    screen: &mut Image,
    color: Color,
) -> io::Result<()> {
    let file = File::open(&fname)?;
    let reader = BufReader::new(file);
    let mut doc_lines = vec![String::new(); 0];
    let mut i = 0;
    cstack.push(Matrix::identity());

    for line in reader.lines() {
        doc_lines.push(line?);
    }

    while i < doc_lines.len() {
        match &*doc_lines[i] {
            "line" => {
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input.parse().unwrap());
                }
                points.add_edge(
                    params[0], params[1], params[2], params[3], params[4], params[5],
                );
                points.multiply_matrixes(cstack.last().unwrap());
                screen.draw_lines(&points, color);

                *points = Matrix::new(0,0);
            }
            // "ident" => {
            //     transform.identity();
            // }
            "scale" => {
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input.parse().unwrap());
                }

                let mut rot = Matrix::make_scale(params[0], params[1], params[2]);
                rot.multiply_matrixes(&cstack.pop().unwrap());
                cstack.push(rot);
            }
            "translate" | "move" => {
                i += 1;
                let mut params = vec![0; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input.parse().unwrap());
                }

                let mut rot = Matrix::make_translate(params[0], params[1], params[2]);
                rot.multiply_matrixes(&cstack.pop().unwrap());
                cstack.push(rot);
            }
            "rotate" => {
                i += 1;
                let mut params = vec![""; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input);
                }

                match params[0] {
                    "x" => {
                        let mut rot = Matrix::make_rot_x(params[1].parse().unwrap());
                        rot.multiply_matrixes(&cstack.pop().unwrap());
                        cstack.push(rot);
                    }
                    "y" => {
                        let mut rot = Matrix::make_rot_y(params[1].parse().unwrap());
                        rot.multiply_matrixes(&cstack.pop().unwrap());
                        cstack.push(rot);
                    }
                    "z" => {
                        let mut rot = Matrix::make_rot_z(params[1].parse().unwrap());
                        rot.multiply_matrixes(&cstack.pop().unwrap());
                        cstack.push(rot);
                    }
                    _ => {
                        panic!(
                            "Invalid input {} at 0 for rotation: please use x, y, or z.",
                            params[0]
                        );
                    }
                }
            }
            // "apply" => {
            //     if points.matrix_array.len() > 0 {
            //         points.multiply_matrixes(&transform);
            //     }
            //     if polygons.matrix_array.len() > 0 {
            //         polygons.multiply_matrixes(&transform);
            //     }
            // }
            "display" => {
                // screen.clear();
                // if points.matrix_array.len() > 0 {
                //     screen.draw_lines(&points, color);
                // }
                // if polygons.matrix_array.len() > 0 {
                //     screen.draw_polygons(&polygons, color);
                // }
                screen.display();
            }
            "save" => {
                // screen.clear();
                // if points.matrix_array.len() > 0 {
                //     screen.draw_lines(&points, color);
                // }
                // if polygons.matrix_array.len() > 0 {
                //     screen.draw_polygons(&polygons, color);
                // }
                i += 1;
                screen.create_file(&*doc_lines[i]);
                Command::new("magick")
                    .arg("convert")
                    .arg(&*doc_lines[i])
                    .arg(&*doc_lines[i])
                    .spawn()
                    .expect("failed to convert image to desired format");
            }
            "quit" => {
                break;
            }
            "circle" => {
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input.parse().unwrap());
                }

                points.add_circle(params[0], params[1], params[2], params[3], 100);
                points.multiply_matrixes(cstack.last().unwrap());
                screen.draw_lines(&points, color);

                *points = Matrix::new(0,0);
            }
            "hermite" => {
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input.parse().unwrap());
                }

                points.add_curve(
                    params[0],
                    params[1],
                    params[2],
                    params[3],
                    params[4],
                    params[5],
                    params[6],
                    params[7],
                    100,
                    &CurveType::Hermite,
                );
                points.multiply_matrixes(cstack.last().unwrap());
                screen.draw_lines(&points, color);

                *points = Matrix::new(0,0);
            }
            "bezier" => {
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input.parse().unwrap());
                }

                points.add_curve(
                    params[0],
                    params[1],
                    params[2],
                    params[3],
                    params[4],
                    params[5],
                    params[6],
                    params[7],
                    100,
                    &CurveType::Bezier,
                );
                points.multiply_matrixes(cstack.last().unwrap());
                screen.draw_lines(&points, color);

                *points = Matrix::new(0,0);
            }
            _ if doc_lines[i].starts_with('#') => {}
            // "clear" => {
            //     *points = Matrix::new(0, 0);
            //     *polygons = Matrix::new(0, 0);
            // }
            "box" => {
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input.parse().unwrap());
                }

                polygons.add_box(
                    params[0], params[1], params[2], params[3], params[4], params[5],
                );
                polygons.multiply_matrixes(cstack.last().unwrap());
                screen.draw_polygons(&polygons, color);

                *polygons = Matrix::new(0,0);
            }
            "sphere" => {
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input.parse().unwrap());
                }

                polygons.add_sphere(params[0], params[1], params[2], params[3], 20);
                polygons.multiply_matrixes(cstack.last().unwrap());
                screen.draw_polygons(&polygons, color);

                *polygons = Matrix::new(0,0);
            }
            "torus" => {
                i += 1;
                let mut params = vec![0.0; 0];
                for input in doc_lines[i].split(' ') {
                    params.push(input.parse().unwrap());
                }

                polygons.add_torus(params[0], params[1], params[2], params[3], params[4], 20);
                polygons.multiply_matrixes(cstack.last().unwrap());
                screen.draw_polygons(&polygons, color);

                *polygons = Matrix::new(0,0);
            }
            "push" =>{
                cstack.push(cstack.last().unwrap().clone());
            }
            "pop" =>{
                cstack.pop();
            }
            _ => {
                panic!("Invalid command {} at line {}.", doc_lines[i], i + 1);
            }
        }
        i += 1;
    }
    Ok(())
}
