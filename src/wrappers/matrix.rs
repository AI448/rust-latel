use crate::{impls, ColumnMatrixTrait, RowMatrixTrait, SequentialMatrixTrait};

#[derive(Default, Clone, Debug)]
pub struct SequentialMatrix<M: SequentialMatrixTrait> {
    pub(crate) object: M,
}

impl<M: SequentialMatrixTrait> SequentialMatrix<M> {
    #[allow(non_snake_case)]
    pub fn T(&self) -> SequentialMatrix<impl SequentialMatrixTrait + '_> {
        SequentialMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }

    #[allow(non_snake_case)]
    pub fn into_T(self) -> SequentialMatrix<impl SequentialMatrixTrait> {
        SequentialMatrix { object: impls::TransposedMatrix::new(self.object) }
    }
}

#[derive(Default, Clone, Debug)]
pub struct RowMatrix<M: RowMatrixTrait> {
    pub(crate) object: M,
}

impl<M: RowMatrixTrait> RowMatrix<M> {
    #[allow(non_snake_case)]
    pub fn T(&self) -> ColumnMatrix<impl ColumnMatrixTrait + '_> {
        ColumnMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }

    #[allow(non_snake_case)]
    pub fn into_T(self) -> ColumnMatrix<impl ColumnMatrixTrait> {
        ColumnMatrix { object: impls::TransposedMatrix::new(self.object) }
    }
}

#[derive(Default, Clone, Debug)]
pub struct ColumnMatrix<M: ColumnMatrixTrait> {
    pub(crate) object: M,
}

impl<M: ColumnMatrixTrait> ColumnMatrix<M> {
    #[allow(non_snake_case)]
    pub fn T(&self) -> RowMatrix<impl RowMatrixTrait + '_> {
        RowMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }

    #[allow(non_snake_case)]
    pub fn into_T(self) -> RowMatrix<impl RowMatrixTrait> {
        RowMatrix { object: impls::TransposedMatrix::new(self.object) }
    }
}

pub type CompressedMatrix = SequentialMatrix<impls::CompressedMatrix>;

impl CompressedMatrix {
    pub fn new<I: Iterator<Item = ([usize; 2], f64)>>(dimension: [usize; 2], nonzero_elements: I) -> Self {
        Self { object: impls::CompressedMatrix::new(dimension, nonzero_elements) }
    }
}
