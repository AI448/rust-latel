use crate::{
    impls::{ccs_matrix::CCSMatrix, crs_matrix::CRSMatrix},
    traits::{MatrixTrait, RowMatrixTrait, SequentialMatrixTrait, SequentialMutMatrixTrait},
    types::{COLUMN, ROW},
    ColumnMatrixTrait,
};

#[derive(Default, Clone, Debug)]
pub struct CRCSMatrix {
    crs_matrix: CRSMatrix,
    ccs_matrix: CCSMatrix,
}

impl CRCSMatrix {
    pub fn estimated_number_of_nonzeros(&self) -> usize {
        self.crs_matrix.estimated_number_of_nonzeros()
    }
}

impl SequentialMutMatrixTrait for CRCSMatrix {
    fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(&mut self, dimension: [usize; 2], nonzero_elements: I) {
        self.crs_matrix.replace_by_iter(dimension, nonzero_elements);
        self.ccs_matrix.replace_by_iter(dimension, self.crs_matrix.iter());
    }
}

impl MatrixTrait for CRCSMatrix {
    #[inline(always)]
    fn dimension(&self) -> [usize; 2] {
        self.crs_matrix.dimension()
    }
}

impl SequentialMatrixTrait for CRCSMatrix {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.crs_matrix.iter()
    }
}

impl RowMatrixTrait for CRCSMatrix {
    #[inline(always)]
    fn iter_row(&self, i: usize) -> impl DoubleEndedIterator<Item = (usize, f64)> + Clone + '_ {
        self.crs_matrix.iter_row(i)
    }
}

impl ColumnMatrixTrait for CRCSMatrix {
    #[inline(always)]
    fn iter_column(&self, j: usize) -> impl DoubleEndedIterator<Item = (usize, f64)> + Clone + '_ {
        self.ccs_matrix.iter_column(j)
    }
}
