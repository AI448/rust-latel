use crate::traits::{
    RandomMutVectorTrait, RandomVectorTrait, SequentialMutVectorTrait, SequentialVectorTrait, VectorTrait,
};
use crate::wrappers::{RandomVectorWrapper, SequentialVectorWrapper, VectorWrapper};

pub trait GenerateFrom<R> {
    fn generate_from(rhs: R) -> Self;
}

macro_rules! impl_generate_from_using_from_iter {
    (
        $lhs_wrapper: ident, $lhs_trait1: ident $(+ $lhs_trait2: ident)*,
        $rhs_wrapper: ident, $rhs_trait1: ident $(+ $rhs_trait2: ident)*
    ) => {
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> GenerateFrom<$rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn generate_from(rhs: $rhs_wrapper<R>) -> Self {
                Self::from_iter(rhs.dimension(), rhs.iter())
            }
        }
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> GenerateFrom<&$rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn generate_from(rhs: &$rhs_wrapper<R>) -> Self {
                Self::from_iter(rhs.dimension(), rhs.iter())
            }
        }
    }
}

impl_generate_from_using_from_iter!(
    SequentialVectorWrapper,
    SequentialMutVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait
);
impl_generate_from_using_from_iter!(
    SequentialVectorWrapper,
    SequentialMutVectorTrait,
    RandomVectorWrapper,
    RandomVectorTrait
);

impl_generate_from_using_from_iter!(
    RandomVectorWrapper,
    RandomMutVectorTrait + SequentialMutVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait
);
impl_generate_from_using_from_iter!(
    RandomVectorWrapper,
    RandomMutVectorTrait + SequentialMutVectorTrait,
    RandomVectorWrapper,
    RandomVectorTrait
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
