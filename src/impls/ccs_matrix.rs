use crate::{
    traits::{ColumnMatrixTrait, MatrixTrait, RowMatrixTrait, SequentialMatrixTrait, SequentialMutMatrixTrait},
    types::Transposed,
};

use super::crs_matrix::CRSMatrix;

#[derive(Default, Clone, Debug)]
pub struct CCSMatrix {
    crs_matrix: CRSMatrix,
}

impl SequentialMutMatrixTrait for CCSMatrix {
    fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(&mut self, dimension: [usize; 2], nonzero_elements: I) {
        self.crs_matrix
            .replace_by_iter(dimension.transposed(), nonzero_elements.map(|([i, j], x)| ([j, i], x)));
    }
}

impl MatrixTrait for CCSMatrix {
    #[inline(always)]
    fn dimension(&self) -> [usize; 2] {
        self.crs_matrix.dimension().transposed()
    }
}

impl SequentialMatrixTrait for CCSMatrix {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.crs_matrix.iter().map(|([i, j], x)| ([j, i], x))
    }
}

impl ColumnMatrixTrait for CCSMatrix {
    #[inline(always)]
    fn iter_column(&self, j: usize) -> impl DoubleEndedIterator<Item = (usize, f64)> + Clone + '_ {
        self.crs_matrix.iter_row(j)
    }
}
