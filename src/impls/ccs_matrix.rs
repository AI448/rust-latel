use crate::{
    traits::{ColumnMatrixTrait, MatrixTrait, RowMatrixTrait, SequentialMatrixTrait, SequentialVectorTrait},
    types::{Transposed, COLUMN, ROW},
};

use super::crs_matrix::CRSMatrix;

#[derive(Default, Clone, Debug)]
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

    pub fn replace<I: Iterator<Item = ([usize; 2], f64)>>(&mut self, dimension: [usize; 2], nonzero_elements: I) {
        self.crs_matrix.replace(dimension.transposed(), nonzero_elements.map(|([i, j], x)| ([j, i], x)));
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
    fn column(&self, j: usize) -> impl SequentialVectorTrait + '_ {
        self.crs_matrix.row(j)
    }
}
