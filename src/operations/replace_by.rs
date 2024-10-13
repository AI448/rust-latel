use crate::traits::{
    RandomMutVectorTrait, RandomVectorTrait, SequentialMutVectorTrait, SequentialVectorTrait, VectorTrait,
};
use crate::wrappers::{RandomVectorWrapper, SequentialVectorWrapper, VectorWrapper};

pub trait ReplaceBy<R> {
    fn replace_by(&mut self, rhs: R);
}

macro_rules! impl_replace_by_using_replace_by_iter {
    (
        $lhs_wrapper: ident, $lhs_trait1: ident $(+ $lhs_trait2: ident)*,
        $rhs_wrapper: ident, $rhs_trait1: ident $(+ $rhs_trait2: ident)*
    ) => {
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> ReplaceBy<$rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn replace_by(&mut self, rhs: $rhs_wrapper<R>) {
                self.object.replace_by_iter(rhs.object.dimension(), rhs.object.iter());
            }
        }
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> ReplaceBy<&$rhs_wrapper<R>> for $lhs_wrapper<L> {
            fn replace_by(&mut self, rhs: &$rhs_wrapper<R>) {
                self.object.replace_by_iter(rhs.object.dimension(), rhs.object.iter());
            }
        }
    }
}

impl_replace_by_using_replace_by_iter!(
    SequentialVectorWrapper,
    SequentialMutVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait
);
impl_replace_by_using_replace_by_iter!(
    SequentialVectorWrapper,
    SequentialMutVectorTrait,
    RandomVectorWrapper,
    RandomVectorTrait
);

impl_replace_by_using_replace_by_iter!(
    RandomVectorWrapper,
    RandomMutVectorTrait + SequentialMutVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait
);
impl_replace_by_using_replace_by_iter!(
    RandomVectorWrapper,
    RandomMutVectorTrait + SequentialMutVectorTrait,
    RandomVectorWrapper,
    RandomVectorTrait
);

impl<L: RandomMutVectorTrait, R: VectorTrait> ReplaceBy<VectorWrapper<R>> for RandomVectorWrapper<L> {
    fn replace_by(&mut self, rhs: VectorWrapper<R>) {
        rhs.assign_to_random_vector(&mut self.object);
    }
}

impl<L: RandomMutVectorTrait, R: VectorTrait> ReplaceBy<&VectorWrapper<R>> for RandomVectorWrapper<L> {
    fn replace_by(&mut self, rhs: &VectorWrapper<R>) {
        rhs.assign_to_random_vector(&mut self.object);
    }
}
