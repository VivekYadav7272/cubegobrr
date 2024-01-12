use ndarray::prelude::*;
use turtle::*;

const PERSPECTIVE_ANGLE: f64 = 5.0 * std::f64::consts::PI / 180.0;

pub fn rot_y(angle: f64) -> Array2<f64> {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();

    let rotation_matrix = arr2(&[
        [cos_theta, 0.0, sin_theta],
        [0.0, 1.0, 0.0],
        [-sin_theta, 0.0, cos_theta],
    ]);

    rotation_matrix.t().to_owned()
}

pub fn rot_x(angle: f64) -> Array2<f64> {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();

    let rotation_matrix = arr2(&[
        [1.0, 0.0, 0.0],
        [0.0, cos_theta, -sin_theta],
        [0.0, sin_theta, cos_theta],
    ]);

    rotation_matrix.t().to_owned()
}

pub fn perspective_projection_mat() -> Array2<f64> {
    // instead of doing orthogonal projection, which doesn't look that cool,
    // we'll project from a slight angle from both the axes x and y, so that z looks like it's there.
    // say 5 degrees.

    // I want perspective to look like this:

    /*
    y
    |   z
    |  /
    | /
    | - - - - - x
    */

    // Now, since perspective is the opposite of the rotation the objects should have,
    // (i.e if you want to see from the perspective of the top of the object,
    // you rotate the world *down*, not *up*).
    // Which means,
    // y-axis: rotate clockwise => -ve angle
    // x-axis: rotate counter-clockwise +ve angle

    orthogonal_projection_mat()
        .dot(&rot_y(-PERSPECTIVE_ANGLE))
        .dot(&rot_x(PERSPECTIVE_ANGLE))
}

pub fn orthogonal_projection_mat() -> Array2<f64> {
    arr2(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])
}

pub fn scale_mat(d: &Drawing) -> Array2<f64> {
    let Size { width, height } = d.size();

    let scaling_factor = width.min(height);
    arr2(&[
        [scaling_factor as f64 / 2., 0.0],
        [0.0, scaling_factor as f64 / 2.],
    ])
}

pub fn light_source() -> Array1<f64> {
    rot_y(PERSPECTIVE_ANGLE)
        .dot(&rot_x(-PERSPECTIVE_ANGLE))
        .dot(&arr1(&[0.0, 0.0, 1.0]))
}
