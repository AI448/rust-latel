use crate::{
    traits::{SequentialVectorTrait, VectorTrait},
    RandomVectorTrait,
};

use super::operations::{add_assign_sequential_vector, assign_sequential_vector, sub_assign_sequential_vector};

#[derive(Clone, Debug)]
pub struct UnitVector {
    dimension: usize,
    nonzero_index: usize,
}

impl UnitVector {
    #[inline(always)]
    pub fn new(dimension: usize, nonzero_index: usize) -> Self {
        Self { dimension: dimension, nonzero_index: nonzero_index }
    }
}

impl VectorTrait for UnitVector {
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

impl SequentialVectorTrait for UnitVector {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        [(self.nonzero_index, 1.0)].into_iter()
    }
}

impl RandomVectorTrait for UnitVector {
    #[inline(always)]
    fn get(&self, i: usize) -> f64 {
        if i == self.nonzero_index {
            return 1.0;
        } else {
            return 0.0;
        }
    }
}
