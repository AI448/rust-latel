use crate::{
    traits::RandomMutVectorTrait, types::ROW, ColumnMatrixTrait, RandomVectorTrait, RowMatrixTrait,
    SequentialVectorTrait, VectorTrait,
};

use super::operations::{
    add_assign_bidirectional_matrix_multiplied_vector, add_assign_column_matrix_multiplied_vector,
    add_assign_row_matrix_multiplied_vector, assign_bidirectional_matrix_multiplied_vector,
    assign_column_matrix_multiplied_vector, assign_row_matrix_multiplied_vector,
    sub_assign_bidirectional_matrix_multiplied_vector, sub_assign_column_matrix_multiplied_vector,
    sub_assign_row_matrix_multiplied_vector,
};

/// 列アクセス可能な行列とシーケンシャルアクセス可能なベクトルの積
pub struct ColumnMatrixMultipliedVector<M: ColumnMatrixTrait, V: SequentialVectorTrait> {
    matrix: M,
    vector: V,
}

impl<M: ColumnMatrixTrait, V: SequentialVectorTrait> ColumnMatrixMultipliedVector<M, V> {
    #[inline(always)]
    pub fn new(matrix: M, vector: V) -> Self {
        Self { matrix: matrix, vector: vector }
    }
}

impl<M: ColumnMatrixTrait, V: SequentialVectorTrait> VectorTrait for ColumnMatrixMultipliedVector<M, V> {
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.matrix.dimension()[ROW]
    }
    #[inline(always)]
    fn assign_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        assign_column_matrix_multiplied_vector(lhs, &self.matrix, &self.vector);
    }
    #[inline(always)]
    fn add_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        add_assign_column_matrix_multiplied_vector(lhs, &self.matrix, &self.vector);
    }
    #[inline(always)]
    fn sub_from_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        sub_assign_column_matrix_multiplied_vector(lhs, &self.matrix, &self.vector);
    }
}

pub struct RowMatrixMultipliedVector<M: RowMatrixTrait, V: RandomVectorTrait> {
    matrix: M,
    vector: V,
}

impl<M: RowMatrixTrait, V: RandomVectorTrait> RowMatrixMultipliedVector<M, V> {
    #[inline(always)]
    pub fn new(matrix: M, vector: V) -> Self {
        Self { matrix: matrix, vector: vector }
    }
}

impl<M: RowMatrixTrait, V: RandomVectorTrait> VectorTrait for RowMatrixMultipliedVector<M, V> {
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.matrix.dimension()[ROW]
    }
    #[inline(always)]
    fn assign_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        assign_row_matrix_multiplied_vector(lhs, &self.matrix, &self.vector);
    }
    #[inline(always)]
    fn add_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        add_assign_row_matrix_multiplied_vector(lhs, &self.matrix, &self.vector);
    }
    #[inline(always)]
    fn sub_from_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        sub_assign_row_matrix_multiplied_vector(lhs, &self.matrix, &self.vector);
    }
}

pub struct BidirectionalMatrixMultipliedVector<M: RowMatrixTrait + ColumnMatrixTrait, V: RandomVectorTrait> {
    matrix: M,
    vector: V,
}

impl<M: RowMatrixTrait + ColumnMatrixTrait, V: RandomVectorTrait> BidirectionalMatrixMultipliedVector<M, V> {
    #[inline(always)]
    pub fn new(matrix: M, vector: V) -> Self {
        Self { matrix: matrix, vector: vector }
    }
}

impl<M: RowMatrixTrait + ColumnMatrixTrait, V: RandomVectorTrait> VectorTrait
    for BidirectionalMatrixMultipliedVector<M, V>
{
    #[inline(always)]
    fn dimension(&self) -> usize {
        self.matrix.dimension()[ROW]
    }
    #[inline(always)]
    fn assign_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        assign_bidirectional_matrix_multiplied_vector(lhs, &self.matrix, &self.vector);
    }
    #[inline(always)]
    fn add_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        add_assign_bidirectional_matrix_multiplied_vector(lhs, &self.matrix, &self.vector);
    }
    #[inline(always)]
    fn sub_from_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        sub_assign_bidirectional_matrix_multiplied_vector(lhs, &self.matrix, &self.vector);
    }
}
