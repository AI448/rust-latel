use crate::traits::{RandomMutVectorTrait, RandomVectorTrait, SequentialVectorTrait, VectorTrait};
use crate::wrappers::{RandomVectorWrapper, SequentialVectorWrapper, VectorWrapper};

macro_rules! impl_add_assign_to_random_vector {
    (
        $lhs_wrapper: ident, $lhs_trait1: ident $(+ $lhs_trait2: ident)*,
        $rhs_wrapper: ident, $rhs_trait1: ident $(+ $rhs_trait2: ident)*
    ) => {
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::AddAssign<$rhs_wrapper<R>> for $lhs_wrapper<L> {
            #[inline(always)]
            fn add_assign(&mut self, rhs: $rhs_wrapper<R>) {
                rhs.add_to_random_vector(&mut self.object);
            }
        }
        impl<'a, L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::AddAssign<&'a $rhs_wrapper<R>> for $lhs_wrapper<L> {
            #[inline(always)]
            fn add_assign(&mut self, rhs: &'a $rhs_wrapper<R>) {
                rhs.add_to_random_vector(&mut self.object);
            }
        }
        impl<L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::SubAssign<$rhs_wrapper<R>> for $lhs_wrapper<L> {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: $rhs_wrapper<R>) {
                rhs.sub_from_random_vector(&mut self.object);
            }
        }
        impl<'a, L: $lhs_trait1 $(+$lhs_trait2)*, R: $rhs_trait1 $(+$rhs_trait2)*> std::ops::SubAssign<&'a $rhs_wrapper<R>> for $lhs_wrapper<L> {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: &'a $rhs_wrapper<R>) {
                rhs.sub_from_random_vector(&mut self.object);
            }
        }
    };
}

impl_add_assign_to_random_vector!(RandomVectorWrapper, RandomMutVectorTrait, VectorWrapper, VectorTrait);

impl_add_assign_to_random_vector!(
    RandomVectorWrapper,
    RandomMutVectorTrait,
    SequentialVectorWrapper,
    SequentialVectorTrait
);

impl_add_assign_to_random_vector!(RandomVectorWrapper, RandomMutVectorTrait, RandomVectorWrapper, RandomVectorTrait);
