use crate::traits::{ColumnMatrixTrait, MatrixTrait, RowMatrixTrait, SequentialMatrixTrait};
use crate::types::Transposed;

#[derive(Debug)]
pub struct TransposedMatrix<M: MatrixTrait> {
    matrix: M,
}

impl<M: MatrixTrait> TransposedMatrix<M> {
    pub fn new(matrix: M) -> Self {
        Self { matrix: matrix }
    }
}

impl<M: MatrixTrait> MatrixTrait for TransposedMatrix<M> {
    #[inline(always)]
    fn dimension(&self) -> [usize; 2] {
        self.matrix.dimension().transposed()
    }
}

impl<M: SequentialMatrixTrait> SequentialMatrixTrait for TransposedMatrix<M> {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.matrix.iter().map(|([i, j], x)| ([j, i], x))
    }
}

impl<M: ColumnMatrixTrait> RowMatrixTrait for TransposedMatrix<M> {
    #[inline(always)]
    fn row(&self, i: usize) -> impl crate::SequentialVectorTrait + '_ {
        self.matrix.column(i)
    }
}

impl<M: RowMatrixTrait> ColumnMatrixTrait for TransposedMatrix<M> {
    #[inline(always)]
    fn column(&self, j: usize) -> impl crate::SequentialVectorTrait + '_ {
        self.matrix.row(j)
    }
}
