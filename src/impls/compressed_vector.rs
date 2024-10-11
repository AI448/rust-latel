use crate::traits::{SequentialVectorTrait, VectorTrait};

#[derive(Default, Clone, Debug)]
pub struct CompressedVector {
    dimension: usize,
    indices: Vec<usize>,
    values: Vec<f64>,
}

impl CompressedVector {
    pub fn new<I: Iterator<Item = (usize, f64)>>(dimension: usize, nonzero_elements: I) -> Self {
        let mut indices = Vec::with_capacity(nonzero_elements.size_hint().0);
        let mut values = Vec::with_capacity(nonzero_elements.size_hint().0);
        for (i, x) in nonzero_elements {
            debug_assert!(i < dimension);
            if x != 0.0 {
                indices.push(i);
                values.push(x);
            }
        }
        Self { dimension: dimension, indices: indices, values: values }
    }

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
}

impl SequentialVectorTrait for CompressedVector {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.indices.iter().cloned().zip(self.values.iter().cloned())
    }
}
