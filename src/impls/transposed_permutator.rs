use crate::traits::PermutatorTrait;
use crate::types::Direction::{self, COLUMN, ROW};

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
    fn dimension<const D: Direction>(&self) -> usize {
        match D {
            ROW => self.permutator.dimension::<{ COLUMN }>(),
            COLUMN => self.permutator.dimension::<{ ROW }>(),
        }
    }

    fn permutate(&self, i: usize) -> Option<usize> {
        self.permutator.unpermutate(i)
    }

    fn unpermutate(&self, i: usize) -> Option<usize> {
        self.permutator.permutate(i)
    }
}
