use crate::{
    ColumnMatrixTrait, RandomMutVectorTrait, RandomVectorTrait, RowMatrixTrait, SequentialVectorTrait, VectorView,
    COLUMN, ROW,
};

pub fn mul_random_vector_and_sequential_vector(lhs: &impl RandomVectorTrait, rhs: &impl SequentialVectorTrait) -> f64 {
    debug_assert!(lhs.dimension() == rhs.dimension());
    let mut x = 0.0;
    for (i, r) in rhs.iter() {
        let l = lhs.get(i);
        x = f64::mul_add(l, r, x);
    }
    return x;
}

pub fn mul_sequential_vector_and_random_vector(lhs: &impl SequentialVectorTrait, rhs: &impl RandomVectorTrait) -> f64 {
    debug_assert!(lhs.dimension() == rhs.dimension());
    let mut x = 0.0;
    for (i, l) in lhs.iter() {
        let r = rhs.get(i);
        x = f64::mul_add(l, r, x);
    }
    return x;
}

pub fn mul_random_vector_and_random_vector(lhs: &impl RandomVectorTrait, rhs: &impl RandomVectorTrait) -> f64 {
    debug_assert!(lhs.dimension() == rhs.dimension());
    if lhs.estimated_number_of_nonzeros() <= rhs.estimated_number_of_nonzeros() {
        return mul_sequential_vector_and_random_vector(lhs, rhs);
    } else {
        return mul_random_vector_and_sequential_vector(lhs, rhs);
    }
}

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
    let [m, n] = rhs_matrix.dimension();
    debug_assert!(rhs_vector.dimension() == n);
    lhs.replace_by_iter(m, [].into_iter());
    add_assign_column_matrix_multiplied_vector(lhs, rhs_matrix, rhs_vector);
}

pub fn add_assign_column_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &impl ColumnMatrixTrait,
    rhs_vector: &impl SequentialVectorTrait,
) {
    let [m, n] = rhs_matrix.dimension();
    debug_assert!(lhs.dimension() == m);
    debug_assert!(rhs_vector.dimension() == n);
    for (j, a) in rhs_vector.iter() {
        add_assign_scalar_multipled_sequential_vector(lhs, a, &VectorView::new(m, rhs_matrix.iter_column(j)));
    }
}

pub fn sub_assign_column_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &impl ColumnMatrixTrait,
    rhs_vector: &impl SequentialVectorTrait,
) {
    let [m, n] = rhs_matrix.dimension();
    debug_assert!(lhs.dimension() == m);
    debug_assert!(rhs_vector.dimension() == n);
    for (j, a) in rhs_vector.iter() {
        add_assign_scalar_multipled_sequential_vector(lhs, -a, &VectorView::new(m, rhs_matrix.iter_column(j)));
    }
}

pub fn assign_row_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &impl RowMatrixTrait,
    rhs_vector: &impl RandomVectorTrait,
) {
    debug_assert!(rhs_matrix.dimension()[COLUMN] == rhs_vector.dimension());
    lhs.replace_by_iter(rhs_matrix.dimension()[ROW], [].into_iter());
    add_assign_row_matrix_multiplied_vector(lhs, rhs_matrix, rhs_vector);
}

pub fn add_assign_row_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &impl RowMatrixTrait,
    rhs_vector: &impl RandomVectorTrait,
) {
    let [m, n] = rhs_matrix.dimension();
    debug_assert!(lhs.dimension() == m);
    debug_assert!(rhs_vector.dimension() == n);
    for i in 0..m {
        let x = mul_sequential_vector_and_random_vector(&VectorView::new(n, rhs_matrix.iter_row(i)), rhs_vector);
        if x != 0.0 {
            lhs[i] += x;
        }
    }
}

pub fn sub_assign_row_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &impl RowMatrixTrait,
    rhs_vector: &impl RandomVectorTrait,
) {
    let [m, n] = rhs_matrix.dimension();
    debug_assert!(lhs.dimension() == m);
    debug_assert!(rhs_vector.dimension() == n);
    for i in 0..m {
        let x = mul_sequential_vector_and_random_vector(&VectorView::new(n, rhs_matrix.iter_row(i)), rhs_vector);
        if x != 0.0 {
            lhs[i] -= x;
        }
    }
}

pub fn assign_bidirectional_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &(impl RowMatrixTrait + ColumnMatrixTrait),
    rhs_vector: &impl RandomVectorTrait,
) {
    let [_, n] = rhs_matrix.dimension();
    debug_assert!(rhs_vector.dimension() == n);
    if rhs_vector.iter().size_hint().1.is_some_and(|nz| nz <= n / 2) {
        assign_column_matrix_multiplied_vector(lhs, rhs_matrix, rhs_vector);
    } else {
        assign_row_matrix_multiplied_vector(lhs, rhs_matrix, rhs_vector);
    }
}

pub fn add_assign_bidirectional_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &(impl RowMatrixTrait + ColumnMatrixTrait),
    rhs_vector: &impl RandomVectorTrait,
) {
    let [m, n] = rhs_matrix.dimension();
    debug_assert!(lhs.dimension() == m);
    debug_assert!(rhs_vector.dimension() == n);
    if rhs_vector.iter().size_hint().1.is_some_and(|nz| nz <= n / 2) {
        add_assign_column_matrix_multiplied_vector(lhs, rhs_matrix, rhs_vector);
    } else {
        add_assign_row_matrix_multiplied_vector(lhs, rhs_matrix, rhs_vector);
    }
}

pub fn sub_assign_bidirectional_matrix_multiplied_vector(
    lhs: &mut impl RandomMutVectorTrait,
    rhs_matrix: &(impl RowMatrixTrait + ColumnMatrixTrait),
    rhs_vector: &impl RandomVectorTrait,
) {
    let [m, n] = rhs_matrix.dimension();
    debug_assert!(lhs.dimension() == m);
    debug_assert!(rhs_vector.dimension() == n);
    if rhs_vector.iter().size_hint().1.is_some_and(|nz| nz <= n / 2) {
        sub_assign_column_matrix_multiplied_vector(lhs, rhs_matrix, rhs_vector);
    } else {
        sub_assign_row_matrix_multiplied_vector(lhs, rhs_matrix, rhs_vector);
    }
}
