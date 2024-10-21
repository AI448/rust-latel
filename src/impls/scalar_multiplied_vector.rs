use crate::{
    traits::{RandomVectorTrait, SequentialVectorTrait, VectorTrait},
    RandomMutVectorTrait,
};

use super::operations::{add_assign_scalar_multipled_sequential_vector, assign_sequential_vector};

#[derive(Debug)]
pub struct ScalarMultipledVector<V: SequentialVectorTrait> {
    scalar: f64,
    vector: V,
}

impl<V: SequentialVectorTrait> ScalarMultipledVector<V> {
    #[inline(always)]
    pub fn new(scalar: f64, vector: V) -> Self {
        return Self { scalar: scalar, vector: vector };
    }
}

impl<V: SequentialVectorTrait> VectorTrait for ScalarMultipledVector<V> {
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.vector.dimension()
    }
    #[inline(always)]
    fn assign_to_random_vector(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        assign_sequential_vector(lhs, &self);
    }
    #[inline(always)]
    fn add_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        add_assign_scalar_multipled_sequential_vector(lhs, self.scalar, &self.vector);
    }
    #[inline(always)]
    fn sub_from_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        add_assign_scalar_multipled_sequential_vector(lhs, -self.scalar, &self.vector);
    }
}

impl<V: SequentialVectorTrait> SequentialVectorTrait for ScalarMultipledVector<V> {
    #[inline(always)]
    fn iter(&self) -> impl DoubleEndedIterator<Item = (usize, f64)> + Clone + '_ {
        self.vector.iter().map(|(i, x)| (i, self.scalar * x))
    }
}

impl<V: VectorTrait + RandomVectorTrait> RandomVectorTrait for ScalarMultipledVector<V> {
    #[inline(always)]
    fn get(&self, i: usize) -> f64 {
        self.scalar * self.vector.get(i)
    }
}
