pub mod cube;
pub mod quad;
pub mod util;

use ndarray::prelude::*;
use std::f64::consts;
use turtle::*;
use util::*;

use crate::cube::Cube;

fn serious_stuff(d: &Drawing, t: &mut Turtle, angle_x: f64, angle_y: f64) {
    let cube = Cube::new([
        [-0.5, 0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, 0.5],
        [-0.5, -0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5],
    ]);

    let rot_mat = rot_x(angle_x * consts::PI / 180.0);
    let rot_mat = rot_mat.dot(&rot_y(angle_y * consts::PI / 180.0));

    let rotated_cube = Cube::from(rot_mat.dot(cube.as_array()));

    t.set_pen_color(colors::BLUE);
    rotated_cube.draw(d, t);
}

fn draw_coordinate_axes(d: &Drawing, t: &mut Turtle) {
    let coordinate_axes = arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let coordinate_axes = scale_mat(&d)
        .dot(&perspective_projection_mat())
        .dot(&coordinate_axes);

    for i in 0..3 {
        t.pen_up();
        t.go_to((0.0, 0.0));
        t.pen_down();
        t.go_to((coordinate_axes[[0, i]], coordinate_axes[[1, i]]));
    }
}

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();
    turtle.set_speed("instant");

    let mut angle_x = 1.5;
    let mut angle_y = 0.15;
    loop {
        turtle.clear();

        turtle.set_pen_color(colors::BLACK);
        draw_coordinate_axes(&drawing, &mut turtle);

        serious_stuff(&drawing, &mut turtle, angle_x, angle_y);

        angle_x = (angle_x + 1.5) % 360.0;
        angle_y = (angle_y + 0.15) % 360.0;

        std::thread::sleep(std::time::Duration::from_millis(15));
    }
}
