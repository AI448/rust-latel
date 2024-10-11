use crate::{
    traits::{ColumnMatrixTrait, MatrixTrait, RowMatrixTrait, SequentialMatrixTrait, SequentialVectorTrait},
    types::Direction::{self, COLUMN, ROW},
};

use super::crs_matrix::CRSMatrix;

#[derive(Clone, Debug)]
pub struct CCSMatrix {
    crs_matrix: CRSMatrix,
}

impl CCSMatrix {
    pub fn new<I: Iterator<Item = ([usize; 2], f64)>>(dimension: [usize; 2], nonzero_elements: I) -> Self {
        Self {
            crs_matrix: CRSMatrix::new(
                [dimension[COLUMN], dimension[ROW]],
                nonzero_elements.map(|([i, j], x)| ([j, i], x)),
            ),
        }
    }
}

impl MatrixTrait for CCSMatrix {
    fn dimension<const D: Direction>(&self) -> usize {
        match D {
            Direction::ROW => self.crs_matrix.dimension::<{ COLUMN }>(),
            Direction::COLUMN => self.crs_matrix.dimension::<{ ROW }>(),
        }
    }
}

impl SequentialMatrixTrait for CCSMatrix {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.crs_matrix.iter().map(|([i, j], x)| ([j, i], x))
    }
}

impl ColumnMatrixTrait for CCSMatrix {
    fn column(&self, j: usize) -> impl SequentialVectorTrait + '_ {
        self.crs_matrix.row(j)
    }
}
