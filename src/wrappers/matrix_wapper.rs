use crate::traits::SequentialMutMatrixTrait;
use crate::types::Direction::{COLUMN, ROW};
use crate::wrappers::SequentialVectorWrapper;
use crate::{impls, ColumnMatrixTrait, RowMatrixTrait, SequentialMatrixTrait, SequentialVectorTrait};

#[derive(Default, Clone)]
pub struct SequentialMatrix<M: SequentialMatrixTrait> {
    pub(crate) object: M,
}

impl<M: SequentialMatrixTrait> std::ops::Deref for SequentialMatrix<M> {
    type Target = M;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<M: SequentialMatrixTrait> std::ops::DerefMut for SequentialMatrix<M> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<M: SequentialMatrixTrait> SequentialMatrix<M> {
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn T(&self) -> SequentialMatrix<impl SequentialMatrixTrait + '_> {
        SequentialMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }

    #[inline(always)]
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

impl<M: SequentialMutMatrixTrait> SequentialMatrix<M> {
    #[inline(always)]
    pub fn generate_from_iter<I: Iterator<Item = ([usize; 2], f64)>>(
        dimension: [usize; 2],
        nonzero_elements: I,
    ) -> Self {
        Self { object: M::generate_from_iter(dimension, nonzero_elements) }
    }
    #[inline(always)]
    pub fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(
        &mut self,
        dimension: [usize; 2],
        nonzero_elements: I,
    ) {
        self.object.replace_by_iter(dimension, nonzero_elements);
    }
}

#[derive(Default, Clone)]
pub struct RowMatrix<M: RowMatrixTrait> {
    pub(crate) object: M,
}

impl<M: RowMatrixTrait> std::ops::Deref for RowMatrix<M> {
    type Target = M;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<M: RowMatrixTrait> std::ops::DerefMut for RowMatrix<M> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<M: RowMatrixTrait> RowMatrix<M> {
    #[inline(always)]
    pub fn row(&self, i: usize) -> SequentialVectorWrapper<impl SequentialVectorTrait + '_> {
        SequentialVectorWrapper {
            object: impls::VectorView::new(self.object.dimension()[COLUMN], self.object.iter_row(i)),
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn T(&self) -> ColumnMatrix<impl ColumnMatrixTrait + '_> {
        ColumnMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }

    #[inline(always)]
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

impl<M: RowMatrixTrait + SequentialMutMatrixTrait> RowMatrix<M> {
    #[inline(always)]
    pub fn generate_from_iter<I: Iterator<Item = ([usize; 2], f64)>>(
        dimension: [usize; 2],
        nonzero_elements: I,
    ) -> Self {
        Self { object: M::generate_from_iter(dimension, nonzero_elements) }
    }
    #[inline(always)]
    pub fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(
        &mut self,
        dimension: [usize; 2],
        nonzero_elements: I,
    ) {
        self.object.replace_by_iter(dimension, nonzero_elements);
    }
}

#[derive(Default, Clone)]
pub struct ColumnMatrix<M: ColumnMatrixTrait> {
    pub(crate) object: M,
}

impl<M: ColumnMatrixTrait> std::ops::Deref for ColumnMatrix<M> {
    type Target = M;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<M: ColumnMatrixTrait> std::ops::DerefMut for ColumnMatrix<M> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<M: ColumnMatrixTrait> ColumnMatrix<M> {
    #[inline(always)]
    pub fn column(&self, j: usize) -> SequentialVectorWrapper<impl SequentialVectorTrait + '_> {
        SequentialVectorWrapper {
            object: impls::VectorView::new(self.object.dimension()[ROW], self.object.iter_column(j)),
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn T(&self) -> RowMatrix<impl RowMatrixTrait + '_> {
        RowMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }

    #[inline(always)]
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

impl<M: ColumnMatrixTrait + SequentialMutMatrixTrait> ColumnMatrix<M> {
    #[inline(always)]
    pub fn generate_from_iter<I: Iterator<Item = ([usize; 2], f64)>>(
        dimension: [usize; 2],
        nonzero_elements: I,
    ) -> Self {
        Self { object: M::generate_from_iter(dimension, nonzero_elements) }
    }
    #[inline(always)]
    pub fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(
        &mut self,
        dimension: [usize; 2],
        nonzero_elements: I,
    ) {
        self.object.replace_by_iter(dimension, nonzero_elements);
    }
}

#[derive(Default, Clone)]
pub struct BidirectionalMatrix<M: RowMatrixTrait + ColumnMatrixTrait> {
    pub(crate) object: M,
}

impl<M: RowMatrixTrait + ColumnMatrixTrait> From<M> for BidirectionalMatrix<M> {
    fn from(matrix: M) -> Self {
        Self { object: matrix }
    }
}

impl<M: RowMatrixTrait + ColumnMatrixTrait> std::ops::Deref for BidirectionalMatrix<M> {
    type Target = M;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<M: RowMatrixTrait + ColumnMatrixTrait> std::ops::DerefMut for BidirectionalMatrix<M> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<M: RowMatrixTrait + ColumnMatrixTrait> BidirectionalMatrix<M> {
    #[inline(always)]
    pub fn row(&self, i: usize) -> SequentialVectorWrapper<impl SequentialVectorTrait + '_> {
        SequentialVectorWrapper {
            object: impls::VectorView::new(self.object.dimension()[COLUMN], self.object.iter_row(i)),
        }
    }
    #[inline(always)]
    pub fn column(&self, j: usize) -> SequentialVectorWrapper<impl SequentialVectorTrait + '_> {
        SequentialVectorWrapper {
            object: impls::VectorView::new(self.object.dimension()[ROW], self.object.iter_column(j)),
        }
    }
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn T(&self) -> BidirectionalMatrix<impl RowMatrixTrait + ColumnMatrixTrait + '_> {
        BidirectionalMatrix { object: impls::TransposedMatrix::new(&self.object) }
    }
    #[inline(always)]
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

impl<M: RowMatrixTrait + ColumnMatrixTrait + SequentialMutMatrixTrait> BidirectionalMatrix<M> {
    #[inline(always)]
    pub fn generate_from_iter<I: Iterator<Item = ([usize; 2], f64)>>(
        dimension: [usize; 2],
        nonzero_elements: I,
    ) -> Self {
        Self { object: M::generate_from_iter(dimension, nonzero_elements) }
    }
    #[inline(always)]
    pub fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(
        &mut self,
        dimension: [usize; 2],
        nonzero_elements: I,
    ) {
        self.object.replace_by_iter(dimension, nonzero_elements);
    }
}

pub type CompressedMatrix = SequentialMatrix<impls::CompressedMatrix>;

pub type CRSMatrix = RowMatrix<impls::CRSMatrix>;

pub type CCSMatrix = ColumnMatrix<impls::CCSMatrix>;

pub type SparseMatrix = BidirectionalMatrix<impls::SparseMatrix>;

fn debug_vector(f: &mut std::fmt::Formatter, matrix: &impl SequentialMatrixTrait) -> std::fmt::Result {
    write!(f, "{{ dimension = [{}, {}], values = [", matrix.dimension()[ROW], matrix.dimension()[COLUMN])?;
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
