pub type Stride = usize;
pub type Shape = (usize, usize);

#[derive(Debug, Default)]
pub struct Base<'a, T: 'a> {
    v: &'a [T],
    stride: Stride,
}

impl<'a, T: 'a> Base<'a, T> {
    pub fn new(v: &'a [T]) -> Self {
        Self { v, stride: 1 }
    }
    pub fn with_stride(v: &'a [T], stride: usize) -> Self {
        Self { v, stride }
    }
}
