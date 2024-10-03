use crate::{impls, PermutatorTrait};

pub struct Permutator<P: PermutatorTrait> {
    pub(crate) object: P,
}

impl<P: PermutatorTrait> Permutator<P> {
    #[allow(non_snake_case)]
    pub fn T(&self) -> Permutator<impl PermutatorTrait + '_> {
        Permutator { object: impls::TransposedPermutator::new(&self.object) }
    }

    #[allow(non_snake_case)]
    pub fn into_T(self) -> Permutator<impl PermutatorTrait> {
        Permutator { object: impls::TransposedPermutator::new(self.object) }
    }
}

impl<V: PermutatorTrait> std::ops::Deref for Permutator<V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<V: PermutatorTrait> std::ops::DerefMut for Permutator<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

pub type FullPermutator = Permutator<impls::FullPermutator>;

impl FullPermutator {
    pub fn new(dimension: [usize; 2]) -> Self {
        Self { object: impls::FullPermutator::new(dimension) }
    }
}
