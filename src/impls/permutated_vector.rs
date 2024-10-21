use crate::traits::{PermutatorTrait, RandomVectorTrait, SequentialVectorTrait, VectorTrait};
use crate::types::{COLUMN, ROW};
use crate::RandomMutVectorTrait;

use super::operations::{add_assign_sequential_vector, assign_sequential_vector, sub_assign_sequential_vector};

#[derive(Debug)]
pub struct PermutatedVector<P: PermutatorTrait, V: SequentialVectorTrait> {
    permutator: P,
    vector: V,
}

impl<P: PermutatorTrait, V: SequentialVectorTrait> PermutatedVector<P, V> {
    pub fn new(permutator: P, vector: V) -> Self {
        assert!(permutator.dimension()[COLUMN] == vector.dimension());
        Self { permutator: permutator, vector: vector }
    }
}

impl<P: PermutatorTrait, V: SequentialVectorTrait> VectorTrait for PermutatedVector<P, V> {
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.permutator.dimension()[ROW]
    }
    #[inline(always)]
    fn assign_to_random_vector(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        assign_sequential_vector(lhs, &self);
    }
    #[inline(always)]
    fn add_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        add_assign_sequential_vector(lhs, &self);
    }
    #[inline(always)]
    fn sub_from_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        sub_assign_sequential_vector(lhs, &self);
    }
}

impl<P: PermutatorTrait, V: SequentialVectorTrait> SequentialVectorTrait for PermutatedVector<P, V> {
    #[inline(always)]
    fn iter(&self) -> impl DoubleEndedIterator<Item = (usize, f64)> + Clone + '_ {
        self.vector.iter().filter_map(|(i1, x)| match self.permutator.permutate(i1) {
            Some(i2) => Some((i2, x)),
            None => None,
        })
    }
}

impl<P: PermutatorTrait, V: RandomVectorTrait> RandomVectorTrait for PermutatedVector<P, V> {
    #[inline(always)]
    fn get(&self, i: usize) -> f64 {
        match self.permutator.unpermutate(i) {
            Some(k) => self.vector.get(k),
            None => 0.0,
        }
    }
}
