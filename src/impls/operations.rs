use crate::{ColumnMatrixTrait, RandomMutVectorTrait, SequentialVectorTrait, VectorView, COLUMN, ROW};

pub fn assign_sequential_vector(lhs: &mut impl RandomMutVectorTrait, rhs: &impl SequentialVectorTrait) {
    lhs.replace_by_iter(rhs.dimension(), rhs.iter());
}

pub fn add_assign_sequential_vector(lhs: &mut impl RandomMutVectorTrait, rhs: &impl SequentialVectorTrait) {
    debug_assert!(lhs.dimension() == rhs.dimension());
    for (i, x) in rhs.iter() {
        lhs[i] += x;
    }
}

pub fn sub_assign_sequential_vector(lhs: &mut impl RandomMutVectorTrait, rhs: &impl SequentialVectorTrait) {
    debug_assert!(lhs.dimension() == rhs.dimension());
    for (i, x) in rhs.iter() {
        lhs[i] -= x;
    }
}

pub fn add_assign_scalar_multipled_sequential_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_scalar: f64,
    rhs_vector: &impl SequentialVectorTrait,
) {
    debug_assert!(lhs.dimension() == rhs_vector.dimension());
    for (i, x) in rhs_vector.iter() {
        lhs[i] = f64::mul_add(rhs_scalar, x, lhs[i]);
    }
}

pub fn assign_column_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &impl ColumnMatrixTrait,
    rhs_vector: &impl SequentialVectorTrait,
) {
    assert!(rhs_matrix.dimension()[COLUMN] == rhs_vector.dimension());
    lhs.replace_by_iter(rhs_matrix.dimension()[ROW], [].into_iter());
    add_assign_column_matrix_multiplied_vector(lhs, rhs_matrix, rhs_vector);
}

pub fn add_assign_column_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &impl ColumnMatrixTrait,
    rhs_vector: &impl SequentialVectorTrait,
) {
    assert!(lhs.dimension() == rhs_matrix.dimension()[ROW]);
    assert!(rhs_matrix.dimension()[COLUMN] == rhs_vector.dimension());
    for (j, a) in rhs_vector.iter() {
        add_assign_scalar_multipled_sequential_vector(lhs, a, &VectorView::new(rhs_matrix.dimension()[ROW], rhs_matrix.iter_column(j)));
    }
}

pub fn sub_assign_column_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &impl ColumnMatrixTrait,
    rhs_vector: &impl SequentialVectorTrait,
) {
    debug_assert!(lhs.dimension() == rhs_matrix.dimension()[ROW]);
    debug_assert!(rhs_matrix.dimension()[COLUMN] == rhs_vector.dimension());
    for (j, a) in rhs_vector.iter() {
        add_assign_scalar_multipled_sequential_vector(lhs, -a, &VectorView::new(rhs_matrix.dimension()[ROW],  rhs_matrix.iter_column(j)));
    }
}
