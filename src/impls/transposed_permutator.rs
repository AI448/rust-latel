use crate::traits::PermutatorTrait;
use crate::types::Transposed;

#[derive(Debug)]
pub struct TransposedPermutator<P: PermutatorTrait> {
    permutator: P,
}

impl<P: PermutatorTrait> TransposedPermutator<P> {
    pub fn new(permutator: P) -> Self {
        Self { permutator: permutator }
    }
}

impl<P: PermutatorTrait> PermutatorTrait for TransposedPermutator<P> {
    #[inline(always)]
    fn dimension(&self) -> [usize; 2] {
        self.permutator.dimension().transposed()
    }

    #[inline(always)]
    fn permutate(&self, i: usize) -> Option<usize> {
        self.permutator.unpermutate(i)
    }

    #[inline(always)]
    fn unpermutate(&self, i: usize) -> Option<usize> {
        self.permutator.permutate(i)
    }
}
