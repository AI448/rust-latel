use core::f64;
use std::ops::{Index, IndexMut};

use crate::{
    traits::{MatrixTrait, RowMatrixTrait, SequentialMatrixTrait, SequentialMutMatrixTrait},
    types::{COLUMN, ROW},
    ColumnMatrixTrait,
};

#[derive(Default, Clone, Debug)]
pub struct DiagonalMatrix {
    values: Vec<f64>,
}

impl DiagonalMatrix {
    pub fn estimated_number_of_nonzeros(&self) -> usize {
        self.values.len()
    }
}

impl Index<usize> for DiagonalMatrix {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for DiagonalMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl SequentialMutMatrixTrait for DiagonalMatrix {
    fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(&mut self, dimension: [usize; 2], nonzero_elements: I) {
        assert!(dimension[ROW] == dimension[COLUMN]);
        self.values.clear();
        self.values.resize(dimension[ROW], 0.0);
        for ([i, j], a) in nonzero_elements {
            assert!(i == j);
            self.values[i] = a;
        }
    }
}

impl MatrixTrait for DiagonalMatrix {
    #[inline(always)]
    fn dimension(&self) -> [usize; 2] {
        [self.values.len(); 2]
    }
}

impl SequentialMatrixTrait for DiagonalMatrix {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.values.iter().cloned().enumerate().map(|(i, a)| ([i, i], a))
    }
}

impl RowMatrixTrait for DiagonalMatrix {
    #[inline(always)]
    fn iter_row(&self, i: usize) -> impl DoubleEndedIterator<Item = (usize, f64)> + Clone + '_ {
        [(i, self.values[i])].into_iter()
    }
}

impl ColumnMatrixTrait for DiagonalMatrix {
    #[inline(always)]
    fn iter_column(&self, j: usize) -> impl DoubleEndedIterator<Item = (usize, f64)> + Clone + '_ {
        [(j, self.values[j])].into_iter()
    }
}
