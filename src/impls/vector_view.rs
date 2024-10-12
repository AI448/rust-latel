use crate::traits::{SequentialVectorTrait, VectorTrait};

use super::operations::{add_assign_sequential_vector, assign_sequential_vector, sub_assign_sequential_vector};

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
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.dimension
    }
    #[inline(always)]
    fn assign_to_random_vector(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        assign_sequential_vector(lhs, &self);
    }
    #[inline(always)]
    fn add_assign_to_random_vector(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        add_assign_sequential_vector(lhs, &self);
    }
    #[inline(always)]
    fn sub_assign_to_random_vector(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        sub_assign_sequential_vector(lhs, &self);
    }
}

impl<I: Iterator<Item = (usize, f64)> + Clone> SequentialVectorTrait for VectorView<I> {
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.iterator.clone()
    }
}
