use crate::traits::{ColumnMatrixTrait, RandomVectorTrait, RowMatrixTrait, SequentialVectorTrait};
use crate::wrappers::{
    BidirectionalMatrix, ColumnMatrix, RandomVectorWrapper, RowMatrix, SequentialMatrix, SequentialVectorWrapper,
    VectorWrapper,
};

macro_rules! impl_from {
    (
        $lhs_wrapper: ident,
        $rhs_wrapper: ident, $rhs_trait1: ident $(+ $rhs_trait2: ident)*
    ) => {
        impl<V: $rhs_trait1 $(+ $rhs_trait2)*> From<$rhs_wrapper<V>> for $lhs_wrapper<V> {
            #[inline(always)]
            fn from(rhs: $rhs_wrapper<V>) -> Self {
                Self{ object: rhs.object }
            }
        }

        impl<'a, V: $rhs_trait1 $(+ $rhs_trait2)*> From<&'a $rhs_wrapper<V>> for $lhs_wrapper<&'a V> {
            #[inline(always)]
            fn from(rhs: &'a $rhs_wrapper<V>) -> Self {
                Self{ object: &rhs.object }
            }
        }
    }
}

impl_from!(VectorWrapper, SequentialVectorWrapper, SequentialVectorTrait);
impl_from!(VectorWrapper, RandomVectorWrapper, RandomVectorTrait);

impl_from!(SequentialVectorWrapper, RandomVectorWrapper, RandomVectorTrait);

impl_from!(SequentialMatrix, RowMatrix, RowMatrixTrait);
impl_from!(SequentialMatrix, ColumnMatrix, ColumnMatrixTrait);
impl_from!(SequentialMatrix, BidirectionalMatrix, RowMatrixTrait + ColumnMatrixTrait);

impl_from!(RowMatrix, BidirectionalMatrix, RowMatrixTrait + ColumnMatrixTrait);

impl_from!(ColumnMatrix, BidirectionalMatrix, RowMatrixTrait + ColumnMatrixTrait);
