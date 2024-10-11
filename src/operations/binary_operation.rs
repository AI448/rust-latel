use crate::impls::{
    ColumnMatrixMultipliedVector, ColumnPermutatedMatrix, MappedVector, PermutatedPermutator, PermutatedVector,
    RowPermutatedMatrix,
};
use crate::traits::{
    ColumnMatrixTrait, LazyVectorTrait, PermutatorTrait, RandomMutVectorTrait, RandomVectorTrait, RowMatrixTrait,
    SequentialMatrixTrait, SequentialVectorTrait,
};
use crate::wrappers::{
    BidirectionalMatrix, ColumnMatrix, Permutator, RandomVector, RowMatrix, SequentialMatrix, SequentialVector,
};
use crate::LazyVector;
use std::ops::{Div, Mul, Neg};

macro_rules! impl_nonscalar_unary_operation {
    (
        $op_trait: ident, $op_func: ident,
        $wrapper: ident, $trait: ident,
        $result_wrapper: ident, $result_trait: ident,
        $closure: expr
    ) => {
        impl<V: $trait> $op_trait for $wrapper<V> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self) -> Self::Output {
                $result_wrapper { object: $closure(self.object) }
            }
        }
        impl<V: $trait> $op_trait for &$wrapper<V> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self) -> Self::Output {
                $result_wrapper { object: $closure(&self.object) }
            }
        }
    };
}

macro_rules! impl_scalar_nonscalar_binary_operation {
    (
        $op_trait: ident, $op_func: ident,
        $rhs_wrapper: ident, $rhs_trait: ident,
        $result_wrapper: ident, $result_trait: ident,
        $closure: expr
    ) => {
        impl<V: $rhs_trait> $op_trait<$rhs_wrapper<V>> for f64 {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn mul(self, rhs: $rhs_wrapper<V>) -> Self::Output {
                $result_wrapper { object: $closure(self, rhs.object) }
            }
        }
        // impl<V: $rhs_trait> $op_trait<$rhs_wrapper<V>> for &f64 {
        //     type Output = $result_wrapper<impl $result_trait>;
        //     #[inline(always)]
        //     fn mul(self, rhs: $rhs_wrapper<V>) -> Self::Output {
        //         $result_wrapper { object: $closure(*self, rhs.object) }
        //     }
        // }
        impl<'a, V: $rhs_trait> $op_trait<&'a $rhs_wrapper<V>> for f64 {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn mul(self, rhs: &'a $rhs_wrapper<V>) -> Self::Output {
                $result_wrapper { object: $closure(self, &rhs.object) }
            }
        }
        // impl<'a, V: $rhs_trait> $op_trait<&'a $rhs_wrapper<V>> for &f64 {
        //     type Output = $result_wrapper<impl $result_trait>;
        //     #[inline(always)]
        //     fn mul(self, rhs: &'a $rhs_wrapper<V>) -> Self::Output {
        //         $result_wrapper { object: $closure(*self, &rhs.object) }
        //     }
        // }
    };
}

macro_rules! impl_nonscalar_scalar_binary_operation {
    (
        $op_trait: ident, $op_func: ident,
        $lhs_wrapper: ident, $lhs_trait: ident,
        $result_wrapper: ident, $result_trait: ident,
        $closure: expr
    ) => {
        impl<V: $lhs_trait> $op_trait<f64> for $lhs_wrapper<V> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self, rhs: f64) -> Self::Output {
                $result_wrapper { object: $closure(self.object, rhs) }
            }
        }
        // impl<V: $lhs_trait> $op_trait<f64> for &$lhs_wrapper<V> {
        //     type Output = $result_wrapper<impl $result_trait>;
        //     #[inline(always)]
        //     fn $op_func(self, rhs: f64) -> Self::Output {
        //         $result_wrapper { object: $closure(&self.object, rhs) }
        //     }
        // }
        impl<V: $lhs_trait> $op_trait<&f64> for $lhs_wrapper<V> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self, rhs: &f64) -> Self::Output {
                $result_wrapper { object: $closure(self.object, *rhs) }
            }
        }
        // impl<V: $lhs_trait> $op_trait<&f64> for &$lhs_wrapper<V> {
        //     type Output = $result_wrapper<impl $result_trait>;
        //     #[inline(always)]
        //     fn $op_func(self, rhs: &f64) -> Self::Output {
        //         $result_wrapper { object: $closure(&self.object, *rhs) }
        //     }
        // }
    };
}

