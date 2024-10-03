use crate::traits::PermutatorTrait;
use crate::types::Direction::{self, COLUMN, ROW};

#[derive(Debug)]
pub struct PermutatedPermutator<P1: PermutatorTrait, P2: PermutatorTrait> {
    permutator1: P1,
    permutator2: P2,
}

impl<P1: PermutatorTrait, P2: PermutatorTrait> PermutatedPermutator<P1, P2> {
    pub fn new(permutator1: P1, permutator2: P2) -> Self {
        assert!(permutator1.dimension::<{ COLUMN }>() == permutator2.dimension::<{ ROW }>());
        Self { permutator1: permutator1, permutator2: permutator2 }
    }
}

impl<P1: PermutatorTrait, P2: PermutatorTrait> PermutatorTrait for PermutatedPermutator<P1, P2> {
    fn dimension<const D: Direction>(&self) -> usize {
        match D {
            ROW => self.permutator1.dimension::<{ ROW }>(),
            COLUMN => self.permutator2.dimension::<{ COLUMN }>(),
        }
    }

    fn permutate(&self, i: usize) -> Option<usize> {
        if let Some(j) = self.permutator2.permutate(i) {
            self.permutator1.permutate(j)
        } else {
            None
        }
    }

    fn unpermutate(&self, i: usize) -> Option<usize> {
        if let Some(j) = self.permutator1.unpermutate(i) {
            self.permutator2.unpermutate(j)
        } else {
            None
        }
    }
}
