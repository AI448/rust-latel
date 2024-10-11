pub trait PermutatorTrait {
    fn dimension(&self) -> [usize; 2];

    fn permutate(&self, i: usize) -> Option<usize>;

    fn unpermutate(&self, i: usize) -> Option<usize>;
}

impl<P: PermutatorTrait> PermutatorTrait for &P {
    fn dimension(&self) -> [usize; 2] {
        (*self).dimension()
    }
    fn permutate(&self, i: usize) -> Option<usize> {
        (*self).permutate(i)
    }
    fn unpermutate(&self, i: usize) -> Option<usize> {
        (*self).unpermutate(i)
    }
}