macro_rules! impl_nonscalar_nonscalar_binary_operation {
    (
        $op_trait: ident, $op_func: ident,
        $lhs_wrapper: ident, $lhs_trait1: ident $(+ $lhs_trait2: ident)*, // NOTE: この書き方しかできたいというのは中々に耐え難い
        $rhs_wrapper: ident, $rhs_trait1: ident $(+ $rhs_trait2: ident)*, // 耐え難い...
        $result_wrapper: ident, $result_trait: ident,
        $closure: expr
    ) => {
        impl<L: $lhs_trait1 $(+ $lhs_trait2)*, R: $rhs_trait1 $(+ $rhs_trait2)*> $op_trait<$rhs_wrapper<R>> for $lhs_wrapper<L> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self, rhs: $rhs_wrapper<R>) -> Self::Output {
                $result_wrapper { object: $closure(self.object, rhs.object) }
            }
        }
        impl<L: $lhs_trait1 $(+ $lhs_trait2)*, R: $rhs_trait1 $(+ $rhs_trait2)*> $op_trait<$rhs_wrapper<R>> for &$lhs_wrapper<L> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self, rhs: $rhs_wrapper<R>) -> Self::Output {
                $result_wrapper { object: $closure(&self.object, rhs.object) }
            }
        }
        impl<'a, L: $lhs_trait1 $(+ $lhs_trait2)*, R: $rhs_trait1 $(+ $rhs_trait2)*> $op_trait<&'a $rhs_wrapper<R>> for $lhs_wrapper<L> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self, rhs: &'a $rhs_wrapper<R>) -> Self::Output {
                $result_wrapper { object: $closure(self.object, &rhs.object) }
            }
        }
        impl<'a, L: $lhs_trait1 $(+ $lhs_trait2)*, R: $rhs_trait1 $(+ $rhs_trait2)*> $op_trait<&'a $rhs_wrapper<R>> for &$lhs_wrapper<L> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self, rhs: &'a $rhs_wrapper<R>) -> Self::Output {
                $result_wrapper { object: $closure(&self.object, &rhs.object) }
            }
        }
    };
}

// -SequentialVector -> SequentialVector
impl_nonscalar_unary_operation!(
    Neg,
    neg,
    SequentialVector,
    SequentialVectorTrait,
    SequentialVector,
    SequentialVectorTrait,
    |vector| MappedVector::new(|x| -x, vector)
);

// -RandomVector -> RandomVector
impl_nonscalar_unary_operation!(Neg, neg, RandomVector, RandomVectorTrait, RandomVector, RandomVectorTrait, |vector| {
    MappedVector::new(|x| -x, vector)
});

// Scalar * SequentialVector -> SequentialVector
impl_scalar_nonscalar_binary_operation!(
    Mul,
    mul,
    SequentialVector,
    SequentialVectorTrait,
    SequentialVector,
    SequentialVectorTrait,
    |lhs, rhs| MappedVector::new(move |x| lhs * x, rhs)
);

// Scalar * RandomVector -> RandomVector
impl_scalar_nonscalar_binary_operation!(
    Mul,
    mul,
    RandomVector,
    RandomVectorTrait,
    RandomVector,
    RandomVectorTrait,
    |lhs, rhs| MappedVector::new(move |x| lhs * x, rhs)
);

