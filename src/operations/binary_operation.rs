use crate::impls::{
    ColumnMatrixMultipliedVector, ColumnPermutatedMatrix, MappedVector, PermutatedPermutator, PermutatedVector,
    RowPermutatedMatrix,
};
use crate::traits::{
    ColumnMatrixTrait, PermutatorTrait, RandomVectorTrait, RowMatrixTrait, SequentialMatrixTrait,
    SequentialVectorTrait, VectorTrait,
};
use crate::wrappers::{
    BidirectionalMatrix, ColumnMatrix, Permutator, RandomVectorWrapper, RowMatrix, SequentialMatrix,
    SequentialVectorWrapper, VectorWrapper,
};

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
        impl<V: $lhs_trait> $op_trait<f64> for &$lhs_wrapper<V> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self, rhs: f64) -> Self::Output {
                $result_wrapper { object: $closure(&self.object, rhs) }
            }
        }
        impl<V: $lhs_trait> $op_trait<&f64> for $lhs_wrapper<V> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self, rhs: &f64) -> Self::Output {
                $result_wrapper { object: $closure(self.object, *rhs) }
            }
        }
        impl<V: $lhs_trait> $op_trait<&f64> for &$lhs_wrapper<V> {
            type Output = $result_wrapper<impl $result_trait>;
            #[inline(always)]
            fn $op_func(self, rhs: &f64) -> Self::Output {
                $result_wrapper { object: $closure(&self.object, *rhs) }
            }
        }
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
    SequentialVectorWrapper,
    SequentialVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait,
    |vector| MappedVector::new(|x| -x, vector)
);

// -RandomVector -> RandomVector
impl_nonscalar_unary_operation!(
    Neg,
    neg,
    RandomVectorWrapper,
    RandomVectorTrait,
    RandomVectorWrapper,
    RandomVectorTrait,
    |vector| { MappedVector::new(|x| -x, vector) }
);

// Scalar * SequentialVector -> SequentialVector
impl_scalar_nonscalar_binary_operation!(
    Mul,
    mul,
    SequentialVectorWrapper,
    SequentialVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait,
    |lhs, rhs| MappedVector::new(move |x| lhs * x, rhs)
);

// Scalar * RandomVector -> RandomVector
impl_scalar_nonscalar_binary_operation!(
    Mul,
    mul,
    RandomVectorWrapper,
    RandomVectorTrait,
    RandomVectorWrapper,
    RandomVectorTrait,
    |lhs, rhs| MappedVector::new(move |x| lhs * x, rhs)
);

// SequentialVector * Scalar -> SequentialVector
impl_nonscalar_scalar_binary_operation!(
    Mul,
    mul,
    SequentialVectorWrapper,
    SequentialVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait,
    |vector, scalar| MappedVector::new(move |x| x * scalar, vector)
);

// RandomVector * Scalar -> RandomVector
impl_nonscalar_scalar_binary_operation!(
    Mul,
    mul,
    RandomVectorWrapper,
    RandomVectorTrait,
    RandomVectorWrapper,
    RandomVectorTrait,
    |vector, scalar| MappedVector::new(move |x| x * scalar, vector)
);

// SequentialVector / Scalar -> SequentialVector
impl_nonscalar_scalar_binary_operation!(
    Div,
    div,
    SequentialVectorWrapper,
    SequentialVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait,
    |vector, scalar| MappedVector::new(move |x| x / scalar, vector)
);

// RandomVector / Scalar -> SequentialVector
impl_nonscalar_scalar_binary_operation!(
    Div,
    div,
    RandomVectorWrapper,
    RandomVectorTrait,
    RandomVectorWrapper,
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

impl<U: RandomVectorTrait, V: SequentialVectorTrait> std::ops::Mul<&SequentialVectorWrapper<V>>
    for &RandomVectorWrapper<U>
{
    type Output = f64;
    fn mul(self, rhs: &SequentialVectorWrapper<V>) -> Self::Output {
        mul_random_vector_and_sequential_vector(&self.object, &rhs.object)
    }
}

impl<U: SequentialVectorTrait, V: RandomVectorTrait> std::ops::Mul<&RandomVectorWrapper<V>>
    for &SequentialVectorWrapper<U>
{
    type Output = f64;
    fn mul(self, rhs: &RandomVectorWrapper<V>) -> Self::Output {
        mul_sequential_vector_and_random_vector(&self.object, &rhs.object)
    }
}

impl<U: RandomVectorTrait, V: RandomVectorTrait> std::ops::Mul<&RandomVectorWrapper<V>> for &RandomVectorWrapper<U> {
    type Output = f64;
    fn mul(self, rhs: &RandomVectorWrapper<V>) -> Self::Output {
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
    SequentialVectorWrapper,
    SequentialVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait,
    |permutator, vector| PermutatedVector::new(permutator, vector)
);

// Permutator * RandomVector -> RandomVector
impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    Permutator,
    PermutatorTrait,
    RandomVectorWrapper,
    RandomVectorTrait,
    RandomVectorWrapper,
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
    SequentialVectorWrapper,
    SequentialVectorTrait,
    VectorWrapper,
    VectorTrait,
    |matrix, vector| ColumnMatrixMultipliedVector::new(matrix, vector)
);

impl_nonscalar_nonscalar_binary_operation!(
    Mul,
    mul,
    ColumnMatrix,
    ColumnMatrixTrait,
    RandomVectorWrapper,
    RandomVectorTrait,
    VectorWrapper,
    VectorTrait,
    |matrix, vector| ColumnMatrixMultipliedVector::new(matrix, vector)
);
