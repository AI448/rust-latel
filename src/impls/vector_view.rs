use crate::traits::{SequentialVectorTrait, VectorTrait};

use super::operations::{add_assign_sequential_vector, assign_sequential_vector, sub_assign_sequential_vector};

#[derive(Clone, Debug)]
pub struct VectorView<I: Iterator<Item = (usize, f64)> + Clone> {
    dimension: usize,
    iterator: Option<I>,
}

impl<I: Iterator<Item = (usize, f64)> + Clone> VectorView<I> {
    #[inline(always)]
    pub fn new(dimension: usize, nonzero_elements: I) -> Self {
        Self { dimension: dimension, iterator: Some(nonzero_elements) }
    }

    #[inline(always)]
    pub fn new_as_empry(dimension: usize) -> Self {
        Self {dimension: dimension, iterator: None}
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
    fn add_to_random_vector(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        add_assign_sequential_vector(lhs, &self);
    }
    #[inline(always)]
    fn sub_from_random_vector(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        sub_assign_sequential_vector(lhs, &self);
    }
}

impl<I: Iterator<Item = (usize, f64)> + Clone> SequentialVectorTrait for VectorView<I> {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        Iter {iterator: self.iterator.clone()}
        // self.iterator.clone().iter()
    }
}

#[derive(Clone, Debug)]
struct Iter<I: Iterator<Item = (usize, f64)> + Clone> {
    iterator: Option<I>
}

impl<I: Iterator<Item = (usize, f64)> + Clone> Iterator for Iter<I> {
    type Item = (usize, f64);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iterator {
            Some(iterator) => iterator.next(),
            None => None
        }
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.iterator {
            Some(iterator) => iterator.size_hint(),
            None => (0, Some(0)),
        }
    }
}