// SequentialVector * Scalar -> SequentialVector
impl_nonscalar_scalar_binary_operation!(
    Mul,
    mul,
    SequentialVector,
    SequentialVectorTrait,
    SequentialVector,
    SequentialVectorTrait,
    |vector, scalar| MappedVector::new(move |x| x * scalar, vector)
);

// RandomVector * Scalar -> RandomVector
impl_nonscalar_scalar_binary_operation!(
    Mul,
    mul,
    RandomVector,
    RandomVectorTrait,
    RandomVector,
    RandomVectorTrait,
    |vector, scalar| MappedVector::new(move |x| x * scalar, vector)
);

// SequentialVector / Scalar -> SequentialVector
impl_nonscalar_scalar_binary_operation!(
    Div,
    div,
    SequentialVector,
    SequentialVectorTrait,
    SequentialVector,
    SequentialVectorTrait,
    |vector, scalar| MappedVector::new(move |x| x / scalar, vector)
);

// RandomVector / Scalar -> SequentialVector
impl_nonscalar_scalar_binary_operation!(
    Div,
    div,
    RandomVector,
    RandomVectorTrait,
    RandomVector,
    RandomVectorTrait,
    |vector, scalar| MappedVector::new(move |x| x / scalar, vector)
);

// Vector * Vector

fn mul_random_vector_and_sequential_vector(lhs: &impl RandomVectorTrait, rhs: &impl SequentialVectorTrait) -> f64 {
    let mut z = 0.0;
    for (i, y) in rhs.iter() {
        let x = lhs.get(i);
        z = x.mul_add(y, z);
    }
    return z;
}

fn mul_sequential_vector_and_random_vector(lhs: &impl SequentialVectorTrait, rhs: &impl RandomVectorTrait) -> f64 {
    let mut z = 0.0;
    for (i, x) in lhs.iter() {
        let y = rhs.get(i);
        z = x.mul_add(y, z);
    }
    return z;
}

impl<U: RandomVectorTrait, V: SequentialVectorTrait> std::ops::Mul<&SequentialVector<V>> for &RandomVector<U> {
    type Output = f64;
    fn mul(self, rhs: &SequentialVector<V>) -> Self::Output {
        mul_random_vector_and_sequential_vector(&self.object, &rhs.object)
    }
}

impl<U: SequentialVectorTrait, V: RandomVectorTrait> std::ops::Mul<&RandomVector<V>> for &SequentialVector<U> {
    type Output = f64;
    fn mul(self, rhs: &RandomVector<V>) -> Self::Output {
        mul_sequential_vector_and_random_vector(&self.object, &rhs.object)
    }
}

impl<U: RandomVectorTrait, V: RandomVectorTrait> std::ops::Mul<&RandomVector<V>> for &RandomVector<U> {
    type Output = f64;
    fn mul(self, rhs: &RandomVector<V>) -> Self::Output {
        if self.iter().size_hint().0 <= rhs.iter().size_hint().0 {
            mul_sequential_vector_and_random_vector(&self.object, &rhs.object)
        } else {
            mul_random_vector_and_sequential_vector(&self.object, &rhs.object)
        }
    }
}

// Permutator * SequentialVector -> SequentialVector
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    Permutator,
    PermutatorTrait,
    SequentialVector,
    SequentialVectorTrait,
    SequentialVector,
    SequentialVectorTrait,
    |permutator, vector| PermutatedVector::new(permutator, vector)
);

// Permutator * RandomVector -> RandomVector
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    Permutator,
    PermutatorTrait,
    RandomVector,
    RandomVectorTrait,
    RandomVector,
    RandomVectorTrait,
    |permutator, vector| PermutatedVector::new(permutator, vector)
);

// Permutator * Permutator -> Permutator
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    Permutator,
    PermutatorTrait,
    Permutator,
    PermutatorTrait,
    Permutator,
    PermutatorTrait,
    |permutator1, permutator2| PermutatedPermutator::new(permutator1, permutator2)
);

