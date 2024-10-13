use crate::{traits::RandomMutVectorTrait, types::ROW, ColumnMatrixTrait, SequentialVectorTrait, VectorTrait};

use super::operations::{
    add_assign_column_matrix_multiplied_vector, assign_column_matrix_multiplied_vector,
    sub_assign_column_matrix_multiplied_vector,
};

pub struct ColumnMatrixMultipliedVector<M: ColumnMatrixTrait, V: SequentialVectorTrait> {
    matrix: M,
    vector: V,
}

impl<M: ColumnMatrixTrait, V: SequentialVectorTrait> ColumnMatrixMultipliedVector<M, V> {
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
