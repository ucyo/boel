use std::convert::From;

pub struct Shape {
    pub x: usize,
    pub y: usize,
}

impl Shape {
    pub fn with_1_dimension(shape: usize) -> Self {
        Shape::from(shape)
    }
    pub fn with_2_dimensions(shape: (usize, usize)) -> Self {
        Shape::from(shape)
    }
}

impl From<usize> for Shape {
    fn from(length: usize) -> Self {
        Self { x: length, y: 1 }
    }
}

impl From<(usize, usize)> for Shape {
    fn from(length: (usize, usize)) -> Self {
        Self {
            x: length.0,
            y: length.1,
        }
    }
}
