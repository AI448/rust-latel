use crate::{
    traits::{RandomVectorTrait, SequentialVectorTrait, VectorTrait},
    RandomMutVectorTrait,
};

use super::operations::{add_assign_sequential_vector, assign_sequential_vector, sub_assign_sequential_vector};

#[derive(Debug)]
pub struct MappedVector<F: Fn(f64) -> f64, V: SequentialVectorTrait> {
    functor: F,
    vector: V,
}

impl<F: Fn(f64) -> f64, V: SequentialVectorTrait> MappedVector<F, V> {
    pub fn new(functor: F, vector: V) -> Self {
        return Self { functor: functor, vector: vector };
    }
}

impl<F: Fn(f64) -> f64, V: SequentialVectorTrait> VectorTrait for MappedVector<F, V> {
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.vector.dimension()
    }
    #[inline(always)]
    fn assign_to_random_vector(&self, lhs: &mut impl crate::RandomMutVectorTrait) {
        assign_sequential_vector(lhs, &self);
    }
    #[inline(always)]
    fn add_assign_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        add_assign_sequential_vector(lhs, &self);
    }
    #[inline(always)]
    fn sub_assign_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        sub_assign_sequential_vector(lhs, &self);
    }
}

impl<F: Fn(f64) -> f64, V: SequentialVectorTrait> SequentialVectorTrait for MappedVector<F, V> {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.vector.iter().map(|(i, x)| (i, (self.functor)(x)))
    }
}

impl<F: Fn(f64) -> f64, V: VectorTrait + RandomVectorTrait> RandomVectorTrait for MappedVector<F, V> {
    #[inline(always)]
    fn get(&self, i: usize) -> f64 {
        (self.functor)(self.vector.get(i))
    }
}
