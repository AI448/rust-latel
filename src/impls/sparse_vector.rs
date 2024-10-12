// use crate::traits::AssignableVectorTrait;

use crate::traits::{
    RandomMutVectorTrait, RandomVectorTrait, SequentialMutVectorTrait, SequentialVectorTrait, VectorTrait,
};

use super::operations::{add_assign_sequential_vector, assign_sequential_vector, sub_assign_sequential_vector};

#[derive(Default, Clone, Debug)]
pub struct SparseVector {
    values: Vec<f64>,
    flags: Vec<bool>,
    nonzero_indices: Vec<usize>,
}

impl SparseVector {
    pub fn new<I: Iterator<Item = (usize, f64)>>(dimension: usize, nonzero_elements: I) -> Self {
        let mut v = Self::default();
        v.replace(dimension, nonzero_elements);
        return v;
    }

    pub fn zero_clear(&mut self) {
        while let Some(i) = self.nonzero_indices.pop() {
            debug_assert!(self.flags[i]);
            self.values[i] = 0.0;
            self.flags[i] = false;
        }
    }
}

impl VectorTrait for SparseVector {
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.values.len()
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

impl SequentialVectorTrait for SparseVector {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.nonzero_indices.iter().map(|i| (*i, self.values[*i]))
    }
}

impl RandomVectorTrait for SparseVector {
    #[inline(always)]
    fn get(&self, i: usize) -> f64 {
        self.values[i]
    }
}

impl SequentialMutVectorTrait for SparseVector {
    fn replace<I: Iterator<Item = (usize, f64)>>(&mut self, dimension: usize, nonzero_elements: I) {
        self.zero_clear();
        self.values.resize(dimension, 0.0);
        self.flags.resize(dimension, false);
        for (i, x) in nonzero_elements {
            if x != 0.0 {
                self.values[i] = x;
                self.flags[i] = true;
                self.nonzero_indices.push(i);
            }
        }
    }
}

impl std::ops::Index<usize> for SparseVector {
    type Output = f64;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl std::ops::IndexMut<usize> for SparseVector {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if !self.flags[index] {
            debug_assert!(self.values[index] == 0.0);
            self.flags[index] = true;
            self.nonzero_indices.push(index);
        }
        &mut self.values[index]
    }
}

impl RandomMutVectorTrait for SparseVector {}
