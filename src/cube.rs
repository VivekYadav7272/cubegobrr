use crate::quad::Quad;
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
        // HACK: Currently, the order of points in each face is very important,
        // as it decides the orientation of the normal vector (cuz it is
        // dependent on vert2 - vert1, vert3 - vert2)
        // so each face must be defined in anti-clockwise fashion,
        // so normal vector points away from cube for each face.
        // OR, so I thought :)) Now either my code for calculating normal vectors is flawed,
        // or something else is wrong, but instead they need to be ordered such that
        // a sane person would say the normal vector points inside the cube :))))))

        let faces = [
            [0, 1, 2, 3],
            [4, 5, 6, 7],
            [3, 4, 7, 0],
            [1, 6, 5, 2],
            [7, 6, 1, 0],
            [5, 4, 3, 2],
        ];

        for face in &faces {
            let quad = Quad::from(self.vertices.select(Axis(1), face));
            quad.draw(d, t);
        }
    }
}
