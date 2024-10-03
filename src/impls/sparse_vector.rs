use super::super::traits::{RandomVectorTrait, SequentialVectorTrait, VectorTrait};

#[derive(Default, Clone, Debug)]
pub struct SparseVector {
    values: Vec<f64>,
    flags: Vec<bool>,
    nonzero_indices: Vec<usize>,
}

impl SparseVector {
    pub fn new<I: Iterator<Item = (usize, f64)>>(dimension: usize, nonzero_elements: I) -> Self {
        let mut values = Vec::from_iter(std::iter::repeat(0f64).take(dimension));
        let mut flags = Vec::from_iter(std::iter::repeat(false).take(dimension));
        let mut nonzero_indices = Vec::default();
        for (i, x) in nonzero_elements {
            if x != 0.0 {
                values[i] = x;
                flags[i] = true;
                nonzero_indices.push(i);
            }
        }
        Self { values: values, flags: flags, nonzero_indices: nonzero_indices }
    }

    pub fn zero_clear(&mut self) {
        while let Some(i) = self.nonzero_indices.pop() {
            debug_assert!(self.flags[i]);
            self.values[i] = 0.0;
            self.flags[i] = false;
        }
    }

    // pub fn redimension(&mut self, new_dimension: usize) {
    //     debug_assert!(self.values.len() == self.positions.len());
    //     if new_dimension < self.positions.len() {
    //         while self.positions.len() > new_dimension {
    //             self.values.pop();
    //             let position = unsafe { self.positions.pop().unwrap_unchecked() };
    //             if position != Self::NULL_POSITION {
    //                 debug_assert!(!self.nonzero_indices.is_empty());
    //                 debug_assert!(self.nonzero_indices[position] == self.positions.len());
    //                 self.nonzero_indices.swap_remove(position);
    //                 if position < self.nonzero_indices.len() {
    //                     let i = self.nonzero_indices[position];
    //                     debug_assert!(self.positions[i] == self.nonzero_indices.len());
    //                     self.positions[i] = position;
    //                 }
    //             }
    //         }
    //     } else if new_dimension > self.positions.len() {
    //         self.values.resize(new_dimension, 0f64);
    //         self.positions.resize(new_dimension, Self::NULL_POSITION);
    //     }
    // }

    // pub fn insert(&mut self, index: u32, value: f64) {
    //     self.values[index] = value;
    //     if self.positions[index] == Self::NULL_POSITION {
    //         self.positions[index] = self.nonzero_indices.len();
    //         self.nonzero_indices.push(index);
    //     }
    // }

    // pub fn zero_clear(&mut self) {
    //     while let Some(i) = self.nonzero_indices.pop() {
    //         self.values[i] = 0.0;
    //         self.positions[i] = Self::NULL_POSITION;
    //     }
    //     debug_assert!(self.values.len() == self.positions.len());
    //     debug_assert!(self.nonzero_indices.is_empty());
    // }

    // pub fn iter_mut(&mut self) -> impl Iterator<Item=(u32, &mut f64)> + '_ {
    //     self.nonzero_indices.iter().map(|i| (*i, &mut self.values[*i]))
    // }
}

impl std::ops::Index<usize> for SparseVector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl std::ops::IndexMut<usize> for SparseVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if !self.flags[index] {
            debug_assert!(self.values[index] == 0.0);
            self.flags[index] = true;
            self.nonzero_indices.push(index);
        }
        &mut self.values[index]
    }
}

impl VectorTrait for SparseVector {
    fn dimension(&self) -> usize {
        self.values.len()
    }
}

impl SequentialVectorTrait for SparseVector {
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        self.nonzero_indices.iter().map(|i| (*i, self.values[*i]))
    }
}

impl RandomVectorTrait for SparseVector {
    fn get(&self, i: usize) -> f64 {
        self.values[i]
    }
}
