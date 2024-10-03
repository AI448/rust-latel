use crate::traits::{MatrixTrait, SequentialMatrixTrait};
use crate::types::Direction;

#[derive(Default, Clone, Debug)]
pub struct CompressedMatrix {
    dimension: [usize; 2],
    items: Vec<([usize; 2], f64)>,
}

impl CompressedMatrix {
    pub fn new<I: Iterator<Item = ([usize; 2], f64)>>(dimension: [usize; 2], items: I) -> Self {
        Self { dimension: dimension, items: Vec::from_iter(items) }
    }

    // pub fn replace(&mut self, matrix: &impl SequentialMatrixTrait)
    // {
    //     self.dimension = [matrix.dimension::<{ROW}>(), matrix.dimension::<{COLUMN}>()];
    //     self.items.clear();
    //     self.items.extend(matrix.iter());
    // }

    pub fn clear(&mut self) {
        self.dimension = [0, 0];
        self.items.clear();
    }

    pub fn push(&mut self, index: [usize; 2], value: f64) {
        assert!(index[0] < self.dimension[0]);
        assert!(index[1] < self.dimension[1]);
        self.items.push((index, value));
    }
}

impl MatrixTrait for CompressedMatrix {
    fn dimension<const D: Direction>(&self) -> usize {
        self.dimension[D]
    }
}

impl SequentialMatrixTrait for CompressedMatrix {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.items.iter().cloned()
    }
}
