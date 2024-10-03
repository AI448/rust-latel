use super::SequentialVectorTrait;
use crate::types::Direction;

pub trait MatrixTrait {
    fn dimension<const D: Direction>(&self) -> usize;
}

// NOTE: ベクトル同様に into_iter(self) が可能な行列があってもいいのかも

pub trait SequentialMatrixTrait: MatrixTrait {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_;
}

pub trait RowMatrixTrait: SequentialMatrixTrait {
    fn row(&self, i: usize) -> impl SequentialVectorTrait + '_;
}

pub trait ColumnMatrixTrait: SequentialMatrixTrait {
    fn column(&self, j: usize) -> impl SequentialVectorTrait + '_;
}

impl<M: MatrixTrait> MatrixTrait for &M {
    fn dimension<const D: Direction>(&self) -> usize {
        (*self).dimension::<{ D }>()
    }
}

impl<M: SequentialMatrixTrait> SequentialMatrixTrait for &M {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        (*self).iter()
    }
}

impl<M: RowMatrixTrait> RowMatrixTrait for &M {
    fn row(&self, i: usize) -> impl SequentialVectorTrait + '_ {
        (*self).row(i)
    }
}

impl<M: ColumnMatrixTrait> ColumnMatrixTrait for &M {
    fn column(&self, j: usize) -> impl SequentialVectorTrait + '_ {
        (*self).column(j)
    }
}
