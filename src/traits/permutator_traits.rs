use crate::types::Direction;

pub trait PermutatorTrait {
    fn dimension<const D: Direction>(&self) -> usize;

    fn permutate(&self, i: usize) -> Option<usize>;

    fn unpermutate(&self, i: usize) -> Option<usize>;
}

impl<P: PermutatorTrait> PermutatorTrait for &P {
    fn dimension<const D: Direction>(&self) -> usize {
        (*self).dimension::<{ D }>()
    }
    fn permutate(&self, i: usize) -> Option<usize> {
        (*self).permutate(i)
    }
    fn unpermutate(&self, i: usize) -> Option<usize> {
        (*self).unpermutate(i)
    }
}
