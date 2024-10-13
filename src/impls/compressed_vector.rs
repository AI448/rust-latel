use crate::{
    traits::{SequentialVectorTrait, VectorTrait},
    SequentialMutVectorTrait,
};

use super::operations::{add_assign_sequential_vector, assign_sequential_vector, sub_assign_sequential_vector};

#[derive(Default, Clone, Debug)]
pub struct CompressedVector {
    dimension: usize,
    indices: Vec<usize>,
    values: Vec<f64>,
}

impl CompressedVector {
    // pub fn new<I: Iterator<Item = (usize, f64)>>(dimension: usize, nonzero_elements: I) -> Self {
    //     let mut x = Self::default();
    //     x.replace(dimension, nonzero_elements);
    //     return x;
    // }

    pub fn clear(&mut self) {
        self.dimension = 0;
        self.indices.clear();
        self.values.clear();
    }

    #[inline(always)]
    pub fn push(&mut self, i: usize, x: f64) {
        debug_assert!(i < self.dimension);
        if x != 0.0 {
            self.indices.push(i);
            self.values.push(x);
        }
    }
}

impl VectorTrait for CompressedVector {
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

impl SequentialVectorTrait for CompressedVector {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.indices.iter().cloned().zip(self.values.iter().cloned())
    }
}

impl SequentialMutVectorTrait for CompressedVector {
    fn replace_by_iter<I: Iterator<Item = (usize, f64)>>(&mut self, dimension: usize, nonzero_elements: I) {
        self.dimension = dimension;
        self.indices.clear();
        self.values.clear();
        for (i, x) in nonzero_elements {
            debug_assert!(i < dimension);
            if x != 0.0 {
                self.indices.push(i);
                self.values.push(x);
            }
        }
    }
}
