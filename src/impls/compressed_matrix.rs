use crate::traits::{MatrixTrait, SequentialMatrixTrait, SequentialMutMatrixTrait};

#[derive(Default, Clone, Debug)]
pub struct CompressedMatrix {
    dimension: [usize; 2],
    items: Vec<([usize; 2], f64)>,
}

impl SequentialMutMatrixTrait for CompressedMatrix {
    fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(&mut self, dimension: [usize; 2], nonzero_elements: I) {
        self.dimension = dimension;
        self.items.clear();
        self.items.extend(nonzero_elements);
    }
}

impl CompressedMatrix {
    pub fn clear(&mut self) {
        self.dimension = [0, 0];
        self.items.clear();
    }

    #[inline(always)]
    pub fn push(&mut self, index: [usize; 2], value: f64) {
        assert!(index[0] < self.dimension[0]);
        assert!(index[1] < self.dimension[1]);
        self.items.push((index, value));
    }
}

impl MatrixTrait for CompressedMatrix {
    #[inline(always)]
    fn dimension(&self) -> [usize; 2] {
        self.dimension
    }
}

impl SequentialMatrixTrait for CompressedMatrix {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.items.iter().cloned()
    }
}
