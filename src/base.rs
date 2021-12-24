use crate::shape::Shape;
use ndarray::{Array, Ix1, Ix2};

pub type Data<T> = Vec<T>;
#[derive(Debug)]
pub struct Base<T> {
    v: Data<T>,
    shape: Shape,
}

impl<T> Base<T> {
    pub fn new(v: Data<T>, shape: Shape) -> Self {
        Self { v, shape }
    }
}

impl<T> From<Base<T>> for Array<T, Ix2> {
    fn from(base: Base<T>) -> Self {
        match base.shape {
            Shape::D2(s) => Array::from_shape_vec(s, base.v).unwrap(),
            // Shape::D1(s) => Array::from_vec(base.v),
            _ => panic!("Expected D2 shape, got {:?}", base.shape),
        }
    }
}

impl<T> From<Base<T>> for Array<T, Ix1> {
    fn from(base: Base<T>) -> Self {
        match base.shape {
            Shape::D1(_) => Array::from_vec(base.v),
            _ => panic!("Expected D1 shape, got {:?}", base.shape),
        }
    }
}
