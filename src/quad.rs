use crate::util::*;
use ndarray::prelude::*;
use turtle::*;

#[derive(Clone, Debug)]
pub struct Quad {
    vertices: Array2<f64>, // 3x4
}

impl Quad {
    pub fn new(vertices: [[f64; 4]; 3]) -> Self {
        Self {
            vertices: arr2(&vertices),
        }
    }

    pub fn from(vertices: Array2<f64>) -> Self {
        if vertices.dim() != (3, 4) {
            panic!("Quad::from: vertices must be 3x4");
        }
        Self { vertices }
    }

    pub fn as_array(&self) -> &Array2<f64> {
        &self.vertices
    }

    pub fn draw(&self, d: &Drawing, t: &mut Turtle) {
        let projection = perspective_projection_mat().dot(&self.vertices);

        let quad_to_display = scale_mat(d).dot(&projection);
        t.pen_up();
        t.go_to((quad_to_display[[0, 0]], quad_to_display[[1, 0]]));

        t.pen_down();
        for i in 1..4 {
            t.go_to((quad_to_display[[0, i]], quad_to_display[[1, i]]));
        }
        t.go_to((quad_to_display[[0, 0]], quad_to_display[[1, 0]]));
        t.pen_up();
    }
}