// Permutator * SequentialMatrix -> SequentialMatrix
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    Permutator,
    PermutatorTrait,
    SequentialMatrix,
    SequentialMatrixTrait,
    SequentialMatrix,
    SequentialMatrixTrait,
    |permutator, matrix| RowPermutatedMatrix::new(permutator, matrix)
);

// Permutator * RowMatrix -> SequentialMatrix
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    Permutator,
    PermutatorTrait,
    RowMatrix,
    RowMatrixTrait,
    SequentialMatrix,
    SequentialMatrixTrait,
    |permutator, matrix| RowPermutatedMatrix::new(permutator, matrix)
);

// Permutator * ColumnMatrix -> SequentialMatrix
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    Permutator,
    PermutatorTrait,
    ColumnMatrix,
    ColumnMatrixTrait,
    SequentialMatrix,
    SequentialMatrixTrait,
    |permutator, matrix| RowPermutatedMatrix::new(permutator, matrix)
);

// Permutator * BidirectionalMatrix -> SequentialMatrix
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    Permutator,
    PermutatorTrait,
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait,
    SequentialMatrix,
    SequentialMatrixTrait,
    |permutator, matrix| RowPermutatedMatrix::new(permutator, matrix)
);

// SequentialMatrix * Permutator -> SequentialMatrix
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    SequentialMatrix,
    SequentialMatrixTrait,
    Permutator,
    PermutatorTrait,
    SequentialMatrix,
    SequentialMatrixTrait,
    |matrix, permutator| ColumnPermutatedMatrix::new(matrix, permutator)
);

// RowMatrix * Permutator -> SequentialMatrix
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    RowMatrix,
    RowMatrixTrait,
    Permutator,
    PermutatorTrait,
    SequentialMatrix,
    SequentialMatrixTrait,
    |matrix, permutator| ColumnPermutatedMatrix::new(matrix, permutator)
);

// ColumnMatrix * Permutator -> SequentialMatrix
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    ColumnMatrix,
    ColumnMatrixTrait,
    Permutator,
    PermutatorTrait,
    SequentialMatrix,
    SequentialMatrixTrait,
    |matrix, permutator| ColumnPermutatedMatrix::new(matrix, permutator)
);

// BidirectionalMatrix * Permutator -> SequentialMatrix
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait,
    Permutator,
    PermutatorTrait,
    SequentialMatrix,
    SequentialMatrixTrait,
    |matrix, permutator| ColumnPermutatedMatrix::new(matrix, permutator)
);

impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    ColumnMatrix,
    ColumnMatrixTrait,
    SequentialVector,
    SequentialVectorTrait,
    LazyVector,
    LazyVectorTrait,
    |matrix, vector| ColumnMatrixMultipliedVector::new(matrix, vector)
);

impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    ColumnMatrix,
    ColumnMatrixTrait,
    RandomVector,
    RandomVectorTrait,
    LazyVector,
    LazyVectorTrait,
    |matrix, vector| ColumnMatrixMultipliedVector::new(matrix, vector)
);

