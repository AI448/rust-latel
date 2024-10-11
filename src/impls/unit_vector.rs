use crate::{
    traits::{SequentialVectorTrait, VectorTrait},
    RandomVectorTrait,
};

#[derive(Clone, Debug)]
pub struct UnitVector {
    dimension: usize,
    nonzero_index: usize,
}

impl UnitVector {
    pub fn new(dimension: usize, nonzero_index: usize) -> Self {
        Self { dimension: dimension, nonzero_index: nonzero_index }
    }
}

impl VectorTrait for UnitVector {
    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl SequentialVectorTrait for UnitVector {
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        [(self.nonzero_index, 1.0)].into_iter()
    }
}

impl RandomVectorTrait for UnitVector {
    fn get(&self, i: usize) -> f64 {
        if i == self.nonzero_index {
            return 1.0;
        } else {
            return 0.0;
        }
    }
}
