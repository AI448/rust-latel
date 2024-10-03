use crate::traits::{RandomVectorTrait, SequentialVectorTrait, VectorTrait};

#[derive(Debug)]
pub struct MappedVector<F: Fn(f64) -> f64, V: VectorTrait> {
    functor: F,
    vector: V,
}

impl<F: Fn(f64) -> f64, V: VectorTrait> MappedVector<F, V> {
    pub fn new(functor: F, vector: V) -> Self {
        return Self { functor: functor, vector: vector };
    }
}

impl<F: Fn(f64) -> f64, V: VectorTrait> VectorTrait for MappedVector<F, V> {
    fn dimension(&self) -> usize {
        self.vector.dimension()
    }
}

impl<F: Fn(f64) -> f64, V: SequentialVectorTrait> SequentialVectorTrait for MappedVector<F, V> {
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.vector.iter().map(|(i, x)| (i, (self.functor)(x)))
    }
}

impl<F: Fn(f64) -> f64, V: VectorTrait + RandomVectorTrait> RandomVectorTrait for MappedVector<F, V> {
    fn get(&self, i: usize) -> f64 {
        (self.functor)(self.vector.get(i))
    }
}
