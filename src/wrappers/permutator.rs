use crate::{impls, traits::MutPermutatorTrait, PermutatorTrait};

#[derive(Default, Clone, Debug)]
pub struct Permutator<P: PermutatorTrait> {
    pub(crate) object: P,
}

impl<P: MutPermutatorTrait> Permutator<P> {
    #[inline(always)]
    pub fn generate_from_iter<I: Iterator<Item = (usize, usize)>>(dimension: [usize; 2], permutations: I) -> Self {
        Self {object: P::generate_from_iter(dimension, permutations)}
    }
}

impl<P: PermutatorTrait> Permutator<P> {
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn T(&self) -> Permutator<impl PermutatorTrait + '_> {
        Permutator { object: impls::TransposedPermutator::new(&self.object) }
    }
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn into_T(self) -> Permutator<impl PermutatorTrait> {
        Permutator { object: impls::TransposedPermutator::new(self.object) }
    }
}

impl<V: PermutatorTrait> std::ops::Deref for Permutator<V> {
    type Target = V;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<V: PermutatorTrait> std::ops::DerefMut for Permutator<V> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

pub type FullPermutator = Permutator<impls::FullPermutator>;
