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

        // both are unit vectors.
        let light = light_source();
        let normal = self.get_normal();

        let shine = light.dot(&normal);
        // from here on out, shine ranges from -1 to 0. The smaller the magnitude (i.e the closer towards 0),
        // the darker the blue hue should be.
        // so, we need to scale this value from 0 to 255 in blue channel.

        let shadow_blue_hue = 30.;

        if shine < 0. {
            let blue = (255. - shadow_blue_hue) * -shine + shadow_blue_hue;
            t.set_fill_color(Color::rgb(0., 0., blue));
            t.begin_fill();
        }
        for i in 1..4 {
            t.go_to((quad_to_display[[0, i]], quad_to_display[[1, i]]));
        }
        t.go_to((quad_to_display[[0, 0]], quad_to_display[[1, 0]]));
        t.pen_up();

        if shine < 0. {
            t.end_fill();
        }
    }

    pub fn get_normal(&self) -> Array1<f64> {
        // I don't know how to use un-owned versions :(
        let vert1 = self.vertices.column(0).to_owned();
        let vert2 = self.vertices.column(1).to_owned();
        let vert3 = self.vertices.column(2).to_owned();

        let v1 = &vert2 - vert1;
        let v2 = vert3 - vert2;

        // cross product them.
        let i = v1[1] * v2[2] - v1[2] * v2[1];
        let j = v1[2] * v2[0] - v1[0] * v2[2];
        let k = v1[0] * v2[1] - v1[1] * v2[0];

        let magnitude = (i * i + j * j + k * k).sqrt();
        arr1(&[i / magnitude, j / magnitude, k / magnitude])
    }
}