// macro_rules! impl_vector_assignment {
//     (
//         $lhs_wrapper: ident, $lhs_trait1: ident $(+ $lhs_trait2: ident)*,
//         $rhs_wrapper: ident, $rhs_trait1: ident $(+ $rhs_trait2: ident)*
//     ) => {
//         impl<L: $lhs_trait1 $(+$lhs_trait2)* + AssignableVectorTrait<R>, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::ShlAssign<$rhs_wrapper<R>> for $lhs_wrapper<L> {
//             fn shl_assign(&mut self, rhs: $rhs_wrapper<R>) {
//                 self.object.assign(rhs.object);
//             }
//         }
//         impl<'a, L: $lhs_trait1 $(+$lhs_trait2)* + AssignableVectorTrait<&'a R>, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::ShlAssign<&'a $rhs_wrapper<R>> for $lhs_wrapper<L> {
//             fn shl_assign(&mut self, rhs: &'a $rhs_wrapper<R>) {
//                 self.object.assign(&rhs.object);
//             }
//         }
//         impl<L: $lhs_trait1 $(+$lhs_trait2)* + AssignableVectorTrait<R>, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::AddAssign<$rhs_wrapper<R>> for $lhs_wrapper<L> {
//             fn add_assign(&mut self, rhs: $rhs_wrapper<R>) {
//                 self.object.add_assign(rhs.object);
//             }
//         }
//         impl<'a, L: $lhs_trait1 $(+$lhs_trait2)* + AssignableVectorTrait<&'a R>, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::AddAssign<&'a $rhs_wrapper<R>> for $lhs_wrapper<L> {
//             fn add_assign(&mut self, rhs: &'a $rhs_wrapper<R>) {
//                 self.object.add_assign(&rhs.object);
//             }
//         }
//         impl<L: $lhs_trait1 $(+$lhs_trait2)* + AssignableVectorTrait<R>, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::SubAssign<$rhs_wrapper<R>> for $lhs_wrapper<L> {
//             fn sub_assign(&mut self, rhs: $rhs_wrapper<R>) {
//                 self.object.sub_assign(rhs.object);
//             }
//         }
//         impl<'a, L: $lhs_trait1 $(+$lhs_trait2)* + AssignableVectorTrait<&'a R>, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::SubAssign<&'a $rhs_wrapper<R>> for $lhs_wrapper<L> {
//             fn sub_assign(&mut self, rhs: &'a $rhs_wrapper<R>) {
//                 self.object.sub_assign(&rhs.object);
//             }
//         }
//     };
// }

// impl_vector_assignment!(RandomVector, RandomVectorTrait, SequentialVector, SequentialVectorTrait);

// impl_vector_assignment!(RandomVector, RandomVectorTrait, RandomVector, RandomVectorTrait);

macro_rules! impl_add_assign {
    (
        $lhs_wrapper: ident, $lhs_trait1: ident $(+ $lhs_trait2: ident)*,
        $rhs_wrapper: ident, $rhs_trait1: ident $(+ $rhs_trait2: ident)*
    ) => {
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::AddAssign<$rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn add_assign(&mut self, rhs: $rhs_wrapper<R>) {
                assert!(self.dimension() == rhs.dimension());
                for (key, value) in rhs.object.iter() {
                    self.object[key] += value;
                }
            }
        }
        impl<'a, L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::AddAssign<&'a $rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn add_assign(&mut self, rhs: &'a $rhs_wrapper<R>) {
                assert!(self.dimension() == rhs.dimension());
                for (key, value) in rhs.object.iter() {
                    self.object[key] += value;
                }
            }
        }
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::SubAssign<$rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn sub_assign(&mut self, rhs: $rhs_wrapper<R>) {
                assert!(self.dimension() == rhs.dimension());
                for (key, value) in rhs.object.iter() {
                    self.object[key] -= value;
                }
            }
        }
        impl<'a, L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::SubAssign<&'a $rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn sub_assign(&mut self, rhs: &'a $rhs_wrapper<R>) {
                assert!(self.dimension() == rhs.dimension());
                for (key, value) in rhs.object.iter() {
                    self.object[key] -= value;
                }
            }
        }
    };
}

impl_add_assign!(RandomVector, RandomMutVectorTrait, SequentialVector, SequentialVectorTrait);

impl_add_assign!(RandomVector, RandomMutVectorTrait, RandomVector, RandomVectorTrait);

impl<L: RandomMutVectorTrait, R: LazyVectorTrait> std::ops::AddAssign<LazyVector<R>> for RandomVector<L> {
    fn add_assign(&mut self, rhs: LazyVector<R>) {
        rhs.object.add_assign_to(&mut self.object);
    }
}
