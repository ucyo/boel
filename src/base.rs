use crate::shape::Shape;
use crate::stride::Stride;

#[derive(Debug, Default)]
pub struct Base<'a, T: 'a> {
    v: &'a [T],
    stride: Stride,
}

impl<'a, T: 'a> Base<'a, T> {
    pub fn new(v: &'a [T]) -> Self {
        Self { v, stride: 1 }
    }
    pub fn with_shape(v: &'a [T], shape: Shape) -> Self {
        Self { v, stride: shape.y }
    }
}
