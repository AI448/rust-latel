use super::SequentialVectorTrait;

pub trait MatrixTrait {
    fn dimension(&self) -> [usize; 2];
}

// NOTE: ベクトル同様に into_iter(self) が可能な行列があってもいいのかも

/// シーケンシャルアクセス可能な行列
pub trait SequentialMatrixTrait: MatrixTrait {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_;
}

/// 行にランダムアクセス可能な行列
pub trait RowMatrixTrait: SequentialMatrixTrait {
    fn row(&self, i: usize) -> impl SequentialVectorTrait + '_;
}

/// 列にランダムアクセス可能な行列
pub trait ColumnMatrixTrait: SequentialMatrixTrait {
    fn column(&self, j: usize) -> impl SequentialVectorTrait + '_;
}

/// 変更可能な行列
pub trait SequentialMutMatrixTrait: SequentialMatrixTrait + Default {
    fn generate_from_iter<I: Iterator<Item = ([usize; 2], f64)>>(dimension: [usize; 2], nonzero_elements: I) -> Self {
        let mut matrix = Self::default();
        matrix.replace_by_iter(dimension, nonzero_elements);
        return matrix;
    }

    fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(&mut self, dimension: [usize; 2], nonzero_elements: I);
}

impl<M: MatrixTrait> MatrixTrait for &M {
    fn dimension(&self) -> [usize; 2] {
        (*self).dimension()
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
