use crate::traits::{PermutatorTrait, RandomVectorTrait, SequentialVectorTrait, VectorTrait};
use crate::types::{COLUMN, ROW};

#[derive(Debug)]
pub struct PermutatedVector<P: PermutatorTrait, V: VectorTrait> {
    permutator: P,
    vector: V,
}

impl<P: PermutatorTrait, V: VectorTrait> PermutatedVector<P, V> {
    pub fn new(permutator: P, vector: V) -> Self {
        assert!(permutator.dimension()[COLUMN] == vector.dimension());
        Self { permutator: permutator, vector: vector }
    }
}

impl<P: PermutatorTrait, V: VectorTrait> VectorTrait for PermutatedVector<P, V> {
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.permutator.dimension()[ROW]
    }
}

impl<P: PermutatorTrait, V: SequentialVectorTrait> SequentialVectorTrait for PermutatedVector<P, V> {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
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
