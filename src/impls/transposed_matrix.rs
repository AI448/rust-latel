use crate::traits::{ColumnMatrixTrait, MatrixTrait, RowMatrixTrait, SequentialMatrixTrait};
use crate::types::Direction::{self, COLUMN, ROW};

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
    fn dimension<const D: Direction>(&self) -> usize {
        match D {
            ROW => self.matrix.dimension::<{ COLUMN }>(),
            COLUMN => self.matrix.dimension::<{ ROW }>(),
        }
    }
}

impl<M: SequentialMatrixTrait> SequentialMatrixTrait for TransposedMatrix<M> {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.matrix.iter().map(|([i, j], x)| ([j, i], x))
    }
}

impl<M: ColumnMatrixTrait> RowMatrixTrait for TransposedMatrix<M> {
    fn row(&self, i: usize) -> impl crate::SequentialVectorTrait + '_ {
        self.matrix.column(i)
    }
}

impl<M: RowMatrixTrait> ColumnMatrixTrait for TransposedMatrix<M> {
    fn column(&self, j: usize) -> impl crate::SequentialVectorTrait + '_ {
        self.matrix.row(j)
    }
}
