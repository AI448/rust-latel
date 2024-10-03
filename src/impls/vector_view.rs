use crate::traits::{SequentialVectorTrait, VectorTrait};

#[derive(Clone, Debug)]
pub struct VectorView<I: Iterator<Item = (usize, f64)> + Clone> {
    dimension: usize,
    iterator: I,
}

impl<I: Iterator<Item = (usize, f64)> + Clone> VectorView<I> {
    pub fn new(dimension: usize, nonzero_elements: I) -> Self {
        Self { dimension: dimension, iterator: nonzero_elements }
    }
}

impl<I: Iterator<Item = (usize, f64)> + Clone> VectorTrait for VectorView<I> {
    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl<I: Iterator<Item = (usize, f64)> + Clone> SequentialVectorTrait for VectorView<I> {
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.iterator.clone()
    }
}
