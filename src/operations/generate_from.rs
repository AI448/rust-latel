use crate::traits::{
    RandomMutVectorTrait, RandomVectorTrait, SequentialMutMatrixTrait, SequentialMutVectorTrait, SequentialVectorTrait,
    VectorTrait,
};
use crate::wrappers::{BidirectionalMatrix, RandomVectorWrapper, SequentialVectorWrapper, VectorWrapper};
use crate::{ColumnMatrix, ColumnMatrixTrait, RowMatrix, RowMatrixTrait, SequentialMatrix, SequentialMatrixTrait};

pub trait GenerateFrom<R> {
    fn generate_from(rhs: R) -> Self;
}

macro_rules! impl_generate_from_using_generate_from_iter {
    (
        $lhs_wrapper: ident, $lhs_trait1: ident $(+ $lhs_trait2: ident)*,
        $rhs_wrapper: ident, $rhs_trait1: ident $(+ $rhs_trait2: ident)*
    ) => {
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> GenerateFrom<$rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn generate_from(rhs: $rhs_wrapper<R>) -> Self {
                Self{ object: L::generate_from_iter(rhs.dimension(), rhs.iter())}
            }
        }
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> GenerateFrom<&$rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn generate_from(rhs: &$rhs_wrapper<R>) -> Self {
                Self{ object: L::generate_from_iter(rhs.dimension(), rhs.iter())}
            }
        }
    }
}

impl_generate_from_using_generate_from_iter!(
    SequentialVectorWrapper,
    SequentialMutVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait
);
impl_generate_from_using_generate_from_iter!(
    SequentialVectorWrapper,
    SequentialMutVectorTrait,
    RandomVectorWrapper,
    RandomVectorTrait
);

impl_generate_from_using_generate_from_iter!(
    RandomVectorWrapper,
    RandomMutVectorTrait + SequentialMutVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait
);
impl_generate_from_using_generate_from_iter!(
    RandomVectorWrapper,
    RandomMutVectorTrait + SequentialMutVectorTrait,
    RandomVectorWrapper,
    RandomVectorTrait
);

impl_generate_from_using_generate_from_iter!(
    SequentialMatrix,
    SequentialMutMatrixTrait,
    SequentialMatrix,
    SequentialMatrixTrait
);

impl_generate_from_using_generate_from_iter!(SequentialMatrix, SequentialMutMatrixTrait, RowMatrix, RowMatrixTrait);

impl_generate_from_using_generate_from_iter!(
    SequentialMatrix,
    SequentialMutMatrixTrait,
    ColumnMatrix,
    ColumnMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    SequentialMatrix,
    SequentialMutMatrixTrait,
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    ColumnMatrix,
    ColumnMatrixTrait + SequentialMutMatrixTrait,
    SequentialMatrix,
    SequentialMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    ColumnMatrix,
    ColumnMatrixTrait + SequentialMutMatrixTrait,
    RowMatrix,
    RowMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    ColumnMatrix,
    ColumnMatrixTrait + SequentialMutMatrixTrait,
    ColumnMatrix,
    ColumnMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    ColumnMatrix,
    ColumnMatrixTrait + SequentialMutMatrixTrait,
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    RowMatrix,
    RowMatrixTrait + SequentialMutMatrixTrait,
    SequentialMatrix,
    SequentialMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    RowMatrix,
    RowMatrixTrait + SequentialMutMatrixTrait,
    RowMatrix,
    RowMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    RowMatrix,
    RowMatrixTrait + SequentialMutMatrixTrait,
    ColumnMatrix,
    ColumnMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    RowMatrix,
    RowMatrixTrait + SequentialMutMatrixTrait,
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait + SequentialMutMatrixTrait,
    SequentialMatrix,
    SequentialMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait + SequentialMutMatrixTrait,
    RowMatrix,
    RowMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait + SequentialMutMatrixTrait,
    ColumnMatrix,
    ColumnMatrixTrait
);

impl_generate_from_using_generate_from_iter!(
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait + SequentialMutMatrixTrait,
    BidirectionalMatrix,
    RowMatrixTrait + ColumnMatrixTrait
);

impl<L: RandomMutVectorTrait, R: VectorTrait> GenerateFrom<VectorWrapper<R>> for RandomVectorWrapper<L> {
    fn generate_from(rhs: VectorWrapper<R>) -> Self {
        let mut lhs = L::default();
        rhs.assign_to_random_vector(&mut lhs);
        return Self { object: lhs };
    }
}

impl<L: RandomMutVectorTrait, R: VectorTrait> GenerateFrom<&VectorWrapper<R>> for RandomVectorWrapper<L> {
    fn generate_from(rhs: &VectorWrapper<R>) -> Self {
        let mut lhs = L::default();
        rhs.assign_to_random_vector(&mut lhs);
        return Self { object: lhs };
    }
}
