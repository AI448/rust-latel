use crate::{
    traits::{LazyVectorTrait, RandomMutVectorTrait},
    types::{COLUMN, ROW},
    ColumnMatrixTrait, SequentialVectorTrait, VectorTrait,
};

// RandomMutVectorTrait += ColumnMatrixTrait * SequencialVectorTrait
fn add_assign_column_matrix_multiplied_vector(
    lhs_vector: &mut impl RandomMutVectorTrait,
    rhs_matrix: &impl ColumnMatrixTrait,
    rhs_vector: &impl SequentialVectorTrait,
) {
    assert!(lhs_vector.dimension() == rhs_matrix.dimension()[ROW]);
    assert!(rhs_matrix.dimension()[COLUMN] == rhs_vector.dimension());

    for (j, y) in rhs_vector.iter() {
        for (i, x) in rhs_matrix.column(j).iter() {
            lhs_vector[i] += x * y;
        }
    }
}

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
}

impl<M: ColumnMatrixTrait, V: SequentialVectorTrait> LazyVectorTrait for ColumnMatrixMultipliedVector<M, V> {
    fn add_assign_to(&self, vector: &mut impl RandomMutVectorTrait) {
        add_assign_column_matrix_multiplied_vector(vector, &self.matrix, &self.vector);
    }
}
