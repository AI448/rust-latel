use crate::types::Direction::{COLUMN, ROW};
use crate::{impls, ColumnMatrixTrait, RowMatrixTrait, SequentialMatrixTrait, SequentialVectorTrait};
use crate::wrappers::SequentialVector;

#[derive(Default, Clone)]
pub struct SequentialMatrix<M: SequentialMatrixTrait> {
    pub(crate) object: M,
}

impl<M: SequentialMatrixTrait> std::ops::Deref for SequentialMatrix<M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<M: SequentialMatrixTrait> std::ops::DerefMut for SequentialMatrix<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
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

impl<M: SequentialMatrixTrait> std::fmt::Debug for SequentialMatrix<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_vector(f, &self.object)
    }
}

#[derive(Default, Clone)]
pub struct RowMatrix<M: RowMatrixTrait> {
    pub(crate) object: M,
}

impl<M: RowMatrixTrait> std::ops::Deref for RowMatrix<M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<M: RowMatrixTrait> std::ops::DerefMut for RowMatrix<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<M: RowMatrixTrait> RowMatrix<M> {

    pub fn row(&self, i: usize) -> SequentialVector<impl SequentialVectorTrait + '_> {
        SequentialVector{object: self.object.row(i)}
    }

    #[allow(non_snake_case)]
    pub fn T(&self) -> ColumnMatrix<impl ColumnMatrixTrait + '_> {
        ColumnMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }

    #[allow(non_snake_case)]
    pub fn into_T(self) -> ColumnMatrix<impl ColumnMatrixTrait> {
        ColumnMatrix { object: impls::TransposedMatrix::new(self.object) }
    }
}

impl<M: RowMatrixTrait> std::fmt::Debug for RowMatrix<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_vector(f, &self.object)
    }
}

#[derive(Default, Clone)]
pub struct ColumnMatrix<M: ColumnMatrixTrait> {
    pub(crate) object: M,
}

impl<M: ColumnMatrixTrait> std::ops::Deref for ColumnMatrix<M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<M: ColumnMatrixTrait> std::ops::DerefMut for ColumnMatrix<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<M: ColumnMatrixTrait> ColumnMatrix<M> {

    pub fn column(&self, j: usize) -> SequentialVector<impl SequentialVectorTrait + '_> {
        SequentialVector{object: self.object.column(j)}
    }

    #[allow(non_snake_case)]
    pub fn T(&self) -> RowMatrix<impl RowMatrixTrait + '_> {
        RowMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }

    #[allow(non_snake_case)]
    pub fn into_T(self) -> RowMatrix<impl RowMatrixTrait> {
        RowMatrix { object: impls::TransposedMatrix::new(self.object) }
    }
}

impl<M: ColumnMatrixTrait> std::fmt::Debug for ColumnMatrix<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_vector(f, &self.object)
    }
}

#[derive(Default, Clone)]
pub struct BidirectionalMatrix<M: RowMatrixTrait + ColumnMatrixTrait> {
    pub(crate) object: M,
}

impl<M: RowMatrixTrait + ColumnMatrixTrait> std::ops::Deref for BidirectionalMatrix<M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<M: RowMatrixTrait + ColumnMatrixTrait> std::ops::DerefMut for BidirectionalMatrix<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<M: RowMatrixTrait + ColumnMatrixTrait> BidirectionalMatrix<M> {
    #[allow(non_snake_case)]
    pub fn T(&self) -> BidirectionalMatrix<impl RowMatrixTrait + ColumnMatrixTrait + '_> {
        BidirectionalMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }

    #[allow(non_snake_case)]
    pub fn into_T(self) -> BidirectionalMatrix<impl RowMatrixTrait + ColumnMatrixTrait> {
        BidirectionalMatrix { object: impls::TransposedMatrix::new(self.object) }
    }
}

impl<M: RowMatrixTrait + ColumnMatrixTrait> std::fmt::Debug for BidirectionalMatrix<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_vector(f, &self.object)
    }
}

pub type CompressedMatrix = SequentialMatrix<impls::CompressedMatrix>;

impl CompressedMatrix {
    pub fn new<I: Iterator<Item = ([usize; 2], f64)>>(dimension: [usize; 2], nonzero_elements: I) -> Self {
        Self { object: impls::CompressedMatrix::new(dimension, nonzero_elements) }
    }
}

pub type CRSMatrix = RowMatrix<impls::CRSMatrix>;

impl CRSMatrix {
    pub fn new<I: Iterator<Item = ([usize; 2], f64)>>(dimension: [usize; 2], nonzero_elements: I) -> Self {
        Self { object: impls::CRSMatrix::new(dimension, nonzero_elements) }
    }
}

pub type SparseMatrix = BidirectionalMatrix<impls::SparseMatrix>;

impl SparseMatrix {
    pub fn new<I: Iterator<Item = ([usize; 2], f64)>>(dimension: [usize; 2], nonzero_elements: I) -> Self {
        Self { object: impls::SparseMatrix::new(dimension, nonzero_elements) }
    }
}

fn debug_vector(f: &mut std::fmt::Formatter, matrix: &impl SequentialMatrixTrait) -> std::fmt::Result {
    write!(
        f,
        "{{ dimension = [{}, {}], values = [",
        matrix.dimension::<{ ROW }>(),
        matrix.dimension::<{ COLUMN }>()
    )?;
    let mut first = true;
    for (index, value) in matrix.iter() {
        if first {
            first = false;
        } else {
            write!(f, ", ")?;
        }
        write!(f, "([{}, {}], {})", index[ROW], index[COLUMN], value)?;
    }
    write!(f, "] }}")?;
    return Ok(());
}
