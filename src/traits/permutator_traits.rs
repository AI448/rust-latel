pub trait PermutatorTrait {
    fn dimension(&self) -> [usize; 2];

    fn permutate(&self, i: usize) -> Option<usize>;

    fn unpermutate(&self, i: usize) -> Option<usize>;

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + Clone + '_;
}

pub trait MutPermutatorTrait: PermutatorTrait + Default {
    #[inline(always)]
    fn generate_from_iter<I: Iterator<Item = (usize, usize)>>(dimension: [usize; 2], permutations: I) -> Self {
        let mut vector = Self::default();
        vector.replace_by_iter(dimension, permutations);
        return vector;
    }

    fn replace_by_iter<I: Iterator<Item = (usize, usize)>>(&mut self, dimension: [usize; 2], permutations: I);
}

impl<P: PermutatorTrait> PermutatorTrait for &P {
    #[inline(always)]
    fn dimension(&self) -> [usize; 2] {
        (*self).dimension()
    }
    #[inline(always)]
    fn permutate(&self, i: usize) -> Option<usize> {
        (*self).permutate(i)
    }
    #[inline(always)]
    fn unpermutate(&self, i: usize) -> Option<usize> {
        (*self).unpermutate(i)
    }
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + Clone + '_ {
        (*self).iter()
    }
}
