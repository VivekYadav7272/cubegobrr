use crate::{quad::Quad, util::*};
use ndarray::prelude::*;
use turtle::*;

#[derive(Debug, Clone)]
pub struct Cube {
    vertices: Array2<f64>, // 3x8
}

// Cube is expected to be in this order:
// Front face:
// 0   1
// 3   2

// Back face:
// 7  6
// 4  5

impl Cube {
    pub fn new(vertices: [[f64; 8]; 3]) -> Self {
        Self {
            vertices: arr2(&vertices),
        }
    }

    pub fn from(vertices: Array2<f64>) -> Self {
        if vertices.dim() != (3, 8) {
            panic!("Cube::from: vertices must be 3x4");
        }
        Self { vertices }
    }

    pub fn as_array(&self) -> &Array2<f64> {
        &self.vertices
    }

    pub fn draw(&self, d: &Drawing, t: &mut Turtle) {
        let face1 = Quad::from(self.vertices.slice(s![.., 0..4]).to_owned());
        let face2 = Quad::from(self.vertices.slice(s![.., 4..8]).to_owned());
        face1.draw(d, t);
        face2.draw(d, t);

        let projection = perspective_projection_mat().dot(&self.vertices);

        let cube_to_display = scale_mat(d).dot(&projection);

        // now just need to draw connective tissue.
        let connections = [(0, 7), (1, 6), (2, 5), (3, 4)];

        for connection in connections {
            t.pen_up();
            t.go_to((
                cube_to_display[[0, connection.0]],
                cube_to_display[[1, connection.0]],
            ));
            t.pen_down();
            t.go_to((
                cube_to_display[[0, connection.1]],
                cube_to_display[[1, connection.1]],
            ));
        }
    }
}
