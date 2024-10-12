use crate::traits::{
    RandomMutVectorTrait, RandomVectorTrait, SequentialMutVectorTrait, SequentialVectorTrait, VectorTrait,
};

use super::operations::{add_assign_sequential_vector, sub_assign_sequential_vector};

#[derive(Default, Clone, Debug)]
pub struct DenseVector {
    values: Vec<f64>,
}

impl DenseVector {
    pub fn new<I: Iterator<Item = (usize, f64)>>(dimension: usize, nonzero_elements: I) -> Self {
        let mut v = Self::default();
        v.replace(dimension, nonzero_elements);
        return v;
    }

    /// 全ての要素を 0 にする
    pub fn zero_clear(&mut self) {
        self.values.fill(0.0);
    }
}

impl std::ops::Index<usize> for DenseVector {
    type Output = f64;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl std::ops::IndexMut<usize> for DenseVector {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl VectorTrait for DenseVector {
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.values.len()
    }
    #[inline(always)]
    fn add_assign_to(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        add_assign_sequential_vector(lhs, &self);
    }
    #[inline(always)]
    fn sub_assign_from(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        sub_assign_sequential_vector(lhs, &self);
    }
}

impl SequentialVectorTrait for DenseVector {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.values.iter().cloned().enumerate()
    }
}

impl RandomVectorTrait for DenseVector {
    #[inline(always)]
    fn get(&self, i: usize) -> f64 {
        self[i]
    }
}

impl SequentialMutVectorTrait for DenseVector {
    fn replace<I: Iterator<Item = (usize, f64)>>(&mut self, dimension: usize, nonzero_elements: I) {
        if dimension < self.values.len() {
            self.values.truncate(dimension);
            self.values.fill(0.0);
        } else {
            self.values.fill(0.0);
            self.values.extend(std::iter::repeat(0.0).take(dimension - self.values.len()));
        }
        for (i, x) in nonzero_elements {
            debug_assert!(i < self.dimension());
            self.values[i] = x;
        }
    }
}

impl RandomMutVectorTrait for DenseVector {}
